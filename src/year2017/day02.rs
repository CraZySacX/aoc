//! Advent of Code - Day 2 "Corruption Checksum" Solution
use error::Result;
use std::cmp;
use std::io::BufRead;

/// Parse the file at `filename` and generate the checksum.
pub fn find_solution<T: BufRead>(reader: T, use_div: bool) -> Result<u32> {
    let mut checksum = 0;

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        if use_div {
            checksum += row_evenly_divisible_value(line)?;
        } else {
            checksum += row_min_max_diff(line)?;
        }
    }

    Ok(checksum)
}

/// Find the difference between the max and min of a whitespace separated string
fn row_min_max_diff(line: &str) -> Result<u32> {
    let vals_iter = line.split_whitespace();
    let mut min = u32::max_value();
    let mut max = 0;

    for val_str in vals_iter {
        let val = val_str.parse::<u32>()?;
        if val < min {
            min = val;
        }

        if val > max {
            max = val;
        }
    }

    Ok(max - min)
}

/// Find the only two evenly divisible values in a whitespace separated string
fn row_evenly_divisible_value(line: &str) -> Result<u32> {
    let vals_iter = line.split_whitespace();
    let mut vals_vec = Vec::new();
    for val_str in vals_iter {
        vals_vec.push(val_str.parse::<u32>()?);
    }
    let inner_vals_vec = vals_vec.clone();

    for (idx, val) in vals_vec.iter().enumerate() {
        for (next_idx, next_val) in inner_vals_vec.iter().enumerate() {
            if idx == next_idx {
                continue;
            }

            let numerator = cmp::max(*val, *next_val);
            let denominator = cmp::min(*val, *next_val);

            if numerator % denominator == 0 {
                return Ok(numerator / denominator);
            }
        }
    }
    Err("No evenly divisible values".into())
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::row_min_max_diff("5 1 9 5").unwrap_or_else(|_| 0), 8);
        assert_eq!(super::row_min_max_diff("7 5 3").unwrap_or_else(|_| 0), 4);
        assert_eq!(super::row_min_max_diff("2 4 6 8").unwrap_or_else(|_| 0), 6);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::row_evenly_divisible_value("5 9 2 8").unwrap_or_else(|_| 0), 4);
        assert_eq!(super::row_evenly_divisible_value("9 4 7 3").unwrap_or_else(|_| 0), 3);
        assert_eq!(super::row_evenly_divisible_value("3 8 6 5").unwrap_or_else(|_| 0), 2);
    }
}
