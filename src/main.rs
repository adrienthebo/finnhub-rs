extern crate clap;
extern crate tokio;

use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = App::new("Finnhub CLI")
        .version("0.1.0")
        .author("Adrien Thebo <adrien@lagrange-automation.io")
        .about("Interact with the Finnhub API")
        .arg(
            Arg::with_name("token")
                .env("FINNHUB_TOKEN")
                .long("token")
                .value_name("STRING")
                .help("Set the Finnhub API token"),
        )
        .subcommand(SubCommand::with_name("exchanges"))
        .subcommand(
            SubCommand::with_name("symbols").arg(
                Arg::with_name("exchange")
                    .index(1)
                    .required(true)
                    .help("The exchange to query"),
            ),
        )
        .get_matches();

    let token = matches.value_of("token").expect("Finnhub API token");
    let client = finnhub::Client::with_token(token);

    match matches.subcommand() {
        ("exchanges", Some(_)) => println!("{:#?}", client.exchanges().await?),
        ("symbols", Some(matches)) => {
            let exchange_code = finnhub::ExchangeCode(
                matches
                    .value_of("exchange")
                    .expect("Missing exchange code")
                    .to_string(),
            );
            println!("{:#?}", client.symbols(exchange_code).await?);
        }
        //None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    Ok(())
}
