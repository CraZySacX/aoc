//! Advent of Code - Day 12 Solution
use error::Result;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut group_map = HashMap::new();
    let mut group_zero = HashSet::new();
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        parse_and_add(line, &mut group_map)?;
    }

    get_and_add(0, &group_map, &mut group_zero)?;
    Ok(TryFrom::try_from(group_zero.len())?)
}

/// Parse the line and add to group map.
fn parse_and_add(line: &str, group_map: &mut HashMap<u32, Vec<u32>>) -> Result<()> {
    let piped: Vec<&str> = line.split(" <-> ").collect();
    let group_str = piped.get(0).ok_or("Invalid group")?;
    let group = group_str.parse::<u32>()?;
    let piped_to_strs: Vec<&str> = piped.get(1).ok_or("Invalid pipes")?.split(", ").collect();
    let mut piped_to_vec = Vec::new();

    for piped_to_str in piped_to_strs {
        piped_to_vec.push(piped_to_str.parse::<u32>()?);
    }

    group_map.insert(group, piped_to_vec);

    Ok(())
}

/// Get and add
fn get_and_add(group: u32, group_map: &HashMap<u32, Vec<u32>>, group_zero: &mut HashSet<u32>) -> Result<()> {
    let piped_tos = group_map.get(&group).ok_or("Group not found")?;
    group_zero.insert(group);

    for piped_to in piped_tos {
        if !group_zero.contains(piped_to) {
            get_and_add(*piped_to, group_map, group_zero)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod one_star {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn solution() {
        let mut group_map = HashMap::new();
        let mut group_zero = HashSet::new();
        super::parse_and_add("0 <-> 2", &mut group_map).expect("");
        super::parse_and_add("1 <-> 1", &mut group_map).expect("");
        super::parse_and_add("2 <-> 0, 3, 4", &mut group_map).expect("");
        super::parse_and_add("3 <-> 2, 4", &mut group_map).expect("");
        super::parse_and_add("4 <-> 2, 3, 6", &mut group_map).expect("");
        super::parse_and_add("5 <-> 6", &mut group_map).expect("");
        super::parse_and_add("6 <-> 4, 5", &mut group_map).expect("");
        super::get_and_add(0, &group_map, &mut group_zero).expect("");

        assert_eq!(group_zero.len(), 6);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
