//! Advent of Code - Day 8 "Memory Manuver" Solution
use error::Result;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut license_vec = Vec::new();
    for line in reader.lines().filter_map(|x| x.ok()) {
        for tok in line.split(' ').map(|x| x.parse::<u32>()).filter_map(|x| x.ok()) {
            license_vec.push(tok);
        }
    }

    license_vec.reverse();

    Ok(recurse(&mut license_vec, second_star)?)
}

fn recurse(license_vec: &mut Vec<u32>, second_star: bool) -> Result<u32> {
    let children_count = license_vec.pop().ok_or_else(|| "")?;
    let metadata_count = license_vec.pop().ok_or_else(|| "")?;
    let mut result = 0;

    if second_star {
        let mut children_values = Vec::new();

        for _ in 0..children_count {
            children_values.push(recurse(license_vec, second_star)?);
        }

        for _ in 0..metadata_count {
            let metadata = license_vec.pop().ok_or_else(|| "")?;
            if children_count == 0 {
                result += metadata;
            } else {
                result += children_values.get(metadata as usize - 1).unwrap_or(&0);
            }
        }
    } else {
        for _ in 0..children_count {
            result += recurse(license_vec, second_star)?;
        }

        for _ in 0..metadata_count {
            result += license_vec.pop().ok_or_else(|| "")?;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 138);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 66);
        Ok(())
    }
}
