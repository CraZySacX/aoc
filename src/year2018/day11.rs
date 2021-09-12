//! Advent of Code - Day 11 "Chronal Charge" Solution
use error::Result;
use ndarray::Array2;
use std::convert::TryFrom;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut serial_number = 0;

    for line in reader.lines().filter_map(|x| x.ok()) {
        serial_number = line.parse::<usize>()?;
    }

    let result = submatrix_sum_queries(serial_number, second_star)?;
    println!("{},{},{} with power level {}", result.0, result.1, result.2, result.3);

    Ok(0)
}

fn find_cell_power(x: usize, y: usize, serial_number: usize) -> Result<isize> {
    let rack_id = x + 10;
    let mut power_level = isize::try_from(rack_id * y)?;
    power_level += isize::try_from(serial_number)?;
    power_level *= isize::try_from(rack_id)?;
    power_level = if power_level > 100 { (power_level / 100) % 10 } else { 0 };
    power_level -= 5;
    Ok(power_level)
}

fn submatrix_sum_queries(serial_number: usize, second_star: bool) -> Result<(usize, usize, usize, isize)> {
    let mut power_level: Array2<isize> = Array2::zeros((300, 300));

    for i in 0..300 {
        for j in 0..300 {
            power_level[[i, j]] = find_cell_power(i, j, serial_number)?;
        }
    }

    let mut aux: Array2<isize> = Array2::zeros((300, 300));

    for i in 0..300 {
        aux[[0, i]] = power_level[[0, i]];
    }

    for i in 1..300 {
        for j in 0..300 {
            aux[[i, j]] = power_level[[i, j]] + aux[[i - 1, j]];
        }
    }

    for i in 0..300 {
        for j in 1..300 {
            aux[[i, j]] += aux[[i, j - 1]];
        }
    }

    let mut max_power_level = isize::min_value();
    let mut max_cell = (0, 0, 0, 0);

    if second_star {
        for size in 0..300 {
            println!("Checking size {}", size);
            submatrix_sum_query(&aux, size, &mut max_power_level, &mut max_cell);
        }
    } else {
        submatrix_sum_query(&aux, 2, &mut max_power_level, &mut max_cell);
    }

    Ok(max_cell)
}

fn submatrix_sum_query(aux: &Array2<isize>, size: usize, max_power_level: &mut isize, max_cell: &mut (usize, usize, usize, isize)) {
    for i in 0..(300 - size) {
        for j in 0..(300 - size) {
            let result = sum_query(aux, i, j, size);

            if result > *max_power_level {
                *max_power_level = result;
                max_cell.0 = i;
                max_cell.1 = j;
                max_cell.2 = size + 1;
                max_cell.3 = result;
            }
        }
    }
}

fn sum_query(aux: &Array2<isize>, i: usize, j: usize, size: usize) -> isize {
    let tli = i;
    let tlj = j;
    let rbi = i + size;
    let rbj = j + size;
    let mut res = aux[[rbi, rbj]];

    if tli > 0 {
        res -= aux[[tli - 1, rbj]];
    }
    if tlj > 0 {
        res -= aux[[rbi, tlj - 1]];
    }
    if tli > 0 && tlj > 0 {
        res += aux[[tli - 1, tlj - 1]];
    }

    res
}

#[cfg(test)]
mod one_star {
    use super::{find_cell_power, submatrix_sum_queries};
    use error::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_cell_power(3, 5, 8)?, 4);
        assert_eq!(find_cell_power(122, 79, 57)?, -5);
        assert_eq!(find_cell_power(217, 196, 39)?, 0);
        assert_eq!(find_cell_power(101, 153, 71)?, 4);
        assert_eq!(submatrix_sum_queries(18, false)?, (33, 45, 3, 29));
        assert_eq!(submatrix_sum_queries(42, false)?, (21, 61, 3, 30));
        assert_eq!(submatrix_sum_queries(7511, false)?, (21, 22, 3, 34));
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::submatrix_sum_queries;
    use error::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(submatrix_sum_queries(18, true)?, (90, 269, 16, 113));
        assert_eq!(submatrix_sum_queries(42, true)?, (232, 251, 12, 119));
        // assert_eq!(submatrix_sum_queries(7511, true)?, (236, 287, 13, 0));
        Ok(())
    }
}
