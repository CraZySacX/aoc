//! Advent of Code - Day 12 "Subterranean Sustainability" Solution
use error::Result;
use indexmap::IndexMap;
use regex::Regex;
use sliding_windows::{IterExt, Storage};
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut state_map = BTreeMap::new();
    let mut pattern_map = IndexMap::new();

    gen_maps(reader, &mut state_map, &mut pattern_map)?;
    let res = if second_star {
        let mut sub_total = run_generations(95, &mut state_map, &pattern_map)?;
        sub_total += (50_000_000_000 - 95) * 91;
        sub_total
    } else {
        run_generations(20, &mut state_map, &pattern_map)?
    };
    println!("Sum: {}", res);
    Ok(0)
}

fn gen_maps<T: BufRead>(reader: T, state_map: &mut BTreeMap<isize, bool>, pattern_map: &mut IndexMap<Vec<bool>, bool>) -> Result<()> {
    let initial_state_re = Regex::new(r"^initial state: ([\.#]+)")?;
    let patt_re = Regex::new(r"([\.#]+) => ([\.#])")?;

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
    Ok(())
}

fn run_generations(gens: usize, state_map: &mut BTreeMap<isize, bool>, pattern_map: &IndexMap<Vec<bool>, bool>) -> Result<isize> {
    for _ in 0..gens {
        let mut action_map = HashMap::new();
        add_left(state_map)?;
        add_right(state_map)?;
        check_plants(state_map, pattern_map, &mut action_map)?;
        grow_plants(action_map, state_map);
    }
    let total: isize = state_map.iter().filter(|(_, v)| **v).map(|(k, _)| *k).sum();
    Ok(total)
}

fn check_plants(state_map: &BTreeMap<isize, bool>, pattern_map: &IndexMap<Vec<bool>, bool>, action_map: &mut HashMap<isize, bool>) -> Result<()> {
    let mut window: Storage<(&isize, &bool)> = Storage::new(5);
    for x in state_map.iter().sliding_windows(&mut window) {
        let plants: Vec<bool> = x.iter().map(|(_, plant)| **plant).collect();
        let idx: Vec<isize> = x.iter().map(|(idx, _)| **idx).collect();
        let mut found = false;
        let mut action = false;

        for (pattern, outcome) in pattern_map {
            if pattern == &plants {
                found = true;
                action = *outcome;
            }
        }

        if found {
            action_map.insert(idx[2], action);
        } else {
            action_map.insert(idx[2], false);
        }
    }

    Ok(())
}

fn grow_plants(action_map: HashMap<isize, bool>, state_map: &mut BTreeMap<isize, bool>) {
    for (idx, action) in action_map {
        *state_map.entry(idx).or_insert(false) = action;
    }
}

fn find_min_plant(state_map: &BTreeMap<isize, bool>) -> Result<isize> {
    Ok(state_map
        .iter()
        .filter(|(_, v)| **v)
        .min_by_key(|(k, _)| *k)
        .map(|(k, _)| *k)
        .ok_or_else(|| "no minimum key")?)
}

fn find_max_plant(state_map: &BTreeMap<isize, bool>) -> Result<isize> {
    Ok(state_map
        .iter()
        .filter(|(_, v)| **v)
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

    for i in max + 1..max + 4 {
        state_map.insert(i, false);
    }

    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::{gen_maps, run_generations};
    use error::Result;
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
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
        let mut state_map = BTreeMap::new();
        let mut pattern_map = IndexMap::new();
        gen_maps(Cursor::new(TEST_STATE), &mut state_map, &mut pattern_map)?;
        assert_eq!(run_generations(20, &mut state_map, &pattern_map)?, 325);
        Ok(())
    }
}
