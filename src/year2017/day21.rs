//! Advent of Code - Day 21 'Fractal Art' Solution
use error::Result;
use ndarray::Array2;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let two_by_two_re = Regex::new("([.#]{2}/[.#]{2}) => (.*)")?;
    let three_by_three_re = Regex::new("([.#]{3}/[.#]{3}/[.#]{3}) => (.*)")?;
    let mut two_by_two = HashMap::new();
    let mut three_by_three = HashMap::new();
    let input: Array2<char> = Array2::from_shape_vec((3,3), vec!['.','#','.','.','.','#','#','#','#'])?;
    for line_result in reader.lines() {
        let line = &line_result?;
        add_to_map(
            line,
            &mut two_by_two,
            &mut three_by_three,
            &two_by_two_re,
            &three_by_three_re,
        )?;
    }

    let _output = input_to_output(&input, &two_by_two, &three_by_three)?;
    Ok(0)
}

/// Add the given line to one of the rule vectors
fn add_to_map(
    line: &str,
    two_by_two: &mut HashMap<Array2<char>, Array2<char>>,
    three_by_three: &mut HashMap<Array2<char>, Array2<char>>,
    two_by_two_re: &Regex,
    three_by_three_re: &Regex,
) -> Result<()> {
    if two_by_two_re.is_match(line) {
        let caps = two_by_two_re
            .captures(line)
            .ok_or("invalid two_by_two rule captures")?;
        let input_str = caps.get(1)
            .ok_or("invalid input value")?
            .as_str()
            .to_string();
        let output_str = caps.get(2)
            .ok_or("invalid output value")?
            .as_str()
            .to_string();

        let mut input_arr = Array2::default((2,2));
        let mut output_arr = Array2::default((3,3));
        fill_array(&input_str, &mut input_arr);
        fill_array(&output_str, &mut output_arr);
        two_by_two.insert(input_arr, output_arr);
    } else if three_by_three_re.is_match(line) {
        let caps = three_by_three_re
            .captures(line)
            .ok_or("invalid three_by_three rule captures")?;
        let input_str = caps.get(1)
            .ok_or("invalid input value")?
            .as_str()
            .to_string();
        let output_str = caps.get(2)
            .ok_or("invalid output value")?
            .as_str()
            .to_string();
        let mut input_arr = Array2::default((3,3));
        let mut output_arr = Array2::default((4,4));
        fill_array(&input_str, &mut input_arr);
        fill_array(&output_str, &mut output_arr);
        three_by_three.insert(input_arr, output_arr);
    } else {
        return Err(format!("invalid rule: {}", line).into());
    }

    Ok(())
}

/// Fill a x-by-x 2d array with values from a rule.
fn fill_array(rule: &str, arr: &mut Array2<char>) {
    let rule_parts: Vec<&str> = rule.split('/').collect();

    for (i, pattern) in rule_parts.iter().enumerate() {
        for (j, ch) in pattern.chars().enumerate() {
            arr[[i, j]] = ch;
        } 
    }
}

/// Find a rule match in the
fn input_to_output(input: &Array2<char>, rules_two: &HashMap<Array2<char>, Array2<char>>, rules_three: &HashMap<Array2<char>, Array2<char>>) -> Result<Array2<char>> {
    match input.dim() {
        (2,2) => {
            for (rule, output) in rules_two {
                if rule == input {
                    return Ok(output.clone());
                }
            }
        },
        (3,3) => {
            for (rule, output) in rules_three {
                if rule == input {
                    return Ok(output.clone());
                }
            }
        },
        _ => return Err("invalid array dimensions".into()),
    }
    Err("No match found!".into())
}

#[cfg(test)]
mod one_star {
    use ndarray::{Array2, Axis};
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn solution() {
        use std::io::{self, Write};
        let input: Array2<char> = Array2::from_shape_vec((3,3), vec!['.','#','.','.','.','#','#','#','#']).expect("");
        let two_by_two_re = Regex::new("([.#]{2}/[.#]{2}) => (.*)").expect("");
        let three_by_three_re = Regex::new("([.#]{3}/[.#]{3}/[.#]{3}) => (.*)").expect("");
        let mut two_by_two = HashMap::new();
        let mut three_by_three = HashMap::new();
        super::add_to_map(
            "../.# => ##./#../...",
            &mut two_by_two,
            &mut three_by_three,
            &two_by_two_re,
            &three_by_three_re,
        ).expect("");
        super::add_to_map(
            ".#./..#/### => #..#/..../..../#..#",
            &mut two_by_two,
            &mut three_by_three,
            &two_by_two_re,
            &three_by_three_re,
        ).expect("");
        assert_eq!(two_by_two.len(), 1);
        assert_eq!(three_by_three.len(), 1);
        let mut output = super::input_to_output(&input, &two_by_two, &three_by_three).expect("");
        assert_eq!(output.dim(), (4,4));
        let mut one = output.slice(s![0..2, 0..2]);
        assert_eq!(one.dim(), (2,2));
        writeln!(io::stdout(), "{:?}", one).expect("");
        one.invert_axis(Axis(0));
        writeln!(io::stdout(), "{:?}", one).expect("");
        let two = output.slice(s![0..2, 2..4]);
        writeln!(io::stdout(), "{:?}", two).expect("");
        let three = output.slice(s![2..4, 0..2]);
        writeln!(io::stdout(), "{:?}", three).expect("");
        let four = output.slice(s![2..4, 2..4]);
        writeln!(io::stdout(), "{:?}", four).expect("");
        let out_four = super::input_to_output(&four.to_owned(), &two_by_two, &three_by_three).expect("");
        // let out2 = super::input_to_output(&one.to_owned(), &two_by_two, &three_by_three).expect("");
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
