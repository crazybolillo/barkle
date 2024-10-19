mod app;

use crate::app::App;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Database file to be used. Created if it doesn't exist
    #[arg(short, long, default_value = "./barkle.sqlite")]
    file: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the database
    Init,
    /// Upgrade an existing database to the latest version
    Update,
    Portfolio(PortfolioArgs),
}

#[derive(Args, Debug)]
struct PortfolioArgs {
    #[command(subcommand)]
    cmd: PortfolioCommands,
}

#[derive(Subcommand, Debug)]
enum PortfolioCommands {
    /// Create a new portfolio
    Add { name: String },
    /// Delete an existing portfolio
    Delete { name: String },
    /// Short overview of a one or all portfolios.
    Show { name: String },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    run(args).await;
}

async fn run(args: Cli) {
    let mut app = App::new(&args.file).await;
    match args.cmd {
        Commands::Init => app.init().await,
        Commands::Update => app.init().await,
        Commands::Portfolio(portfolio) => app.portfolio(portfolio).await,
    }
}
