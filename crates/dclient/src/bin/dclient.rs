use clap::Parser;
use dclient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_some() {
        //log module
    }
    let cli = dclient::Cli::parse();
    cli.subcommand.run().await
}
