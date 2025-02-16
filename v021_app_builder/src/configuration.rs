use clap::Parser;

#[derive(Parser)]
pub struct Configuration {
    #[arg(short = 'c', long = "candidates", required = true, num_args = 1..)]
    pub candidates: Vec<String>,
}
