//! Advent of Code - Day 18 "Settlers of The North Pole" Solution
use anyhow::{Result, anyhow};
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut lca = lca(reader, 50, 50, second_star, false)?;
    let result = if second_star {
        run(&mut lca, 50, 50, 1000)?
    } else {
        run(&mut lca, 50, 50, 10)?
    };

    Ok(result as u32)
}

fn run(lca: &mut Array2<char>, max_i: usize, max_j: usize, minutes: usize) -> Result<usize> {
    let mut results_map = HashMap::new();
    for i in 0..minutes {
        tick(lca, max_i, max_j)?;

        let wooded = lca.iter().filter(|x| **x == '|').count();
        let lumber_yards = lca.iter().filter(|x| **x == '#').count();

        let results_vec = results_map.entry(wooded * lumber_yards).or_insert_with(Vec::new);
        if i > 583 {
            results_vec.push(i + 1);
        }
        // println!("At minutes {}: {}", i, wooded * lumber_yards);
    }

    let wooded = lca.iter().filter(|x| **x == '|').count();
    let lumber_yards = lca.iter().filter(|x| **x == '#').count();

    let blah: HashMap<&usize, &Vec<usize>> = results_map.iter().filter(|(_, y)| y.len() > 2).collect();

    let mut to_sort: Vec<(&usize, &Vec<usize>)> = blah.into_iter().collect();
    to_sort.sort_by(|a, b| (a.1)[0].cmp(&(b.1)[0]));
    for (x, y) in to_sort {
        println!("{x}: {y:?}");
    }

    Ok(wooded * lumber_yards)
}

fn tick(lca: &mut Array2<char>, max_i: usize, max_j: usize) -> Result<()> {
    let mut change_map = HashMap::new();

    for (idx, val) in lca.indexed_iter() {
        let i = idx.0;
        let j = idx.1;

        let next = match val {
            '.' => check_open(lca, i, j, max_i, max_j),
            '|' => check_trees(lca, i, j, max_i, max_j),
            '#' => check_lumberyard(lca, i, j, max_i, max_j),
            _ => return Err(anyhow!("invalid lumber area")),
        };

        change_map.insert([i, j], next);
    }

    for (idx, next) in change_map {
        lca[idx] = next;
    }

    Ok(())
}

fn check_open(lca: &Array2<char>, i: usize, j: usize, max_i: usize, max_j: usize) -> char {
    let mut tree_count = 0;
    let mut result = '.';
    // check the up left cell if valid
    if i.checked_sub(1).is_some() && j.checked_sub(1).is_some() && lca[[i - 1, j - 1]] == '|' {
        tree_count += 1;
    }

    // check the up cell if valid
    if j.checked_sub(1).is_some() && lca[[i, j - 1]] == '|' {
        tree_count += 1;
    }

    // check the up right cell if valid
    if i + 1 < max_i && j.checked_sub(1).is_some() && lca[[i + 1, j - 1]] == '|' {
        tree_count += 1;
    }

    // Check the left cell if valid
    if i.checked_sub(1).is_some() && lca[[i - 1, j]] == '|' {
        tree_count += 1;
    }

    // Check the right cell if valid
    if i + 1 < max_i && lca[[i + 1, j]] == '|' {
        tree_count += 1;
    }

    // Check the down left cell if valid
    if i.checked_sub(1).is_some() && j + 1 < max_j && lca[[i - 1, j + 1]] == '|' {
        tree_count += 1;
    }

    // Check the down cell if valid
    if j + 1 < max_j && lca[[i, j + 1]] == '|' {
        tree_count += 1;
    }

    // Check the down right cell if valid
    if i + 1 < max_i && j + 1 < max_j && lca[[i + 1, j + 1]] == '|' {
        tree_count += 1;
    }

    if tree_count >= 3 {
        result = '|';
    }

    result
}

