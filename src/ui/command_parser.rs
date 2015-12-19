// Copyright (C) 2015  Samuel Doiron
use ui::command::Command;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseError;

const PARSE_ERROR_MESSAGE: &'static str = "Failed to parse serialized UI command" ;

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("ParseError: {}", &PARSE_ERROR_MESSAGE))
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &PARSE_ERROR_MESSAGE
    }
    fn cause(&self) -> Option<&Error> { None }
}

pub fn parse_command(message: &str) -> Result<Command, ParseError> {
    if is_undo_command(message) {
        Ok(Command::Undo)
    } else {
        parse_do_command(message)
    }
}

fn is_undo_command(message: &str) -> bool {
    message == "undo"
}

fn parse_do_command(message: &str) -> Result<Command, ParseError> {
    Ok(Command::Undo)
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::parse_command;

    #[test]
    fn parse_command__neither_do_nor_undo__returns_err() {
        assert!(parse_command("invalid command").is_err())
    }
}

