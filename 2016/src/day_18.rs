fn is_trap(a: bool, b: bool, c: bool) -> bool {
    // clippy simplification of (a && b && !c) || (b && c && !a) || ((a ^ c) && !b)
    !(!b || a && c || !a && !c) || ((a ^ c) && !b)
}
fn get_next_row(x: Vec<bool>) -> Vec<bool> {
    let mut new = vec![is_trap(false, x[0], x[1])];
    for t in x.windows(3) {
        new.push(is_trap(t[0], t[1], t[2]));
    }
    new.push(is_trap(x[x.len() - 2], x[x.len() - 1], false));
    new
}
#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let mut row: Vec<bool> = input.chars().map(|c| c == '^').collect();
    let mut cnt = 0;
    for _ in 0..40 {
        cnt += row.iter().filter(|&&x| !x).count();
        row = get_next_row(row);
    }
    cnt
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let mut row: Vec<bool> = input.chars().map(|c| c == '^').collect();
    let mut cnt = 0;
    for _ in 0..400000 {
        cnt += row.iter().filter(|&&x| !x).count();
        row = get_next_row(row);
    }
    cnt
}
