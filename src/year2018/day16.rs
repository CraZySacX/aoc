//! Advent of Code - Day 16 "Chronal Classification" Solution
use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::io::BufRead;

type Registers = [usize; 4];
type Instruction = [usize; 4];

#[derive(Clone, Debug, Eq, PartialEq)]
enum OpCode {
    /// (add register) stores into register C the result of adding register A and register B.
    Addr,
    /// (add immediate) stores into register C the result of adding register A and value B.
    Addi,
    /// (multiply register) stores into register C the result of multiplying register A and register B.
    Mulr,
    /// (multiply immediate) stores into register C the result of multiplying register A and value B.
    Muli,
    /// (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Banr,
    /// (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Bani,
    /// (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Borr,
    /// (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Bori,
    /// (set register) copies the contents of register A into register C. (Input B is ignored.)
    Setr,
    /// (set immediate) stores value A into register C. (Input B is ignored.)
    Seti,
    /// (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtir,
    /// (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtri,
    /// (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Gtrr,
    /// (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqir,
    /// (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqri,
    /// (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    Eqrr,
}

impl OpCode {
    fn execute(&self, reg: &mut Registers, ins: Instruction) {
        match self {
            OpCode::Addr => reg[ins[3]] = reg[ins[1]] + reg[ins[2]],
            OpCode::Addi => reg[ins[3]] = reg[ins[1]] + ins[2],
            OpCode::Mulr => reg[ins[3]] = reg[ins[1]] * reg[ins[2]],
            OpCode::Muli => reg[ins[3]] = reg[ins[1]] * ins[2],
            OpCode::Banr => reg[ins[3]] = reg[ins[1]] & reg[ins[2]],
            OpCode::Bani => reg[ins[3]] = reg[ins[1]] & ins[2],
            OpCode::Borr => reg[ins[3]] = reg[ins[1]] | reg[ins[2]],
            OpCode::Bori => reg[ins[3]] = reg[ins[1]] | ins[2],
            OpCode::Setr => reg[ins[3]] = reg[ins[1]],
            OpCode::Seti => reg[ins[3]] = ins[1],
            OpCode::Gtir => reg[ins[3]] = usize::from(ins[1] > reg[ins[2]]),
            OpCode::Gtri => reg[ins[3]] = usize::from(reg[ins[1]] > ins[2]),
            OpCode::Gtrr => reg[ins[3]] = usize::from(reg[ins[1]] > reg[ins[2]]),
            OpCode::Eqir => reg[ins[3]] = usize::from(ins[1] == reg[ins[2]]),
            OpCode::Eqri => reg[ins[3]] = usize::from(reg[ins[1]] == ins[2]),
            OpCode::Eqrr => reg[ins[3]] = usize::from(reg[ins[1]] == reg[ins[2]]),
        }
    }
}

fn execute_opcode(
    opcode: OpCode,
    before: [usize; 4],
    ins: [usize; 4],
    after: [usize; 4],
    count: &mut usize,
    second_star: bool,
    opcode_map: &mut BTreeMap<usize, Vec<OpCode>>,
) {
    let mut regs = before;
    opcode.execute(&mut regs, ins);
    if regs == after {
        *count += 1;
    }
    if second_star && regs == after {
        let opcode_vec = opcode_map.entry(ins[0]).or_insert_with(Vec::new);

        if !opcode_vec.contains(&opcode) {
            opcode_vec.push(opcode);
        }
    }
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let before_re = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)\]")?;
    let after_re = Regex::new(r"After:  \[(\d), (\d), (\d), (\d)\]")?;
    let instruction_re = Regex::new(r"(\d+) (\d) (\d) (\d)")?;

    let mut before_vec = Vec::new();
    let mut after_vec = Vec::new();
    let mut instructions_vec = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if before_re.is_match(&line) {
            for caps in before_re.captures_iter(&line) {
                let opcode = (caps[1]).parse::<usize>()?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                before_vec.push([opcode, reg_a, reg_b, reg_c]);
            }
        } else if after_re.is_match(&line) {
            for caps in after_re.captures_iter(&line) {
                let opcode = (caps[1]).parse::<usize>()?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                after_vec.push([opcode, reg_a, reg_b, reg_c]);
            }
        } else if instruction_re.is_match(&line) {
            for caps in instruction_re.captures_iter(&line) {
                let opcode = (caps[1]).parse::<usize>()?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                instructions_vec.push([opcode, reg_a, reg_b, reg_c]);
            }
        }
    }

    if before_vec.len() != after_vec.len() {
        return Err(anyhow!("Bad input file"));
    }

    let instructions = instructions_vec.split_off(before_vec.len());

    let tuples_vec: Vec<(Registers, Instruction, Registers)> = before_vec
        .into_iter()
        .zip(instructions_vec)
        .zip(after_vec)
        .map(|((rb, i), ra)| (rb, i, ra))
        .collect();

    let mut three_or_more = 0;
    let mut opcode_map = BTreeMap::new();

    for (before, ins, after) in tuples_vec {
        let mut count = 0;

        execute_opcode(OpCode::Addr, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Addi, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Mulr, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Muli, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Banr, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Bani, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Borr, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Bori, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Setr, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Seti, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Gtir, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Gtri, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Gtrr, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Eqir, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Eqri, before, ins, after, &mut count, second_star, &mut opcode_map);
        execute_opcode(OpCode::Eqrr, before, ins, after, &mut count, second_star, &mut opcode_map);

        if count >= 3 {
            three_or_more += 1;
        }
    }

    if second_star {
        let mut known = OpCode::Addr;
        let mut op_map = BTreeMap::new();

        while op_map.len() != 16 {
            for (opcode, poss) in &opcode_map {
                if poss.len() == 1 {
                    known = poss[0].clone();
                    op_map.insert(*opcode, known.clone());
                    break;
                }
            }

            for poss in opcode_map.values_mut() {
                poss.retain(|x| *x != known);
            }
        }

        let mut registers = [0, 0, 0, 0];
        for ins in instructions {
            let opcode = op_map.get(&ins[0]).ok_or(anyhow!("invalid opcode"))?;
            opcode.execute(&mut registers, ins);
        }
        Ok(registers[0] as u32)
    } else {
        Ok(three_or_more)
    }
}

#[cfg(test)]
mod one_star {
    use super::find_solution;
    use anyhow::Result;
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
    use anyhow::Result;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn solution() -> Result<()> {
        let data_file = File::open("data/2018/day16/data_file")?;
        let reader = BufReader::new(data_file);
        assert_eq!(find_solution(reader, true)?, 481);
        Ok(())
    }
}
