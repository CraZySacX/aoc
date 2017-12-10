//! Advent of Code - Day 1 Solution
use error::Result;
use std::io::BufRead;

/// Parse the input file and calculate the captcha.
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut result = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        result = val(line, second_star)?;
    }
    Ok(result)
}

/// Calculate the 'inverse captcha' value for a byte array.
fn val(input: &str, lookahead: bool) -> Result<u32> {
    let byte_arr = input.as_bytes();
    let len = byte_arr.len();
    let la_idx = if lookahead { len / 2 } else { 1 };

    let mut sum = 0;

    for (idx, curr) in byte_arr.iter().enumerate() {
        let next_idx = (idx + la_idx) % len;

        if *curr == byte_arr[next_idx] {
            sum += u32::from(byte_arr[idx]) - 48;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod one_star {
    use super::val;

    #[test]
    fn solution() {
        assert_eq!(val("1221", false).unwrap_or(0), 3);
        assert_eq!(val("1111", false).unwrap_or(0), 4);
        assert_eq!(val("1234", false).unwrap_or(0), 0);
        assert_eq!(val("91212129", false).unwrap_or(0), 9);
    }
}

#[cfg(test)]
mod two_star {
    use super::val;

    #[test]
    fn solution() {
        assert_eq!(val("1212", true).unwrap_or(0), 6);
        assert_eq!(val("1221", true).unwrap_or(0), 0);
        assert_eq!(val("123425", true).unwrap_or(0), 4);
        assert_eq!(val("123123", true).unwrap_or(0), 12);
    }
}
