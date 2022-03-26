extern crate exitcode;

mod app;
mod books;
mod cli;
mod config;
mod input;

use app::{FetchCommand, Stats};
use clap::Parser;
use cli::{Args, BuildError, ConfigBuilder};
use input::{InputError, InputReader};
use std::error::Error as StdError;
use thiserror::Error;

/// Fetches input ISBN and save the resulted book in a JSON library file.
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let cli = Args::parse();
    let res = print(run(&cli).await);

    match res {
        Ok(_) => std::process::exit(exitcode::OK),
        Err(_) => std::process::exit(exitcode::IOERR),
    };
}

// Run the application and manage the app result
//
async fn run(cli: &Args) -> Result<Stats, AppError> {
    let config = ConfigBuilder::from_cli_args(&cli)?;
    let isbns = InputReader::read(&cli)?;

    let mut command = FetchCommand::create(&config);
    let res = command.run(&isbns).await?;

    Ok(res)
}

// Print output
//
fn print(res: Result<Stats, AppError>) -> Result<Stats, AppError> {
    match res {
        Ok(stats) => {
            println!("Successfully completed!");
            println!("Provided ISBN: {0}", stats.input_list);
            println!("Fetched Volumes: {0}", stats.new_volumes);
            Ok(stats)
        }
        Err(e) => {
            eprintln!("ERROR! {0}", e);
            Err(e)
        }
    }
}

// Available application errors
//
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Input data error. {0}")]
    InputError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl From<BuildError> for AppError {
    fn from(err: BuildError) -> AppError {
        AppError::ConfigError(err.to_string())
    }
}

impl From<InputError> for AppError {
    fn from(err: InputError) -> AppError {
        AppError::InputError(err.to_string())
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> AppError {
        AppError::RuntimeError(err.to_string())
    }
}
