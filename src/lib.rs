extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate http;
extern crate url;

mod client;

pub use client::Client;

use serde::{Deserialize, Serialize};

/// A supported stock exchanges
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
