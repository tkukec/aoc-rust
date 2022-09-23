use std::collections::{HashMap, HashSet};

use itertools::Itertools;
#[aoc_generator(day04)]
//                                 guard  day   asleep_mins
fn generate(input: &str) -> HashMap<(i32, i32), Vec<i32>> {
    let mut asleep_times = HashMap::new();
    let mut last_guard = -1;
    let mut time_asleep = (0..60).collect_vec();
    let mut fell_asleep_time = -1;
    let mut date = 0; // month * 100 + day
    for x in input.lines().sorted() {
        match &x[19..=19] {
            "f" => {
                fell_asleep_time = (x[15..=16]).parse().unwrap();
            }
            "w" => {
                assert_ne!(fell_asleep_time, -1);
                let wake_up_time = x[15..=16].parse().unwrap();
                time_asleep.retain(|t| !(fell_asleep_time..wake_up_time).contains(t));
                fell_asleep_time = -1;
                date = ((x[6..=7]).to_owned() + &x[9..=10]).parse().unwrap()
            }
            "G" => {
                if last_guard != -1 {
                    assert_ne!(date, 0);
                    asleep_times.insert((last_guard, date), time_asleep.clone());
                }
                time_asleep = (0..60).collect_vec();
                last_guard = x.split(' ').nth(3).unwrap()[1..].parse().unwrap();
                fell_asleep_time = -1;
            }
            _ => unreachable!(),
        }
    }
    asleep_times
}
#[aoc(day04, part1)]
pub fn part1(input: &HashMap<(i32, i32), Vec<i32>>) -> i32 {
    let guards: HashSet<i32> = input.keys().map(|x| x.0).collect();
    let mut best_total_asleep = 0;
    let mut best_res = 0;
    for g in guards {
        let days = input.iter().filter(|((i, _), _)| *i == g).collect_vec();
        let mut asleep_times = HashMap::new();
        for t in 0..60 {
            let cnt = days.iter().filter(|x| !x.1.contains(&t)).count() as i32;
            asleep_times.insert(t, cnt);
        }
        let total_asleep: i32 = asleep_times.values().sum();
        if total_asleep > best_total_asleep {
            best_total_asleep = total_asleep;
            let best_min = asleep_times.iter().max_by_key(|x| x.1).unwrap().0;
            best_res = g * best_min;
        }
    }
    best_res
}

#[aoc(day04, part2)]
pub fn part2(input: &HashMap<(i32, i32), Vec<i32>>) -> i32 {
    let guards: HashSet<i32> = input.keys().map(|x| x.0).collect();
    let mut best_on_same_min = 0;
    let mut best_res = 0;
    for g in guards {
        let days = input.iter().filter(|((i, _), _)| *i == g).collect_vec();
        let mut asleep_times = HashMap::new();
        for t in 0..60 {
            let cnt = days.iter().filter(|x| !x.1.contains(&t)).count() as i32;
            asleep_times.insert(t, cnt);
        }
        let g_best_min_asleep = *asleep_times.values().max().unwrap();
        if g_best_min_asleep > best_on_same_min {
            best_on_same_min = g_best_min_asleep;
            let best_min = asleep_times.iter().max_by_key(|x| x.1).unwrap().0;
            best_res = g * best_min;
        }
    }
    best_res
}
