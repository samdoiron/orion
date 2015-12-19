// Copyright (C) 2015  Samuel Doiron
pub mod series;
pub mod histogram;
pub mod main;

pub use self::main::Main;
pub use self::series::Series;
pub use self::histogram::Histogram;