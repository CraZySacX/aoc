//! Advent of Code - Day 6 "Chronal Coordinates" Solution
use error::Result;
use indexmap::IndexSet;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
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

    if second_star {
        let d_to_check = if coords.len() == 6 { 32 } else { 10000 };
        let mut less_than_d = 0;
        for y in 0..=max_y {
            for x in 0..=max_x {
                let total_of_mds = total_of_mds((x, y), &coords);

                if total_of_mds < d_to_check {
                    less_than_d += 1;
                }
            }
        }
        Ok(less_than_d)
    } else {
        let mut md_map = BTreeMap::new();
        let mut on_boundary = IndexSet::new();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let closest = find_closest((x, y), &coords);

                if closest.len() == 1 {
                    md_map.insert((x, y), closest[0]);

                    if x == 0 || y == 0 || x == max_x || y == max_y {
                        on_boundary.insert(closest[0]);
                    }
                }
            }
        }

        let rest: BTreeMap<(i32, i32), (i32, i32)> = md_map
            .iter()
            .filter(|(_, closest)| !on_boundary.contains(*closest))
            .map(|(x, v)| (*x, *v))
            .collect();

        let mut frequency: HashMap<(i32, i32), u32> = HashMap::new();
        for (_, bounded_closest) in rest {
            *frequency.entry(bounded_closest).or_insert(0) += 1;
        }

        let max = frequency.iter().max_by_key(|(_, x)| *x).map(|(_, x)| *x).ok_or_else(|| "no maximum")?;
        Ok(max)
    }
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

fn total_of_mds(point: (i32, i32), coords: &[(i32, i32)]) -> u32 {
    let sum: i32 = coords.iter().map(|coord| manhattan_distance(point, *coord)).sum();
    sum as u32
}

/// Calculate the manhattan distance between two (x,y) tuples.
fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn max_coords(coords: &[(i32, i32)]) -> (i32, i32) {
    let max_x = coords.iter().max_by_key(|(x, _)| x).unwrap_or_else(|| &(0, 0)).0;
    let max_y = coords.iter().max_by_key(|(_, y)| y).unwrap_or_else(|| &(0, 0)).1;
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
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 16);
        Ok(())
    }
}
