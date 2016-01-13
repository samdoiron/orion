// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

// Disabled during early development because the architecture isn't entirely
// in place, and having this many warnings makes other ones harder to see.
//
// Obviously re-enable this later on.
#![allow(dead_code)]

extern crate test_util;
extern crate net2;
extern crate time;
extern crate websocket;
extern crate rustc_serialize;

mod log;
pub mod entities;
pub mod use_cases;
pub mod gateways;
pub mod io;
pub mod ui;
