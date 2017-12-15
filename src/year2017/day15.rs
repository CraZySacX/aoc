//! Advent of Code - Day 15 'Dueling Generators' Solution
use error::Result;
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
    let next = (if is_a { 16807 * prev } else { 48271 * prev }) % 2147483647;

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
        assert_eq!(super::calculate_next(65, true, false).expect(""), 1092455);
        assert_eq!(
            super::calculate_next(8921, false, false).expect(""),
            430625591
        );
        assert_eq!(
            super::calculate_next(1092455, true, false).expect(""),
            1181022009
        );
        assert_eq!(
            super::calculate_next(430625591, false, false).expect(""),
            1233683848
        );
        assert_eq!(
            super::calculate_next(1181022009, true, false).expect(""),
            245556042
        );
        assert_eq!(
            super::calculate_next(1233683848, false, false).expect(""),
            1431495498
        );
        assert_eq!(
            super::calculate_next(245556042, true, false).expect(""),
            1744312007
        );
        assert_eq!(
            super::calculate_next(1431495498, false, false).expect(""),
            137874439
        );
        assert_eq!(
            super::calculate_next(1744312007, true, false).expect(""),
            1352636452
        );
        assert_eq!(
            super::calculate_next(137874439, false, false).expect(""),
            285222916
        );
        assert_eq!(
            super::check_x_for_matches(65, 8921, 40_000_000, false).expect(""),
            588
        );
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert_eq!(super::calculate_next(65, true, true).expect(""), 1352636452);
        assert_eq!(
            super::calculate_next(8921, false, true).expect(""),
            1233683848
        );
        assert_eq!(
            super::calculate_next(1352636452, true, true).expect(""),
            1992081072
        );
        assert_eq!(
            super::calculate_next(1233683848, false, true).expect(""),
            862516352
        );
        assert_eq!(
            super::calculate_next(1992081072, true, true).expect(""),
            530830436
        );
        assert_eq!(
            super::calculate_next(862516352, false, true).expect(""),
            1159784568
        );
        assert_eq!(
            super::calculate_next(530830436, true, true).expect(""),
            1980017072
        );
        assert_eq!(
            super::calculate_next(1159784568, false, true).expect(""),
            1616057672
        );
        assert_eq!(
            super::calculate_next(1980017072, true, true).expect(""),
            740335192
        );
        assert_eq!(
            super::calculate_next(1616057672, false, true).expect(""),
            412269392
        );
        assert_eq!(
            super::check_x_for_matches(65, 8921, 5_000_000, true).expect(""),
            309
        );
    }
}
