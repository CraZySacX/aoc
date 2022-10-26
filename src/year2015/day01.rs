//! Advent of Code - Day 1 "Not Quite Lisp" Solution
use error::Result;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    if second_star {
        let found_at = find_basement(reader)?;
        println!("{found_at}");
    } else {
        let floor = find_floor(reader)?;
        println!("{floor}");
    }
    Ok(0)
}

fn find_floor<T: BufRead>(reader: T) -> Result<isize> {
    let mut floor: isize = 0;

    for line in reader.lines().filter_map(|x| x.ok()) {
        for ch in line.chars() {
            match ch {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => return Err("invalid floor".into()),
            }
        }
    }

    Ok(floor)
}

fn find_basement<T: BufRead>(reader: T) -> Result<u32> {
    let mut floor = 0;
    let mut idx = 1;

    'outer: for line in reader.lines().filter_map(|x| x.ok()) {
        for ch in line.chars() {
            match ch {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => return Err("invalid floor".into()),
            }
            if floor == -1 {
                break 'outer;
            }
            idx += 1;
        }
    }

    Ok(idx)
}

#[cfg(test)]
mod one_star {
    use super::find_floor;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"(())";
    const TEST_CHAIN_1: &str = r"()()";
    const TEST_CHAIN_2: &str = r"(((";
    const TEST_CHAIN_3: &str = r"(()(()(";
    const TEST_CHAIN_4: &str = r"))(((((";
    const TEST_CHAIN_5: &str = r"())";
    const TEST_CHAIN_6: &str = r"))(";
    const TEST_CHAIN_7: &str = r")))";
    const TEST_CHAIN_8: &str = r")())())";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN))?, 0);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_1))?, 0);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_2))?, 3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_3))?, 3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_4))?, 3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_5))?, -1);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_6))?, -1);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_7))?, -3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_8))?, -3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_basement;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r")";
    const TEST_CHAIN_1: &str = r"()())";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_basement(Cursor::new(TEST_CHAIN))?, 1);
        assert_eq!(find_basement(Cursor::new(TEST_CHAIN_1))?, 5);
        Ok(())
    }
}
