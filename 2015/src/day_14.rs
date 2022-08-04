#[derive(Debug)]
struct Reindeer {
    speed: u16,
    fly_time: u16,
    rest_time: u16,
}

const TIME: u16 = 2503;

fn generate(input: &str) -> Vec<Reindeer> {
    let mut out = Vec::new();
    for mut i in input.lines().map(|x| x.split(' ')) {
        out.push(Reindeer {
            speed: i.nth(3).unwrap().parse().unwrap(),
            fly_time: i.nth(2).unwrap().parse().unwrap(),
            rest_time: i.nth_back(1).unwrap().parse().unwrap(),
        })
    }
    out
}

fn dist_travelled(x: &Reindeer, time: u16) -> u16 {
    let mut time_left = time;
    let mut flying = true;
    let mut dist = 0;
    while time_left != 0 {
        if flying && time_left >= x.fly_time {
            dist += x.speed * x.fly_time;
            time_left -= x.fly_time;
        } else if flying && time_left < x.fly_time {
            // Can't use the same saturating_sub trick here, because you only have a part of the
            // last sprint, and not the entire thing
            dist += x.speed * time_left;
            time_left = 0;
        } else {
            // overflows lead to infinite loops, wouldn't recommend
            time_left = time_left.saturating_sub(x.rest_time);
        }
        flying = !flying;
    }
    dist
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u16 {
    let racers = generate(input);
    racers
        .into_iter()
        .map(|x| dist_travelled(&x, TIME))
        .max()
        .unwrap()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u16 {
    let racers = generate(input);
    let mut racers: Vec<(u16, Reindeer)> = racers.into_iter().map(|x| (0, x)).collect();
    // REALLY brute force, might try to find a better way later
    for j in 1..=TIME {
        let scores: Vec<u16> = racers.iter().map(|(_, y)| dist_travelled(y, j)).collect();
        let best = scores.iter().max().unwrap();
        scores.iter().enumerate().for_each(|(i, x)| {
            if x == best {
                racers.get_mut(i).unwrap().0 += 1;
            }
        });
    }
    *racers.iter().map(|(x, _)| x).max().unwrap()
}
