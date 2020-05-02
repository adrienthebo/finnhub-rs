use clap::{crate_version, Clap};

#[derive(Debug, clap::Clap)]
#[clap(version = crate_version!(), author = "Adrien Thebo <adrien@lagrange-automation.io>", after_help = "The market remain irrational longer than you can remain solvent.")]
struct RootCommand {
    #[clap(subcommand)]
    command: Command,

    #[clap(long, env = "FINNHUB_TOKEN")]
    /// The Finnhub API token.
    token: String,
}

#[derive(Debug, clap::Clap)]
#[clap(name = rename_all, rename_all = "kebab-case")]
enum Command {
    /// List supported exchanges.
    Exchanges,

    /// Get general news.
    News { category: finnhub::NewsCategory },

    /// List supported stocks for an exchange.
    Symbols { exchange: finnhub::ExchangeCode },

    /// Get quote data. Constant polling is not recommended.
    Quote { symbol: finnhub::Symbol },

    /// News sentiment and statistics for US companies.
    NewsSentiment { symbol: finnhub::Symbol },

    /// Get company peers in the same country and GICS sub-industry.
    Peers { symbol: finnhub::Symbol },

    /// Get a list of company's executives and members of the Board.
    Executives { symbol: finnhub::Symbol },

    /// List latest company news by symbol. (US companies only.)
    CompanyNews { symbol: finnhub::Symbol },

    /// Get latest price target consensus.
    PriceTarget { symbol: finnhub::Symbol },

    /// Get latest analyst recommendation trends for a company.
    PriceRecommendation { symbol: finnhub::Symbol },
}

macro_rules! async_to_json {
    ($e:expr) => {
        $e.await
            .and_then(|v| serde_json::value::to_value(v.inner).map_err(Box::from))
    };
}

impl Command {
    pub async fn call(
        &self,
        client: &finnhub::Client<'_>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Command::Exchanges => async_to_json!(client.exchanges()),
            Command::Symbols { exchange } => async_to_json!(client.symbols(exchange)),
            Command::Quote { symbol } => async_to_json!(client.quote(symbol)),
            Command::NewsSentiment { symbol } => async_to_json!(client.news_sentiment(symbol)),
            Command::Peers { symbol } => async_to_json!(client.peers(symbol)),
            Command::Executives { symbol } => async_to_json!(client.executives(symbol)),
            Command::News { category } => async_to_json!(client.news(category)),
            Command::CompanyNews { symbol } => async_to_json!(client.company_news(symbol)),
            Command::PriceTarget { symbol } => async_to_json!(client.price_target(symbol)),
            Command::PriceRecommendation { symbol } => {
                async_to_json!(client.price_recommendation(symbol))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = RootCommand::parse();

    // let token = matches.value_of("token").expect("Finnhub API token");
    let client = finnhub::Client::with_token(opts.token);

    let value = opts.command.call(&client).await?;
    println!("{}", serde_json::to_string_pretty(&value)?);

    Ok(())
}
