//! Advent of Code - Day 10 Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut result = 0;
    let mut hash = Vec::new();
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        parse_list_and_hash(&mut hash, line, 256, second_star)?;

        if second_star {
            writeln!(io::stdout(), "{}", squash_and_hex(&hash)?)?;
        } else {
            result = hash[0] * hash[1];
        }
    }
    Ok(result)
}

/// Parse the list of lengths and calculate the hash.
fn parse_list_and_hash(hash: &mut Vec<u32>, line: &str, num_elements: u32, second_star: bool) -> Result<()> {
    let lengths = generate_lengths(line, second_star)?;

    for i in 0..num_elements {
        hash.push(i);
    }

    let rounds = if second_star { 64 } else { 1 };
    let mut curr_pos: u32 = 0;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for length in &lengths {
            let mut indices = Vec::new();
            let mut slice = Vec::new();

            for j in curr_pos..u32::from(*length) + curr_pos {
                let actual_idx = j % num_elements;
                indices.push(actual_idx);
                slice.push(hash.get(actual_idx as usize).ok_or("invalid")?.clone());
            }

            slice.reverse();
            for (idx, val) in indices.iter().zip(slice.iter()) {
                *hash.get_mut(*idx as usize).ok_or("invalid")? = *val;
            }

            curr_pos = (curr_pos + u32::from(*length) + skip_size) % num_elements;
            skip_size += 1;
        }
    }

    Ok(())
}

/// Generate the list of lengths.
fn generate_lengths(line: &str, second_star: bool) -> Result<Vec<u8>> {
    let mut lengths = Vec::new();

    if second_star {
        if !line.is_empty() {
            lengths.extend(line.as_bytes());
        }
        lengths.extend(vec![17, 31, 73, 47, 23]);
    } else {
        let length_strs: Vec<&str> = line.split(',').collect();
        for length_str in length_strs {
            lengths.push(length_str.parse::<u8>()?);
        }
    }

    Ok(lengths)
}

/// Create dense hash and hexify
fn squash_and_hex(hash: &[u32]) -> Result<String> {
    let chunks = hash.chunks(16);
    let mut byte_vec = Vec::new();

    for chunk in chunks {
        let val = chunk.iter().fold(0, |acc, x| x ^ acc);
        byte_vec.push(val);
    }

    let mut result = String::new();
    for byte in byte_vec {
        result.push_str(&format!("{:02x}", byte));
    }
    Ok(result)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        let mut hash = Vec::new();
        super::parse_list_and_hash(&mut hash, "3,4,1,5", 5, false).expect("");
        assert_eq!(hash[0] * hash[1], 12);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        let mut hash = Vec::new();
        super::parse_list_and_hash(&mut hash, "", 256, true).expect("");
        assert_eq!(
            super::squash_and_hex(&hash).expect(""),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        hash.clear();
        super::parse_list_and_hash(&mut hash, "AoC 2017", 256, true).expect("");
        assert_eq!(
            super::squash_and_hex(&hash).expect(""),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        hash.clear();
        super::parse_list_and_hash(&mut hash, "1,2,3", 256, true).expect("");
        assert_eq!(
            super::squash_and_hex(&hash).expect(""),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        hash.clear();
        super::parse_list_and_hash(&mut hash, "1,2,4", 256, true).expect("");
        assert_eq!(
            super::squash_and_hex(&hash).expect(""),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
