//! Advent of Code - Day 6 Solution
use error::Result;
use ndarray::Array2;
use regex::Regex;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let result = if _second_star { brighten(reader)? } else { decorate(reader)? };
    Ok(result as u32)
}

fn decorate<T: BufRead>(reader: T) -> Result<usize> {
    let mut lights: Array2<bool> = Array2::default((1000, 1000));
    let line_re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")?;

    for line in reader.lines().filter_map(|x| x.ok()) {
        for cap in line_re.captures_iter(&line) {
            let action = &cap[1];
            let x1 = (cap[2]).parse::<usize>()?;
            let y1 = (cap[3]).parse::<usize>()?;
            let x2 = (cap[4]).parse::<usize>()?;
            let y2 = (cap[5]).parse::<usize>()?;

            // println!("Action: {}", action);
            match action {
                "turn on" => {
                    // println!( "Turning on  {},{} through {},{}", x1, y1, x2, y2);
                    turn_on(&mut lights, x1, y1, x2, y2);
                }
                "turn off" => {
                    // println!("Turning off {},{} through {},{}", x1, y1, x2, y2);
                    turn_off(&mut lights, x1, y1, x2, y2);
                }
                "toggle" => {
                    // println!("Toggling    {},{} through {},{}", x1, y1, x2, y2);
                    toggle(&mut lights, x1, y1, x2, y2);
                }
                _ => return Err("invalid command".into()),
            }
        }
    }

    Ok(lights.iter().filter(|x| **x).count())
}

fn turn_on(lights: &mut Array2<bool>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] = true;
        }
    }
}

fn toggle(lights: &mut Array2<bool>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] = !lights[[x, y]];
        }
    }
}

fn turn_off(lights: &mut Array2<bool>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] = false;
        }
    }
}

fn brighten<T: BufRead>(reader: T) -> Result<usize> {
    let mut lights: Array2<usize> = Array2::zeros((1000, 1000));
    let line_re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")?;

    for line in reader.lines().filter_map(|x| x.ok()) {
        for cap in line_re.captures_iter(&line) {
            let action = &cap[1];
            let x1 = (cap[2]).parse::<usize>()?;
            let y1 = (cap[3]).parse::<usize>()?;
            let x2 = (cap[4]).parse::<usize>()?;
            let y2 = (cap[5]).parse::<usize>()?;

            // println!("Action: {}", action);
            match action {
                "turn on" => {
                    // println!( "Turning on  {},{} through {},{}", x1, y1, x2, y2);
                    increase_brightness(&mut lights, x1, y1, x2, y2);
                }
                "turn off" => {
                    // println!("Turning off {},{} through {},{}", x1, y1, x2, y2);
                    decrease_brightness(&mut lights, x1, y1, x2, y2);
                }
                "toggle" => {
                    // println!("Toggling    {},{} through {},{}", x1, y1, x2, y2);
                    really_brighten(&mut lights, x1, y1, x2, y2);
                }
                _ => return Err("invalid command".into()),
            }
        }
    }

    Ok(lights.iter().sum())
}

fn increase_brightness(lights: &mut Array2<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] += 1;
        }
    }
}

fn decrease_brightness(lights: &mut Array2<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            if lights[[x, y]] >= 1 {
                lights[[x, y]] -= 1;
            }
        }
    }
}

fn really_brighten(lights: &mut Array2<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] += 2;
        }
    }
}

#[cfg(test)]
mod one_star {
    use super::decorate;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"turn on 0,0 through 999,999";
    const TEST_CHAIN_1: &str = r"toggle 0,0 through 999,0";
    const TEST_CHAIN_2: &str = r"turn on 0,0 through 999,999
turn off 499,499 through 500,500";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(decorate(Cursor::new(TEST_CHAIN))?, 1_000_000);
        assert_eq!(decorate(Cursor::new(TEST_CHAIN_1))?, 1_000);
        assert_eq!(decorate(Cursor::new(TEST_CHAIN_2))?, 999_996);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::brighten;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"turn on 0,0 through 0,0";
    const TEST_CHAIN_1: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(brighten(Cursor::new(TEST_CHAIN))?, 1);
        assert_eq!(brighten(Cursor::new(TEST_CHAIN_1))?, 2_000_000);
        Ok(())
    }
}
