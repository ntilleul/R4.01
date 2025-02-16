use clap::Parser;
use v021_app_builder::app_builder::run_app;
use v021_app_builder::configuration::Configuration;

#[tokio::main]
async fn main() {
    let config = Configuration::parse();
    let _ = run_app(config).await;
}
