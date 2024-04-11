use crate::args::Args::{One, Two};
use crate::{get, help, log};

use log::*;
use std::path::PathBuf;

pub enum ArgsError {
    InvalidDir,
    InvalidArgs,
    MissingArgs,
}
impl ArgsError {
    pub fn handle(&self) {
        match self {
            ArgsError::InvalidDir => msg("current directory needs higher permissions to access."),
            ArgsError::MissingArgs => msg("missing arguments, use mcas help for valid arguments."),
            ArgsError::InvalidArgs => msg("invalid arguments, use mcas help for valid arguments."),
        }
    }
}

pub enum Args {
    One(String),
    Two(PathBuf, String, String),
}

pub fn parse() -> Result<Args, ArgsError> {
    return match (
        std::env::current_dir(),
        std::env::args().nth(1),
        std::env::args().nth(2),
    ) {
        (Ok(_), Some(f), None) => Ok(One(f)),
        (Ok(d), Some(f), Some(v)) => Ok(Two(d, f, v)),
        (Ok(_), None, _) => Err(ArgsError::MissingArgs),
        _ => Err(ArgsError::InvalidDir),
    };
}

pub async fn run(args_res: Result<Args, ArgsError>) {
    match args_res {
        Err(err) => err.handle(),
        Ok(args) => match run_args(args).await {
            Ok(_) => return,
            Err(err) => err.handle(),
        },
    }
}

async fn run_args(args: Args) -> Result<(), ArgsError> {
    match args {
        One(f) => match f.as_str() {
            "help" => help::help(),
            _ => return Err(ArgsError::InvalidArgs),
        },
        Two(d, f, v) => match f.as_str() {
            "get" => match get::try_get(v.as_str(), &d).await {
                Ok(_) => msg("saved!"),
                Err(err) => println!("{} err", err),
            },
            _ => return Err(ArgsError::InvalidArgs),
        },
    }
    Ok(())
}
