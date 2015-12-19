// Copyright (C) 2015  Samuel Doiron
#[derive(PartialEq, Eq, Clone)]
pub enum CommandAction {
    Unknown,
    CreateHistogram(String)
}

pub enum Command {
    Do(CommandAction),
    Undo
}

pub fn is_known(action: &CommandAction) -> bool {
    *action != CommandAction::Unknown
}
