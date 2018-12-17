//! Advent of Code - Day 14 "Chocolate Charts" Solution
use error::Result;
use std::collections::VecDeque;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut recipe_count = String::new();
    for line in reader.lines().filter_map(|x| x.ok()) {
        recipe_count.push_str(&line);
    }

    let result = score_recipes(&recipe_count, second_star)?;
    println!("{}", result);
    Ok(0)
}

fn check_patt(recipes: &VecDeque<u8>, pattern: &str, len: usize) -> bool {
    if recipes.len() >= len {
        let rev_patt: String = pattern.chars().rev().collect();
        let mut check_str = String::new();
        for i in recipes.iter().rev().take(len) {
            check_str.push_str(&i.to_string());
        }

        if rev_patt == check_str {
            return true;
        }
    }
    false
}

fn score_recipes(count: &str, second_star: bool) -> Result<String> {
    let len = count.len();
    let count_u = count.parse::<usize>()?;
    let mut recipe_deque = VecDeque::<u8>::new();
    recipe_deque.push_back(3);
    recipe_deque.push_back(7);

    let mut idx_e1 = 0;
    let mut idx_e2 = 1;

    loop {
        if !second_star && recipe_deque.len() > count_u + 10 {
            break;
        }
        let e1_r = recipe_deque[idx_e1];
        let e2_r = recipe_deque[idx_e2];
        let next = e1_r + e2_r;

        if next > 9 {
            let tens = next / 10;
            let ones = next % 10;

            recipe_deque.push_back(tens);
            if second_star && check_patt(&recipe_deque, count, len) {
                break;
            }
            recipe_deque.push_back(ones);
            if second_star && check_patt(&recipe_deque, count, len) {
                break;
            }
        } else {
            recipe_deque.push_back(next);
            if second_star && check_patt(&recipe_deque, count, len) {
                break;
            }
        }

        let len = recipe_deque.len();

        idx_e1 = (idx_e1 + e1_r as usize + 1) % len;
        idx_e2 = (idx_e2 + e2_r as usize + 1) % len;
    }

    let output = if second_star {
        let total_len = recipe_deque.len();
        let _ = recipe_deque.split_off(total_len - len);
        recipe_deque.len().to_string()
    } else {
        let after_count = recipe_deque.split_off(count_u);
        after_count.into_iter().take(10).map(|x| x.to_string()).collect()
    };

    Ok(output)
}

#[cfg(test)]
mod one_star {
    use super::score_recipes;
    use error::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(&score_recipes("5", false)?, "0124515891");
        assert_eq!(&score_recipes("9", false)?, "5158916779");
        assert_eq!(&score_recipes("18", false)?, "9251071085");
        assert_eq!(&score_recipes("2018", false)?, "5941429882");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::score_recipes;
    use error::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(&score_recipes("51589", true)?, "9");
        assert_eq!(&score_recipes("01245", true)?, "5");
        // assert_eq!(&score_recipes(9, true)?, "5158916779");
        // assert_eq!(&score_recipes(18, true)?, "9251071085");
        // assert_eq!(&score_recipes(2018, true)?, "5941429882");
        Ok(())
    }
}
