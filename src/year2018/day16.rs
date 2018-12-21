//! Advent of Code - Day 16 "Chronal Classification" Solution
use error::Result;
use regex::Regex;
use std::io::BufRead;

type Registers = (usize, usize, usize, usize);
type Instruction = (usize, usize, usize, usize);

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let before_re = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)\]")?;
    let after_re = Regex::new(r"After:  \[(\d), (\d), (\d), (\d)\]")?;
    let instruction_re = Regex::new(r"(\d) (\d) (\d) (\d)")?;

    let mut before_vec = Vec::new();
    let mut after_vec = Vec::new();
    let mut instructions_vec = Vec::new();

    for line in reader.lines().filter_map(|x| x.ok()) {
        if before_re.is_match(&line) {
            println!("Matched before!");
            for caps in before_re.captures_iter(&line) {
                let opcode = (&caps[1]).parse::<usize>()?;
                let reg_a = (&caps[2]).parse::<usize>()?;
                let reg_b = (&caps[3]).parse::<usize>()?;
                let reg_c = (&caps[4]).parse::<usize>()?;

                before_vec.push((opcode, reg_a, reg_b, reg_c));
            }
        } else if after_re.is_match(&line) {
            println!("Matched after!");
            for caps in after_re.captures_iter(&line) {
                let opcode = (&caps[1]).parse::<usize>()?;
                let reg_a = (&caps[2]).parse::<usize>()?;
                let reg_b = (&caps[3]).parse::<usize>()?;
                let reg_c = (&caps[4]).parse::<usize>()?;

                after_vec.push((opcode, reg_a, reg_b, reg_c));
            }
        } else if instruction_re.is_match(&line) {
            println!("Matched instruction!");
            for caps in instruction_re.captures_iter(&line) {
                let opcode = (&caps[1]).parse::<usize>()?;
                let reg_a = (&caps[2]).parse::<usize>()?;
                let reg_b = (&caps[3]).parse::<usize>()?;
                let reg_c = (&caps[4]).parse::<usize>()?;

                instructions_vec.push((opcode, reg_a, reg_b, reg_c));
            }
        }
    }

    if before_vec.len() != after_vec.len() {
        return Err("Bad input file".into());
    }

    let _ = instructions_vec.split_off(before_vec.len());

    let tuples_vec: Vec<(Registers, Instruction, Registers)> = before_vec
        .into_iter()
        .zip(instructions_vec.into_iter())
        .zip(after_vec.into_iter())
        .map(|((rb, i), ra)| (rb, i, ra))
        .collect();

    for (before, ins, after) in tuples_vec {
        println!("Before: [{}, {}, {}, {}]", before.0, before.1, before.2, before.3);
        println!("{} {} {} {}", ins.0, ins.1, ins.2, ins.3);
        println!("After:  [{}, {}, {}, {}]", after.0, after.1, after.2, after.3);
    }
    Ok(1)
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]


5 2 3 2
5 0 1 3
5 3 3 0
2 3 2 3";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CODE), false)?, 1);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_solution;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]


5 2 3 2
5 0 1 3
5 3 3 0
2 3 2 3";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_CODE), true)?, 1);
        Ok(())
    }
}
