use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    R,
    L,
    U,
    D,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Line {
    dir: Direction,
    amount: u32,
}

impl From<char> for Direction {
    fn from(i: char) -> Self {
        match i {
            'R' => Direction::R,
            'L' => Direction::L,
            'U' => Direction::U,
            'D' => Direction::D,
            _ => panic!("Bad direction"),
        }
    }
}

impl From<&str> for Line {
    fn from(i: &str) -> Self {
        let mut a = i.chars();
        Line {
            dir: Direction::from(a.next().unwrap()),
            amount: a.collect::<String>().parse().unwrap(),
        }
    }
}

#[aoc_generator(day03)]
fn generate(input: &str) -> (Vec<Line>, Vec<Line>) {
    let (one, two) = input.split_once('\n').unwrap();
    let one = one.split(',').map(Line::from).collect();
    let two = two.split(',').map(Line::from).collect();
    (one, two)
}

type Point = (i32, i32);
#[aoc(day03, part1)]
pub fn part1(input: &(Vec<Line>, Vec<Line>)) -> i32 {
    let (one, two) = input.clone();
    let mut one_seen: HashSet<Point> = HashSet::new();
    let mut two_seen: HashSet<Point> = HashSet::new();

    for (list, set) in [(one, &mut one_seen), (two, &mut two_seen)] {
        let mut last = (0, 0);
        for Line { dir, amount } in list {
            match dir {
                Direction::R => {
                    for _ in 1..=amount {
                        last.0 += 1;
                        set.insert(last);
                    }
                }
                Direction::L => {
                    for _ in 1..=amount {
                        last.0 -= 1;
                        set.insert(last);
                    }
                }
                Direction::U => {
                    for _ in 1..=amount {
                        last.1 += 1;
                        set.insert(last);
                    }
                }
                Direction::D => {
                    for _ in 1..=amount {
                        last.1 -= 1;
                        set.insert(last);
                    }
                }
            }
        }
    }

    let best = one_seen
        .intersection(&two_seen)
        .min_by_key(|x| x.0.abs() + x.1.abs())
        .unwrap();
    best.0.abs() + best.1.abs()
}

#[aoc(day03, part2)]
pub fn part2(input: &(Vec<Line>, Vec<Line>)) -> usize {
    let (one, two) = input.clone();
    let mut one_seen: Vec<Point> = vec![];
    let mut two_seen: Vec<Point> = vec![];

    for (list, set) in [(one, &mut one_seen), (two, &mut two_seen)] {
        let mut last = (0, 0);
        for Line { dir, amount } in list {
            match dir {
                Direction::R => {
                    for _ in 1..=amount {
                        last.0 += 1;
                        set.push(last);
                    }
                }
                Direction::L => {
                    for _ in 1..=amount {
                        last.0 -= 1;
                        set.push(last);
                    }
                }
                Direction::U => {
                    for _ in 1..=amount {
                        last.1 += 1;
                        set.push(last);
                    }
                }
                Direction::D => {
                    for _ in 1..=amount {
                        last.1 -= 1;
                        set.push(last);
                    }
                }
            }
        }
    }

    let h1: HashSet<Point> = HashSet::from_iter(one_seen.iter().copied());
    let h2: HashSet<Point> = HashSet::from_iter(two_seen.iter().copied());

    // creating two new hashsets and doing this intersection is ~11s faster than just iterating
    // over the vectors directly. There must be a better way to do this. I tried it with HashMaps,
    // but they don't implement .intersection(), so that wouldn't work
    h1.intersection(&h2)
        .map(|x| {
            one_seen.iter().position(|p| p == x).unwrap()
                + two_seen.iter().position(|p| p == x).unwrap()
        })
        .min()
        .unwrap()
        + 2 // off by one error, but twice
}
