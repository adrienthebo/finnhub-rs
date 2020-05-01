mod client;

pub use client::Client;

use serde::{Deserialize, Serialize};

/// Supported stock exchanges
///
/// Definition: https://finnhub.io/docs/api#stock-exchanges
#[derive(Debug, Deserialize, Serialize)]
pub struct Exchange {
    /// The stock exchange (eg "US", "VN")
    code: String,
    /// The currency used at the exchange (eg "USD", "VND")
    currency: String,
    /// The full exchange name ("US exchanges", "HSX and HOSE")
    name: String,
}

#[derive(Debug)]
pub struct ExchangeCode(pub String);

impl std::str::FromStr for ExchangeCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

/// Supported stock symbol
///
/// Definition: https://finnhub.io/docs/api#stock-symbols
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockDesc {
    /// Symbol description
    description: String,

    /// Display symbol name.
    display_symbol: String,

    /// Unique symbol used to identify this symbol used in /stock/candle endpoint.
    symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol(pub String);

impl std::str::FromStr for Symbol {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

/// Get quote data. Constant polling is not recommended. Use websocket if you need real-time
/// update.
///
/// Definition: https://finnhub.io/docs/api#quote
#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    /// Open price of the day
    #[serde(rename = "o")]
    open: f32,

    /// High price of the day
    #[serde(rename = "h")]
    high: f32,

    /// Low price of the day
    #[serde(rename = "l")]
    low: f32,

    /// Current price
    #[serde(rename = "c")]
    current: f32,

    /// Previous close price
    #[serde(rename = "pc")]
    previous_close: f32,
}

/// Get company's news sentiment and statistics. This endpoint is only available for US companies.
///
/// Definition: https://finnhub.io/docs/api#news-sentiment
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsSentiment {
    /// Statistics of company news in the past week.
    pub buzz: Option<Buzz>,

    /// News score.
    pub company_news_score: Option<f32>,

    /// Sector average bullish percent.
    pub sector_average_bullish_percent: Option<f32>,

    /// Sector average score.
    pub sector_average_news_score: Option<f32>,

    /// Sentiment
    pub sentiment: Option<Sentiment>,

    /// TODO: convert this to a crate::Symbol
    symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Buzz {
    articles_in_last_week: Option<f32>,

    buzz: Option<f32>,

    weekly_average: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiment {
    bearish_percent: Option<f32>,
    bullish_percent: Option<f32>,
}

/// Get a list of company's executives and members of the Board.
///
/// Definition: https://finnhub.io/docs/api#company-executive
#[derive(Debug, Deserialize, Serialize)]
pub struct Executive {
    age: Option<u8>,
    compensation: Option<f32>,
    currency: String,
    name: String,
    position: String,
    sex: String,

    // TODO: chrono::Date
    since: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsArticle {
    category: String,
    datetime: u32,
    headline: String,
    id: u32,
    //image: Option<url::Url>,
    related: String,
    source: String,
    summary: String,
    url: url::Url,
}

#[derive(Debug)]
pub enum NewsCategory {
    General,
    Forex,
    Crypto,
    Merger,
}

impl NewsCategory {
    pub fn as_str(&self) -> &str {
        match self {
            NewsCategory::General => "general",
            NewsCategory::Forex => "forex",
            NewsCategory::Crypto => "crypto",
            NewsCategory::Merger => "Merger",
        }
    }
}

impl std::string::ToString for NewsCategory {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Debug)]
pub struct ParseNewsCategoryError;

impl std::fmt::Display for ParseNewsCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "not a valid news category")
    }
}

impl std::str::FromStr for NewsCategory {
    type Err = ParseNewsCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "general" => Ok(NewsCategory::General),
            "forex" => Ok(NewsCategory::Forex),
            "crypto" => Ok(NewsCategory::Crypto),
            "merger" => Ok(NewsCategory::Merger),
            _ => Err(ParseNewsCategoryError),
        }
    }
}

/// Latest price target consensus.
///
/// Definition: https://finnhub.io/docs/api#price-target
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTarget {
    // "lastUpdated": "2019-06-03 00:00:00",
    // @see https://serde.rs/custom-date-format.html
    pub last_updated: String,

    symbol: Symbol,
    target_high: f32,
    target_low: f32,
    target_mean: f32,
    target_median: f32,
}
