//! Advent of Code - Day 11 "Chronal Charge" Solution
use error::Result;
use std::collections::HashMap;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut pl_map: HashMap<(isize, isize), isize> = HashMap::new();

    let mut serial_number = 0;

    for line in reader.lines().filter_map(|x| x.ok()) {
        serial_number = line.parse::<isize>()?;
    }

    for y in 1..=300 {
        for x in 1..=300 {
            pl_map.insert((x, y), find_cell_power(x, y, serial_number)?);
        }
    }

    if second_star {
        let mut result_map: HashMap<(isize, isize, isize), isize> = HashMap::new();

        for size in 1..=300 {
            println!("Checking size {}", size);
            for y in 1..=(300 - size + 1) {
                for x in 1..=(300 - size + 1) {
                    find_sum_at_size(x, y, size, &pl_map, &mut result_map)?;
                }
            }
        }
        let (max_x, max_y, max_size) = result_map.iter().max_by_key(|(_, pl)| *pl).map(|((x, y, size), _)| (*x, *y, *size)).ok_or_else(|| "")?;
        println!("{},{},{}", max_x, max_y, max_size);
    } else {
        let mut result_map: HashMap<(isize, isize), isize> = HashMap::new();

        for y in 1..=298 {
            for x in 1..=298 {
                result_map.insert((x, y), find_3x3_sum(x, y, &pl_map)?);
            }
        }
        let (max_x, max_y) = result_map.iter().max_by_key(|(_, pl)| *pl).map(|((x, y), _)| (*x, *y)).ok_or_else(|| "")?;
        println!("{},{}", max_x, max_y);
    }

    Ok(0)
}

fn find_cell_power(x: isize, y: isize, serial_number: isize) -> Result<isize> {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = if power_level > 100 { (power_level / 100) % 10 } else { 0 };
    power_level -= 5;
    Ok(power_level)
}

fn find_3x3_sum(x: isize, y: isize, pl_map: &HashMap<(isize, isize), isize>) -> Result<isize> {
    let mut power_level = 0;
    for ny in y..y + 3 {
        for nx in x..x + 3 {
            power_level += pl_map.get(&(nx, ny)).ok_or_else(|| "")?;
        }
    }
    Ok(power_level)
}

fn find_sum_at_size(x: isize, y: isize, size: isize, pl_map: &HashMap<(isize, isize), isize>, result_map: &mut HashMap<(isize, isize, isize), isize>) -> Result<()> {
    if size == 1 {
        for ny in y..y + size {
            for nx in x..x + size {
                result_map.insert((x, y, size), *pl_map.get(&(nx, ny)).ok_or_else(|| "")?);
            }
        }
    } else {
        // println!("checking {},{} at {}", x, y, size);
        let previous_map = result_map.clone();
        let mut power_level = *previous_map.get(&(x, y, size - 1)).ok_or_else(|| "no prev")?;

        // println!("prev pl: {}", power_level);
        // Add the last column
        let last_x = x + size - 1;

        for ny in y..y + size {
            power_level += *pl_map.get(&(last_x, ny)).ok_or_else(|| "bad col")?;
        }

        // println!("after last col: {}", power_level);
        // Add the bottom row - last cell (included in last column calc)
        let last_y = y + size - 1;

        for nx in x..x + size - 1 {
            power_level += *pl_map.get(&(nx, last_y)).ok_or_else(|| "bad row")?;
        }

        // println!("after last row: {}", power_level);

        result_map.insert((x, y, size), power_level);
    }

    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::{find_cell_power, find_solution};
    use error::Result;
    use std::io::Cursor;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_cell_power(3, 5, 8)?, 4);
        assert_eq!(find_cell_power(122, 79, 57)?, -5);
        assert_eq!(find_cell_power(217, 196, 39)?, 0);
        assert_eq!(find_cell_power(101, 153, 71)?, 4);
        assert_eq!(find_solution(Cursor::new("18"), false)?, 0);
        assert_eq!(find_solution(Cursor::new("42"), false)?, 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new("18"), true)?, 0);
        assert_eq!(find_solution(Cursor::new("42"), true)?, 0);
        Ok(())
    }
}