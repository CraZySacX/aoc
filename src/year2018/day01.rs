//! Advent of Code - Day 1 "Chronal Calibration" Solution
use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut acc_vec = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        let chars: Vec<char> = line.chars().collect();
        let num_str: String = chars[1..].iter().collect();
        let num_to_add = num_str.parse::<i32>()?;
        let tuple = (chars[0] == '+', num_to_add);
        acc_vec.push(tuple);
    }
    let acc = val(&acc_vec, second_star)?;
    println!("Result: {acc}");
    Ok(0)
}

/// Calculate the 'inverse captcha' value for a byte array.
fn val(acc_vec: &[(bool, i32)], second_star: bool) -> Result<i32> {
    if second_star {
        let mut result_set = HashSet::new();
        result_set.insert(0);
        let mut acc = 0;
        loop {
            for (add, val) in acc_vec {
                if *add {
                    acc += val
                } else {
                    acc -= val
                }

                if !result_set.insert(acc) {
                    return Ok(acc);
                }
            }
        }
    } else {
        Ok(acc_vec.iter().fold(0, |acc, x| if x.0 { acc + x.1 } else { acc - x.1 }))
    }
}

#[cfg(test)]
mod one_star {
    use super::val;
    use anyhow::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(val(&[(true, 1), (true, 1), (true, 1)], false)?, 3);
        assert_eq!(val(&[(true, 1), (true, 1), (false, 2)], false)?, 0);
        assert_eq!(val(&[(false, 1), (false, 2), (false, 3)], false)?, -6);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::val;
    use anyhow::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(val(&[(true, 1), (false, 1)], true)?, 0);
        assert_eq!(val(&[(true, 3), (true, 3), (true, 4), (false, 2), (false, 4)], true)?, 10);
        assert_eq!(val(&[(false, 6), (true, 3), (true, 8), (true, 5), (false, 6)], true)?, 5);
        assert_eq!(val(&[(true, 7), (true, 7), (false, 2), (false, 7), (false, 4)], true)?, 14);
        Ok(())
    }
}
