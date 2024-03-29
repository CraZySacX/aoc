//! Advent of Code - Day 3 "No Matter How You Slice It" Solution
use anyhow::{anyhow, Result};
use ndarray::Array2;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.top_left, self.bottom_right)
    }
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")?;
    let mut rectangles = BTreeMap::new();

    for line in reader.lines().map_while(Result::ok) {
        for cap in line_re.captures_iter(&line) {
            let id = (cap[1]).parse::<usize>()?;
            let l = (cap[2]).parse::<usize>()?;
            let t = (cap[3]).parse::<usize>()?;
            let w = (cap[4]).parse::<usize>()?;
            let h = (cap[5]).parse::<usize>()?;
            let top_left = Point { x: l, y: t };
            let bottom_right = Point { x: l + w - 1, y: t + h - 1 };
            let rectangle = Rectangle { top_left, bottom_right };
            rectangles.insert(id, rectangle);
        }
    }

    if second_star {
        Ok(find_non_overlaps(&rectangles)? as u32)
    } else {
        Ok(check_points(&rectangles, 1000, 1000)?)
    }
}

fn check_points(all_claims: &BTreeMap<usize, Rectangle>, width: usize, height: usize) -> Result<u32> {
    let mut cloth: Array2<u8> = Array2::zeros((width, height));

    for y in 0..height {
        for x in 0..width {
            let point = Point { x, y };

            for rectangle in all_claims.values() {
                if contains_point(*rectangle, point) {
                    if let Some(ps) = cloth.get_mut((x, y)) {
                        *ps += 1;
                    }
                }
            }
        }
    }

    let mut multi_count = 0;
    for ps in cloth.iter() {
        match ps {
            0 | 1 => {}
            _ => multi_count += 1,
        }
    }

    Ok(multi_count)
}

fn contains_point(rect: Rectangle, point: Point) -> bool {
    point.x >= rect.top_left.x && point.x <= rect.bottom_right.x && point.y >= rect.top_left.y && point.y <= rect.bottom_right.y
}

fn overlap(rect1: Rectangle, rect2: Rectangle) -> bool {
    // If one rectangle is on left side of other
    if rect1.top_left.x > rect2.bottom_right.x || rect2.top_left.x > rect1.bottom_right.x {
        return false;
    }

    // If one rectangle is above other
    if rect1.top_left.y > rect2.bottom_right.y || rect2.top_left.y > rect1.bottom_right.y {
        return false;
    }

    true
}

#[allow(clippy::needless_collect)]
fn find_non_overlaps(rectangles: &BTreeMap<usize, Rectangle>) -> Result<usize> {
    for (id, rect1) in rectangles {
        let overlaps: Vec<Rectangle> = rectangles
            .iter()
            .filter(|(id2, _)| *id != **id2)
            .filter(|(_, rect2)| overlap(*rect1, **rect2))
            .map(|(_, v)| *v)
            .collect();

        if overlaps.is_empty() {
            return Ok(*id);
        }
    }

    Err(anyhow!("failed to find an non-overlapping rectangle"))
}

#[cfg(test)]
mod one_star {
    use super::{check_points, contains_point, Point, Rectangle};
    use anyhow::Result;
    use std::collections::BTreeMap;

    #[test]
    fn solution() -> Result<()> {
        let mut rectangles = BTreeMap::new();
        let rect1 = Rectangle {
            top_left: Point { x: 1, y: 3 },
            bottom_right: Point { x: 4, y: 6 },
        };
        let rect2 = Rectangle {
            top_left: Point { x: 3, y: 1 },
            bottom_right: Point { x: 6, y: 4 },
        };
        let rect3 = Rectangle {
            top_left: Point { x: 5, y: 5 },
            bottom_right: Point { x: 6, y: 6 },
        };
        let tl = Point { x: 3, y: 1 };
        let tr = Point { x: 6, y: 1 };
        let bl = Point { x: 3, y: 4 };
        let br = Point { x: 6, y: 4 };
        let inside = Point { x: 4, y: 3 };
        let outside = Point { x: 2, y: 2 };

        rectangles.insert(1, rect1);
        rectangles.insert(2, rect2);
        rectangles.insert(3, rect3);

        assert!(contains_point(rect2, tl));
        assert!(contains_point(rect2, tr));
        assert!(contains_point(rect2, bl));
        assert!(contains_point(rect2, br));
        assert!(contains_point(rect2, inside));
        assert!(!contains_point(rect2, outside));

        assert_eq!(check_points(&rectangles, 8, 8)?, 4);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find_non_overlaps, Point, Rectangle};
    use anyhow::Result;
    use std::collections::BTreeMap;

    #[test]
    fn solution() -> Result<()> {
        let mut rectangles = BTreeMap::new();
        let rect1 = Rectangle {
            top_left: Point { x: 1, y: 3 },
            bottom_right: Point { x: 4, y: 6 },
        };
        let rect2 = Rectangle {
            top_left: Point { x: 3, y: 1 },
            bottom_right: Point { x: 6, y: 4 },
        };
        let rect3 = Rectangle {
            top_left: Point { x: 5, y: 5 },
            bottom_right: Point { x: 6, y: 6 },
        };

        rectangles.insert(1, rect1);
        rectangles.insert(2, rect2);
        rectangles.insert(3, rect3);

        assert_eq!(find_non_overlaps(&rectangles)?, 3);
        Ok(())
    }
}
