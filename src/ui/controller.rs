// Orion UI Controller
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

// For BTreeMap -> Repo mapping

use std::collections::BTreeMap;

use use_cases::create_histogram::{create_histogram, CreateHistogram, OnHistogramCreated};

use ui::presenter::{Presenter};
use ui::command_input::CommandInput;
use ui::command::{Command, CommandAction, is_known};
use io::transport::TransportError;

use log;
use std::marker::PhantomData;

pub struct Controller<'a, P: 'a, T: 'a>
    where P: Presenter<'a, T> + OnHistogramCreated {
    presenter: &'a mut P,
    command_input: &'a mut CommandInput,
    action_history: Vec<CommandAction>,

    // Needed because otherwise the `T` is only used for other type
    // parameters, which Rust doesn't like.
    // See https://github.com/rust-lang/rust/issues/23246
    _phantom: PhantomData<T>
}

impl<'a, P, T> Controller<'a, P, T>
    where P: Presenter<'a, T> + OnHistogramCreated {
    pub fn new(commands: &'a mut CommandInput, presenter: &'a mut P) 
        -> Controller<'a, P, T> {
        Controller {
            presenter: presenter,
            command_input: commands,
            action_history: Vec::new(),
            _phantom: PhantomData
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
            Err(err) => panic!("{}", err),
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
        if !is_known(&action) {
            return
        }

        match action {
            CommandAction::CreateHistogram(name, series_id) => {
                // XXX temporary, just create and throw away
                let request = CreateHistogram {
                    histogram_repo: &mut BTreeMap::new(),
                    series_repo: &mut BTreeMap::new(),

                    histogram_name: &name,
                    series_id: series_id,

                    output: self.presenter
                };
                create_histogram(request)
            },
            CommandAction::Unknown => {
                log::fatal("Received unknown command action", 500)
            }
        }
    }

    fn undo_last_action(&mut self) {
        // TODO Implement UNDO
        log::fatal("UNDO not implemented", 404);
    }
}
