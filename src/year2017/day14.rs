//! Advent of Code - Day 14 "Disk Defragmentation" Solution
use error::Result;
use std::convert::TryFrom;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut count = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());

        for i in 0..128 {
            let mut hash = Vec::new();
            let mut input = line.clone();
            input.push('-');
            input.push_str(&i.to_string());
            parse_list_and_hash(&mut hash, &input, 256, true)?;
            let hex = &squash_and_hex(&hash)?;
            count += count_ones(hex)?;
        }
    }
    Ok(TryFrom::try_from(count)?)
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

/// Convert to a binary string
fn count_ones(hex: &str) -> Result<usize> {
    let mut binary_str = String::new();
    for c in hex.chars() {
        let binary = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => return Err("Invalid hex digit".into()),
        };
        binary_str.push_str(binary);
    }
    let count: String = binary_str.chars().filter(|c| *c == '1').collect();
    Ok(count.len())
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        let line = String::from("flqrgnkx");
        let mut count = 0;
        for i in 0..128 {
            let mut hash = Vec::new();
            let mut input = line.clone();
            input.push('-');
            input.push_str(&i.to_string());
            super::parse_list_and_hash(&mut hash, &input, 256, true).expect("");
            let hex = &super::squash_and_hex(&hash).expect("");
            count += super::count_ones(&hex).expect("");
        }
        assert_eq!(count, 8108);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
