//! Advent of Code - Day 1 Solution

/// Calculate the 'inverse captcha' value for a byte array.
pub fn val(input: &str, lookahead: bool) -> u64 {
    let byte_arr = input.as_bytes();
    let len = byte_arr.len();
    let la_idx = if lookahead { len / 2 } else { 1 };

    let mut sum: u64 = 0;

    for (idx, curr) in byte_arr.iter().enumerate() {
        let next_idx = (idx + la_idx) % len;

        if *curr == byte_arr[next_idx] {
            sum += u64::from(byte_arr[idx]) - 48;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::val;

    #[test]
    fn default_lookahead_of_1() {
        assert_eq!(val("1221", false), 3);
        assert_eq!(val("1111", false), 4);
        assert_eq!(val("1234", false), 0);
        assert_eq!(val("91212129", false), 9);
    }

    #[test]
    fn lookahead_of_half_len() {
        assert_eq!(val("1212", true), 6);
        assert_eq!(val("1221", true), 0);
        assert_eq!(val("123425", true), 4);
        assert_eq!(val("123123", true), 12);
    }
}
