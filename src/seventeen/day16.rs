//! Advent of Code - Day 16 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    for line_result in reader.lines() {
        let _line = &line_result.unwrap_or_else(|_| "".to_string());
    }
    Ok(0)
}

#[cfg(test)]
mod test {
    #[test]
    fn one_star_2017() {
        assert!(true);
    }

    #[test]
    fn two_star_2017() {
        assert!(true);
    }
}
