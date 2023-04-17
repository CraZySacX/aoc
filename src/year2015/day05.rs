//! Advent of Code - Day 5 Solution
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

/// Find the solution
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    find_nice(reader, second_star)
}

fn is_nice(line: &[char]) -> bool {
    let mut idx0 = 0;
    let mut idx1 = 1;
    let len = line.len();
    let mut is_bad = false;
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut skip_ch1 = false;

    while idx1 < len && !is_bad {
        if idx1 < len {
            let ch1 = line[idx0];
            let ch2 = line[idx1];

            let pair = format!("{}{}", line[idx0], line[idx1]);
            match &pair[..] {
                "ab" | "cd" | "pq" | "xy" => {
                    is_bad = true;
                    break;
                }
                _ => {}
            }

            if skip_ch1 {
                skip_ch1 = false
            } else {
                match ch1 {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                    _ => {}
                }
            }

            match ch2 {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                    skip_ch1 = true
                }
                _ => {}
            }

            if ch1 == ch2 {
                has_double = true
            }
        }
        idx0 += 1;
        idx1 += 1;
    }

    !is_bad && has_double && vowel_count > 2
}

fn is_nice2(line: &[char]) -> bool {
    let mut has_separated = false;
    let mut has_non_overlapping_pairs = false;
    let mut chunk_map: HashMap<(char, char), usize> = HashMap::new();

    let pairs = line.windows(2);

    for (idx, pair) in pairs.enumerate() {
        if pair.len() == 2 {
            let entry = chunk_map.entry((pair[0], pair[1])).or_insert(idx);

            if idx >= *entry + 2 {
                has_non_overlapping_pairs = true;
                break;
            }
        }
    }

    let windows = line.windows(3);

    for window in windows {
        if window.len() == 3 && window[0] == window[2] {
            has_separated = true;
            break;
        }
    }

    has_non_overlapping_pairs && has_separated
}

fn find_nice<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut nice = 0;
    for line in reader.lines().map_while(Result::ok) {
        let ch: Vec<char> = line.chars().collect();

        if (second_star && is_nice2(&ch)) || (!second_star && is_nice(&ch)) {
            nice += 1;
        }
    }
    Ok(nice)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"ugknbfddgicrmopn";
    const TEST_CHAIN_1: &str = r"aaa";
    const TEST_CHAIN_2: &str = r"jchzalrnumimnmhp";
    const TEST_CHAIN_3: &str = r"haegwjzuvuyypxyu";
    const TEST_CHAIN_4: &str = r"dvszwmarrgswjxmb";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 1);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), false)?, 1);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_2), false)?, 0);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_3), false)?, 0);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_4), false)?, 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"qjhvhtzxzqqjkmpb";
    const TEST_CHAIN_1: &str = r"xxyxx";
    const TEST_CHAIN_2: &str = r"uurcxstgmygtbstg";
    const TEST_CHAIN_3: &str = r"ieodomkazucvgmuy";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 1);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), true)?, 1);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_2), true)?, 0);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_3), false)?, 0);
        Ok(())
    }
}
