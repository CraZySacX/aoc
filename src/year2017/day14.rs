//! Advent of Code - Day 14 "Disk Defragmentation" Solution

use crate::utils::PrivateTryFromUsize;
use anyhow::{anyhow, Result};
use ndarray::Array2;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut count = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        let mut disk_arr: Array2<u32> = Array2::zeros((128, 128));
        let mut visited: Array2<bool> = Array2::from_elem((128, 128), false);

        for i in 0..128 {
            let mut hash = Vec::new();
            let mut input = line.clone();
            input.push('-');
            input.push_str(&i.to_string());
            parse_list_and_hash(&mut hash, &input, 256, true)?;
            let hex = &squash_and_hex(&hash)?;

            if second_star {
                fill_row(i, hex, &mut disk_arr)?;
            } else {
                let binary_str = to_binary_string(hex)?;
                let ones_str: String = binary_str.chars().filter(|c| *c == '1').collect();
                count += ones_str.len();
            }
        }

        if second_star {
            for i in 0..128 {
                for j in 0..128 {
                    if disk_arr[[i, j]] == 1 && !visited[[i, j]] {
                        // Do some DFS
                        depth_first_search(i, j, &mut disk_arr, &mut visited)?;
                        count += 1;
                    }
                }
            }
        }
    }
    u32::private_try_from(count)
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
                slice.push(*hash.get(actual_idx as usize).ok_or(anyhow!("invalid"))?);
            }

            slice.reverse();
            for (idx, val) in indices.iter().zip(slice.iter()) {
                *hash.get_mut(*idx as usize).ok_or(anyhow!("invalid"))? = *val;
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
    use std::fmt::Write;

    let chunks = hash.chunks(16);
    let mut byte_vec = Vec::new();

    for chunk in chunks {
        let val = chunk.iter().fold(0, |acc, x| x ^ acc);
        byte_vec.push(val);
    }

    let mut result = String::new();
    for byte in byte_vec {
        write!(result, "{byte:02x}").expect("Unable to write string");
    }
    Ok(result)
}

/// Convert to a binary string
fn to_binary_string(hex: &str) -> Result<String> {
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
            _ => return Err(anyhow!("Invalid hex digit")),
        };
        binary_str.push_str(binary);
    }
    Ok(binary_str)
}

/// Fill the given row in the array
fn fill_row(row: usize, hex: &str, arr: &mut Array2<u32>) -> Result<()> {
    let binary_str = to_binary_string(hex)?;
    for (idx, c) in binary_str.chars().enumerate() {
        let bit = c.to_string().parse::<u32>()?;
        arr[[row, idx]] = bit;
    }
    Ok(())
}

/// Depth first search for adjacent neighbors
fn depth_first_search(row: usize, col: usize, disk_arr: &mut Array2<u32>, visited: &mut Array2<bool>) -> Result<()> {
    visited[[row, col]] = true;

    let row_deltas: Vec<isize> = vec![-1, 0, 0, 1];
    let col_deltas: Vec<isize> = vec![0, -1, 1, 0];
    let row_i: isize = TryFrom::try_from(row)?;
    let col_i: isize = TryFrom::try_from(col)?;

    // Check the four adjacent neighbors (left, down, up, right)
    for k in 0..4 {
        if let Ok(adj_row) = TryFrom::try_from(row_i + row_deltas[k]) {
            if let Ok(adj_col) = TryFrom::try_from(col_i + col_deltas[k]) {
                if adj_row < 128 && adj_col < 128 && disk_arr[[adj_row, adj_col]] == 1 && !visited[[adj_row, adj_col]] {
                    depth_first_search(adj_row, adj_col, disk_arr, visited)?;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    Ok(())
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        let line = String::from("flqrgnkx");
        let mut count = 0;
        let mut hash = Vec::new();
        for i in 0..128 {
            hash.clear();
            let mut input = line.clone();
            input.push('-');
            input.push_str(&i.to_string());
            super::parse_list_and_hash(&mut hash, &input, 256, true).expect("");
            let hex = &super::squash_and_hex(&hash).expect("");
            let binary_str = super::to_binary_string(hex).expect("");
            let ones_str: String = binary_str.chars().filter(|c| *c == '1').collect();
            count += ones_str.len();
        }
        assert_eq!(count, 8108);
    }
}

#[cfg(test)]
mod two_star {
    use ndarray::Array2;

    #[test]
    fn solution() {
        let line = String::from("flqrgnkx");
        let mut count = 0;
        let mut hash = Vec::new();
        let mut disk_arr: Array2<u32> = Array2::zeros((128, 128));
        let mut visited: Array2<bool> = Array2::from_elem((128, 128), false);

        for i in 0..128 {
            hash.clear();
            let mut input = line.clone();
            input.push('-');
            input.push_str(&i.to_string());
            super::parse_list_and_hash(&mut hash, &input, 256, true).expect("");
            let hex = &super::squash_and_hex(&hash).expect("");
            super::fill_row(i, hex, &mut disk_arr).expect("");
        }

        for i in 0..128 {
            for j in 0..128 {
                if disk_arr[[i, j]] == 1 && !visited[[i, j]] {
                    // Do some DFS
                    super::depth_first_search(i, j, &mut disk_arr, &mut visited).expect("");
                    count += 1;
                }
            }
        }
        assert_eq!(count, 1242);
    }
}
