// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use ui::command::Command;
use io::transport;

pub trait CommandInput {
    fn receive_command(&mut self) -> transport::Result<Command>;
    fn receive_command_no_wait(&mut self) -> transport::Result<Option<Command>>;
}
