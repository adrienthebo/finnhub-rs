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

pub type ApiResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl Client {
    const BASEURL: &'static str = "https://finnhub.io/api/v1/";

    pub fn with_token(token: &str) -> Self {
        Self {
            token: token.to_owned().to_string(),
            baseurl: Self::BASEURL.parse().unwrap(),
        }
    }

    pub async fn exchanges(&self) -> ApiResult<Vec<crate::Exchange>> {
        self.get::<Vec<crate::Exchange>>(self.url_for_path("/stock/exchange", None))
            .await
    }

    pub async fn symbols(&self, exchange: crate::ExchangeCode) -> ApiResult<Vec<crate::StockDesc>> {
        self.get::<Vec<crate::StockDesc>>(self.url_for_path(
            "/stock/symbol",
            Some(vec![("exchange", exchange.0.as_ref())]),
        ))
        .await
    }

    pub async fn quote(&self, symbol: crate::Symbol) -> ApiResult<crate::Quote> {
        self.get::<crate::Quote>(
            self.url_for_path("/quote", Some(vec![("symbol", symbol.0.as_ref())])),
        )
        .await
    }

    pub async fn news_sentiment(&self, symbol: crate::Symbol) -> ApiResult<crate::NewsSentiment> {
        self.get::<crate::NewsSentiment>(
            self.url_for_path("/news-sentiment", Some(vec![("symbol", symbol.0.as_ref())])),
        )
        .await
    }

    pub async fn peers(&self, symbol: crate::Symbol) -> ApiResult<Vec<crate::Symbol>> {
        self.get::<Vec<crate::Symbol>>(
            self.url_for_path("/stock/peers", Some(vec![("symbol", symbol.0.as_ref())])),
        )
        .await
    }

    fn url_for_path(&self, path: &str, params: Option<Vec<(&str, &str)>>) -> Url {
        let mut url = self.baseurl.clone();
        {
            let mut segments = url.path_segments_mut().unwrap();
            segments.push(path);
        }
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("token", &self.token);
            if let Some(pairs) = params {
                for pair in pairs {
                    query_pairs.append_pair(pair.0, pair.1);
                }
            }
        }
        url
    }

    async fn get<T>(&self, url: Url) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let response: T = reqwest::get(url).await?.json().await?;
        Ok(response)
    }
}
