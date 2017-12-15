//! Advent of Code - Day 15 'Dueling Generators' Solution
use error::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut factors: Vec<u64> = Vec::new();

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        factors.push(line.parse::<u64>()?);
    }

    let mut a = factors[0];
    let mut b = factors[1];
    let mut matches = 0;

    if second_star {
        for _ in 0..5_000_000 {
            let next_a = calculate_next(a, true)?;
            let next_b = calculate_next(b, false)?;
            if is_lowest16_match(next_a, next_b) {
                matches += 1;
            }
            a = next_a;
            b = next_b;
        }
    } else {
        for _ in 0..40_000_000 {
            let (next_a, next_b) = calculate_next_value(a, b)?;
            if is_lowest16_match(next_a, next_b) {
                matches += 1;
            }
            a = next_a;
            b = next_b;
        }
    }

    Ok(matches)
}

fn calculate_next(prev: u64, is_a: bool) -> Result<u64> {
    let next_x = if is_a { 16807 * prev } else { 48271 * prev };
    let next = next_x % 2147483647;

    let factor = if is_a { 4 } else { 8 };

    if next % factor == 0 {
        Ok(next)
    } else {
        Ok(calculate_next(next, is_a)?)
    }
}

fn calculate_next_value(prev_a: u64, prev_b: u64) -> Result<(u64, u64)> {
    let ax = 16807 * prev_a;
    let bx = 48271 * prev_b;

    let next_a = ax % 2147483647;
    let next_b = bx % 2147483647;
    Ok((next_a, next_b))
}

fn is_lowest16_match(a: u64, b: u64) -> bool {
    a & 0xFFFF == b & 0xFFFF
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(
            super::calculate_next_value(65, 8921).expect(""),
            (1092455, 430625591)
        );
        assert!(!super::is_lowest16_match(1092455, 430625591));
        assert_eq!(
            super::calculate_next_value(1092455, 430625591).expect(""),
            (1181022009, 1233683848)
        );
        assert!(!super::is_lowest16_match(1181022009, 1233683848));
        assert_eq!(
            super::calculate_next_value(1181022009, 1233683848).expect(""),
            (245556042, 1431495498)
        );
        assert!(super::is_lowest16_match(245556042, 1431495498));
        assert_eq!(
            super::calculate_next_value(245556042, 1431495498).expect(""),
            (1744312007, 137874439)
        );
        assert!(!super::is_lowest16_match(1744312007, 137874439));
        assert_eq!(
            super::calculate_next_value(1744312007, 137874439).expect(""),
            (1352636452, 285222916)
        );
        assert!(!super::is_lowest16_match(1352636452, 285222916));
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::calculate_next(65, true).expect(""), 1352636452);
        assert_eq!(super::calculate_next(8921, false).expect(""), 1233683848);
    }
}
