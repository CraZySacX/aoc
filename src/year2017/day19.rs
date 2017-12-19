//! Advent of Code - Day 19 Solution
use error::Result;
use ndarray::Array2;
use std::convert::TryFrom;
use std::fmt;
use std::io::{self, BufRead, Write};

/// Direction of Movement
#[derive(Debug, PartialEq)]
enum Direction {
    /// We are moving up.
    Up,
    /// We are moving down.
    Down,
    /// We are moving left.
    Left,
    /// We are moving right.
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir_str = match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
        write!(f, "{}", dir_str)
    }
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut network_map: Array2<u8> = Array2::zeros((201, 201));
    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        fill_row(line, idx, &mut network_map);
    }

    let (letters, steps) = traverse_map(&network_map)?;
    writeln!(io::stdout(), "{}", letters)?;

    Ok(steps)
}

/// Fill a row in the network map array.
fn fill_row(line: &str, row: usize, network_map: &mut Array2<u8>) {
    for (idx, bit) in line.as_bytes().iter().enumerate() {
        network_map[[row, idx]] = *bit;
    }
}

/// Traverse the map.
fn traverse_map(network_map: &Array2<u8>) -> Result<(String, u32)> {
    let mut bytes = Vec::new();
    let mut curr_row = 0;
    let mut curr_col = 0;
    let max_col = network_map.cols();
    let max_row = network_map.rows();
    let mut curr_direction = Direction::Down;
    let mut steps = 0;

    loop {
        if curr_row == max_row {
            return Err(format!("Invalid row value: {}", curr_row).into());
        }
        if curr_col == max_col {
            return Err(format!("Invalid col value: {}", curr_col).into());
        }

        let curr_byte = network_map[[curr_row, curr_col]];

        match curr_byte {
            32 => {
                if curr_row == 0 {
                    // We are still in the first row.  We need to find
                    // the down byte.
                    curr_col += 1;
                } else {
                    break;
                }
            }
            124 | 45 => {
                match curr_direction {
                    Direction::Down => curr_row += 1,
                    Direction::Up => curr_row -= 1,
                    Direction::Right => curr_col += 1,
                    Direction::Left => curr_col -= 1,
                }
                steps += 1;
            }
            43 => {
                let (next_row, next_col) = get_next_neighbor(
                    curr_row,
                    curr_col,
                    max_row,
                    max_col,
                    &curr_direction,
                    network_map,
                )?;
                let next_direction = get_next_dir(curr_row, curr_col, next_row, next_col, &curr_direction)?;
                curr_row = next_row;
                curr_col = next_col;
                curr_direction = next_direction;
                steps += 1;
            }
            x => {
                bytes.push(x);

                match curr_direction {
                    Direction::Up => curr_row -= 1,
                    Direction::Down => curr_row += 1,
                    Direction::Left => curr_col -= 1,
                    Direction::Right => curr_col += 1,
                }
                steps += 1;
            }
        }
    }

    Ok((String::from_utf8_lossy(&bytes).into_owned(), steps))
}

/// Check the three nearest neighbors for the next valid direction.
fn get_next_neighbor(row: usize, col: usize, max_row: usize, max_col: usize, direction: &Direction, network_map: &Array2<u8>) -> Result<(usize, usize)> {
    let row_i: isize = TryFrom::try_from(row)?;
    let col_i: isize = TryFrom::try_from(col)?;

    let (row_deltas, col_deltas) = match *direction {
        Direction::Down | Direction::Up => (vec![0, 0], vec![-1, 1]),
        Direction::Right | Direction::Left => (vec![-1, 1], vec![0, 0]),
    };

    // Check the four adjacent neighbors (left, down, up, right)
    for k in 0..2 {
        if let Ok(adj_row) = TryFrom::try_from(row_i + row_deltas[k]) {
            if let Ok(adj_col) = TryFrom::try_from(col_i + col_deltas[k]) {
                if adj_row < max_row && adj_col < max_col && network_map[[adj_row, adj_col]] != 32 {
                    return Ok((adj_row, adj_col));
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    Err("Unable to find valid next neighbor".into())
}

/// Get the next direction
fn get_next_dir(curr_row: usize, curr_col: usize, next_row: usize, next_col: usize, curr_direction: &Direction) -> Result<Direction> {
    let next = match *curr_direction {
        Direction::Down | Direction::Up => {
            if next_col < curr_col {
                Direction::Left
            } else {
                Direction::Right
            }
        }
        Direction::Right | Direction::Left => {
            if next_row < curr_row {
                Direction::Up
            } else {
                Direction::Down
            }
        }
    };

    Ok(next)
}

#[cfg(test)]
mod one_star {
    use super::{fill_row, traverse_map};
    use ndarray::Array2;

    #[test]
    fn solution() {
        let mut network_map: Array2<u8> = Array2::zeros((6, 15));
        fill_row("     |         ", 0, &mut network_map);
        assert_eq!(network_map[[0, 5]], 124);
        fill_row("     |  +--+   ", 1, &mut network_map);
        assert_eq!(network_map[[1, 5]], 124);
        fill_row("     A  |  C   ", 2, &mut network_map);
        assert_eq!(network_map[[2, 5]], 65);
        fill_row(" F---|----E|--+", 3, &mut network_map);
        assert_eq!(network_map[[3, 5]], 124);
        fill_row("     |  |  |  D", 4, &mut network_map);
        assert_eq!(network_map[[4, 5]], 124);
        fill_row("     +B-+  +--+", 5, &mut network_map);
        assert_eq!(network_map[[5, 5]], 43);
        assert_eq!(
            traverse_map(&network_map).expect(""),
            ("ABCDEF".to_string(), 38)
        );
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
