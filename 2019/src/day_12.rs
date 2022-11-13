use regex::Regex;
use std::{cmp::Ordering, collections::HashSet};

type Point3D = (i16, i16, i16);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Moon {
    pos: Point3D,
    vel: Point3D,
}

type Moons = [Moon; 4];

#[aoc_generator(day12)]
fn generate(input: &str) -> Moons {
    let reg = Regex::new(r"<x=(-?\w+), y=(-?\w+), z=(-?\w+)>").unwrap();
    let mut moons = [Moon::default(); 4];
    let mut lines = input.lines();
    for x in moons.iter_mut() {
        let l = lines.next().unwrap();
        let caps = reg.captures(l).unwrap();
        x.pos.0 = caps[1].parse().unwrap();
        x.pos.1 = caps[2].parse().unwrap();
        x.pos.2 = caps[3].parse().unwrap();
    }
    moons
}

fn apply_gravity(input: &mut Moons) {
    let old = *input;
    for moon in input.iter_mut() {
        for other in old.iter() {
            moon.vel.0 += match moon.pos.0.cmp(&other.pos.0) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
            moon.vel.1 += match moon.pos.1.cmp(&other.pos.1) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
            moon.vel.2 += match moon.pos.2.cmp(&other.pos.2) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            }
        }
    }
}

fn apply_vel(input: &mut Moons) {
    for moon in input.iter_mut() {
        moon.pos.0 += moon.vel.0;
        moon.pos.1 += moon.vel.1;
        moon.pos.2 += moon.vel.2;
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Moons) -> i16 {
    let mut moons = *input;
    for _ in 0..1000 {
        apply_gravity(&mut moons);
        apply_vel(&mut moons);
    }
    moons
        .iter()
        .map(|x| {
            (x.pos.0.abs() + x.pos.1.abs() + x.pos.2.abs())
                * (x.vel.0.abs() + x.vel.1.abs() + x.vel.2.abs())
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &Moons) -> usize {
    let mut moons = *input;

    // they are independent, so each axis has its own loop
    let mut seenx = HashSet::new();
    let mut seeny = HashSet::new();
    let mut seenz = HashSet::new();

    let mut resx = None;
    let mut resy = None;
    let mut resz = None;

    let mut i = 0usize;
    let (x, y, z) = loop {
        i += 1;
        apply_gravity(&mut moons);
        apply_vel(&mut moons);
        if resx.is_none() {
            let x = get_all(0, &moons);
            if seenx.contains(&x) {
                resx = Some(i);
            } else {
                seenx.insert(x);
            }
        }
        if resy.is_none() {
            let y = get_all(1, &moons);
            if seeny.contains(&y) {
                resy = Some(i);
            } else {
                seeny.insert(y);
            }
        }
        if resz.is_none() {
            let z = get_all(2, &moons);
            if seenz.contains(&z) {
                resz = Some(i);
            } else {
                seenz.insert(z);
            }
        }
        if let Some(x) = resx {
            if let Some(y) = resy {
                if let Some(z) = resz {
                    // the starting pos isn't in the hashset, so we found the 2nd position twice
                    break (x - 1, y - 1, z - 1);
                }
            }
        }
    };

    let lcd1 = x * (y / gcd(x, y));
    let lcd2 = y * (z / gcd(y, z));
    lcd1 * (lcd2 / gcd(lcd1, lcd2))
}

fn get_all(xyz: usize, input: &Moons) -> [(i16, i16); 4] {
    let mut res = [Default::default(); 4];
    for x in 0..4 {
        let m = input[x];
        let mpos = [m.pos.0, m.pos.1, m.pos.2];
        let mvel = [m.vel.0, m.vel.1, m.vel.2];
        res[x] = (mpos[xyz], mvel[xyz]);
    }

    res
}

// https://github.com/frewsxcv/rust-gcd/blob/ab61dfd06b59f639c7acd6dc8b0493ac868e637c/src/lib.rs#L36
fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == v {
        return u;
    }
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}
