use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
#[derive(Debug)]
struct Item(i32, i32, i32); // cost, damage, armor

fn get_shop_items() -> Vec<Vec<Item>> {
    vec![
        vec![
            Item(8, 4, 0),
            Item(10, 5, 0),
            Item(25, 6, 0),
            Item(40, 7, 0),
            Item(74, 8, 0),
        ],
        vec![
            Item(0, 0, 0),
            Item(13, 0, 1),
            Item(31, 0, 2),
            Item(53, 0, 3),
            Item(75, 0, 4),
            Item(102, 0, 5),
        ],
        vec![
            Item(0, 0, 0),
            Item(25, 1, 0),
            Item(50, 2, 0),
            Item(100, 3, 0),
            Item(20, 0, 1),
            Item(40, 0, 2),
            Item(80, 0, 3),
        ],
    ]
}

fn do_i_win(
    my_hp: i32,
    my_arm: i32,
    my_dmg: i32,
    boss_hp: i32,
    boss_arm: i32,
    boss_dmg: i32,
) -> bool {
    let my_calc_dmg = max(1, my_dmg - boss_arm);
    let boss_calc_dmg = max(1, boss_dmg - my_arm);

    (boss_hp + my_calc_dmg - 1) / my_calc_dmg <= (my_hp + boss_calc_dmg - 1) / boss_calc_dmg
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> i32 {
    let mut data = input
        .lines()
        .map(|x| x.split(' ').find_map(|n| n.parse::<i32>().ok()));
    let (bh, bd, ba) = (
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
    );
    let items = get_shop_items();
    let (weapons, armor, rings) = (&items[0], &items[1], &items[2]);
    let mut best = i32::MAX;
    for we in weapons {
        for ar in armor {
            for r in rings
                .iter()
                .combinations(2)
                // adding the last option here to prevent double counting
                // ~45% speed increase from having 2 (0, 0, 0) rings
                .chain([vec![&Item(0, 0, 0), &Item(0, 0, 0)]])
            {
                let (r1, r2) = (r[0], r[1]);
                let total_cost = we.0 + ar.0 + r1.0 + r2.0;
                let total_dmg = we.1 + r1.1 + r2.1;
                let total_arm = ar.2 + r1.2 + r2.2;
                if do_i_win(100, total_arm, total_dmg, bh, ba, bd) {
                    best = min(best, total_cost);
                }
            }
        }
    }
    best
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i32 {
    let mut data = input
        .lines()
        .map(|x| x.split(' ').find_map(|n| n.parse::<i32>().ok()));
    let (bh, bd, ba) = (
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
    );
    let items = get_shop_items();
    let (weapons, armor, rings) = (&items[0], &items[1], &items[2]);
    let mut best = 0;
    for we in weapons {
        for ar in armor {
            for r in rings
                .iter()
                .combinations(2)
                .chain([vec![&Item(0, 0, 0), &Item(0, 0, 0)]])
            {
                let (r1, r2) = (r[0], r[1]);
                let total_cost = we.0 + ar.0 + r1.0 + r2.0;
                let total_dmg = we.1 + r1.1 + r2.1;
                let total_arm = ar.2 + r1.2 + r2.2;
                if !do_i_win(100, total_arm, total_dmg, bh, ba, bd) {
                    best = max(best, total_cost);
                }
            }
        }
    }
    best
}

#[aoc(day21, part1, optimized)]
// 89.90% faster, by using lookup tables for best prices
pub fn part1_better(input: &str) -> i32 {
    let mut data = input
        .lines()
        .map(|x| x.split(' ').find_map(|n| n.parse::<i32>().ok()));
    let (bh, bd, ba) = (
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
    );

    // tables that give the best price for a given armor/weapon level
    let cost_ar = HashMap::from([
        (0, 0),
        (1, 13),
        (2, 31),
        (3, 51),
        (4, 71),
        (5, 93),
        (6, 115),
        (7, 142),
        (8, 182),
    ]);
    let cost_we = HashMap::from([
        (4, 8),
        (5, 10),
        (6, 25),
        (7, 40),
        (8, 65),
        (9, 90),
        (10, 124),
        (11, 174),
    ]);

    let mut cost = i32::MAX;
    for we in 4..=11 {
        for ar in 0..=8 {
            if do_i_win(100, ar, we, bh, ba, bd) {
                cost = min(cost, cost_ar[&ar] + cost_we[&we]);
            }
        }
    }
    cost
}

#[aoc(day21, part2, optimized)]
// 94.06% faster, by using lookup tables for worst prices
pub fn part2_better(input: &str) -> i32 {
    let mut data = input
        .lines()
        .map(|x| x.split(' ').find_map(|n| n.parse::<i32>().ok()));
    let (bh, bd, ba) = (
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
        data.next().unwrap().unwrap(),
    );

    // tables that give the worst price for a given armor/weapon level
    let cost_ar = HashMap::from([
        (0, 0),
        (1, 20),
        (2, 40),
        (3, 80),
        (4, 93),
        (5, 111),
        (6, 133),
        (7, 155),
        (8, 182),
    ]);
    let cost_we = HashMap::from([
        (4, 8),
        (5, 33),
        (6, 58),
        (7, 108),
        (8, 110),
        (9, 125),
        (10, 140),
        (11, 174),
    ]);

    let mut cost = 0;
    for we in 4..=11 {
        for ar in 0..=8 {
            if !do_i_win(100, ar, we, bh, ba, bd) {
                cost = max(cost, cost_ar[&ar] + cost_we[&we]);
            }
        }
    }
    cost
}
