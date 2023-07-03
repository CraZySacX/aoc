//! Advent of Code - Day 3 "Spiral Memory" Solution
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

/// Parse the file at `filename` and generate the checksum.
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut checksum = 0;

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        let value = line.parse::<u32>()?;
        if second_star {
            checksum += next_biggest(value)?;
        } else {
            checksum += calculate_steps(value)?;
        }
    }

    Ok(checksum)
}

/// Calculate the number of steps it will take to drain the given
/// value out of the (0,0) port.
fn calculate_steps(value: u32) -> Result<u32> {
    let final_tuple = calculate_tuple(value)?;
    Ok(TryFrom::try_from(manhattan_distance(final_tuple, (0, 0))?)?)
}

/// Calculate (x,y) tuple for a given value.
fn calculate_tuple(value: u32) -> Result<(i32, i32)> {
    let mut current_tuple: (i32, i32) = (0, 0);
    let mut generated = 1;

    for shell in 0.. {
        let upper_limit = generated + (8 * shell);
        if value <= upper_limit {
            let needed = value - generated;
            if needed > 0 {
                generate_next_n_tuples(&mut current_tuple, shell, value - generated)?;
            }
            break;
        } else {
            let ty_shell: i32 = TryFrom::try_from(shell)?;
            // We know the last tuple in any shell `x` is `(x, -x)`, so we just set it.
            current_tuple = (ty_shell, -ty_shell);
            // Bump the upper limit
            generated = upper_limit;
        }
    }
    Ok(current_tuple)
}

/// Calculate the manhattan distance between two (x,y) tuples.
fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> Result<i32> {
    Ok((from.0 - to.0).abs() + (from.1 - to.1).abs())
}

/// Calculate the last tuple in the given shell
fn generate_next_n_tuples(start_tuple: &mut (i32, i32), shell: u32, count: u32) -> Result<()> {
    start_tuple.0 += 1;
    let side_length = (8 * shell) / 4;
    let max_y: i32 = TryFrom::try_from(shell)?;
    let min_x: i32 = -TryFrom::try_from(shell)?;
    let min_y: i32 = -TryFrom::try_from(shell)?;

    for idx in 1..count {
        if start_tuple.1 < max_y && idx <= side_length {
            start_tuple.1 += 1;
        } else if start_tuple.0 > min_x && idx <= side_length * 2 {
            start_tuple.0 -= 1;
        } else if start_tuple.1 > min_y && idx <= side_length * 3 {
            start_tuple.1 -= 1;
        } else {
            start_tuple.0 += 1;
        }
    }

    Ok(())
}

/// Find the next biggest value after the given maximum value.
pub fn next_biggest(max_value: u32) -> Result<u32> {
    let mut tuple_map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut current_tuple = (0, 0);
    tuple_map.insert(current_tuple, 1);

    for shell in 1.. {
        let shell_len = 8 * shell;
        let side_len = shell_len / 4;
        let max_y: i32 = shell;
        let min_x: i32 = -shell;
        let min_y: i32 = -shell;

        for idx in 0..shell_len {
            if idx == 0 {
                current_tuple.0 += 1;
            } else if current_tuple.1 < max_y && idx <= side_len {
                current_tuple.1 += 1;
            } else if current_tuple.0 > min_x && idx <= side_len * 2 {
                current_tuple.0 -= 1;
            } else if current_tuple.1 > min_y && idx <= side_len * 3 {
                current_tuple.1 -= 1;
            } else {
                current_tuple.0 += 1;
            }

            let value = calculate_tuple_val(current_tuple, &tuple_map)?;
            if value <= max_value {
                tuple_map.insert(current_tuple, value);
            } else {
                return Ok(value);
            }
        }
    }

    Err(anyhow!("Unable to find next biggest value"))
}

/// Calculate the value for the given tuple given the tuple map
fn calculate_tuple_val(tuple: (i32, i32), tuple_map: &HashMap<(i32, i32), u32>) -> Result<u32> {
    let x = tuple.0;
    let y = tuple.1;

    // Add 8 nearest neighbors.  Only previously populated neighbors will have values (`Some(x)`).
    // The rest will return `None` on get.
    let results = [
        // Add current column (not including self)
        tuple_map.get(&(x, y + 1)),
        tuple_map.get(&(x, y - 1)),
        // Add one column to right
        tuple_map.get(&(x + 1, y)),
        tuple_map.get(&(x + 1, y + 1)),
        tuple_map.get(&(x + 1, y - 1)),
        // Add one column to left
        tuple_map.get(&(x - 1, y)),
        tuple_map.get(&(x - 1, y + 1)),
        tuple_map.get(&(x - 1, y - 1)),
    ];

    Ok(results
        .iter()
        .filter(|x| x.is_some())
        .fold(0, |sum, i| sum + i.expect("Invalid tuple_map value")))
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::calculate_steps(1).unwrap_or(1), 0);
        assert_eq!(super::calculate_steps(12).unwrap_or(0), 3);
        assert_eq!(super::calculate_steps(23).unwrap_or(0), 2);
        assert_eq!(super::calculate_steps(1024).unwrap_or(0), 31);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::next_biggest(1).unwrap_or(0), 2);
        assert_eq!(super::next_biggest(5).unwrap_or(0), 10);
        assert_eq!(super::next_biggest(11).unwrap_or(0), 23);
        assert_eq!(super::next_biggest(26).unwrap_or(0), 54);
        assert_eq!(super::next_biggest(59).unwrap_or(0), 122);
        assert_eq!(super::next_biggest(362).unwrap_or(0), 747);
    }
}
