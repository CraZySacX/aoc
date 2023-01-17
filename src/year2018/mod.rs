//! Advent of Code 2018 Days

use crate::constants::AoCDay;
use anyhow::Result;
use std::io::BufRead;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

/// Find the solution.
pub fn find_solution<T: BufRead>(reader: T, day: &AoCDay, is_second_star: bool) -> Result<u32> {
    match *day {
        AoCDay::AOCD01 => day01::find_solution(reader, is_second_star),
        AoCDay::AOCD02 => day02::find_solution(reader, is_second_star),
        AoCDay::AOCD03 => day03::find_solution(reader, is_second_star),
        AoCDay::AOCD04 => day04::find_solution(reader, is_second_star),
        AoCDay::AOCD05 => day05::find_solution(reader, is_second_star),
        AoCDay::AOCD06 => day06::find_solution(reader, is_second_star),
        AoCDay::AOCD07 => day07::find_solution(reader, is_second_star),
        AoCDay::AOCD08 => day08::find_solution(reader, is_second_star),
        AoCDay::AOCD09 => day09::find_solution(reader, is_second_star),
        AoCDay::AOCD10 => day10::find_solution(reader, is_second_star),
        AoCDay::AOCD11 => day11::find_solution(reader, is_second_star),
        AoCDay::AOCD12 => day12::find_solution(reader, is_second_star),
        AoCDay::AOCD13 => day13::find_solution(reader, is_second_star),
        AoCDay::AOCD14 => day14::find_solution(reader, is_second_star),
        AoCDay::AOCD15 => day15::find_solution(reader, is_second_star),
        AoCDay::AOCD16 => day16::find_solution(reader, is_second_star),
        AoCDay::AOCD17 => day17::find_solution(reader, is_second_star),
        AoCDay::AOCD18 => day18::find_solution(reader, is_second_star),
        AoCDay::AOCD19 => day19::find_solution(reader, is_second_star),
        AoCDay::AOCD20 => day20::find_solution(reader, is_second_star),
        AoCDay::AOCD21 => day21::find_solution(reader, is_second_star),
        AoCDay::AOCD22 => day22::find_solution(reader, is_second_star),
        AoCDay::AOCD23 => day23::find_solution(reader, is_second_star),
        AoCDay::AOCD24 => day24::find_solution(reader, is_second_star),
        AoCDay::AOCD25 => day25::find_solution(reader, is_second_star),
    }
}
