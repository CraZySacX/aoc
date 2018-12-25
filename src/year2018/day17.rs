//! Advent of Code - Day 17 "Reservoir Research" Solution
use error::Result;
use ndarray::Array2;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;

#[derive(Clone, Debug, Eq, PartialEq)]
enum SoilKind {
    Clay,
    FlowingWater,
    Sand,
    SettledWater,
    Spring,
}

impl Default for SoilKind {
    fn default() -> Self {
        SoilKind::Sand
    }
}

impl fmt::Display for SoilKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SoilKind::Clay => "#",
                SoilKind::FlowingWater => "|",
                SoilKind::Sand => ".",
                SoilKind::SettledWater => "~",
                SoilKind::Spring => "+",
            }
        )
    }
}

#[derive(Clone, Default)]
struct Soil {
    kind: SoilKind,
    moved: bool,
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let result = run_scan(reader, second_star, false)?;
    Ok(result as u32)
}

fn run_scan<T: BufRead>(reader: T, _second_star: bool, test: bool) -> Result<usize> {
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
                }
                "y" => {
                    let range_vec = y_coord_map.entry(v1).or_insert_with(|| vec![]);

                    for i in r1..=r2 {
                        range_vec.push(i);
                    }
                }
                _ => return Err("invalid coordinate".into()),
            }
        }
    }

    let mins_maxes = calculate_mins_maxes(&x_coord_map, &y_coord_map)?;
    let mut scan_arr = setup_scan(mins_maxes, &x_coord_map, &y_coord_map, test);

    for _ in 0..17 {
        drip(mins_maxes, &mut scan_arr, test)?;
    }

    Ok(1)
}

fn drip(mins_maxes: (usize, usize, usize, usize), scan_arr: &mut Array2<Soil>, test: bool) -> Result<()> {
    move_flowing_water(mins_maxes, scan_arr)?;

    if scan_arr[[500, 1]].kind == SoilKind::Sand {
        scan_arr[[500, 1]].kind = SoilKind::FlowingWater;
    }

    if test {
        print_scan_arr(mins_maxes, &scan_arr);
    }

    Ok(())
}

fn move_flowing_water(mins_maxes: (usize, usize, usize, usize), scan_arr: &mut Array2<Soil>) -> Result<()> {
    let (_, max_i, _, max_j) = mins_maxes;

    let mut flowing_water: Vec<[usize; 2]> = scan_arr
        .indexed_iter()
        .filter_map(|(x, y)| if y.kind == SoilKind::FlowingWater { Some(x) } else { None })
        .map(|x| [x.0, x.1])
        .collect();

    flowing_water.sort_by(|a, b| if a[1] == b[1] { a[0].cmp(&b[0]) } else { b[1].cmp(&a[1]) });
    println!("Flowing Water: {:?}", &flowing_water);

    for idx in &flowing_water {
        let i = idx[0];
        let j = idx[1];

        // Check if down is sand and move if it is, else check
        // if left is sand and move if it is, else check if right
        // is sand and move if it is.
        if j < max_j && scan_arr[[i, j + 1]].kind == SoilKind::Sand {
            scan_arr[[i, j]].kind = SoilKind::Sand;
            scan_arr[[i, j + 1]].kind = SoilKind::FlowingWater;
            scan_arr[[i, j + 1]].moved = true;
        } else if i > 0 && scan_arr[[i - 1, j]].kind == SoilKind::Sand {
            scan_arr[[i, j]].kind = SoilKind::Sand;
            scan_arr[[i - 1, j]].kind = SoilKind::FlowingWater;
            scan_arr[[i - 1, j]].moved = true;
        } else if i < max_i && scan_arr[[i + 1, j]].kind == SoilKind::Sand {
            scan_arr[[i, j]].kind = SoilKind::Sand;
            scan_arr[[i + 1, j]].kind = SoilKind::FlowingWater;
            scan_arr[[i + 1, j]].moved = true;
        } else {
            // Am i bounded left and right?  If so, I'm standing
            scan_arr[[i, j]].kind = SoilKind::SettledWater;
        }
    }

    Ok(())
}

fn calculate_mins_maxes(x_coord_map: &HashMap<usize, Vec<usize>>, y_coord_map: &HashMap<usize, Vec<usize>>) -> Result<(usize, usize, usize, usize)> {
    let mut min_x = *x_coord_map.keys().min().ok_or_else(|| "no min x")?;
    let mut max_x = *x_coord_map.keys().max().ok_or_else(|| "no max x")?;
    let mut min_y = *y_coord_map.keys().min().ok_or_else(|| "no min y")?;
    let mut max_y = *y_coord_map.keys().max().ok_or_else(|| "no max y")?;

    for yv in x_coord_map.values() {
        for y in yv {
            if *y > max_y {
                max_y = *y;
            }

            if *y < min_y {
                min_y = *y;
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

    Ok((min_x, max_x, min_y, max_y))
}

fn setup_scan(
    mins_maxes: (usize, usize, usize, usize),
    x_coord_map: &HashMap<usize, Vec<usize>>,
    y_coord_map: &HashMap<usize, Vec<usize>>,
    test: bool,
) -> Array2<Soil> {
    let (_, max_x, _, max_y) = mins_maxes;
    let mut clay_arr = Array2::<Soil>::default((max_x, max_y));
    clay_arr[[500, 0]] = Soil {
        kind: SoilKind::Spring,
        moved: false,
    };

    for (i, jv) in x_coord_map {
        for j in jv {
            clay_arr[[*i, *j]] = Soil {
                kind: SoilKind::Clay,
                moved: false,
            };
        }
    }

    for (j, iv) in y_coord_map {
        for i in iv {
            clay_arr[[*i, *j]] = Soil {
                kind: SoilKind::Clay,
                moved: false,
            };
        }
    }

    if test {
        print_scan_arr(mins_maxes, &clay_arr);
    }

    clay_arr
}

fn print_scan_arr(mins_maxes: (usize, usize, usize, usize), scan_arr: &Array2<Soil>) {
    let (min_x, max_x, _, max_y) = mins_maxes;
    println!();
    for j in 0..max_y {
        for i in min_x..max_x {
            print!("{}", scan_arr[[i, j]].kind);
        }
        println!();
    }
}

#[cfg(test)]
mod one_star {
    use super::run_scan;
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
        assert_eq!(run_scan(Cursor::new(TEST_CODE), false, true)?, 1);
        Ok(())
    }
}

// #[cfg(test)]
// mod two_star {
//     use super::find_solution;
//     use error::Result;
//     use std::io::Cursor;

//     const TEST_CODE: &str = r"x=495, y=2..7
// y=7, x=495..501
// x=501, y=3..7
// x=498, y=2..4
// x=506, y=1..2
// x=498, y=10..13
// x=504, y=10..13
// y=13, x=498..504";

//     #[test]
//     fn solution() -> Result<()> {
//         assert_eq!(find_solution(Cursor::new(TEST_CODE), true)?, 1);
//         Ok(())
//     }
// }
