//! Advent of Code - Day 12 "Digital Plumber" Solution
use error::Result;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use utils::PrivateTryFromUsize;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut group_map = HashMap::new();

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        parse_and_add(line, &mut group_map)?;
    }

    if second_star {
        let mut groups: Vec<HashSet<u32>> = Vec::new();
        let group_map_clone = group_map.clone();

        for k in group_map.keys() {
            add_to_groups(*k, &group_map_clone, &mut groups)?;
        }

        Ok(u32::private_try_from(groups.len())?)
    } else {
        let mut group_zero = HashSet::new();
        get_and_add(0, &group_map, &mut group_zero)?;
        Ok(u32::private_try_from(group_zero.len())?)
    }
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
fn get_and_add(group: u32, group_map: &HashMap<u32, Vec<u32>>, group_set: &mut HashSet<u32>) -> Result<()> {
    let piped_tos = group_map.get(&group).ok_or("Group not found")?;
    group_set.insert(group);

    for piped_to in piped_tos {
        if !group_set.contains(piped_to) {
            get_and_add(*piped_to, group_map, group_set)?;
        }
    }

    Ok(())
}

/// Add to groups
fn add_to_groups(group: u32, group_map: &HashMap<u32, Vec<u32>>, group_sets: &mut Vec<HashSet<u32>>) -> Result<()> {
    let mut found = false;

    for group_set in group_sets.iter() {
        if group_set.contains(&group) {
            found = true;
            break;
        }
    }

    if !found {
        let mut new_set = HashSet::new();
        get_and_add(group, group_map, &mut new_set)?;
        group_sets.push(new_set);
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
    use std::collections::{HashMap, HashSet};

    #[test]
    fn solution() {
        let mut group_map = HashMap::new();
        super::parse_and_add("0 <-> 2", &mut group_map).expect("");
        super::parse_and_add("1 <-> 1", &mut group_map).expect("");
        super::parse_and_add("2 <-> 0, 3, 4", &mut group_map).expect("");
        super::parse_and_add("3 <-> 2, 4", &mut group_map).expect("");
        super::parse_and_add("4 <-> 2, 3, 6", &mut group_map).expect("");
        super::parse_and_add("5 <-> 6", &mut group_map).expect("");
        super::parse_and_add("6 <-> 4, 5", &mut group_map).expect("");

        let mut groups: Vec<HashSet<u32>> = Vec::new();
        let group_map_clone = group_map.clone();

        for k in group_map.keys() {
            super::add_to_groups(*k, &group_map_clone, &mut groups).expect("");
        }

        assert_eq!(groups.len(), 2);
    }
}
