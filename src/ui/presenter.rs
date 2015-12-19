// Copyright (C) 2015  Samuel Doiron
use ui::view_model_output::ViewModelOutput;

pub trait Presenter<'a, T> {
    fn new(&'a ViewModelOutput<T>) -> Self;
}
