//! Advent of Code - Day 3 Solution
use error::Result;
use std::collections::HashMap;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let result = if second_star { count_houses_2(reader)? } else { count_houses(reader)? };

    Ok(result as u32)
}

fn count_houses<T: BufRead>(reader: T) -> Result<usize> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut house_map = HashMap::new();

    house_map.entry((x, y)).or_insert(1_usize);

    for line in reader.lines().filter_map(|x| x.ok()) {
        for ch in line.chars() {
            match ch {
                '^' => y += 1,
                '>' => x += 1,
                'v' => y -= 1,
                '<' => x -= 1,
                _ => return Err("invalid floor".into()),
            }

            *house_map.entry((x, y)).or_insert(0) += 1;
        }
    }

    Ok(house_map.iter().filter_map(|(_, y)| if *y > 0 { Some(*y) } else { None }).count())
}

fn count_houses_2<T: BufRead>(reader: T) -> Result<usize> {
    let mut s_x: isize = 0;
    let mut s_y: isize = 0;
    let mut r_x: isize = 0;
    let mut r_y: isize = 0;
    let mut is_santa = true;
    let mut house_map = HashMap::new();

    house_map.entry((s_x, s_y)).or_insert(1_usize);
    *house_map.entry((r_x, r_y)).or_insert(1_usize) += 1;

    for line in reader.lines().filter_map(|x| x.ok()) {
        for ch in line.chars() {
            match ch {
                '^' => {
                    if is_santa {
                        s_y += 1
                    } else {
                        r_y += 1
                    }
                }
                '>' => {
                    if is_santa {
                        s_x += 1
                    } else {
                        r_x += 1
                    }
                }
                'v' => {
                    if is_santa {
                        s_y -= 1
                    } else {
                        r_y -= 1
                    }
                }
                '<' => {
                    if is_santa {
                        s_x -= 1
                    } else {
                        r_x -= 1
                    }
                }
                _ => return Err("invalid floor".into()),
            }

            if is_santa {
                *house_map.entry((s_x, s_y)).or_insert(0) += 1;
            } else {
                *house_map.entry((r_x, r_y)).or_insert(0) += 1;
            }

            is_santa = !is_santa;
        }
    }

    Ok(house_map.iter().filter_map(|(_, y)| if *y > 0 { Some(*y) } else { None }).count())
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r">";
    const TEST_CHAIN_1: &str = r"^>v<";
    const TEST_CHAIN_2: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 2);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), false)?, 4);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_2), false)?, 2);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"^v";
    const TEST_CHAIN_1: &str = r"^>v<";
    const TEST_CHAIN_2: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 3);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), true)?, 3);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_2), true)?, 11);
        Ok(())
    }
}
