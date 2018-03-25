//! Advent of Code - Day 24 Solution
use error::Result;
use std::collections::HashSet;
use std::io::BufRead;

/// Component
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Component {
    /// left port
    left: u32,
    /// right port
    right: u32,
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    // use std::io::{self, Write};
    let mut all = reader.lines().filter_map(to_component).collect::<HashSet<Component>>();

    let mut scores = Vec::new();
    next(0, &[], &mut all, &mut scores);

    if second_star {
        let mut max_length = 0;
        let mut ml_scores = Vec::new();
        for (s, l) in scores {
            if l > max_length {
                ml_scores.clear();
                ml_scores.push((s, l));
                max_length = l;
            } else if l == max_length {
                ml_scores.push((s, l));
            }
        }

        let max = ml_scores.iter().map(|&(s, _)| s).max().ok_or("no max")?;
        Ok(max)
    } else {
        let max = scores.iter().map(|&(s, _)| s).max().ok_or("no max")?;
        Ok(max)
    }
}

/// Convert a String to a Component
fn to_component(line_res: ::std::result::Result<String, ::std::io::Error>) -> Option<Component> {
    if let Ok(line) = line_res {
        let parts = line.split('/').map(|s| s.parse::<u32>().expect("")).collect::<Vec<u32>>();

        Some(Component {
            left: parts[0],
            right: parts[1],
        })
    } else {
        None
    }
}

/// Find the next component given a start, the current path, and the set of components.
fn next(start: u32, path: &[Component], components: &mut HashSet<Component>, scores: &mut Vec<(u32, usize)>) {
    let mut found = false;
    for c in components.iter() {
        if c.left == start || c.right == start {
            let mut new_components = components.clone();
            new_components.remove(c);
            let mut new_path = path.to_owned();
            new_path.push(*c);
            next(if c.left == start { c.right } else { c.left }, &new_path, &mut new_components, scores);
            found = true;
        }
    }
    if !found {
        let score = path.iter().map(|c| c.left + c.right).sum::<u32>();
        let length = path.len();
        scores.push((score, length));
    }
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
