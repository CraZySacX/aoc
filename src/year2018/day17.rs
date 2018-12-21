//! Advent of Code - Day 17 "Reservoir Research" Solution
use error::Result;
use ndarray::Array2;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;

enum SoilKind {
    Sand,
    Clay,
    Water,
}

impl Default for SoilKind {
    fn default() -> Self {
        SoilKind::Sand
    }
}

impl fmt::Display for SoilKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            SoilKind::Water => "+",
            SoilKind::Clay => "#",
            SoilKind::Sand => ".",
        })
    }
}

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let vein_re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)")?;
    let mut x_coord_map = HashMap::new();
    let mut y_coord_map = HashMap::new();

    for line in reader.lines().filter_map(|x| x.ok()) {
        for caps in vein_re.captures_iter(&line) {
            let c1 = (&caps[1]).to_string();
            let v1 = (&caps[2]).parse::<usize>()?;
            let r1 = (&caps[4]).parse::<usize>()?;
            let r2 = (&caps[5]).parse::<usize>()?;

            match &c1[..] {
                "x" => {
                    let range_vec = x_coord_map.entry(v1).or_insert_with(|| vec![]);

                    for i in r1..=r2 {
                        range_vec.push(i);
                    }
                },
                "y" => {
                    let range_vec = y_coord_map.entry(v1).or_insert_with(|| vec![]);

                    for i in r1..=r2 {
                        range_vec.push(i);
                    }
                },
                _ => return Err("invalid coordinate".into()),
            }
        }
    }

    let mut min_x = *x_coord_map.keys().min().ok_or_else(|| "no min x")?;
    let mut max_x = *x_coord_map.keys().max().ok_or_else(|| "no max x")?;
    let min_y = 0;
    let mut max_y = *y_coord_map.keys().max().ok_or_else(|| "no max y")?;

    for yv in x_coord_map.values() {
        for y in yv {
            if *y > max_y {
                max_y = *y;
            }
        }
    }

    for xv in y_coord_map.values() {
        for x in xv {
            if *x > max_x {
                max_x = *x;
            }

            if *x < min_x {
                min_x = *x;
            }
        }
    }

    min_x = min_x.checked_sub(1).ok_or_else(|| "underflow x")?;
    max_x = max_x.checked_add(2).ok_or_else(|| "overflow x")?;
    max_y = max_y.checked_add(1).ok_or_else(|| "overflow y")?;

    let mut clay_arr = Array2::<SoilKind>::default((max_x, max_y));
    clay_arr[[500, 0]] = SoilKind::Water;

    for (i, jv) in &x_coord_map {
        for j in jv {
            clay_arr[[*i, *j]] = SoilKind::Clay;
        }
    }

    for (j, iv) in &y_coord_map {
        for i in iv {
            clay_arr[[*i, *j]] = SoilKind::Clay;
        }
    }

    println!();
    for j in min_y..max_y {
        for i in min_x..max_x {
            print!("{}", clay_arr[[i, j]]);
        }
        println!();
    }
    Ok(1)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CODE), false)?, 1);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CODE), true)?, 1);
        Ok(())
    }
}
