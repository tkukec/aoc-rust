use std::collections::HashMap;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let mut registers = HashMap::new();
    for i in input.lines() {
        let mut it = i.split(' ');
        let to_change = it.next().unwrap();
        let change_by = match it.next().unwrap() {
            "inc" => it.next().unwrap().parse().unwrap(),
            "dec" => -it.next().unwrap().parse::<i32>().unwrap(),
            _ => unreachable!(),
        };
        let check_reg = it.nth(1).unwrap();
        let check_val = *registers.get(check_reg).unwrap_or(&0);
        let sign = it.next().unwrap();
        let val = it.next().unwrap().parse().unwrap();
        let if_passes = match sign {
            "==" => check_val == val,
            "!=" => check_val != val,
            ">" => check_val > val,
            ">=" => check_val >= val,
            "<" => check_val < val,
            "<=" => check_val <= val,
            _ => unreachable!(),
        };
        if if_passes {
            *registers.entry(to_change).or_insert(0i32) += change_by;
        }
    }
    *registers.values().max().unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let mut registers = HashMap::new();
    let mut max_ever = 0;
    for i in input.lines() {
        let mut it = i.split(' ');
        let to_change = it.next().unwrap();
        let change_by = match it.next().unwrap() {
            "inc" => it.next().unwrap().parse().unwrap(),
            "dec" => -it.next().unwrap().parse::<i32>().unwrap(),
            _ => unreachable!(),
        };
        let check_reg = it.nth(1).unwrap();
        let check_val = *registers.get(check_reg).unwrap_or(&0);
        let sign = it.next().unwrap();
        let val = it.next().unwrap().parse().unwrap();
        let if_passes = match sign {
            "==" => check_val == val,
            "!=" => check_val != val,
            ">" => check_val > val,
            ">=" => check_val >= val,
            "<" => check_val < val,
            "<=" => check_val <= val,
            _ => unreachable!(),
        };
        if if_passes {
            *registers.entry(to_change).or_insert(0i32) += change_by;
        }
        max_ever = std::cmp::max(max_ever, *registers.values().max().unwrap());
    }
    max_ever
}
