use clap::Parser;
use v030_domain::app_builder::run_app;
use v030_domain::configuration::Configuration;

#[tokio::main]
async fn main() {
    let config = Configuration::parse();
    let _ = run_app(config).await;
}
