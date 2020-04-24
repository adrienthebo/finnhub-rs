extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate http;
extern crate url;

mod client;

pub use client::Client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Exchange {
    /// The stock exchange (eg "US", "VN")
    code: String,
    /// The currency used at the exchange (eg "USD", "VND")
    currency: String,
    /// The full exchange name ("US exchanges", "HSX and HOSE")
    name: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
