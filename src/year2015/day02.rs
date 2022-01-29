//! Advent of Code - Day 2 Solution
use error::Result;
use regex::Regex;
use std::cmp::min;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r"(\d+)x(\d+)x(\d+)")?;
    let mut answer = 0;

    for line in reader.lines().filter_map(|x| x.ok()) {
        let mut length = 0;
        let mut width = 0;
        let mut height = 0;
        for cap in line_re.captures_iter(&line) {
            length = (cap[1]).parse::<usize>()?;
            width = (cap[2]).parse::<usize>()?;
            height = (cap[3]).parse::<usize>()?;
        }

        if second_star {
            let mut tmp = vec![length, width, height];
            tmp.sort_unstable();

            answer += 2 * tmp[0] + 2 * tmp[1] + length * width * height;
        } else {
            let lw = length * width;
            let wh = width * height;
            let hl = height * length;

            answer += (2 * lw) + (2 * wh) + (2 * hl) + min(hl, min(lw, wh));
        }
    }

    Ok(answer as u32)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"2x3x4";
    const TEST_CHAIN_1: &str = r"1x1x10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 58);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), false)?, 43);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"2x3x4";
    const TEST_CHAIN_1: &str = r"1x1x10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 34);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), true)?, 14);
        Ok(())
    }
}
