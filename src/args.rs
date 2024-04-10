use crate::log;

use crate::args::Args::{One, Two};
use log::msg;
use std::path::PathBuf;

pub enum ParseError {
    InvalidDir,
    MissingArg1,
    MissingArg2,
}
impl ParseError {
    pub fn handle(&self) {
        match self {
            ParseError::InvalidDir => msg("current directory needs higher permissions to access."),
            ParseError::MissingArg1 => msg("missing function, use mcas help for valid functions."),
            ParseError::MissingArg2 => msg("missing version, use mcas help for valid format."),
        }
    }
}

pub enum Args {
    One(String),
    Two(PathBuf, String, String),
}

impl Args {
    pub fn parse() -> Result<Args, ParseError> {
        return match (
            std::env::current_dir(),
            std::env::args().nth(1),
            std::env::args().nth(2),
        ) {
            (Ok(d), Some(f), Some(v)) => Ok(Two(d, f, v)),
            (Ok(_), Some(f), None) => Ok(One(f)),
            (Ok(_), None, None) => Err(ParseError::MissingArg1),
            _ => Err(ParseError::InvalidDir),
        };
    }
}
