//! Advent of Code - Day 21 'Fractal Art' Solution
use bytecount;
use error::Result;
use itertools::Itertools;
use pathfinding::Matrix;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
#[cfg_attr(feature = "cargo-clippy", allow(filter_map))]
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let subst = reader
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|line| {
            let (k, v) = line.trim().split(" => ").map(matrix).next_tuple().ok_or("no tuple").expect("");
            iproduct!(vec![k.clone(), k.flipped_ud(), k.flipped_lr()], 0..4).map(move |(m, i)| (m.rotated_cw(i), v.clone()))
        })
        .collect::<HashMap<_, _>>();
    let mut sharps = (0..).scan(matrix(".#./..#/###"), |grid, _| {
        let pt = 2 + (grid.rows % 2);
        let b = grid.rows / pt;
        let mut new_grid = Matrix::new_square(grid.rows + b, b'?');
        for (c, l) in iproduct!(0..b, 0..b) {
            let new = &subst[&grid.slice(l * pt..l * pt + pt, c * pt..c * pt + pt)];
            new_grid.set_slice(&(l * (pt + 1), c * (pt + 1)), new);
        }
        *grid = new_grid;
        Some(bytecount::count(grid.as_ref(), b'#'))
    });

    if second_star {
        Ok(TryFrom::try_from(sharps.nth(4).unwrap_or(0))?)
    } else {
        Ok(TryFrom::try_from(sharps.nth(12).unwrap_or(0))?)
    }
}

/// Make a matrix of bytes for a rule.
fn matrix(i: &str) -> Matrix<u8> {
    Matrix::square_from_vec(i.bytes().filter(|&c| c != b'/').collect())
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
