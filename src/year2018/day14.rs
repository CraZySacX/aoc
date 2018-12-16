use error::Result;
use std::collections::VecDeque;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut recipe_count = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
        recipe_count = line.parse::<usize>()?;
    }

    score_recipes(recipe_count)?;
    Ok(0)
}

fn score_recipes(count: usize) -> Result<String> {
    let mut recipe_deque = VecDeque::<u8>::new();
    recipe_deque.push_back(3);
    recipe_deque.push_back(7);

    let mut idx_e1 = 0;
    let mut idx_e2 = 1;

    loop {
        // println!("Elf 1: {}, Elf 2: {}", idx_e1, idx_e2);
        // println!("Recipes: {:?}", recipe_deque);

        if recipe_deque.len() > count + 10 {
            break;
        }
        let e1_r = recipe_deque[idx_e1];
        let e2_r = recipe_deque[idx_e2];
        let next = e1_r + e2_r;

        if next > 9 {
            let tens = next / 10;
            let ones = next % 10;

            recipe_deque.push_back(tens);
            recipe_deque.push_back(ones);
        } else {
            recipe_deque.push_back(next);
        }

        let len = recipe_deque.len();

        idx_e1 = (idx_e1 + e1_r as usize + 1) % len;
        idx_e2 = (idx_e2 + e2_r as usize + 1) % len;
    }

    let after_count = recipe_deque.split_off(count);
    let output: String = after_count.into_iter().take(10).map(|x| x.to_string()).collect();

    Ok(output)
}

#[cfg(test)]
mod one_star {
    use super::score_recipes;
    use error::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(&score_recipes(5)?, "0124515891");
        assert_eq!(&score_recipes(9)?, "5158916779");
        assert_eq!(&score_recipes(18)?, "9251071085");
        assert_eq!(&score_recipes(2018)?, "5941429882");
        Ok(())
    }
}