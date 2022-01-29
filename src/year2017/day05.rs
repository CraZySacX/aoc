//! Advent of Code - Day 5 "A Maze of Twisty Trampolines All Alike" Solution
use error::Result;
use std::convert::TryFrom;
use std::io::BufRead;

/// Parse the file at `filename` and generate the checksum.
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut jump_vec = Vec::new();

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        jump_vec.push(line.parse::<i32>()?);
    }

    jump_away(&mut jump_vec, second_star)
}

/// Find an exit
fn jump_away(jump_vec: &mut [i32], crazy_jumps: bool) -> Result<u32> {
    let list_len = jump_vec.len();
    let mut step = 0;
    let mut current_idx: i32 = 0;

    loop {
        if current_idx < 0 {
            break;
        }

        let idx: usize = TryFrom::try_from(current_idx)?;

        if idx < list_len {
            let next_idx = jump_vec[idx];
            jump_vec[idx] = if crazy_jumps {
                if next_idx > 2 {
                    next_idx - 1
                } else {
                    next_idx + 1
                }
            } else {
                next_idx + 1
            };
            current_idx += next_idx;
        } else {
            break;
        }

        step += 1;
    }

    Ok(step)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::jump_away(&mut vec![0, 3, 0, 1, -3], false).unwrap_or(0), 5);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::jump_away(&mut vec![0, 3, 0, 1, -3], true).unwrap_or(0), 10);
    }
}