fn check_trees(lca: &Array2<char>, i: usize, j: usize, max_i: usize, max_j: usize) -> char {
    let mut lumber_yard_count = 0;
    let mut result = '|';
    // check the up left cell if valid
    if i.checked_sub(1).is_some() && j.checked_sub(1).is_some() && lca[[i - 1, j - 1]] == '#' {
        lumber_yard_count += 1;
    }

    // check the up cell if valid
    if j.checked_sub(1).is_some() && lca[[i, j - 1]] == '#' {
        lumber_yard_count += 1;
    }

    // check the up right cell if valid
    if i + 1 < max_i && j.checked_sub(1).is_some() && lca[[i + 1, j - 1]] == '#' {
        lumber_yard_count += 1;
    }

    // Check the left cell if valid
    if i.checked_sub(1).is_some() && lca[[i - 1, j]] == '#' {
        lumber_yard_count += 1;
    }

    // Check the right cell if valid
    if i + 1 < max_i && lca[[i + 1, j]] == '#' {
        lumber_yard_count += 1;
    }

    // Check the down left cell if valid
    if i.checked_sub(1).is_some() && j + 1 < max_j && lca[[i - 1, j + 1]] == '#' {
        lumber_yard_count += 1;
    }

    // Check the down cell if valid
    if j + 1 < max_j && lca[[i, j + 1]] == '#' {
        lumber_yard_count += 1;
    }

    // Check the down right cell if valid
    if i + 1 < max_i && j + 1 < max_j && lca[[i + 1, j + 1]] == '#' {
        lumber_yard_count += 1;
    }

    if lumber_yard_count >= 3 {
        result = '#';
    }

    result
}

#[allow(clippy::cognitive_complexity)]
fn check_lumberyard(lca: &Array2<char>, i: usize, j: usize, max_i: usize, max_j: usize) -> char {
    let mut lumber_yard_count = 0;
    let mut tree_count = 0;
    let mut result = '.';
    // check the up left cell if valid
    if i.checked_sub(1).is_some() && j.checked_sub(1).is_some() {
        if lca[[i - 1, j - 1]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i - 1, j - 1]] == '|' {
            tree_count += 1;
        }
    }

    // check the up cell if valid
    if j.checked_sub(1).is_some() {
        if lca[[i, j - 1]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i, j - 1]] == '|' {
            tree_count += 1;
        }
    }

    // check the up right cell if valid
    if i + 1 < max_i && j.checked_sub(1).is_some() {
        if lca[[i + 1, j - 1]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i + 1, j - 1]] == '|' {
            tree_count += 1;
        }
    }

    // Check the left cell if valid
    if i.checked_sub(1).is_some() {
        if lca[[i - 1, j]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i - 1, j]] == '|' {
            tree_count += 1;
        }
    }

    // Check the right cell if valid
    if i + 1 < max_i {
        if lca[[i + 1, j]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i + 1, j]] == '|' {
            tree_count += 1;
        }
    }

    // Check the down left cell if valid
    if i.checked_sub(1).is_some() && j + 1 < max_j {
        if lca[[i - 1, j + 1]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i - 1, j + 1]] == '|' {
            tree_count += 1;
        }
    }

    // Check the down cell if valid
    if j + 1 < max_j {
        if lca[[i, j + 1]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i, j + 1]] == '|' {
            tree_count += 1;
        }
    }

    // Check the down right cell if valid
    if i + 1 < max_i && j + 1 < max_j {
        if lca[[i + 1, j + 1]] == '#' {
            lumber_yard_count += 1;
        } else if lca[[i + 1, j + 1]] == '|' {
            tree_count += 1;
        }
    }

    if lumber_yard_count >= 1 && tree_count >= 1 {
        result = '#';
    }

    result
}

fn lca<T: BufRead>(reader: T, max_i: usize, max_j: usize, _second_star: bool, test: bool) -> Result<Array2<char>> {
    let mut lca = Array2::<char>::default((max_i, max_j));

    for (j, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '.' => lca[[i, j]] = ch,
                '|' => lca[[i, j]] = ch,
                '#' => lca[[i, j]] = ch,
                _ => return Err(anyhow!("invalid lumber area")),
            }
        }
    }

    if test {
        print_lca(&lca, 0);
    }
    Ok(lca)
}

fn print_lca(lca: &Array2<char>, max: usize) {
    println!();
    if max == 0 {
        println!("Initially:");
    } else {
        println!("After {max} minutes:");
    }
    for row in lca.axis_iter(Axis(1)) {
        for v in row {
            print!("{v}");
        }
        println!();
    }
}

#[cfg(test)]
mod one_star {
    use super::{lca, print_lca, run};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn solution() -> Result<()> {
        let mut lca = lca(Cursor::new(TEST_CODE), 10, 10, false, true)?;
        assert_eq!(run(&mut lca, 10, 10, 10)?, 1147);
        print_lca(&lca, 10);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{lca, print_lca, run};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn solution() -> Result<()> {
        let mut lca = lca(Cursor::new(TEST_CODE), 10, 10, false, true)?;
        assert_eq!(run(&mut lca, 10, 10, 200)?, 0);
        print_lca(&lca, 10);
        Ok(())
    }
}
