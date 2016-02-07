// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

use entities::series;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CommandAction {
    Unknown,
    CreateHistogram(String, series::Id)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Command {
    Do(CommandAction),
    Undo
}

pub fn is_known(action: &CommandAction) -> bool {
    *action != CommandAction::Unknown
}
