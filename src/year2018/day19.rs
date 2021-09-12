//! Advent of Code - Day 19 "Go With The Flow" Solution
use error::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::io::BufRead;

type Registers = [usize; 6];
type Instruction = [usize; 3];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            OpCode::Addr => reg[ins[2]] = reg[ins[0]] + reg[ins[1]],
            OpCode::Addi => reg[ins[2]] = reg[ins[0]] + ins[1],
            OpCode::Mulr => reg[ins[2]] = reg[ins[0]] * reg[ins[1]],
            OpCode::Muli => reg[ins[2]] = reg[ins[0]] * ins[1],
            OpCode::Banr => reg[ins[2]] = reg[ins[0]] & reg[ins[1]],
            OpCode::Bani => reg[ins[2]] = reg[ins[0]] & ins[1],
            OpCode::Borr => reg[ins[2]] = reg[ins[0]] | reg[ins[1]],
            OpCode::Bori => reg[ins[2]] = reg[ins[0]] | ins[1],
            OpCode::Setr => reg[ins[2]] = reg[ins[0]],
            OpCode::Seti => reg[ins[2]] = ins[0],
            OpCode::Gtir => reg[ins[2]] = if ins[0] > reg[ins[1]] { 1 } else { 0 },
            OpCode::Gtri => reg[ins[2]] = if reg[ins[0]] > ins[1] { 1 } else { 0 },
            OpCode::Gtrr => reg[ins[2]] = if reg[ins[0]] > reg[ins[1]] { 1 } else { 0 },
            OpCode::Eqir => reg[ins[2]] = if ins[0] == reg[ins[1]] { 1 } else { 0 },
            OpCode::Eqri => reg[ins[2]] = if reg[ins[0]] == ins[1] { 1 } else { 0 },
            OpCode::Eqrr => reg[ins[2]] = if reg[ins[0]] == reg[ins[1]] { 1 } else { 0 },
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Addr => "addr",
                OpCode::Addi => "addi",
                OpCode::Mulr => "mulr",
                OpCode::Muli => "muli",
                OpCode::Banr => "banr",
                OpCode::Bani => "bani",
                OpCode::Borr => "borr",
                OpCode::Bori => "bori",
                OpCode::Setr => "setr",
                OpCode::Seti => "seti",
                OpCode::Gtir => "gtir",
                OpCode::Gtri => "gtri",
                OpCode::Gtrr => "gtrr",
                OpCode::Eqir => "eqir",
                OpCode::Eqri => "eqri",
                OpCode::Eqrr => "eqrr",
            }
        )
    }
}
impl TryFrom<&str> for OpCode {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "addr" => OpCode::Addr,
            "addi" => OpCode::Addi,
            "mulr" => OpCode::Mulr,
            "muli" => OpCode::Muli,
            "banr" => OpCode::Banr,
            "bani" => OpCode::Bani,
            "borr" => OpCode::Borr,
            "bori" => OpCode::Bori,
            "setr" => OpCode::Setr,
            "seti" => OpCode::Seti,
            "gtir" => OpCode::Gtir,
            "gtri" => OpCode::Gtri,
            "gtrr" => OpCode::Gtrr,
            "eqir" => OpCode::Eqir,
            "eqri" => OpCode::Eqri,
            "eqrr" => OpCode::Eqrr,
            _ => return Err("invalid opcode".into()),
        })
    }
}

struct Ip {
    register: usize,
    value: usize,
}

impl fmt::Display for Ip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.register, self.value)
    }
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    Ok(run_program(reader, second_star, false)? as u32)
}

fn run_program<T: BufRead>(reader: T, second_star: bool, test: bool) -> Result<usize> {
    let ip_re = Regex::new(r"#ip (\d+)")?;
    let instruction_re = Regex::new(r"([a-z]+) (\d+) (\d+) (\d+)")?;

    let mut instructions_vec = Vec::new();
    let mut register = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
        if ip_re.is_match(&line) {
            for caps in ip_re.captures_iter(&line) {
                register = (&caps[1]).parse::<usize>()?;
            }
        } else if instruction_re.is_match(&line) {
            for caps in instruction_re.captures_iter(&line) {
                let mut instruction_map = HashMap::new();
                let opcode = OpCode::try_from(&caps[1])?;
                let reg_a = (&caps[2]).parse::<usize>()?;
                let reg_b = (&caps[3]).parse::<usize>()?;
                let reg_c = (&caps[4]).parse::<usize>()?;

                instruction_map.insert(opcode, [reg_a, reg_b, reg_c]);
                instructions_vec.push(instruction_map);
            }
        }
    }

    let mut ip = Ip { register, value: 0 };
    if test {
        println!("IP: {}", ip);
    }
    let mut registers: Registers = if second_star { [1, 0, 0, 0, 0, 0] } else { [0, 0, 0, 0, 0, 0] };

    while is_ins(&ip, &instructions_vec).is_some() {
        if test {
            println!();
            print!("ip={} ", ip.value);
        }
        update_register_with_ip(&mut registers, &ip);
        if test {
            print_registers(&registers);
        }
        execute(&mut registers, &ip, &instructions_vec, test);
        if test {
            print_registers(&registers);
        } else if second_star && registers[2] % 100_000 == 0 {
            println!();
            print_registers(&registers);
        }
        update_ip_with_register(&registers, &mut ip);
    }

    Ok(registers[0])
}

fn is_ins(ip: &Ip, ins_vec: &[HashMap<OpCode, Instruction>]) -> Option<()> {
    ins_vec.get(ip.value).map(|_| ())
}

fn update_register_with_ip(registers: &mut Registers, ip: &Ip) {
    registers[ip.register] = ip.value;
}

fn execute(registers: &mut Registers, ip: &Ip, ins_vec: &[HashMap<OpCode, Instruction>], test: bool) {
    if let Some(ins_map) = ins_vec.get(ip.value) {
        if ins_map.len() == 1 {
            for (opcode, ins) in ins_map.iter() {
                if test {
                    print!("{} {} {} {} ", opcode, ins[0], ins[1], ins[2]);
                }
                opcode.execute(registers, *ins);
            }
        }
    }
}

fn update_ip_with_register(registers: &Registers, ip: &mut Ip) {
    ip.value = registers[ip.register];
    ip.value += 1;
}

fn print_registers(registers: &Registers) {
    print!("[");
    for (idx, reg) in registers.iter().enumerate() {
        print!("{}", reg);

        if idx < registers.len() - 1 {
            print!(", ");
        }
    }
    print!("] ");
}

#[cfg(test)]
mod one_star {
    use super::run_program;
    use error::Result;
    use std::io::Cursor;

    const TEST_CODE: &str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(run_program(Cursor::new(TEST_CODE), false, true)?, 6);
        Ok(())
    }
}
