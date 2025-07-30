use crate::func;

use std::path::PathBuf;

#[derive(Debug)]
pub enum ArgsError {
    InvalidArgs,
    MissingArgs,
}
impl ArgsError {
    pub fn handle(&self) {
        func::msg_error(match self {
            ArgsError::MissingArgs => "missing arguments, run 'mcas help' for info.",
            ArgsError::InvalidArgs => "invalid arguments, run 'mcas help' for info.",
        });
    }
}

#[derive(Debug)]
pub enum Func {
    Help,
    Info,
    Get(String, Option<PathBuf>),
}

#[derive(Debug)]
pub struct Args {
    dir: PathBuf,
    func: Func,
}

pub async fn go() {
    match parse() {
        Err(err) => err.handle(),
        Ok(args) => match run(args).await {
            Err(err) => err.handle(),
            Ok(_) => return,
        },
    }
}

fn parse() -> Result<Args, ArgsError> {
    let missing = Err(ArgsError::MissingArgs);
    let invalid = Err(ArgsError::InvalidArgs);

    let (dir, token_0, token_1, token_2) = (
        std::env::current_dir(),
        std::env::args().nth(1),
        std::env::args().nth(2),
        std::env::args().nth(3),
    );
    let args = Args {
        dir: match dir {
            Ok(d) => d,
            Err(_) => return invalid,
        },
        func: match (token_0, token_1, token_2) {
            (None, _, _) => return missing,
            (Some(f), v, o) => match (f.to_lowercase().as_str(), v, o) {
                ("help", _, _) => Func::Help,
                ("info", _, _) => Func::Info,
                ("get", Some(v), None) => Func::Get(v, None),
                ("get", Some(v), Some(o)) => Func::Get(v, Some(PathBuf::from(o))),
                ("get", None, None) => return missing,
                _ => return invalid,
            },
        },
    };
    Ok(args)
}

async fn run(mut args: Args) -> Result<(), ArgsError> {
    match args.func {
        Func::Help => func::help(),
        Func::Info => func::info(),
        Func::Get(v, o) => match func::get(&v, &mut args.dir, Option::from(&o)).await {
            Err(err) => func::msg_error(&format!("{err}").to_lowercase()),
            Ok(_) => return Ok(()),
        },
    }
    Ok(())
}
