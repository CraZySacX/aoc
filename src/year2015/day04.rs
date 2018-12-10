//! Advent of Code - Day 4 Solution
use error::Result;
use md5;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let result = if _second_star {
        find_lowest(reader, "000000")?
    } else {
        find_lowest(reader, "00000")?
    };
    Ok(result)
}

fn find_lowest<T: BufRead>(reader: T, start_str: &str) -> Result<u32> {
    let mut count = 1;
    for line in reader.lines().filter_map(|x| x.ok()) {
        loop {
            let check = format!("{}{}", line, count);
            let digest = md5::compute(check.as_bytes());
            let digest_str = format!("{:x}", digest);

            if digest_str.starts_with(start_str) {
                break;
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"abcdef";
    const TEST_CHAIN_1: &str = r"pqrstuv";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 609_043);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), false)?, 1_048_970);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
