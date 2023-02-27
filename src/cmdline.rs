use std::{env::Args, error, fmt};

macro_rules! usage {
    ($program: expr) => {
        format! {
            "Usage: {program} <date-offset-string>

Date offset string has the following format:
    * A 'Plus' (<+>) or a 'Minus' (<->)
    * A number
    * A unit of time (<Y> or <Year>, <M> or <Month>, <D> or <Day>)

A sequence of these three can be repeated as many times as needed.

Example:
    {program} +2Y-3Month+5D",
            program = $program,
        }
    };
}

pub fn parse_args(mut args: Args) -> Result<String, CmdlineError> {
    let program = args.next().unwrap();
    let mut input: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            _ => {
                input = Some(arg);
                break;
            }
        }
    }

    if let Some(input) = input {
        Ok(input)
    } else {
        Err(CmdlineError::NoArguments {
            usage: Some(usage!(program)),
        })
    }
}

pub enum CmdlineError {
    NoArguments { usage: Option<String> },
}

impl fmt::Debug for CmdlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArguments { usage } => {
                write!(
                    f,
                    "No arguments provided\n{}",
                    if let Some(usage) = usage { usage } else { "" }
                )
            }
        }
    }
}

impl fmt::Display for CmdlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArguments { usage } => {
                write!(
                    f,
                    "No arguments provided\n{}",
                    if let Some(usage) = usage { usage } else { "" }
                )
            }
        }
    }
}

impl error::Error for CmdlineError {}
