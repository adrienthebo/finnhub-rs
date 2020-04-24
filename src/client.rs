//! Finnhub Client
//!
//! The Finnhub client provides synchronous and asynchronous access to the Finnhub API.

#[derive(Debug)]
pub struct Client {
    /// The Finnhub API token
    pub token: String,
}
