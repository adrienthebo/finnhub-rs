extern crate clap;
extern crate serde_json;
extern crate tokio;

use clap::{App, Arg, SubCommand, crate_version};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = App::new("Finnhub")
        .version(crate_version!())
        .author("Adrien Thebo <adrien@lagrange-automation.io")
        .about("Interact with the Finnhub API")
        .after_help("The market can stay irrational longer than you can remain solvent.")
        .arg(
            Arg::with_name("token")
                .env("FINNHUB_TOKEN")
                .long("token")
                .value_name("STRING")
                .required(true)
                .help("Set the Finnhub API token"),
        )
        .subcommand(SubCommand::with_name("exchanges").about("List supported exchanges."))
        .subcommand(
            SubCommand::with_name("symbols")
                .about("List supported stocks for an exchange.")
                .arg(
                    Arg::with_name("exchange")
                        .index(1)
                        .required(true)
                        .help("The exchange to query"),
                ),
        )
        .subcommand(
            SubCommand::with_name("quote")
                .about("Get quote data. Constant polling is not recommended.")
                .arg(
                    Arg::with_name("symbol")
                        .index(1)
                        .required(true)
                        .help("The stock symbol to quote"),
                ),
        )
        .subcommand(
            SubCommand::with_name("news-sentiment")
                .about("Get company's news sentiment and statistics for US companies.")
                .arg(
                    Arg::with_name("symbol")
                        .index(1)
                        .required(true)
                        .help("The stock symbol to quote"),
                ),
        )
        .subcommand(
            SubCommand::with_name("peers")
                .about("Get company peers in the same country and GICS sub-industry.")
                .arg(
                    Arg::with_name("symbol")
                        .index(1)
                        .required(true)
                        .help("The stock symbol to quote"),
                ),
        )
        .subcommand(
            SubCommand::with_name("executives")
                .about("Get company peers in the same country and GICS sub-industry.")
                .arg(
                    Arg::with_name("symbol")
                        .index(1)
                        .required(true)
                        .help("The company stock symbol"),
                ),
        )
        .subcommand(
            SubCommand::with_name("news").about("Get general news").arg(
                Arg::with_name("category")
                    .index(1)
                    .required(true)
                    .help("TODO"),
            ),
        )
        .subcommand(
            SubCommand::with_name("company-news")
                .about("List latest company news by symbol. This endpoint is only available for US companies.")
                .arg(
                    Arg::with_name("symbol")
                        .index(1)
                        .required(true)
                        .help("The company stock symbol"),
                ),
        )
        .get_matches();

    let token = matches.value_of("token").expect("Finnhub API token");
    let client = finnhub::Client::with_token(token);

    match matches.subcommand() {
        ("exchanges", Some(_)) => println!("{:#?}", client.exchanges().await?.inner),
        ("symbols", Some(matches)) => {
            let exchange_code = finnhub::ExchangeCode(
                matches
                    .value_of("exchange")
                    .expect("Missing exchange code")
                    .to_string(),
            );
            println!("{:#?}", client.symbols(exchange_code).await?.inner);
        }
        ("quote", Some(matches)) => {
            let stock_code = finnhub::Symbol(
                matches
                    .value_of("symbol")
                    .expect("Missing stock code")
                    .to_string(),
            );
            println!("{:#?}", client.quote(stock_code).await?.inner);
        }
        ("news-sentiment", Some(matches)) => {
            let stock_code = finnhub::Symbol(
                matches
                    .value_of("symbol")
                    .expect("Missing stock code")
                    .to_string(),
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&client.news_sentiment(stock_code).await?.inner)
                    .unwrap()
            );
        }
        ("peers", Some(matches)) => {
            let stock_code = finnhub::Symbol(
                matches
                    .value_of("symbol")
                    .expect("Missing stock code")
                    .to_string(),
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&client.peers(stock_code).await?.inner).unwrap()
            );
        }
        ("executives", Some(matches)) => {
            let stock_code = finnhub::Symbol(
                matches
                    .value_of("symbol")
                    .expect("Missing stock code")
                    .to_string(),
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&client.executives(stock_code).await?.inner).unwrap()
            );
        }
        ("news", Some(matches)) => {
            let category: finnhub::NewsCategory = matches
                .value_of("category")
                .expect("missing category")
                .parse()
                .expect("news category");
            println!(
                "{}",
                serde_json::to_string_pretty(&client.news(category).await?.inner).unwrap()
            );
        }
        ("company-news", Some(matches)) => {
            let stock_code = finnhub::Symbol(
                matches
                    .value_of("symbol")
                    .expect("Missing stock code")
                    .to_string(),
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&client.company_news(stock_code).await?.inner).unwrap()
            );
        }
        //None => println!("No subcommand was used"),
        ("", _) => println!("No subcommand given"),
        (unknown, _) => println!("Unhandled: {}", unknown),
    }

    Ok(())
}
