use crate::apis::configuration::InnerConfig;
use crate::signing::generate_exoscale_auth_header;
use async_trait::async_trait;
use bytes::Bytes;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Error as MiddlewareError, Middleware, Next};
use std::fmt;
use std::sync::Arc;

// Custom error type that is Send and Sync
#[derive(Debug)]
struct CustomMiddlewareError(String);

impl fmt::Display for CustomMiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomMiddlewareError {}

/// New type wrapper for storing original body bytes in request extensions.
#[derive(Clone)]
pub struct OriginalBodyBytes(pub Bytes);

pub(crate) struct ExoscaleAuthMiddleware {
    config: Arc<InnerConfig>,
}

impl ExoscaleAuthMiddleware {
    pub fn new(config: Arc<InnerConfig>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Middleware for ExoscaleAuthMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response, MiddlewareError> {
        // 1. Retrieve pre-serialized body bytes from extensions (if using that strategy)
        //    The API call methods should place OriginalBodyBytes into extensions if the body is present.
        let body_for_signing: Option<Bytes> = extensions
            .get::<OriginalBodyBytes>()
            .map(|obb| obb.0.clone());

        // 2. Call the signing function
        let auth_header = generate_exoscale_auth_header(
            req.method(),
            req.url(),
            body_for_signing.as_ref().map(|b| b.as_ref()), // Convert Option<Bytes> to Option<&[u8]>
            &self.config,                                  // Pass reference to InnerConfig
        )
        .map_err(|e| {
            // You might want to log the error here using the `log` crate
            // log::error!("Error during Exoscale request signing: {:?}", e);
            let error_message = e.to_string(); // Convert the original error to a String
            MiddlewareError::middleware(CustomMiddlewareError(error_message)) // Wrap it in our Send and Sync error
        })?;

        req.headers_mut()
            .insert(reqwest::header::AUTHORIZATION, auth_header.clone());

        next.run(req, extensions).await
    }
}

impl Clone for ExoscaleAuthMiddleware {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
        }
    }
}
