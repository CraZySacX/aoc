//! Advent of Code - Day 20 'Particle Swarm' Solution
use error::Result;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

struct Particle {
    coords: Coords,
    vel: Velocity,
    acc: Acc,
    md: usize,
}

struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

struct Velocity {
    vx: i64,
    vy: i64,
    vz: i64,
}

struct Acc {
    ax: i64,
    ay: i64,
    az: i64,
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut particle_map: HashMap<usize, Particle> = HashMap::new();
    let coords_re = Regex::new(r"p=< *(-?\d+),(-?\d+),(-?\d+)>")?;
    let vel_re = Regex::new(r"v=< *(-?\d+),(-?\d+),(-?\d+)>")?;
    let acc_re = Regex::new(r"a=< *(-?\d+),(-?\d+),(-?\d+)>")?;

    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        add_particle_to_map(idx, line, &mut particle_map, &coords_re, &vel_re, &acc_re)?;
    }

    for _ in 0..10_000 {
        for mut particle in particle_map.values_mut() {
            update_particle(&mut particle)?;
        }
        let idx = find_minimum_md(&particle_map)?;
        write!(io::stdout(), "{}\r", idx)?;
    }
    Ok(0)
}

/// Add a particle to the particle map
fn add_particle_to_map(idx: usize, line: &str, particle_map: &mut HashMap<usize, Particle>, coords_re: &Regex, vel_re: &Regex, acc_re: &Regex) -> Result<()> {
    let parts: Vec<&str> = line.split(", ").collect();

    let coords = if coords_re.is_match(parts[0]) {
        let caps = coords_re
            .captures(parts[0])
            .ok_or("invalid coords captures")?;
        let x_str = caps.get(1).ok_or("invalid x value")?.as_str();
        let y_str = caps.get(2).ok_or("invalid y value")?.as_str();
        let z_str = caps.get(3).ok_or("invalid z value")?.as_str();
        let x = x_str.parse::<i64>()?;
        let y = y_str.parse::<i64>()?;
        let z = z_str.parse::<i64>()?;
        Coords { x: x, y: y, z: z }
    } else {
        return Err("invalid coordinates".into());
    };

    let velocity = if vel_re.is_match(parts[1]) {
        let caps = vel_re
            .captures(parts[1])
            .ok_or("invalid velocity captures")?;
        let vx_str = caps.get(1).ok_or("invalid vx value")?.as_str();
        let vy_str = caps.get(2).ok_or("invalid vy value")?.as_str();
        let vz_str = caps.get(3).ok_or("invalid vz value")?.as_str();
        let vx = vx_str.parse::<i64>()?;
        let vy = vy_str.parse::<i64>()?;
        let vz = vz_str.parse::<i64>()?;
        Velocity {
            vx: vx,
            vy: vy,
            vz: vz,
        }
    } else {
        return Err("invalid velocity".into());
    };

    let acc = if acc_re.is_match(parts[2]) {
        let caps = acc_re
            .captures(parts[2])
            .ok_or("invalid acceleration captures")?;
        let ax_str = caps.get(1).ok_or("invalid ax value")?.as_str();
        let ay_str = caps.get(2).ok_or("invalid ay value")?.as_str();
        let az_str = caps.get(3).ok_or("invalid az value")?.as_str();
        let ax = ax_str.parse::<i64>()?;
        let ay = ay_str.parse::<i64>()?;
        let az = az_str.parse::<i64>()?;
        Acc {
            ax: ax,
            ay: ay,
            az: az,
        }
    } else {
        return Err("invalid acceleration".into());
    };

    let md: usize = TryFrom::try_from(coords.x.abs() + coords.y.abs() + coords.z.abs())?;
    let particle = Particle {
        coords: coords,
        vel: velocity,
        acc: acc,
        md: md,
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
    let mut minimum_md = usize::max_value();
    let mut idx = usize::max_value();

    for (k, particle) in particle_map {
        if particle.md < minimum_md {
            minimum_md = particle.md;
            idx = *k;
        }
    }

    Ok(idx)
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
        super::add_particle_to_map(
            0,
            "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>",
            &mut particle_map,
            &coords_re,
            &vel_re,
            &acc_re,
        ).expect("");
        super::add_particle_to_map(
            1,
            "p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>",
            &mut particle_map,
            &coords_re,
            &vel_re,
            &acc_re,
        ).expect("");

        for mut particle in particle_map.values_mut() {
            super::update_particle(&mut particle).expect("");
        }

        assert_eq!(super::find_minimum_md(&particle_map).expect(""), 1);

        for mut particle in particle_map.values_mut() {
            super::update_particle(&mut particle).expect("");
        }

        assert_eq!(super::find_minimum_md(&particle_map).expect(""), 1);

        for mut particle in particle_map.values_mut() {
            super::update_particle(&mut particle).expect("");
        }

        assert_eq!(super::find_minimum_md(&particle_map).expect(""), 0);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
