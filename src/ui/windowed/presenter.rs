// Copyright (C) 2015  Samuel Doiron
use entities::Series;

use ui::presenter;
use ui::windowed::view_models;
use ui::view_model_output::ViewModelOutput;

use log;

pub struct Presenter<'a> {
    vm_output: &'a mut ViewModelOutput<view_models::Main>
}

impl<'a> presenter::Presenter<'a, view_models::Main> for Presenter<'a> {
    // See note in ui::presenter
    fn send_initial_vm(&mut self) {
        self.vm_output.send_vm(&view_models::Main{
            series: vec![],
            charts: vec![]
        })
    }
}

impl<'a> Presenter<'a> {
    pub fn new(view_output: &'a mut ViewModelOutput<view_models::Main>)
        -> Presenter<'a> {
        Presenter {
            vm_output: view_output
        }
    }
}
