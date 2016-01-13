// Orion UI Controller
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use ui::presenter::{Presenter};
use ui::command_input::CommandInput;
use ui::command::{Command, CommandAction, is_known};
use io::transport::TransportError;
use log;

pub struct Controller<'a, T: 'a> {
    presenter: &'a mut  Presenter<'a, T>,

    // Must be box and not a reference, because it we use a function to restore
    // the connection when it's lost, which can't return a reference because of
    // what seems to be a rust bug.
    command_input: &'a mut CommandInput,
    action_history: Vec<CommandAction>,
}

impl<'a, T> Controller<'a, T> {
    pub fn new(commands: &'a mut CommandInput, presenter: &'a mut Presenter<'a, T>) 
        -> Controller<'a, T> {
        Controller {
            presenter: presenter,
            command_input: commands,
            action_history: Vec::new()
        }
    }

    pub fn run(&mut self) {
        self.presenter.send_initial_vm();
        loop {
            self.check_for_commands();
        }
    }

    fn check_for_commands(&mut self) {
        match self.command_input.receive_command_no_wait() {
            Ok(Some(command)) => self.handle_command(command),
            Err(TransportError::Closed) => log::fatal("Web UI was closed", 0),
            Err(err) => panic!(err),
            Ok(None) => ()
        };
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Do(action) => self.perform_action(action),
            Command::Undo => self.undo_last_action()
        }
    }

    fn perform_action(&mut self, action: CommandAction) {
        // WARNING: This could quickly turn into a switch of death.
        // CommandAction can't have methods itself because they might have
        // different arguments (traits implemented by Presenter) and if they
        // do then we can't store them together.
        // Might still be OK if I just make one object they all depend on,
        // but that breaks the rule of "don't depend on functionality you don't need"
        //match action {
            //CommandAction::CreateHistogram(name) => create_histogram(self.presenter, name)
        //}

        if is_known(&action) {
            self.action_history.push(action);
        }
    }

    fn undo_last_action(&mut self) {
        // TODO
    }
}
