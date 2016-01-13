// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CommandAction {
    Unknown,
    CreateHistogram(String)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Command {
    Do(CommandAction),
    Undo
}

pub fn is_known(action: &CommandAction) -> bool {
    *action != CommandAction::Unknown
}
