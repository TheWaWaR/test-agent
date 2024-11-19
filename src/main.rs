use std::{env, net::SocketAddr};

use clap::{Parser, Subcommand};
use env_logger::{Builder, TimestampPrecision};
use log::debug;

mod cex;
mod pb;
mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    TestAgent {
        #[arg(long, default_value = "127.0.0.1:6555")]
        bind: SocketAddr,
    },
    TestMockClient {
        #[arg(long, default_value = "127.0.0.1:6555")]
        agent: SocketAddr,
    },
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    Builder::from_default_env()
        .format_module_path(true)
        .format_target(false)
        .format_timestamp(Some(TimestampPrecision::Millis))
        .init();

    let cli = Cli::parse();
    debug!("cli: {:#?}", cli);

    match cli.command {
        Commands::TestAgent { bind } => {
            cex::test::server::agent::start(bind).await.unwrap();
        }
        Commands::TestMockClient { agent } => {
            cex::test::mock_client::start(agent).await.unwrap();
        }
    }
}
