// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use ui::command::{Command, CommandAction};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str;

#[derive(Debug)]
pub struct ParseError;

type ParseResult<T> = Result<T, ParseError>;

const PARSE_ERROR_MESSAGE: &'static str 
    = "Failed to parse serialized UI command" ;

// Fields are seperated by the "Unit Seperator" field, which (funny enough) 
// was basically intended for this originally 
// (well, database fields, but w/e)
const COMMAND_FIELD_SEPARATOR: char = 0x1F as char;

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

#[inline]
fn is_undo_command(message: &str) -> bool {
    message == "undo"
}

#[inline]
fn is_do_command(fields: &Vec<&str>) -> bool {
    fields[0] == "do"
}

fn get_fields(message: &str) -> Vec<&str> {
    message.split(COMMAND_FIELD_SEPARATOR).collect()
}

fn make_command_string(fields: Vec<&str>) -> String {
    let join_slice = [COMMAND_FIELD_SEPARATOR as u8];
    let join_str = str::from_utf8(&join_slice)
        .ok().expect("COMMAND_FIELD_SEPARATOR is invalid ASCII");
    fields.join(join_str)
}

// Do commands have a format like "do CreateHistogram something" except
// with ASCII unit seperators instead of spaces.
fn parse_do_command(message: &str) -> ParseResult<Command> {
    let mut fields = get_fields(message);
    
    if !is_do_command(&fields) {
        return Err(ParseError);
    }

    // Get rid of leading "do"
    fields.remove(0);

    let command_name = fields[0];
    fields.remove(0);

    let command_fields = fields;
    match command_name {
        "CreateHistogram" => parse_create_histogram(command_fields),
        _ => Err(ParseError)
    }.map(|x| Command::Do(x))
}

// Example: "do CreateHistogram some thing goes here"
fn parse_create_histogram(args: Vec<&str>) -> ParseResult<CommandAction> {
    if args.len() != 1 {
        return Err(ParseError);
    }

    Ok(CommandAction::CreateHistogram(args[0].to_string()))
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use test_util::fuzz;
    
    use super::*;
    use super::make_command_string;
    use ui::command::{Command, CommandAction};

    fn assert_parse_error(fields: Vec<&str>) {
        let command = make_command_string(fields);
        assert!(parse_command(&command).is_err());
    }

    fn assert_parse_ok(fields: Vec<&str>) {
        let command = make_command_string(fields);
        assert!(parse_command(&command).is_ok());
    }

    fn assert_parse_to(fields: Vec<&str>, desired: Command) {
        let command = make_command_string(fields);
        let parsed = parse_command(&command);
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), desired);
    }

    #[test]
    fn parse_command__neither_do_nor_undo__returns_err() {
        assert_parse_error(vec!["invalid", "command"]);
    }

    #[test]
    fn parse_command__just_string_undo__returns_undo_command() {
        assert_parse_to(vec!["undo"], Command::Undo);
    }

    #[test]
    fn parse_command__unicode_stress_test__doesnt_panic() {
        for string in fuzz::strange_unicode_strings() {
            let _ = parse_command(&string);
        }
    }

    #[test]
    fn parse_command__do_with_invalid_name__returns_err() {
        assert_parse_error(vec!["do", "some_invalid_command", "foo", "bar"]);
        assert_parse_error(vec!["do", "CreateHistogra"]);
        assert_parse_error(vec!["do", "CreateHistogram\0"]);
    }

    #[test]
    fn undo__extra_at_end__returns_err() {
        assert_parse_error(vec!["undo", "do"]);
        assert_parse_error(vec!["undo", "undo"," undo"]);
        assert_parse_error(vec!["undoundo"]);
        assert_parse_error(vec!["undo\0undo"]);
    }

    #[test]
    fn create_histogram__no_name_field__returns_err() {
        assert_parse_error(vec!["do", "CreateHistogram"]);
    }

    #[test]
    fn create_histogram__with_name_field__returns_correct_command() {
        let some_histogram_name = "My awesome histogram".to_string();
        assert_parse_to(
            vec!["do", "CreateHistogram", &some_histogram_name],
            Command::Do(CommandAction::CreateHistogram(some_histogram_name.clone()))
        );
    }
}

