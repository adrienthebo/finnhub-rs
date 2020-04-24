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
pub struct StockSymbol {
    /// Symbol description
    description: String,

    /// Display symbol name.
    #[serde(rename = "displaySymbol")]
    display_symbol: String,

    /// Unique symbol used to identify this symbol used in /stock/candle endpoint.
    symbol: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
