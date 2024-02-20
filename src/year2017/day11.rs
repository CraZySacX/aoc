//! Advent of Code - Day 11 "Hex Ed" Solution
use anyhow::{anyhow, Result};
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut result = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        result = parse_and_go(line, second_star)?;
    }
    Ok(result)
}

/// Parse the input and go.
fn parse_and_go(line: &str, second_star: bool) -> Result<u32> {
    let steps: Vec<&str> = line.split(',').collect();
    let mut coords = (0, 0, 0);
    let mut max_distance = 0;

    for step in steps {
        move_in_direction(step, &mut coords)?;
        let curr_distance = manhattan_distance_from_origin(coords)?;

        if curr_distance > max_distance {
            max_distance = curr_distance;
        }
    }

    if second_star {
        Ok(max_distance)
    } else {
        Ok(manhattan_distance_from_origin(coords)?)
    }
}

/// Calculate manhattan distance
fn manhattan_distance_from_origin(coords: (i32, i32, i32)) -> Result<u32> {
    let distance = (coords.0.abs() + coords.1.abs() + coords.2.abs()) / 2;
    Ok(TryFrom::try_from(distance)?)
}

/// Adjust the coordinates given a movement command.
fn move_in_direction(direction: &str, coords: &mut (i32, i32, i32)) -> Result<()> {
    match direction {
        "n" => {
            coords.1 += 1;
            coords.2 -= 1;
        }
        "ne" => {
            coords.0 += 1;
            coords.2 -= 1;
        }
        "se" => {
            coords.0 += 1;
            coords.1 -= 1;
        }
        "s" => {
            coords.1 -= 1;
            coords.2 += 1;
        }
        "sw" => {
            coords.0 -= 1;
            coords.2 += 1;
        }
        "nw" => {
            coords.0 -= 1;
            coords.1 += 1;
        }
        _ => return Err(anyhow!("Invalid movement direction")),
    }
    Ok(())
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::parse_and_go("ne,ne,ne", false).unwrap_or(0), 3);
        assert_eq!(super::parse_and_go("ne,ne,sw,sw", false).unwrap_or(1), 0);
        assert_eq!(super::parse_and_go("ne,ne,s,s", false).unwrap_or(0), 2);
        assert_eq!(super::parse_and_go("se,sw,se,sw,sw", false).unwrap_or(0), 3);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::parse_and_go("ne,ne,ne", true).unwrap_or(0), 3);
        assert_eq!(super::parse_and_go("ne,ne,sw,sw", true).unwrap_or(0), 2);
        assert_eq!(super::parse_and_go("ne,ne,s,s", true).unwrap_or(0), 2);
        assert_eq!(super::parse_and_go("se,sw,se,sw,sw", true).unwrap_or(0), 3);
    }
}
