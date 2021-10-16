const DEFAULT_API_URL: &'static str = "https://thirdparty.qonto.com/";
const DEFAULT_OAUTH_URL: &'static str = "https://oauth.qonto.com/oauth2/";

use curl::{Error, easy::Easy};

pub struct Client {
    pub base_url: String,
    pub oauth_url: String,

}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_API_URL.to_string(),
            oauth_url: DEFAULT_OAUTH_URL.to_string(),
        }
    }
}

impl Client {
    pub fn get(&self) -> Result<u32, Error> {
        let mut easy = Easy::new();
        easy.url(&self.base_url[..]).unwrap();
        easy.perform().unwrap();

        easy.response_code()
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{Client, DEFAULT_API_URL, DEFAULT_OAUTH_URL};

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
    pub fn make_get_request() {
        let client = Client::default();

        let result = client.get();
        assert_eq!(result.unwrap(), 404);
    }
}
