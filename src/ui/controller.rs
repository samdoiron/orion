// Orion UI Controller
// Copyright (C) 2015  Samuel Doiron
use ui::presenter::{Presenter};
use ui::command_input::CommandInput;
use ui::command::{Command, CommandAction, is_known};

struct Controller<'a, T> {
    presenter: &'a mut  Presenter<'a, T>,
    command_input: &'a mut CommandInput,
    action_history: Vec<CommandAction>
}

impl<'a, T> Controller<'a, T> {
    fn new(commands: &'a mut CommandInput, presenter: &'a mut Presenter<'a, T>) 
        -> Controller<'a, T> {
        Controller {
            presenter: presenter,
            command_input: commands,
            action_history: Vec::new()
        }
    }

    fn run(&mut self) {
        loop {
            self.check_for_commands();
        }
    }

    fn check_for_commands(&mut self) {
        match self.command_input.receive_command_no_wait() {
            Ok(Some(command)) => self.handle_command(command),
            Err(err) => panic!(err),
            _ => ()
        }
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
