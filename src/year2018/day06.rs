//! Advent of Code - Day 6 "Chronal Coordinates" Solution
use error::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r#"(\d+), (\d+)"#)?;
    let mut coords: Vec<(i32, i32)> = Vec::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            for cap in line_re.captures_iter(&line) {
                let x = &cap[1].parse::<i32>()?;
                let y = &cap[2].parse::<i32>()?;
                coords.push((*x, *y));
            }
        }
    }

    let (max_x, max_y) = max_coords(&coords);
    // let coords_iter = coords.iter();

    for x in 0..=max_x {
        for y in 0..=max_y {
            let closest = find_closest((x, y), &coords);
            println!("Closest: {:?}", closest);
        }
    }
    Ok(0)
}

fn find_closest(point: (i32, i32), coords: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let distances: HashMap<usize, i32> = coords.iter().enumerate().map(|(idx, coord)| (idx, manhattan_distance(point, *coord))).collect();
    let mut min = i32::max_value();
    let mut result = Vec::new();
    for (idx, distance) in distances {
        if distance < min {
            result.clear();
            result.push(coords[idx]);
            min = distance;
        } else if distance == min {
            result.push(coords[idx]);
        }
    }
    result
}

/// Calculate the manhattan distance between two (x,y) tuples.
fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn max_coords(coords: &[(i32, i32)]) -> (i32, i32) {
    let max_x = coords.iter().max_by_key(|(x,_)| x).unwrap_or_else(|| &(0,0)).0;
    let max_y = coords.iter().max_by_key(|(_,y)| y).unwrap_or_else(|| &(0,0)).1;
    (max_x, max_y)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 17);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find_solution;
    use error::Result;
    // use std::io::Cursor;

    //     const TEST_CHAIN: &str = r"1, 1
    // 1, 6
    // 8, 3
    // 3, 4
    // 5, 5
    // 8, 9";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 4);
        Ok(())
    }
}
