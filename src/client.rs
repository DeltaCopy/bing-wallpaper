use isahc::{config::RedirectPolicy, prelude::*, HttpClient};
use std::time::Duration;
const TIMEOUT: Duration = Duration::from_secs(10);

// create a new http client instance
pub fn build_http_client() -> Result<HttpClient, isahc::Error> {
    let http_client = HttpClient::builder()
        .default_headers(&[(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0",
        )])
        .timeout(TIMEOUT)
        .redirect_policy(RedirectPolicy::Follow)
        .build()?;

    Ok(http_client)
}
