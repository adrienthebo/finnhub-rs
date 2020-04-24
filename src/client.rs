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
        let url = self.url_for_path("/stock/exchange", None);
        let exchanges: Vec<crate::Exchange> = reqwest::get(url).await?.json().await?;
        Ok(exchanges)
    }

    pub async fn symbols(&self, exchange: crate::ExchangeCode) -> Result<Vec<crate::StockDesc>, Box<dyn std::error::Error + Send + Sync>> {
        let params = vec![("exchange", exchange.0.as_ref())];
        let url = self.url_for_path("/stock/symbol", Some(params));
        let exchanges: Vec<crate::StockDesc> = reqwest::get(url).await?.json().await?;
        Ok(exchanges)
    }

    pub async fn quote(&self, symbol: crate::Symbol) -> Result<crate::Quote, Box<dyn std::error::Error + Send + Sync>> {
        let params = vec![("symbol", symbol.0.as_ref())];
        let url = self.url_for_path("/quote", Some(params));
        let quote: crate::Quote = reqwest::get(url).await?.json().await?;
        Ok(quote)
    }

    pub async fn news_sentiment(&self, symbol: crate::Symbol) -> Result<crate::NewsSentiment, Box<dyn std::error::Error + Send + Sync>> {
        let params = vec![("symbol", symbol.0.as_ref())];
        let url = self.url_for_path("/news-sentiment", Some(params));
        let ns: crate::NewsSentiment = reqwest::get(url).await?.json().await?;
        Ok(ns)
    }

    pub async fn peers(&self, symbol: crate::Symbol) -> Result<Vec<crate::Symbol>, Box<dyn std::error::Error + Send + Sync>> {
        let params = vec![("symbol", symbol.0.as_ref())];
        let url = self.url_for_path("/stock/peers", Some(params));
        let peers: Vec<crate::Symbol> = reqwest::get(url).await?.json().await?;
        Ok(peers)
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
}
