extern crate hyper;
extern crate tokio;
extern crate clap;

use clap::{Arg, App, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = App::new("Finnhub CLI")
        .version("0.1.0")
        .author("Adrien Thebo <adrien@lagrange-automation.io")
        .about("Interact with the Finnhub API")
        .arg(Arg::with_name("token")
            .env("FINNHUB_TOKEN")
            .long("token")
            .value_name("STRING")
            .help("Set the Finnhub API token"))
        .get_matches();

    let token = matches.value_of("token").expect("Finnhub API token").to_string();
    let client = finnhub::client::Client { token };
    println!("client={:?}", client);

    Ok(())
}
