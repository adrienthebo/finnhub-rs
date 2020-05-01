extern crate clap;
extern crate serde_json;
extern crate tokio;

use clap::{crate_version, Clap};

#[derive(Debug, clap::Clap)]
#[clap(version = crate_version!(), author = "Adrien Thebo <adrien@lagrange-automation.io", after_help = "The market remain irrational longer than you can remain solvent.")]
struct RootCommand {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long, env = "FINNHUB_TOKEN")]
    /// The Finnhub API token.
    token: String,
}

#[derive(Debug, clap::Clap)]
#[clap(name = rename_all, rename_all = "Kebab case")]
enum Commands {
    /// List supported exchanges.
    Exchanges,

    /// Get general news.
    News {
        category: finnhub::NewsCategory,
    },

    /// List supported stocks for an exchange.
    Symbols {
        exchange: finnhub::ExchangeCode,
    },

    Quote {
        symbol: finnhub::Symbol,
    },

    /// News sentiment and statistics for US companies.
    NewsSentiment {
        symbol: finnhub::Symbol,
    },

    /// Get company peers in the same country and GICS sub-industry.
    Peers {
        symbol: finnhub::Symbol,
    },

    /// Get company peers in the same country and GICS sub-industry.
    Executives {
        symbol: finnhub::Symbol,
    },

    /// List latest company news by symbol. This endpoint is only available for US companies.
    CompanyNews {
        symbol: finnhub::Symbol,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = RootCommand::parse();

    // let token = matches.value_of("token").expect("Finnhub API token");
    let client = finnhub::Client::with_token(opts.token);

    match opts.command {
        Commands::Exchanges { .. } => println!("{:#?}", client.exchanges().await?.inner),
        Commands::Symbols { exchange } => {
            println!("{:#?}", client.symbols(exchange).await?.inner);
        }
        Commands::Quote { symbol } => {
            println!("{:#?}", client.quote(symbol).await?.inner);
        }
        Commands::NewsSentiment { symbol } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&client.news_sentiment(symbol).await?.inner).unwrap()
            );
        }
        Commands::Peers { symbol } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&client.peers(symbol).await?.inner).unwrap()
            );
        }
        Commands::Executives { symbol } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&client.executives(symbol).await?.inner).unwrap()
            );
        }
        Commands::News { category } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&client.news(category).await?.inner).unwrap()
            );
        }
        Commands::CompanyNews { symbol } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&client.company_news(symbol).await?.inner).unwrap()
            );
        }
    }

    Ok(())
}
