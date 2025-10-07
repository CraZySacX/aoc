//! Advent of Code - Day 20 'Particle Swarm' Solution
use anyhow::{Result, anyhow};
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// A particle has x,y,z coords, a velocity, an acceleration,
/// and a Manhattan Distance from the origin.
struct Particle {
    /// x,y,z coords
    coords: Coords,
    /// particle velocity
    vel: Velocity,
    /// particle acceleration
    acc: Acc,
    /// Manhattan Distance from origin.
    md: usize,
}

/// 3-d coordinates
#[derive(Clone, Eq, PartialEq)]
struct Coords {
    /// x coordinate
    x: i64,
    /// y coordinate
    y: i64,
    /// z coordinate
    z: i64,
}

/// particle velocity
struct Velocity {
    /// velocity in the x-direciton.
    vx: i64,
    /// velocity in the y-direciton.
    vy: i64,
    /// velocity in the z-direciton.
    vz: i64,
}

/// particle acceleration
struct Acc {
    /// acceleration in the x direction.
    ax: i64,
    /// acceleration in the x direction.
    ay: i64,
    /// acceleration in the x direction.
    az: i64,
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut particle_map: HashMap<usize, Particle> = HashMap::new();
    let coords_re = Regex::new(r"p=< *(-?\d+),(-?\d+),(-?\d+)>")?;
    let vel_re = Regex::new(r"v=< *(-?\d+),(-?\d+),(-?\d+)>")?;
    let acc_re = Regex::new(r"a=< *(-?\d+),(-?\d+),(-?\d+)>")?;

    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        add_particle_to_map(idx, line, &mut particle_map, &coords_re, &vel_re, &acc_re)?;
    }

    for _ in 0..1000 {
        for particle in particle_map.values_mut() {
            update_particle(particle)?;
        }

        if second_star {
            let matches = find_collisions(&particle_map)?;

            for idx in matches {
                particle_map.remove(&idx);
            }
            writeln!(io::stdout(), "{}", particle_map.len())?;
        } else {
            let idx = find_minimum_md(&particle_map)?;
            writeln!(io::stdout(), "{idx}")?;
        }
    }

    writeln!(io::stdout())?;
    Ok(0)
}

/// Add a particle to the particle map
#[allow(clippy::similar_names)]
fn add_particle_to_map(idx: usize, line: &str, particle_map: &mut HashMap<usize, Particle>, coords_re: &Regex, vel_re: &Regex, acc_re: &Regex) -> Result<()> {
    let parts: Vec<&str> = line.split(", ").collect();

    let coords = if coords_re.is_match(parts[0]) {
        let caps = coords_re.captures(parts[0]).ok_or(anyhow!("invalid coords captures"))?;
        let x_str = caps.get(1).ok_or(anyhow!("invalid x value"))?.as_str();
        let y_str = caps.get(2).ok_or(anyhow!("invalid y value"))?.as_str();
        let z_str = caps.get(3).ok_or(anyhow!("invalid z value"))?.as_str();
        let x = x_str.parse::<i64>()?;
        let y = y_str.parse::<i64>()?;
        let z = z_str.parse::<i64>()?;
        Coords { x, y, z }
    } else {
        return Err(anyhow!("invalid coordinates"));
    };

    let velocity = if vel_re.is_match(parts[1]) {
        let caps = vel_re.captures(parts[1]).ok_or(anyhow!("invalid velocity captures"))?;
        let vx_str = caps.get(1).ok_or(anyhow!("invalid vx value"))?.as_str();
        let vy_str = caps.get(2).ok_or(anyhow!("invalid vy value"))?.as_str();
        let vz_str = caps.get(3).ok_or(anyhow!("invalid vz value"))?.as_str();
        let vx = vx_str.parse::<i64>()?;
        let vy = vy_str.parse::<i64>()?;
        let vz = vz_str.parse::<i64>()?;
        Velocity { vx, vy, vz }
    } else {
        return Err(anyhow!("invalid velocity"));
    };

    let acc = if acc_re.is_match(parts[2]) {
        let caps = acc_re.captures(parts[2]).ok_or(anyhow!("invalid acceleration captures"))?;
        let ax_str = caps.get(1).ok_or(anyhow!("invalid ax value"))?.as_str();
        let ay_str = caps.get(2).ok_or(anyhow!("invalid ay value"))?.as_str();
        let az_str = caps.get(3).ok_or(anyhow!("invalid az value"))?.as_str();
        let ax = ax_str.parse::<i64>()?;
        let ay = ay_str.parse::<i64>()?;
        let az = az_str.parse::<i64>()?;
        Acc { ax, ay, az }
    } else {
        return Err(anyhow!("invalid acceleration"));
    };

    let md: usize = TryFrom::try_from(coords.x.abs() + coords.y.abs() + coords.z.abs())?;
    let particle = Particle {
        coords,
        vel: velocity,
        acc,
        md,
    };
    particle_map.insert(idx, particle);
    Ok(())
}

/// Update a particle
fn update_particle(particle: &mut Particle) -> Result<()> {
    particle.vel.vx += particle.acc.ax;
    particle.vel.vy += particle.acc.ay;
    particle.vel.vz += particle.acc.az;
    particle.coords.x += particle.vel.vx;
    particle.coords.y += particle.vel.vy;
    particle.coords.z += particle.vel.vz;
    particle.md = TryFrom::try_from(particle.coords.x.abs() + particle.coords.y.abs() + particle.coords.z.abs())?;
    Ok(())
}

/// Find the minimum Manhattan distance in the map.
fn find_minimum_md(particle_map: &HashMap<usize, Particle>) -> Result<usize> {
    let (min_idx, _) = particle_map
        .iter()
        .min_by_key(|&(_, particle)| particle.md)
        .ok_or(anyhow!("No minimum found"))?;

    Ok(*min_idx)
}

/// Remove collisions
fn find_collisions(particle_map: &HashMap<usize, Particle>) -> Result<Vec<usize>> {
    let all_coords: HashMap<usize, Coords> = particle_map.iter().map(|(k, p)| (*k, p.coords.clone())).collect();
    let mut matches = Vec::new();

    for (k, v) in particle_map {
        for (j, c1) in &all_coords {
            if *c1 == v.coords && j != k {
                matches.push(*k);
            }
        }
    }

    matches.sort_unstable();
    matches.dedup();

    Ok(matches)
}

#[cfg(test)]
mod one_star {
    use super::Particle;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut particle_map: HashMap<usize, Particle> = HashMap::new();
        let coords_re = Regex::new(r"p=< *(-?\d+),(-?\d+),(-?\d+)>").expect("");
        let vel_re = Regex::new(r"v=< *(-?\d+),(-?\d+),(-?\d+)>").expect("");
        let acc_re = Regex::new(r"a=< *(-?\d+),(-?\d+),(-?\d+)>").expect("");
        super::add_particle_to_map(0, "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>", &mut particle_map, &coords_re, &vel_re, &acc_re).expect("");
        super::add_particle_to_map(1, "p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>", &mut particle_map, &coords_re, &vel_re, &acc_re).expect("");

        for particle in particle_map.values_mut() {
            super::update_particle(particle).expect("");
        }

        assert_eq!(super::find_minimum_md(&particle_map).expect(""), 1);

        for particle in particle_map.values_mut() {
            super::update_particle(particle).expect("");
        }

        assert_eq!(super::find_minimum_md(&particle_map).expect(""), 1);

        for particle in particle_map.values_mut() {
            super::update_particle(particle).expect("");
        }

        assert_eq!(super::find_minimum_md(&particle_map).expect(""), 0);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
