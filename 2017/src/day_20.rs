use itertools::Itertools;
use regex::Regex;
#[derive(Clone, Copy)]
pub struct Particle {
    id: u32,
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

impl Particle {
    fn dist(&self) -> i64 {
        self.p.0.abs() + self.p.1.abs() + self.p.2.abs()
    }
    fn update(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
}
#[aoc_generator(day20)]
pub fn generate(input: &str) -> Vec<Particle> {
    let reg = Regex::new(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>",
    )
    .unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let a = reg.captures(l).unwrap();

            Particle {
                id: i as u32,
                p: (
                    a[1].parse().unwrap(),
                    a[2].parse().unwrap(),
                    a[3].parse().unwrap(),
                ),
                v: (
                    a[4].parse().unwrap(),
                    a[5].parse().unwrap(),
                    a[6].parse().unwrap(),
                ),
                a: (
                    a[7].parse().unwrap(),
                    a[8].parse().unwrap(),
                    a[9].parse().unwrap(),
                ),
            }
        })
        .collect()
}
#[aoc(day20, part1)]
pub fn part1(input: &[Particle]) -> u32 {
    let mut particles = input.to_vec();
    for _ in 0..500 {
        for p in particles.iter_mut() {
            p.update();
        }
    }
    let closest = particles
        .iter()
        .min_by(|x, y| x.dist().cmp(&y.dist()))
        .unwrap();
    closest.id
}

#[aoc(day20, part2)]
pub fn part2(input: &[Particle]) -> usize {
    let mut particles = input.to_vec();
    for _ in 0..500 {
        particles.sort_by(|x, y| x.p.cmp(&y.p));

        let (_, dupl) = particles.partition_dedup_by_key(|x| x.p);
        let dupl = dupl.iter().map(|x| x.p).collect_vec();

        particles.retain(|x| !dupl.contains(&x.p));

        for p in particles.iter_mut() {
            p.update();
        }
    }
    particles.len()
}
