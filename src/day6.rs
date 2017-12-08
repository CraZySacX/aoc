//! Advent of Code - Day 6 Solution
use error::Result;
use std::collections::HashSet;
use std::io::BufRead;

/// Parse the file at `filename` and generate the checksum.
pub fn reallocations_until_match<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut reallocations = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        if second_star {
            // Not implemented
        } else {
            reallocations = reallocate_memory(line)?;
        }
    }

    Ok(reallocations)
}

/// Reallocate some memory blocks
fn reallocate_memory(line: &str) -> Result<u32> {
    // use std::io::{self, Write};
    let mut steps = 1;
    let mut seen = HashSet::new();
    let vals_iter = line.split_whitespace();
    let mut vals_vec = Vec::new();
    for val_str in vals_iter {
        vals_vec.push(val_str.parse::<u32>()?);
    }
    let len = vals_vec.len();

    seen.insert(vals_vec.clone());

    loop {
        let max_vec = vals_vec.clone();
        let pos_vec = vals_vec.clone();
        let max = max_vec.iter().max().ok_or("Unable to find max")?;
        // write!(io::stdout(), "Max: {}", max)?;
        let pos = pos_vec
            .iter()
            .position(|&x| x == *max)
            .ok_or("Unable to find pos of max")?;
        // writeln!(io::stdout(), ", Pos: {}", pos)?;

        vals_vec[pos] = 0;

        for i in 0..*max {
            let idx = (pos + (i + 1) as usize) % len;
            vals_vec[idx] += 1;
            // write!(io::stdout(), "{},", idx)?;
        }
        // writeln!(io::stdout(), "")?;
        // writeln!(io::stdout(), "Vec: {:?}", vals_vec)?;
        if seen.get(&vals_vec).is_some() {
            break;
        } else {
            seen.insert(vals_vec.clone());
            steps += 1;
        }
    }
    Ok(steps)
}

#[cfg(test)]
mod test {
    use super::reallocate_memory;

    #[test]
    fn reallocate() {
        assert_eq!(reallocate_memory("0 2 7 0").unwrap_or(0), 5);
    }
}
