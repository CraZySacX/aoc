// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` 0.1.0
#![deny(missing_docs)]
#![feature(try_from)]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate itertools;

extern crate bytecount;
extern crate chrono;
extern crate indexmap;
extern crate md5;
extern crate ndarray;
extern crate pathfinding;
extern crate primal;
extern crate regex;
extern crate sliding_windows;

mod constants;
mod error;
mod run;
mod utils;
mod year2015;
mod year2016;
mod year2017;
mod year2018;

use std::io::{self, Write};
use std::process;

/// CLI Entry Point
fn main() {
    match run::run() {
        Ok(i) => process::exit(i),
        Err(e) => {
            writeln!(io::stderr(), "{}", e).expect("Unable to write to stderr!");
            process::exit(1)
        }
    }
}
