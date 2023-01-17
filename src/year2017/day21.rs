//! Advent of Code - Day 21 'Fractal Art' Solution

use crate::utils::PrivateTryFromUsize;
use anyhow::{anyhow, Result};
use itertools::{iproduct, Itertools};
use pathfinding::matrix::Matrix;
use std::collections::HashMap;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let subst = reader
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|line| {
            let (k, v) = line
                .trim()
                .split(" => ")
                .filter_map(matrix)
                .next_tuple()
                .ok_or(anyhow!("blah"))
                .expect("tough cookies");
            iproduct!(vec![k.clone(), k.flipped_ud(), k.flipped_lr()], 0..4).map(move |(m, i)| (m.rotated_cw(i), v.clone()))
        })
        .collect::<HashMap<_, _>>();
    let mut sharps = (0..).scan(matrix(".#./..#/###"), |grid, _| {
        if let Some(grid) = grid {
            let pt = 2 + (grid.rows % 2);
            let b = grid.rows / pt;
            let mut new_grid = Matrix::new_square(grid.rows + b, b'?');
            for (c, l) in iproduct!(0..b, 0..b) {
                let new = &subst[&grid.slice(l * pt..l * pt + pt, c * pt..c * pt + pt).ok()?];
                new_grid.set_slice((l * (pt + 1), c * (pt + 1)), new);
            }
            *grid = new_grid;
            Some(bytecount::count(grid.as_ref(), b'#'))
        } else {
            None
        }
    });

    if second_star {
        Ok(u32::private_try_from(sharps.nth(4).unwrap_or(0))?)
    } else {
        Ok(u32::private_try_from(sharps.nth(12).unwrap_or(0))?)
    }
}

/// Make a matrix of bytes for a rule.
fn matrix(i: &str) -> Option<Matrix<u8>> {
    Matrix::square_from_vec(i.bytes().filter(|&c| c != b'/').collect()).ok()
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {}
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
