// rcli csv -i input.csv -o output.json -d ','
use clap::Parser;
use rcli::{CmdExector, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    opts.cmd.execute().await?;
    anyhow::Ok(())
}
