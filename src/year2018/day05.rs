use error::Result;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let mut ch_vec: Vec<char> = line.chars().collect();

            for (idx, ch) in ch_vec.iter().enumerate() {}
        }
    }

    if second_star {
        Ok(1)
    } else {
        Ok(1)
    }
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CHAIN), false)?, 10);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use error::Result;

    #[test]
    fn solution() -> Result<()> {
        Ok(())
    }
}
