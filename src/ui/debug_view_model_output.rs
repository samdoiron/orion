// Fake view model output for debugging
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

use ui::view_model_output::ViewModelOutput;
use log;

pub struct DebugViewModelOutput;

impl DebugViewModelOutput {
    pub fn new() -> DebugViewModelOutput {
        DebugViewModelOutput
    }
}

impl<'a, T> ViewModelOutput<T> for DebugViewModelOutput {
    fn send_vm(&mut self, vm: &T) {
        log::debug("Would send on ViewModelOutput");
    }
}
