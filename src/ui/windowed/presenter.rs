// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use ui::presenter;
use ui::windowed::view_models;
use ui::view_model_output::ViewModelOutput;

use test_util::random;

pub struct Presenter<'a> {
    vm_output: &'a mut ViewModelOutput<view_models::Main>
}

fn random_series() -> view_models::Series {
    view_models::Series{
        name: "First Series".to_string(),
        current_value: view_models::DataPoint{
            time: 123u64, 
            value: random::in_range(1, 1000) as f64
        }
    }
}

impl<'a> presenter::Presenter<'a, view_models::Main> for Presenter<'a> {
    // See note in ui::presenter
    fn send_initial_vm(&mut self) {
        let mut series = Vec::new();
        for _ in 0..10 {
            series.push(random_series());
        }
        self.vm_output.send_vm(&view_models::Main{
            series: series,
            charts: vec![]
        });
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
