use clap::Parser;
use hetu as ht;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_some() {
        //log module
    }
    let cli = ht::Cli::parse();
    cli.subcommand.run().await
}
