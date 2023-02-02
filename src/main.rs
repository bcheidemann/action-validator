use action_validator::CliConfig;
use clap::Parser;
use std::process;

fn main() {
    let config = CliConfig::parse();

    if let Err(e) = action_validator::run_cli(&config) {
        println!(
            "Fatal error validating {}: {}",
            config.src.to_str().unwrap(),
            e
        );
        process::exit(1);
    }
}
