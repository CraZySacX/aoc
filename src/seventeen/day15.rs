//! Advent of Code - Day 15 Solution
use error::Result;
use run::AoCYear;
use std::io::BufRead;

/// Find the solution.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn find_solution<T: BufRead>(_reader: T, year: &AoCYear, _second_star: bool) -> Result<u32> {
    match *year {
        _ => Err("Not Implemented!".into()),
    }
}
