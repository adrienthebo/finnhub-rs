//! Finnhub Client
//!
//! The Finnhub client provides synchronous and asynchronous access to the Finnhub API.

use url::Url;

/// A hyper HTTPS client
type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

pub struct Client {
    /// The Finnhub API token
    token: String,

    /// The Finnhub API base URL
    baseurl: Url,

    /// The hyper HTTPS client
    //client: hyper::Client<hyper_tls::HttpsConnector>,
    client: HttpsClient,
}

impl Client {
    const BASEURL: &'static str = "https://finnhub.io/api/v1/";

    pub fn with_token(token: &str) -> Self {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Self {
            token: token.to_owned().to_string(),
            baseurl: Self::BASEURL.parse().unwrap(),
            client
        }
    }

    pub async fn exchanges(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = self.url_for_path("/stock/exchange");
        let uri = url.to_string().parse().unwrap();
        // Await the response...
        let resp = self.client.get(uri).await?;

        println!("Response: {}", resp.status());
        println!("Headers: {:#?}", resp.headers());

        let body = hyper::body::to_bytes(resp.into_body()).await;

        println!("Body: {:?}", body);
        unimplemented!()
    }

    fn url_for_path(&self, path: &str) -> http::Uri {
        let mut url = self.baseurl.clone();
        {
            let mut segments = url.path_segments_mut().unwrap();
            segments.push(path);
        }
        let query = format!("token={}", self.token);
        url.set_query(Some(query.as_ref()));

        // XXX casting Url -> String -> Uri is not ideal
        url.to_string().parse().unwrap()
    }
}
