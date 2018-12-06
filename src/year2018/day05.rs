use error::Result;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if second_star {
                let mut results = HashMap::new();
                for lower in 97..=122 {
                    let upper = lower - 32;
                    let bytes_vec = line.as_bytes().to_vec();
                    let mut filtered: Vec<u8> = bytes_vec.iter().filter(|x| **x != lower && **x != upper).cloned().collect();

                    results.insert(lower, collapse_polymer(&mut filtered));
                }

                if let Some(min) = results.values().min() {
                    return Ok(*min);
                } else {
                    return Err("unable to find minimum".into());
                }
            } else {
                return Ok(collapse_polymer(&mut line.as_bytes().to_vec()));
            }
        }
    }
    Err("unable to parse input".into())
}

fn collapse_polymer(bytes: &mut Vec<u8>) -> u32 {
    'outer: loop {
        let cloned = bytes.clone();
        for i in 0..bytes.len() {
            if let Some(first) = cloned.get(i) {
                if let Some(second) = cloned.get(i + 1) {
                    let max = max(first, second);
                    let min = min(first, second);

                    if max - min == 32 {
                        bytes.remove(i);
                        bytes.remove(i);
                        break;
                    }
                } else {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
    }

    bytes.len() as u32
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 10);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 4);
        Ok(())
    }
}
