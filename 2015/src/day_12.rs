use regex::Regex;
use serde_json::{json, Value};
#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;
    for i in re.captures_iter(input) {
        sum += &i[0].parse().unwrap();
    }
    sum
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
    parse_value(&serde_json::from_str(input).unwrap())
}

fn parse_value(thing: &Value) -> i32 {
    match thing {
        Value::Null => 0,
        Value::Bool(_) => unreachable!(),
        Value::Number(x) => x.as_i64().unwrap() as i32,
        Value::String(_) => 0,
        Value::Array(x) => x.iter().map(parse_value).sum(),
        Value::Object(x) => {
            if !x.values().any(|x| x == &json!("red")) {
                x.values().map(parse_value).sum()
            } else {
                0
            }
        }
    }
}
