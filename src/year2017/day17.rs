//! Advent of Code - Day 17 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut steps = 0;
    let mut buf = vec![0, 0];
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        steps = line.parse::<u64>()?;
    }

    let result = if second_star {
        spinlock(&mut buf, steps, 50_000_000, second_star)?
    } else {
        spinlock(&mut buf, steps, 2017, second_star)?
    };
    writeln!(io::stdout(), "{result}")?;
    Ok(0)
}

/// Run the spinlock.
fn spinlock(buf: &mut Vec<u64>, steps: u64, iterations: u64, second_star: bool) -> Result<u64> {
    let mut curr_index = 0;
    for i in 0..iterations {
        let next_index = next_index(curr_index, i, steps)?;

        if second_star && next_index + 1 == 1 {
            buf[1] = i + 1;
        } else if !second_star {
            let next_idx = (next_index + 1) as usize;
            buf.insert(next_idx, i + 1);
        }
        curr_index = next_index + 1;
    }

    if second_star {
        Ok(buf[1])
    } else {
        let curr_idx = (curr_index + 1) as usize;
        Ok(buf[curr_idx])
    }
}

/// Calculate the next index
fn next_index(curr_index: u64, max_index: u64, steps: u64) -> Result<u64> {
    let mut idx = curr_index;

    for _ in 0..steps {
        #[allow(clippy::comparison_chain)]
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
        assert_eq!(super::spinlock(&mut buf, 3, 2017, false).expect(""), 638);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
