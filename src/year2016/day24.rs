//! Advent of Code - Day 24 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    for line_result in reader.lines() {
        let _line = &line_result.unwrap_or_else(|_| "".to_string());
    }
    Ok(0)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {}
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
