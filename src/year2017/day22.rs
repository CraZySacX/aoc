//! Advent of Code - Day 22 Solution
use error::Result;
use ndarray::Array2;
use std::io::BufRead;

/// The direction the virus is facing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    /// Up
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
}

/// The current state of the coord.
#[derive(Debug, Eq, PartialEq)]
enum State {
    /// Clean
    Clean,
    /// Weakened
    Weakened,
    /// Infected
    Infected,
    /// Flagged
    Flagged,
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let dim = 1001;
    let mut arr: Array2<char> = Array2::from_elem((dim, dim), '.');
    let center = dim / 2;

    for (i, line_result) in reader.lines().enumerate() {
        let line = &line_result?;
        add_to_arr(i, center, line, &mut arr);
    }

    let mut curr_dir = Direction::Up;
    let mut coords = (center, center);
    let mut count = 0;
    for _ in 0..if second_star { 10_000_000 } else { 10_000 } {
        curr_dir = change_direction(coords, &curr_dir, &arr)?;

        if change_state(coords, &mut arr, second_star)? {
            count += 1;
        }
        move_virus(&mut coords, &curr_dir);
    }

    Ok(count)
}

/// Add the line at index to the array.
fn add_to_arr(row: usize, center: usize, line: &str, arr: &mut Array2<char>) {
    let half = line.len() / 2;
    let start_col = center - half;
    let actual_row = row + center - half;
    for (j, c) in line.chars().enumerate() {
        arr[[actual_row, start_col + j]] = c;
    }
}

/// Determine new direction
fn change_direction(coords: (usize, usize), curr_direction: &Direction, arr: &Array2<char>) -> Result<Direction> {
    let curr_state = get_state(coords, arr)?;

    Ok(match curr_state {
        State::Infected => match *curr_direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        },
        State::Clean => match *curr_direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        },
        State::Flagged => match *curr_direction {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        },
        State::Weakened => *curr_direction,
    })
}

/// Determine the state of the current coord.
fn get_state(coords: (usize, usize), arr: &Array2<char>) -> Result<State> {
    let val = arr[[coords.0, coords.1]];
    let res = match val {
        '#' => State::Infected,
        'W' => State::Weakened,
        'F' => State::Flagged,
        '.' => State::Clean,
        _ => return Err(format!("invalid state: {}", val).into()),
    };
    Ok(res)
}
/// Infect or clean the given coords.
fn change_state(coords: (usize, usize), arr: &mut Array2<char>, second_star: bool) -> Result<bool> {
    let mut new_infection = false;
    let curr_state = get_state(coords, arr)?;

    if second_star {
        match curr_state {
            State::Clean => arr[[coords.0, coords.1]] = 'W',
            State::Weakened => {
                arr[[coords.0, coords.1]] = '#';
                new_infection = true;
            }
            State::Infected => arr[[coords.0, coords.1]] = 'F',
            State::Flagged => arr[[coords.0, coords.1]] = '.',
        }
    } else {
        match curr_state {
            State::Clean => {
                arr[[coords.0, coords.1]] = '#';
                new_infection = true;
            }
            State::Infected => {
                arr[[coords.0, coords.1]] = '.';
            }
            _ => return Err("invalid state for one star".into()),
        }
    }

    Ok(new_infection)
}

/// Move the virus
fn move_virus(coords: &mut (usize, usize), direction: &Direction) {
    match *direction {
        Direction::Up => coords.0 -= 1,
        Direction::Left => coords.1 -= 1,
        Direction::Down => coords.0 += 1,
        Direction::Right => coords.1 += 1,
    }
}

#[cfg(test)]
mod one_star {
    use super::Direction;
    use ndarray::Array2;

    #[test]
    fn solution() {
        let dim = 1001;
        let mut arr: Array2<char> = Array2::from_elem((dim, dim), '.');
        let center = dim / 2;
        super::add_to_arr(0, center, "..#", &mut arr);
        super::add_to_arr(1, center, "#..", &mut arr);
        super::add_to_arr(2, center, "...", &mut arr);

        let mut curr_dir = Direction::Up;
        let mut coords = (center, center);
        let mut count = 0;
        for _ in 0..10000 {
            curr_dir = super::change_direction(coords, &curr_dir, &arr).expect("");

            if super::change_state(coords, &mut arr, false).expect("") {
                count += 1;
            }
            super::move_virus(&mut coords, &curr_dir);
        }
        assert_eq!(count, 5587);
    }
}

#[cfg(test)]
mod two_star {
    use super::Direction;
    use ndarray::Array2;

    #[test]
    fn solution() {
        let dim = 1001;
        let mut arr: Array2<char> = Array2::from_elem((dim, dim), '.');
        let center = dim / 2;
        super::add_to_arr(0, center, "..#", &mut arr);
        super::add_to_arr(1, center, "#..", &mut arr);
        super::add_to_arr(2, center, "...", &mut arr);

        let mut curr_dir = Direction::Up;
        let mut coords = (center, center);
        let mut count = 0;
        for _ in 0..10_000_000 {
            curr_dir = super::change_direction(coords, &curr_dir, &arr).expect("");

            if super::change_state(coords, &mut arr, true).expect("") {
                count += 1;
            }
            super::move_virus(&mut coords, &curr_dir);
        }
        assert_eq!(count, 2_511_944);
    }
}
