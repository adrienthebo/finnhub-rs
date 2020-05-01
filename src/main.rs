use clap::{crate_version, Clap};

#[derive(Debug, clap::Clap)]
#[clap(version = crate_version!(), author = "Adrien Thebo <adrien@lagrange-automation.io", after_help = "The market remain irrational longer than you can remain solvent.")]
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

    /// List latest company news by symbol. (US companies only.)
    CompanyNews {
        symbol: finnhub::Symbol,
    },
}

impl Command {
    pub async fn call(
        &self,
        client: &finnhub::Client<'_>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Command::Exchanges => client
                .exchanges()
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::Symbols { exchange } => client
                .symbols(exchange)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::Quote { symbol } => client
                .quote(symbol)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::NewsSentiment { symbol } => client
                .news_sentiment(symbol)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::Peers { symbol } => client
                .peers(symbol)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::Executives { symbol } => client
                .executives(symbol)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::News { category } => client
                .news(category)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
            Command::CompanyNews { symbol } => client
                .company_news(symbol)
                .await
                .map(|v| serde_json::value::to_value(v.inner).unwrap()),
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
