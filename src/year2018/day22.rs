//! Advent of Code - Day 22 "Mode Maze" Solution
use error::Result;
use ndarray::{Array2, Axis};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryFrom;
use std::fmt;
use std::io::BufRead;
use std::slice::Iter;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
enum RegionKind {
    #[default]
    Rocky,
    Narrow,
    Wet,
}

impl fmt::Display for RegionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegionKind::Rocky => '.',
                RegionKind::Narrow => '|',
                RegionKind::Wet => '=',
            }
        )
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Region {
    kind: RegionKind,
    erosion_level: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Equipment {
    Neither,
    Torch,
    Climbing,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct State {
    time: usize,
    min_dist: usize,
    pos: (usize, usize),
    equipped: Equipment,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.time + self.min_dist).cmp(&(other.time + other.min_dist)).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    Ok(determine_risk(reader, second_star, false, 20, 1000)? as u32)
}

fn determine_risk<T: BufRead>(reader: T, second_star: bool, test: bool, max_i: usize, max_j: usize) -> Result<usize> {
    let depth_re = Regex::new(r"depth: (\d+)")?;
    let target_re = Regex::new(r"target: (\d+),(\d+)")?;
    let mut region_map = Array2::<Region>::default((max_i, max_j));
    let mut depth = 0;
    let mut target_coord = (0, 0);

    for line in reader.lines().filter_map(|x| x.ok()) {
        if depth_re.is_match(&line) {
            for caps in depth_re.captures_iter(&line) {
                depth = (caps[1]).parse::<usize>()?;
            }
        } else if target_re.is_match(&line) {
            for caps in target_re.captures_iter(&line) {
                let i = (caps[1]).parse::<usize>()?;
                let j = (caps[2]).parse::<usize>()?;
                target_coord = (i, j);
            }
        }
    }

    for j in 0..max_j {
        for i in 0..max_i {
            let gi = geologic_index(i, j, target_coord, &region_map);
            let el = (gi + depth) % 20183;
            let kind = match el % 3 {
                0 => RegionKind::Rocky,
                1 => RegionKind::Wet,
                2 => RegionKind::Narrow,
                _ => return Err("unknown region kind".into()),
            };
            region_map[[i, j]].kind = kind;
            region_map[[i, j]].erosion_level = el;
        }
    }

    if test {
        print_map(&region_map);
    }

    let mut result = 0;
    let mut memo = HashMap::new();

    if second_star {
        result = navigate(target_coord, depth, &mut memo)?;
    } else {
        for j in 0..=target_coord.1 {
            for i in 0..=target_coord.0 {
                result += match region_map[[i, j]].kind {
                    RegionKind::Rocky => 0,
                    RegionKind::Wet => 1,
                    RegionKind::Narrow => 2,
                };
            }
        }
    }

    Ok(result)
}

fn get_index(pos: (usize, usize), depth: usize, target: (usize, usize), memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(level) = memo.get(&pos) {
        return *level;
    }
    let level = if pos == (0, 0) {
        0
    } else if pos == target {
        return 0;
    } else if pos.1 == 0 {
        pos.0 * 16807
    } else if pos.0 == 0 {
        pos.1 * 48271
    } else {
        get_erosion((pos.0 - 1, pos.1), depth, target, memo) * get_erosion((pos.0, pos.1 - 1), depth, target, memo)
    };

    memo.insert(pos, level);
    level
}

fn get_erosion(pos: (usize, usize), depth: usize, target: (usize, usize), memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let index = get_index(pos, depth, target, memo);
    (depth + index) % 20183
}

fn get_type(pos: (usize, usize), depth: usize, target: (usize, usize), memo: &mut HashMap<(usize, usize), usize>) -> RegionKind {
    let erosion = get_erosion(pos, depth, target, memo);
    match erosion % 3 {
        0 => RegionKind::Rocky,
        1 => RegionKind::Wet,
        2 => RegionKind::Narrow,
        _ => unreachable!(),
    }
}

fn min_dist(pos: (usize, usize), target: (usize, usize)) -> Result<usize> {
    let pos_i = isize::try_from(pos.0)?;
    let targ_i = isize::try_from(target.0)?;
    let pos_j = isize::try_from(pos.1)?;
    let targ_j = isize::try_from(target.1)?;
    Ok(usize::try_from((pos_i - targ_i).abs() + (pos_j - targ_j).abs())?)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        DIRECTIONS.iter()
    }

    fn checked_nav(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                let next_j = pos.1.checked_sub(1);
                next_j.map(|j| (pos.0, j))
            }
            Direction::Down => Some((pos.0, pos.1 + 1)),
            Direction::Left => {
                let next_i = pos.0.checked_sub(1);
                next_i.map(|i| (i, pos.1))
            }
            Direction::Right => Some((pos.0 + 1, pos.1)),
        }
    }
}

fn navigate(target: (usize, usize), depth: usize, type_memo: &mut HashMap<(usize, usize), usize>) -> Result<usize> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut seen = HashMap::new();

    queue.push(State {
        pos: (0, 0),
        min_dist: min_dist((0, 0), target)?,
        time: 0,
        equipped: Equipment::Torch,
    });

    while !queue.is_empty() {
        let state = queue.pop().ok_or("queue pop failed")?;
        let region = get_type((state.pos.0, state.pos.1), depth, target, type_memo);

        if (region == RegionKind::Rocky && state.equipped == Equipment::Neither)
            || (region == RegionKind::Wet && state.equipped == Equipment::Torch)
            || (region == RegionKind::Narrow && state.equipped == Equipment::Climbing)
        {
            continue;
        }

        if *seen.get(&(state.pos, state.equipped)).unwrap_or(&usize::max_value()) <= state.time {
            continue;
        } else {
            seen.insert((state.pos, state.equipped), state.time);
        }

        if state.pos.0 == target.0 && state.pos.1 == target.1 && state.equipped == Equipment::Torch {
            return Ok(state.time);
        }

        // Queue up all possible movements
        for direction in Direction::iterator() {
            if let Some(pos) = direction.checked_nav(state.pos) {
                queue.push(State {
                    pos,
                    min_dist: min_dist(pos, target)?,
                    time: state.time + 1,
                    equipped: state.equipped,
                });
            }
        }

        // Queue up all possible equipment switches
        for equipment in &[Equipment::Neither, Equipment::Torch, Equipment::Climbing] {
            if &state.equipped != equipment {
                queue.push(State {
                    pos: state.pos,
                    min_dist: state.min_dist,
                    time: state.time + 7,
                    equipped: *equipment,
                });
            }
        }
    }

    unreachable!()
}

fn geologic_index(i: usize, j: usize, target_coord: (usize, usize), region_map: &Array2<Region>) -> usize {
    if (i == 0 && j == 0) || (i == target_coord.0 && j == target_coord.1) {
        0
    } else if j == 0 {
        i * 16807
    } else if i == 0 {
        j * 48271
    } else {
        region_map[[i - 1, j]].erosion_level * region_map[[i, j - 1]].erosion_level
    }
}

fn print_map(region_map: &Array2<Region>) {
    println!();

    for row in region_map.axis_iter(Axis(1)) {
        for cell in row {
            print!("{}", cell.kind);
        }
        println!();
    }
}

#[cfg(test)]
mod one_star {
    use super::determine_risk;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"depth: 510
target: 10,10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(determine_risk(Cursor::new(TEST_CODE), false, true, 16, 16)?, 114);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::determine_risk;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"depth: 510
target: 10,10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(determine_risk(Cursor::new(TEST_CODE), true, true, 16, 16)?, 45);
        Ok(())
    }
}
