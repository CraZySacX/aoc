//! Advent of Code - Day 12 "Subterranean Sustainability" Solution
use error::Result;
use indexmap::IndexMap;
use regex::Regex;
use sliding_windows::{IterExt, Storage};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let initial_state_re = Regex::new(r"^initial state: ([\.#]+)")?;
    let patt_re = Regex::new(r"([\.#]+) => ([\.#])")?;
    let mut state_map: BTreeMap<isize, bool> = BTreeMap::new();
    let mut pattern_map: IndexMap<Vec<bool>, bool> = IndexMap::new();

    for line in reader.lines().filter_map(|x| x.ok()) {
        for cap in initial_state_re.captures_iter(&line) {
            let state_str = &cap[1];

            for (idx, ch) in state_str.chars().enumerate() {
                match ch {
                    '#' => state_map.insert(isize::try_from(idx)?, true),
                    '.' => state_map.insert(isize::try_from(idx)?, false),
                    _ => return Err("invalid state character".into()),
                };
            }
        }

        for cap in patt_re.captures_iter(&line) {
            let pattern: Vec<bool> = (&cap[1]).chars().map(|ch| ch == '#').collect();

            match &cap[2] {
                "#" => pattern_map.insert(pattern, true),
                "." => pattern_map.insert(pattern, false),
                _ => return Err("invalid pattern character".into()),
            };
        }
    }

    for _generation in 0..1 {
        add_left(&mut state_map)?;
        add_right(&mut state_map)?;

        let mut window: Storage<(&isize, &bool)> = Storage::new(5);

        for x in state_map.iter().sliding_windows(&mut window) {
            let plants: Vec<bool> = x.iter().map(|(_, plant)| **plant).collect();
            let idx: Vec<isize> = x.iter().map(|(idx, _)| **idx).collect();
            let mut found = false;
            let mut action = false;

            for (pattern, outcome) in &pattern_map {
                if pattern == &plants {
                    found = true;
                    action = *outcome;
                }
            }

            if found {
                println!("Found match for Pot {}.  Next Gen is {}", idx[2], action);
            }
        }
    }

    println!("state map: {:?}", state_map);
    println!("pattern map: {:?}", pattern_map);

    Ok(1)
}

fn find_min_plant(state_map: &BTreeMap<isize, bool>) -> Result<isize> {
    Ok(state_map
        .iter()
        .filter(|(_, v)| **v == true)
        .min_by_key(|(k, _)| *k)
        .map(|(k, _)| *k)
        .ok_or_else(|| "no minimum key")?)
}

fn find_max_plant(state_map: &BTreeMap<isize, bool>) -> Result<isize> {
    Ok(state_map
        .iter()
        .filter(|(_, v)| **v == true)
        .max_by_key(|(k, _)| *k)
        .map(|(k, _)| *k)
        .ok_or_else(|| "no maximum key")?)
}

fn add_left(state_map: &mut BTreeMap<isize, bool>) -> Result<()> {
    let min = find_min_plant(state_map)?;

    for i in (min - 4)..min {
        state_map.insert(i, false);
    }

    Ok(())
}

fn add_right(state_map: &mut BTreeMap<isize, bool>) -> Result<()> {
    let max = find_max_plant(state_map)?;

    for i in max..max + 4 {
        state_map.insert(i, false);
    }

    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_STATE: &str = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_STATE), false)?, 0);
        Ok(())
    }
}
