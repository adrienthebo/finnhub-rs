extern crate http;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

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

pub struct ExchangeCode(pub String);

/// Supported stock symbol
///
/// Definition: https://finnhub.io/docs/api#stock-symbols
#[derive(Debug, Deserialize, Serialize)]
pub struct StockDesc {
    /// Symbol description
    description: String,

    /// Display symbol name.
    #[serde(rename = "displaySymbol")]
    display_symbol: String,

    /// Unique symbol used to identify this symbol used in /stock/candle endpoint.
    symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol(pub String);

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
pub struct NewsSentiment {
    /// Statistics of company news in the past week.
    pub buzz: Option<Buzz>,

    /// News score.
    #[serde(rename = "companyNewsScore:")]
    pub company_news_score: Option<f32>,

    /// Sector average bullish percent.
    #[serde(rename = "sectorAverageBullishPercent")]
    pub sector_average_bullish_percent: Option<f32>,

    /// Sector average score.
    #[serde(rename = "sectorAverageNewsScore")]
    pub sector_average_news_score: Option<f32>,

    /// Sentiment
    pub sentiment: Option<Sentiment>,

    /// TODO: convert this to a crate::Symbol
    symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Buzz {
    #[serde(rename = "articlesInLastWeek")]
    articles_in_last_week: Option<f32>,

    buzz: Option<f32>,

    #[serde(rename = "weeklyAverage")]
    weekly_average: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sentiment {
    #[serde(rename = "bearishPercent")]
    bearish_percent: Option<f32>,

    #[serde(rename = "bullishPercent")]
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
    image: url::Url,
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

impl std::string::ToString for NewsCategory {
    fn to_string(&self) -> String {
        match self {
            NewsCategory::General => "general",
            NewsCategory::Forex => "forex",
            NewsCategory::Crypto => "crypto",
            NewsCategory::Merger => "Merger",
        }
        .into()
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
