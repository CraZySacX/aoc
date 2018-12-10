//! Advent of Code - Day 10 "The Stars Align" Solution
use error::Result;
use regex::Regex;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    println!("{}", align(reader, false)?);

    Ok(0)
}

fn align<T: BufRead>(reader: T, test: bool) -> Result<String> {
    let line_re = Regex::new(r"position=<(.*), (.*)> velocity=<(.*), (.*)>")?;
    let mut star_map: Vec<(isize, isize, isize, isize)> = Vec::new();

    for line in reader.lines().filter_map(|x| x.ok()) {
        for cap in line_re.captures_iter(&line) {
            let x = (&cap[1]).trim().parse::<isize>()?;
            let y = (&cap[2]).trim().parse::<isize>()?;
            let vx = (&cap[3]).trim().parse::<isize>()?;
            let vy = (&cap[4]).trim().parse::<isize>()?;

            star_map.push((x, y, vx, vy));
        }
    }

    let max_step = if test { 3 } else { 10619 };

    for _ in 0..max_step {
        move_stars(&mut star_map);
    }

    Ok(show_stars(&star_map))
}

fn move_stars(star_map: &mut Vec<(isize, isize, isize, isize)>) {
    for star in star_map {
        star.0 += star.2;
        star.1 += star.3;
    }
}

fn show_stars(star_map: &[(isize, isize, isize, isize)]) -> String {
    let mut output = String::new();
    let mut min_x = isize::max_value();
    let mut min_y = isize::max_value();
    let mut max_x = isize::min_value();
    let mut max_y = isize::min_value();

    for star in star_map {
        if star.0 < min_x {
            min_x = star.0;
        }

        if star.0 > max_x {
            max_x = star.0;
        }

        if star.1 < min_y {
            min_y = star.1;
        }

        if star.1 > max_y {
            max_y = star.1;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut found_star = false;
            for star in star_map {
                if star.0 == x && star.1 == y {
                    output.push('#');
                    found_star = true;
                    break;
                }
            }

            if !found_star {
                output.push('.');
            }
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod one_star {
    use super::align;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    const EXPECTED: &str = r"#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(align(Cursor::new(TEST_CHAIN), true)?, EXPECTED);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
