//! Advent of Code - Day 21 'Fractal Art' Solution
use error::Result;
use ndarray::{Array2, Axis};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
#[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation, cast_precision_loss))]
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let two_by_two_re = Regex::new("([.#]{2}/[.#]{2}) => (.*)")?;
    let three_by_three_re = Regex::new("([.#]{3}/[.#]{3}/[.#]{3}) => (.*)")?;
    let mut two_by_two = HashMap::new();
    let mut three_by_three = HashMap::new();

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

    let mut input: Array2<char> = Array2::from_shape_vec((3, 3), vec!['.', '#', '.', '.', '.', '#', '#', '#', '#']).expect("");

    for _ in 0..5 {
        writeln!(io::stdout(), "Processing:\n{}", input).expect("");
        let width = input.len_of(Axis(0));
        let mut next_vec = Vec::new();
        if width % 2 == 0 {
            for chunk in input.exact_chunks((2, 2)) {
                let blah = input_to_output(&chunk.to_owned(), &two_by_two, &three_by_three).expect("");
                next_vec.extend(blah.iter());
            }
            let len = next_vec.len();
            let dim: usize = TryFrom::try_from((len as f64).sqrt() as i64).expect("");
            let mut next_arr: Array2<char> = Array2::default((dim, dim));

            for three_by_three_rows in 0..dim / 3 {
                let start_row = three_by_three_rows * 3;

                for col in 0..dim {
                    for row in start_row..start_row + 3 {
                        next_arr[[row, col]] = next_vec.remove(0);
                    }
                }
            }
            input = next_arr;
        } else {
            for chunk in input.exact_chunks((3, 3)) {
                let blah = input_to_output(&chunk.to_owned(), &two_by_two, &three_by_three).expect("");
                next_vec.extend(blah.iter());
            }

            let len = next_vec.len();
            let dim: usize = TryFrom::try_from((len as f64).sqrt() as i64).expect("");
            let mut next_arr: Array2<char> = Array2::default((dim, dim));

            for two_by_two_rows in 0..dim / 2 {
                let start_row = two_by_two_rows * 2;

                for col in 0..dim {
                    for row in start_row..start_row + 2 {
                        next_arr[[row, col]] = next_vec.remove(0);
                    }
                }
            }
            input = next_arr;
        }
    }

    let on_count = input.iter().filter(|c| **c == '#').count();

    Ok(TryFrom::try_from(on_count)?)
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

        let mut input_arr = Array2::default((2, 2));
        let mut output_arr = Array2::default((3, 3));
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
        let mut input_arr = Array2::default((3, 3));
        let mut output_arr = Array2::default((4, 4));
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

/// Rotate a square array
fn rotate(arr: &mut Array2<char>) {
    // let width = arr.len_of(Axis(0));

    // for i in 0..width / 2 {
    //     let k = width - i - 1;
    //     for j in i..(width - i - 1) {
    //         let l = width - j - 1;

    //         let tmp = arr[[i, j]];
    //         arr[[i, j]] = arr[[j, k]];
    //         arr[[j, k]] = arr[[k, l]];
    //         arr[[k, l]] = arr[[l, i]];
    //         arr[[l, i]] = tmp;
    //     }
    // }
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(1));
}

/// Find a rule match in the
fn input_to_output(
    input: &Array2<char>,
    rules_two: &HashMap<Array2<char>, Array2<char>>,
    rules_three: &HashMap<Array2<char>, Array2<char>>,
) -> Result<Array2<char>> {
    match input.dim() {
        (2, 2) => {
            let mut to_check = input.clone();
            for _ in 0..4 {
                for (rule, output) in rules_two {
                    if *rule == to_check {
                        return Ok(output.clone());
                    }
                }
                rotate(&mut to_check);
            }
        },
        (3, 3) => {
            let mut to_check = input.clone();
            for _ in 0..4 {
                for (rule, output) in rules_three {
                    if *rule == to_check {
                        return Ok(output.clone());
                    }
                }
                rotate(&mut to_check);
            }

            to_check.invert_axis(Axis(0));
            for _ in 0..4 {
                for (rule, output) in rules_three {
                    if *rule == to_check {
                        return Ok(output.clone());
                    }
                }
                rotate(&mut to_check);
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

        let mut test = Array2::from_shape_vec((2,2), vec![0,1,2,3]).expect("");
        writeln!(io::stdout(), "orig:\n{}", test).expect("");
        test.swap_axes(0, 1);
        writeln!(io::stdout(), "trans:\n{}", test).expect("");
        test.invert_axis(Axis(1));
        writeln!(io::stdout(), "rot:\n{}", test).expect("");
        // let output = super::input_to_output(&input, &two_by_two, &three_by_three).expect("");
        // assert_eq!(output.dim(), (4, 4));

        // let mut next_vec = Vec::new();
        // for chunk in output.exact_chunks((2,2)).into_iter() {
        //     let blah = super::input_to_output(&chunk.to_owned(), &two_by_two, &three_by_three).expect("");
        //     for e in blah.iter() {
        //         next_vec.push(*e);
        //     }
        // }
        // input = Array2::from_shape_vec((6,6), next_vec).expect("");
        // writeln!(io::stdout(), "strides: {:?}", input.strides()).expect("");
        // writeln!(io::stdout(), "{}", input).expect("");

        //  0  1  2 18 19 20
        //  3  4  5 21 22 23
        //  6  7  8 24 25 26
        //  9 10 11 27 28 29
        // 12 13 14 30 31 32
        // 15 16 17 33 34 35
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
