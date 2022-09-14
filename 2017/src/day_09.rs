#[aoc(day9, part1)]
pub fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let mut in_garbage = false;
    let mut cur_level = 0;
    let mut input = input.chars();
    while let Some(cur) = input.next() {
        if cur == '!' {
            input.next();
            continue;
        }

        if in_garbage {
            if cur == '>' {
                in_garbage = false;
            }
        } else if cur == '{' {
            cur_level += 1;
        } else if cur == '}' {
            sum += cur_level;
            cur_level -= 1;
        } else if cur == '<' {
            in_garbage = true;
        }
    }
    sum
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut in_garbage = false;
    let mut input = input.chars();
    while let Some(cur) = input.next() {
        if cur == '!' {
            input.next();
            continue;
        }

        if in_garbage {
            if cur == '>' {
                in_garbage = false;
            } else {
                sum += 1;
            }
        } else if cur == '<' {
            in_garbage = true;
        }
    }
    sum
}
