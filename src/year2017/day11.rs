//! Advent of Code - Day 11 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut result = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        result = parse_and_go(line)?;
    }
    Ok(result)
}

/// Parse the input and go.
fn parse_and_go(line: &str) -> Result<u32> {
    let steps: Vec<&str> = line.split(',').collect();

    Ok(0)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
