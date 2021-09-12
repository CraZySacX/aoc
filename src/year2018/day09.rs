//! Advent of Code - Day 9 "Marble Mania" Solution
use error::Result;
use regex::Regex;
use std::collections::VecDeque;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r"(\d+) players; last marble is worth (\d+) points")?;
    let mut players = 0;
    let mut final_marble = 0;

    for line in reader.lines().filter_map(|x| x.ok()) {
        for cap in line_re.captures_iter(&line) {
            players = (&cap[1]).parse::<usize>()?;
            final_marble = (&cap[2]).parse::<usize>()?;
        }
    }

    if second_star {
        final_marble *= 100;
    }

    let result = play_game(players, final_marble)?;
    Ok(result as u32)
}

fn rotate_left(circle: &mut VecDeque<usize>, amt: usize) -> Result<()> {
    for _ in 0..amt {
        let tmp = circle.pop_back().ok_or("rotate left err")?;
        circle.push_front(tmp);
    }
    Ok(())
}

fn rotate_right(circle: &mut VecDeque<usize>, amt: usize) -> Result<()> {
    for _ in 0..amt {
        let tmp = circle.pop_front().ok_or("rotate right err")?;
        circle.push_back(tmp);
    }
    Ok(())
}

fn play_game(players: usize, final_marble: usize) -> Result<usize> {
    let mut scores = vec![0; players];
    let mut circle = VecDeque::new();
    circle.push_front(0);

    for marble in 1..=final_marble {
        if marble % 23 == 0 {
            rotate_left(&mut circle, 7)?;
            scores[marble % players] += marble + circle.pop_front().ok_or("no front")?;
        } else {
            rotate_right(&mut circle, 2)?;
            circle.push_front(marble);
        }
    }

    Ok(*scores.iter().max().ok_or("no max")?)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"9 players; last marble is worth 25 points";
    const TEST_CHAIN_1: &str = r"10 players; last marble is worth 1618 points";
    const TEST_CHAIN_2: &str = r"13 players; last marble is worth 7999 points";
    const TEST_CHAIN_3: &str = r"17 players; last marble is worth 1104 points";
    const TEST_CHAIN_4: &str = r"21 players; last marble is worth 6111 points";
    const TEST_CHAIN_5: &str = r"30 players; last marble is worth 5807 points";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 32);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), false)?, 8317);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_2), false)?, 146_373);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_3), false)?, 2764);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_4), false)?, 54718);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_5), false)?, 37305);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"9 players; last marble is worth 25 points";
    const TEST_CHAIN_1: &str = r"10 players; last marble is worth 1618 points";
    const TEST_CHAIN_2: &str = r"13 players; last marble is worth 7999 points";
    const TEST_CHAIN_3: &str = r"17 players; last marble is worth 1104 points";
    const TEST_CHAIN_4: &str = r"21 players; last marble is worth 6111 points";
    const TEST_CHAIN_5: &str = r"30 players; last marble is worth 5807 points";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), true)?, 22563);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_1), true)?, 74_765_078);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_2), true)?, 1_406_506_154);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_3), true)?, 20_548_882);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_4), true)?, 507_583_214);
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN_5), true)?, 320_997_431);
        Ok(())
    }
}
