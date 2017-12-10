//! Advent of Code - Day 10 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut result = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        result = parse_list_and_hash(line, 256)?;
    }
    Ok(result)
}

/// Parse the list of lengths and calculate the hash.
fn parse_list_and_hash(line: &str, num_elements: u32) -> Result<u32> {
    let length_strs: Vec<&str> = line.split(',').collect();
    let mut lengths = Vec::new();
    let mut hash = Vec::new();

    for i in 0..num_elements {
        hash.push(i);
    }

    for length_str in length_strs {
        lengths.push(length_str.parse::<u32>()?);
    }

    let mut curr_pos = 0;
    let mut skip_size = 0;

    for length in lengths {
        let mut indices = Vec::new();
        let mut slice = Vec::new();

        for j in curr_pos..length + curr_pos {
            let actual_idx = j % num_elements;
            indices.push(actual_idx);
            slice.push(hash.get(actual_idx as usize).ok_or("invalid")?.clone());
        }

        slice.reverse();
        for (idx, val) in indices.iter().zip(slice.iter()) {
            *hash.get_mut(*idx as usize).ok_or("invalid")? = *val;
        }

        curr_pos = (curr_pos + length + skip_size) % num_elements;
        skip_size += 1;
    }

    Ok(hash[0] * hash[1])
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::parse_list_and_hash("3,4,1,5", 5).unwrap_or(0), 12);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
