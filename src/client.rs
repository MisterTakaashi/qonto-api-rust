const DEFAULT_API_URL: &'static str = "https://thirdparty.qonto.com/";
const DEFAULT_OAUTH_URL: &'static str = "https://oauth.qonto.com/oauth2/";

use curl::{Error, easy::Easy};
use crate::http::HttpResult;

pub struct Client {
    pub base_url: String,
    pub oauth_url: String,
}

trait QontoClient {
    fn get(&self) -> Result<HttpResult, Error>;
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_API_URL.to_string(),
            oauth_url: DEFAULT_OAUTH_URL.to_string(),
        }
    }
}

impl QontoClient for Client {
    fn get(&self) -> Result<HttpResult, Error> {
        let mut easy = Easy::new();
        easy.url(&self.base_url[..]).unwrap();
        let result = easy.perform();

        if result.is_ok() {
            Ok(HttpResult { status_code: easy.response_code().unwrap() })
        } else {
            Err(result.unwrap_err())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{Client, QontoClient, DEFAULT_API_URL, DEFAULT_OAUTH_URL};

    #[test]
    pub fn build_client() {
        let client = Client {
            base_url: "https://example.com/".to_string(),
            oauth_url: "https://oauth.com/".to_string(),
        };

        assert_eq!(client.base_url, "https://example.com/");
        assert_eq!(client.oauth_url, "https://oauth.com/");
    }

    #[test]
    pub fn build_client_with_default() {
        let client = Client::default();

        assert_eq!(client.base_url, DEFAULT_API_URL);
        assert_eq!(client.oauth_url, DEFAULT_OAUTH_URL);
    }

    #[test]
    pub fn make_get_not_found_request() {
        let client = Client::default();

        let result = client.get();
        assert_eq!(result.unwrap().status_code, 404);
    }
}
