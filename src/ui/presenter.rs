// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

pub trait Presenter<'a, T> {
    // TODO Should this need to be requested? Eg. the view needs to
    // ask for a certain session, and then we send it?
    fn send_initial_vm(&mut self);
}
