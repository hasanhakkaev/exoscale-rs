use base64::{engine::general_purpose, Engine as _};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use apis::configuration::Configuration;

use hmac::{Hmac, Mac as _};

use reqwest::{header::HeaderValue, Request};

use sha2::Sha256;

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

/// Sign a request according to the exoscale authentication scheme
pub fn sign_request(
    request: &mut Request,
    configuration: &Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    let api_key = &configuration.api_key;

    let secret = &configuration.api_secret;

    let body = request
        .body()
        .and_then(|x| String::from_utf8_lossy(x.as_bytes().unwrap_or_default()).into());

    writeln!(&mut buffer, "{} {}", request.method(), request.url().path()).unwrap();

    writeln!(&mut buffer, "{}", body.as_deref().unwrap_or("")).unwrap();

    let mut query = request.url().query_pairs().collect::<Vec<_>>();

    query.sort_by(|a, b| a.0.cmp(&b.0));

    let (args, values): (Vec<_>, Vec<_>) = query.into_iter().unzip();

    values
        .iter()
        .try_for_each(|x| write!(&mut buffer, "{} ", x))
        .unwrap();

    writeln!(&mut buffer).unwrap();

    let signed_query_args = if !args.is_empty() {
        format!("signed-query-args={},", args.join(":"))
    } else {
        "".into()
    };

    // XXX: add an empty line for header values
    writeln!(&mut buffer).unwrap();

    let expiration = (SystemTime::now() + configuration.expiration)
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    write!(&mut buffer, "{}", expiration).unwrap();

    println!("{}", buffer);

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;

    mac.update(buffer.as_bytes());

    let signature = mac.finalize().into_bytes();

    let signature = general_purpose::STANDARD.encode(signature);

    let header_value = format!(
        "EXO2-HMAC-SHA256 credential={},{}expires={},signature={}",
        api_key, signed_query_args, expiration, signature,
    );

    let header_value = HeaderValue::from_str(&header_value)?;

    request.headers_mut().insert("Authorization", header_value);

    Ok(())
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    use reqwest::{Method, Url};

    static BASE_URL: &str = "https://api-ch-gva-2.exoscale.com/v2";

    pub fn test_config() -> Configuration {
        Configuration {
            base_path: BASE_URL.parse::<Url>().unwrap().to_string(),
            user_agent: None,
            client: Default::default(),
            api_key: env!("EXOSCALE_API_KEY").into(),
            api_secret: env!("EXOSCALE_API_SECRET").into(),
            expiration: Default::default(),
            zone: env!("EXOSCALE_ZONE").into(),
            content_type: None,
        }
    }

    #[tokio::test]
    async fn sign_request_works() {
        let url: Url = format!("{}/{}", BASE_URL, "instance").parse().unwrap();
        let configuration = test_config();

        let mut request = reqwest::Request::new(Method::GET, url);

        sign_request(&mut request, &configuration).expect("failed to sign request");

        reqwest::Client::new()
            .execute(request)
            .await
            .expect("failed to list instances")
            .error_for_status()
            .expect("failed to list instances");
    }
}
