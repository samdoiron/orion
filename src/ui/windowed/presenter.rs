// Copyright (C) 2015  Samuel Doiron
use entities::Series;

use ui::presenter;
use ui::windowed::view_models;
use ui::view_model_output::ViewModelOutput;

use log;

pub struct Presenter<'a> {
    vmOutput: &'a ViewModelOutput<view_models::Main>
}

impl<'a> presenter::Presenter<'a, view_models::Main> for Presenter<'a> {
}

impl<'a> Presenter<'a> {
    pub fn new(viewOutput: &'a mut ViewModelOutput<view_models::Main>)
        -> Presenter<'a> {
        Presenter {
            vmOutput: viewOutput
        }
    }
}
