// Fake view model output for debugging
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

use io::serialize::Serialize;
use ui::view_model_output::ViewModelOutput;

pub struct DebugViewModelOutput;

impl DebugViewModelOutput {
    pub fn new() -> DebugViewModelOutput {
        DebugViewModelOutput
    }
}

impl<'a, T: Serialize> ViewModelOutput<T> for DebugViewModelOutput {
    fn send_vm(&mut self, vm: &T) {
        println!("Viewmodel was sent to debug viewmodel output {}", vm.serialize());
    }
}

