//! Advent of Code - Day 15 'Dueling Generators' Solution
use anyhow::Result;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut factors: Vec<u64> = Vec::new();

    for line_result in reader.lines() {
        let line = &line_result?;
        factors.push(line.parse::<u64>()?);
    }

    Ok(if second_star {
        check_x_for_matches(factors[0], factors[1], 5_000_000, second_star)?
    } else {
        check_x_for_matches(factors[0], factors[1], 40_000_000, second_star)?
    })
}

/// Check `x` results for matches.
fn check_x_for_matches(init_a: u64, init_b: u64, count: u32, second_star: bool) -> Result<u32> {
    let mut a = init_a;
    let mut b = init_b;
    let mut matches = 0;

    for _ in 0..count {
        if a & 0xFFFF == b & 0xFFFF {
            matches += 1;
        }
        a = calculate_next(a, true, second_star)?;
        b = calculate_next(b, false, second_star)?;
    }

    Ok(matches)
}

/// Calculate the next value for `A` or `B`.
fn calculate_next(prev: u64, is_a: bool, is_second_star: bool) -> Result<u64> {
    let next = (if is_a { 16_807 * prev } else { 48_271 * prev }) % 2_147_483_647;

    if is_second_star {
        if next % if is_a { 4 } else { 8 } == 0 {
            Ok(next)
        } else {
            Ok(calculate_next(next, is_a, is_second_star)?)
        }
    } else {
        Ok(next)
    }
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert_eq!(super::calculate_next(65, true, false).expect(""), 1_092_455);
        assert_eq!(super::calculate_next(8921, false, false).expect(""), 430_625_591);
        assert_eq!(super::calculate_next(1_092_455, true, false).expect(""), 1_181_022_009);
        assert_eq!(super::calculate_next(430_625_591, false, false).expect(""), 1_233_683_848);
        assert_eq!(super::calculate_next(1_181_022_009, true, false).expect(""), 245_556_042);
        assert_eq!(super::calculate_next(1_233_683_848, false, false).expect(""), 1_431_495_498);
        assert_eq!(super::calculate_next(245_556_042, true, false).expect(""), 1_744_312_007);
        assert_eq!(super::calculate_next(1_431_495_498, false, false).expect(""), 137_874_439);
        assert_eq!(super::calculate_next(1_744_312_007, true, false).expect(""), 1_352_636_452);
        assert_eq!(super::calculate_next(137_874_439, false, false).expect(""), 285_222_916);
        assert_eq!(super::check_x_for_matches(65, 8921, 40_000_000, false).expect(""), 588);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::calculate_next(65, true, true).expect(""), 1_352_636_452);
        assert_eq!(super::calculate_next(8921, false, true).expect(""), 1_233_683_848);
        assert_eq!(super::calculate_next(1_352_636_452, true, true).expect(""), 1_992_081_072);
        assert_eq!(super::calculate_next(1_233_683_848, false, true).expect(""), 862_516_352);
        assert_eq!(super::calculate_next(1_992_081_072, true, true).expect(""), 530_830_436);
        assert_eq!(super::calculate_next(862_516_352, false, true).expect(""), 1_159_784_568);
        assert_eq!(super::calculate_next(530_830_436, true, true).expect(""), 1_980_017_072);
        assert_eq!(super::calculate_next(1_159_784_568, false, true).expect(""), 1_616_057_672);
        assert_eq!(super::calculate_next(1_980_017_072, true, true).expect(""), 740_335_192);
        assert_eq!(super::calculate_next(1_616_057_672, false, true).expect(""), 412_269_392);
        assert_eq!(super::check_x_for_matches(65, 8921, 5_000_000, true).expect(""), 309);
    }
}
