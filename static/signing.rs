use crate::apis::configuration::InnerConfig;
use base64::{engine::general_purpose::STANDARD as Base64Standard, Engine as _};
use hmac::{Hmac, KeyInit, Mac};
use http::HeaderValue;
use reqwest::{Method, Url};
use secrecy::ExposeSecret;
use sha2::Sha256;
use std::fmt::Write as FmtWrite;
use std::os::unix::raw::pthread_t;
// Alias to avoid conflict with std::io::Write
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates the Exoscale V2 Authorization header.
///
/// This function constructs the string to sign according to Exoscale's V2 specifications,
/// computes the HMAC-SHA256 signature, and formats the final Authorization header.
///
/// # Arguments
/// * `method` - The HTTP method of the request.
/// * `url` - The full `reqwest::Url` of the request.
/// * `body_bytes_for_signing` - Optional raw bytes of the request body.
///   Exoscale expects the body to be treated as a UTF-8 string for signing.
/// * `config` - A reference to `InnerConfig` containing API key, secret, and expiration.
///
/// # Errors
/// Returns a `Box<dyn std::error::Error>` if any step in the signing process fails,
/// including UTF-8 conversion of the body, HMAC computation, or header formatting.
pub(crate) fn generate_exoscale_auth_header(
    method: &Method,
    url: &Url,
    body_bytes_for_signing: Option<&[u8]>,
    config: &InnerConfig,
) -> Result<HeaderValue, Box<dyn std::error::Error>> {
    let mut string_to_sign = String::new();

    // 1. HTTP Method and Path (path should not include query string)
    writeln!(&mut string_to_sign, "{} {}", method, url.path())?;

    // 2. Request Body (as a UTF-8 string)
    // If body_bytes_for_signing is None or empty, an empty string is used.
    // Exoscale expects the body to be treated as a UTF-8 string.
    let body_str = match body_bytes_for_signing {
        Some(bytes) if !bytes.is_empty() => std::str::from_utf8(bytes)?,
        _ => "",
    };
    writeln!(&mut string_to_sign, "{}", body_str)?;

    // 3. Canonicalized Query Parameters
    //    - Sort query parameters alphabetically by key.
    //    - URL-decode keys and values before using them (reqwest::Url::query_pairs() does this).
    //    - Concatenate sorted, decoded values with a single space.
    let mut query_pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    query_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let (sorted_query_keys, sorted_query_values): (Vec<String>, Vec<String>) =
        query_pairs.into_iter().unzip();

    let query_values_string = sorted_query_values.join(" "); // Space-separated values
    writeln!(&mut string_to_sign, "{}", query_values_string)?;

    // 4. Empty line for (other) signed headers
    //    Exoscale V2 signing, as reflected in your original code and common patterns,
    //    uses an empty line here if no other specific headers are part of the signature string itself.
    //    `signed-query-args` is part of the auth header, not this part of the string-to-sign.
    writeln!(&mut string_to_sign)?;

    // 5. Expiration Timestamp (Unix timestamp in seconds)
    let expiration_secs = (SystemTime::now() + config.expiration)
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    // No newline after the expiration timestamp in the string-to-sign
    write!(&mut string_to_sign, "{}", expiration_secs)?;

    // --- Create Signature ---
    let mut mac = Hmac::<Sha256>::new_from_slice(config.api_secret.expose_secret().as_bytes())
        .map_err(|e| format!("Failed to initialize HMAC: {}", e))?; // Convert InvalidLength to Error
    mac.update(string_to_sign.as_bytes());
    let signature_bytes = mac.finalize().into_bytes();
    let signature_encoded = Base64Standard.encode(signature_bytes);

    // --- Prepare the ` signed-query-args ` component for the Authorization header ---
    // This lists the sorted query parameter *keys*.
    let signed_query_args_component = if !sorted_query_keys.is_empty() {
        format!("signed-query-args={},", sorted_query_keys.join(":"))
    } else {
        String::new() // No "signed-query-args" component if no query params
    };

    // --- Construct final Authorization header string ---
    let auth_header_str = format!(
        "EXO2-HMAC-SHA256 credential={},{}expires={},signature={}",
        config.api_key.expose_secret(),
        signed_query_args_component,
        expiration_secs,
        signature_encoded
    );

    HeaderValue::from_str(&auth_header_str).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apis::configuration::InnerConfig;
    use reqwest::Method;
    use secrecy::SecretString;
    use std::time::Duration;
    // We'll need a way to control time for reproducible signatures in tests.
    // For now, tests will have varying signatures due to SystemTime::now().
    // Consider using a library like `faketime` or passing `expiration_secs` directly
    // to `generate_exoscale_auth_header` during tests for precise signature matching.

    fn create_test_config() -> InnerConfig {
        InnerConfig {
            api_key: SecretString::new(Box::from("test_api_key".to_string())),
            api_secret: SecretString::new(Box::from("test_api_secret".to_string())),
            expiration: Duration::from_secs(600),
        }
    }

    #[test]
    fn test_signing_get_no_query_no_body() -> Result<(), Box<dyn std::error::Error>> {
        let config = create_test_config();
        let method = Method::GET;
        let url = Url::parse("https://api-test.exoscale.com/v2/instance")?;

        // For testing, we need a fixed expiration time to get a predictable signature
        // This requires either mocking SystemTime or modifying generate_exoscale_auth_header
        // to accept a pre-calculated expiration_secs for testing.
        // Let's assume for now we check the structure, not the exact signature.

        let header = generate_exoscale_auth_header(&method, &url, None, &config)?;
        let header_str = header.to_str()?;

        assert!(header_str.starts_with("EXO2-HMAC-SHA256 credential=test_api_key,"));
        assert!(header_str.contains("expires="));
        assert!(header_str.contains("signature="));
        assert!(!header_str.contains("signed-query-args=")); // No query args

        // To test the string_to_sign (requires exposing it or testing components):
        // METHOD PATH\n
        // \n (empty body)
        // \n (empty query values)
        // \n (empty line for headers)
        // TIMESTAMP
        // Example expected string_to_sign format (timestamp will vary):
        // "GET /v2/instance\n\n\n\n<timestamp>"

        Ok(())
    }

    #[test]
    fn test_signing_post_with_json_body() -> Result<(), Box<dyn std::error::Error>> {
        let config = create_test_config();
        let method = Method::POST;
        let url = Url::parse("https://api-test.exoscale.com/v2/instance")?;
        let body_json = r#"{"name":"test-instance","type":"medium"}"#;
        let body_bytes = body_json.as_bytes();

        let header = generate_exoscale_auth_header(&method, &url, Some(body_bytes), &config)?;
        let header_str = header.to_str()?;

        assert!(header_str.starts_with("EXO2-HMAC-SHA256 credential=test_api_key,"));
        assert!(!header_str.contains("signed-query-args="));

        // Expected string_to_sign format (timestamp will vary):
        // "POST /v2/instance\n{\"name\":\"test-instance\",\"type\":\"medium\"}\n\n\n<timestamp>"
        Ok(())
    }

    #[test]
    fn test_signing_get_with_query_params() -> Result<(), Box<dyn std::error::Error>> {
        let config = create_test_config();
        let method = Method::GET;
        // Params: zone=alpha, name=my-instance (sorted: name, zone)
        let url =
            Url::parse("https://api-test.exoscale.com/v2/instance?zone=alpha&name=my-instance")?;

        let header = generate_exoscale_auth_header(&method, &url, None, &config)?;
        let header_str = header.to_str()?;

        assert!(header_str.contains("signed-query-args=name:zone,"));

        // Expected string_to_sign query values part: "my-instance alpha" (sorted by key "name", then "zone")
        // Full string_to_sign format (timestamp will vary):
        // "GET /v2/instance\n\nmy-instance alpha\n\n<timestamp>"
        Ok(())
    }

    #[test]
    fn test_signing_query_param_ordering_and_values() -> Result<(), Box<dyn std::error::Error>> {
        let config = create_test_config();
        let method = Method::GET;
        // Params: c=3, a=1, b=2 (sorted: a, b, c)
        let url = Url::parse("https://api-test.exoscale.com/v2/instance?c=3&a=1&b=2")?;

        let header = generate_exoscale_auth_header(&method, &url, None, &config)?;
        let header_str = header.to_str()?;

        assert!(header_str.contains("signed-query-args=a:b:c,"));

        // To verify the exact query_values_string component of string_to_sign ("1 2 3"),
        // you would need to either:
        // 1. Modify generate_exoscale_auth_header to return the string_to_sign for tests.
        // 2. Reconstruct the string_to_sign within the test (less ideal as it duplicates logic).
        // For now, we trust the implementation based on the `signed-query-args` output.
        Ok(())
    }

    #[test]
    fn test_signing_empty_body_string() -> Result<(), Box<dyn std::error::Error>> {
        let config = create_test_config();
        let method = Method::POST;
        let url = Url::parse("https://api-test.exoscale.com/v2/instance")?;
        let body_bytes = "".as_bytes(); // Empty string body

        let header = generate_exoscale_auth_header(&method, &url, Some(body_bytes), &config)?;
        let header_str = header.to_str()?;
        assert!(header_str.starts_with("EXO2-HMAC-SHA256 credential=test_api_key,"));
        // Expected string_to_sign (timestamp will vary):
        // "POST /v2/instance\n\n\n\n<timestamp>" (Body is empty string, so \n after it)
        Ok(())
    }
}
