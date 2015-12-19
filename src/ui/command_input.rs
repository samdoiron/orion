// Copyright (C) 2015  Samuel Doiron
use ui::command::Command;
use transport;
use transport::ReadTransport;
use ui::command_parser::parse_command;

pub trait CommandInput {
    fn receive_command(&mut self) -> transport::Result<Command>;
    fn receive_command_no_wait(&mut self) -> transport::Result<Option<Command>>;
}
