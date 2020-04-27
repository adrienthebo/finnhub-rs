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

#[derive(Debug)]
pub struct Ratelimit {
    /// The maximum number of weighted API calls for this time period.
    limit: u32,

    /// The remaining number of weighted API calls for this time period.
    remaining: u32,

    /// The time when the API limit resets.
    ///
    reset: chrono::DateTime<chrono::Utc>,
}

impl Ratelimit {
    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> Option<Self> {
        use chrono::{DateTime, NaiveDateTime, Utc};

        let maybe_limit = headers
            .get("x-ratelimit-limit")
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());
        let maybe_remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());
        let maybe_reset = headers
            .get("x-ratelimit-reset")
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok())
            .and_then(|timestamp| NaiveDateTime::from_timestamp_opt(i64::from(timestamp), 0))
            .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc));

        if let (Some(limit), Some(remaining), Some(reset)) =
            (maybe_limit, maybe_remaining, maybe_reset)
        {
            Some(Self {
                limit,
                remaining,
                reset,
            })
        } else {
            None
        }
    }

    pub fn till_reset(&self) -> chrono::Duration {
        self.reset - chrono::offset::Utc::now()
    }
}

pub struct ApiCall<T> {
    pub ratelimit: Option<Ratelimit>,
    pub inner: T,
}

pub type ApiResult<T> = Result<ApiCall<T>, Box<dyn std::error::Error + Send + Sync>>;

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

    pub async fn executives(&self, symbol: crate::Symbol) -> ApiResult<Vec<crate::Executive>> {
        use serde_json::Value;

        self.get_with::<Vec<crate::Executive>, _>(
            self.url_for_path(
                "/stock/executive",
                Some(vec![("symbol", symbol.0.as_ref())]),
            ),
            |value: Value| {
                value
                    .as_object()
                    .and_then(|m| m.get("executive"))
                    .ok_or_else(|| Box::from(DeserializeError::new("JSON missing key 'executive'")))
                    // XXX: probably unnecessary clone
                    .map(|v| Value::from(v.clone()))
                    .and_then(|v| {
                        serde_json::value::from_value::<Vec<crate::Executive>>(v)
                            .map_err(|e| Box::from(e))
                    })
            },
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

    /// Fetch the given `url` and deserialize the object into T.
    async fn get<T>(&self, url: Url) -> ApiResult<T>
    where
        for<'de> T: serde::Deserialize<'de> + std::fmt::Debug,
        T: std::fmt::Debug,
    {
        self.get_with(url, |value: serde_json::Value| {
            serde_json::from_value::<T>(value).map_err(|e| Box::from(e))
        })
        .await
    }

    /// Fetch the given `url` and deserialize the object with the given fn.
    async fn get_with<T, F>(&self, url: Url, f: F) -> ApiResult<T>
    where
        F: FnOnce(serde_json::Value) -> Result<T, Box<dyn std::error::Error + Send + Sync>>,
        for<'de> T: serde::Deserialize<'de> + std::fmt::Debug,
        T: std::fmt::Debug,
    {
        //let duplicate = reqwest::get(url.clone()).await?;
        //dbg!(duplicate.text().await?);

        let response = reqwest::get(url).await?;
        let ratelimit = Ratelimit::from_headers(&response.headers());
        eprintln!("ratelimit={:#?}", ratelimit);
        response
            .json()
            .await
            .map_err(|err| err.into())
            .and_then(|body| f(body))
            .map(|inner| ApiCall { ratelimit, inner })
    }
}

#[derive(Debug)]
pub struct DeserializeError<'a> {
    msg: std::borrow::Cow<'a, str>,
}

impl<'a> DeserializeError<'a> {
    pub fn new<T>(m: T) -> DeserializeError<'a>
    where
        T: Into<std::borrow::Cow<'a, str>>,
    {
        Self { msg: m.into() }
    }
}

impl<'a> std::fmt::Display for DeserializeError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> std::error::Error for DeserializeError<'a> {}
