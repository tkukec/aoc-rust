#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let mut j: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut ptr = 0;
    let len = j.len();
    let mut cnt = 0;
    while (0..len).contains(&ptr) {
        cnt += 1;
        j[ptr] += 1;
        // this overflows if j[ptr] is negative, but it works out, because it overflows twice back
        // to where it should be
        ptr += j[ptr] as usize - 1;
    }

    cnt
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let mut j: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut ptr = 0isize;
    let len = j.len() as isize;
    let mut cnt = 0;
    while (0..len).contains(&ptr) {
        cnt += 1;
        let old_ptr = ptr as usize;
        let old = j[old_ptr];
        ptr += old as isize;
        if old >= 3 {
            j[old_ptr] -= 1;
        } else {
            j[old_ptr] += 1;
        }
    }

    cnt
}
