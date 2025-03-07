use c1::app_builder::run_app;
use c1::configuration::Configuration;
use clap::Parser;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await?;
  
    Ok(())
}