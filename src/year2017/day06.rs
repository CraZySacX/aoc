//! Advent of Code - Day 6 Solution
use error::Result;
use std::collections::HashSet;
use std::io::BufRead;

/// Parse the file at `filename` and generate the checksum.
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut reallocations = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        reallocations = reallocate_memory(line, second_star)?;
    }

    Ok(reallocations)
}

/// Reallocate some memory blocks
fn reallocate_memory(line: &str, find_again: bool) -> Result<u32> {
    // Convert the line to a vector of u32.
    let vals_iter = line.split_whitespace();
    let mut vals_vec = Vec::new();
    for val_str in vals_iter {
        vals_vec.push(val_str.parse::<u32>()?);
    }

    // Setup some state.
    let len = vals_vec.len();
    let mut once_more = find_again;
    let mut steps = 1;
    let mut seen = HashSet::new();

    loop {
        // We've seen the current vec, so put into set.
        seen.insert(vals_vec.clone());

        // Find the max and first position of max.
        let max_vec = vals_vec.clone();
        let pos_vec = vals_vec.clone();
        let max = max_vec.iter().max().ok_or("Unable to find max")?;
        let pos = pos_vec
            .iter()
            .position(|&x| x == *max)
            .ok_or("Unable to find pos of max")?;

        // Reset the max to 0
        vals_vec[pos] = 0;

        // Cycle through the vec, reallocating
        for i in 0..*max {
            let idx = (pos + (i + 1) as usize) % len;
            vals_vec[idx] += 1;
        }

        // Check if we have seen the resulting vec
        if seen.get(&vals_vec).is_some() {
            // If we have, but we want to find the next occurence
            // then reset some state and continue.
            if once_more {
                steps = 1;
                seen.clear();
                once_more = false;
                continue;
            }
            // Otherwise we are done.
            break;
        } else {
            // If we haven't, increment the step count and loop
            steps += 1;
        }
    }
    Ok(steps)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::reallocate_memory("0 2 7 0", false).unwrap_or(0), 5);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::reallocate_memory("0 2 7 0", true).unwrap_or(0), 4);
    }
}
