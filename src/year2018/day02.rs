//! Advent of Code - Day 2 "Inventory Management System" Solution
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut twos = 0;
    let mut threes = 0;
    let mut all_ids = Vec::new();

    for line in reader.lines().flatten() {
        if second_star {
            all_ids.push(line);
        } else {
            let (has_two, has_three) = has_two_or_three(&line);
            if has_two {
                twos += 1
            };
            if has_three {
                threes += 1
            };
        }
    }

    if second_star {
        println!("{}", find_closest(&mut all_ids));
    }

    Ok(twos * threes)
}

fn has_two_or_three(line: &str) -> (bool, bool) {
    let mut char_freq = HashMap::new();
    let mut result = (false, false);
    let chars: Vec<char> = line.chars().collect();
    for ch in chars {
        let freq = char_freq.entry(ch).or_insert(0);
        *freq += 1;
    }

    for val in char_freq.values() {
        if *val == 2 {
            result.0 = true;
            break;
        }
    }

    for val in char_freq.values() {
        if *val == 3 {
            result.1 = true;
            break;
        }
    }
    result
}

fn find_closest(all_ids: &mut Vec<String>) -> String {
    let mut matches = Vec::new();

    while !all_ids.is_empty() {
        find_match(all_ids, &mut matches);
    }

    if let Some(longest) = matches.iter().max_by_key(|x| x.len()) {
        longest.clone()
    } else {
        "".to_string()
    }
}

fn find_match(all_ids: &mut Vec<String>, matches: &mut Vec<String>) {
    let current = all_ids.remove(0);
    let curr_ch: Vec<char> = current.chars().collect();

    for id in all_ids {
        matches.push(curr_ch.iter().zip(id.chars()).filter(|(a, b)| *a == b).map(|(_, b)| b).collect());
    }
}

#[cfg(test)]
mod one_star {
    use super::has_two_or_three;

    #[test]
    fn solution() {
        assert_eq!(has_two_or_three("abcdef"), (false, false));
        assert_eq!(has_two_or_three("bababc"), (true, true));
        assert_eq!(has_two_or_three("abbcde"), (true, false));
        assert_eq!(has_two_or_three("abcccd"), (false, true));
        assert_eq!(has_two_or_three("aabcdd"), (true, false));
        assert_eq!(has_two_or_three("abcdee"), (true, false));
        assert_eq!(has_two_or_three("ababab"), (false, true));
    }
}

#[cfg(test)]
mod two_star {
    use super::find_closest;

    #[test]
    fn solution() {
        let mut ids: Vec<String> = ["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(find_closest(&mut ids), "fgij");
    }
}
