//! Advent of Code - Day 17 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut steps = 0;
    let mut buf = vec![0];
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        steps = line.parse::<u32>()?;
    }

    let result = spinlock(&mut buf, steps)?;
    Ok(result)
}

/// Run the spinlock.
fn spinlock(buf: &mut Vec<u32>, steps: u32) -> Result<u32> {
    let mut curr_index = 0;
    for i in 0..2017 {
        let next_index = next_index(curr_index, buf.len() - 1, steps)?;
        buf.insert(next_index + 1, i + 1);
        curr_index = next_index + 1;
    }
    Ok(buf[curr_index + 1])
}

/// Calculate the next index
fn next_index(curr_index: usize, max_index: usize, steps: u32) -> Result<usize> {
    let mut idx = curr_index;

    for _ in 0..steps {
        if idx < max_index {
            idx += 1
        } else if idx == max_index {
            idx = 0;
        }
    }

    Ok(idx)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        let mut buf = vec![0];
        assert_eq!(super::spinlock(&mut buf, 3).expect(""), 638);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
