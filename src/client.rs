//! Finnhub Client
//!
//! The Finnhub client provides synchronous and asynchronous access to the Finnhub API.

use url::Url;

pub struct Client {
    /// The Finnhub API token
    token: String,

    /// The Finnhub API base URL
    baseurl: Url,
}

impl Client {
    const BASEURL: &'static str = "https://finnhub.io/api/v1/";

    pub fn with_token(token: &str) -> Self {
        Self {
            token: token.to_owned().to_string(),
            baseurl: Self::BASEURL.parse().unwrap(),
        }
    }

    pub async fn exchanges(&self) -> Result<Vec<crate::Exchange>, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.url_for_path("/stock/exchange");
        let exchanges: Vec<crate::Exchange> = reqwest::get(url).await?.json().await?;
        Ok(exchanges)
    }

    fn url_for_path(&self, path: &str) -> Url {
        let mut url = self.baseurl.clone();
        {
            let mut segments = url.path_segments_mut().unwrap();
            segments.push(path);
        }
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("token", &self.token);
        }
        url
    }
}
