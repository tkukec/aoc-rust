fn generate(input: &str) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split(' ')
                .filter(|x| !x.is_empty())
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect();
    out.iter_mut().for_each(|x| x.sort());
    out
}

fn generate_pt2(input: &str) -> Vec<Vec<i32>> {
    let out: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split(' ')
                .filter(|x| !x.is_empty())
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect();
    let mut out: Vec<Vec<i32>> = out
        .chunks(3)
        .flat_map(|x| {
            vec![
                vec![x[0][0], x[1][0], x[2][0]],
                vec![x[0][1], x[1][1], x[2][1]],
                vec![x[0][2], x[1][2], x[2][2]],
            ]
        })
        .collect();
    out.iter_mut().for_each(|x| x.sort());
    out
}

#[aoc(day03, part1)]
pub fn part1(input: &str) -> i32 {
    generate(input)
        .into_iter()
        .filter(|x| x[0] + x[1] > x[2])
        .count() as i32
}

#[aoc(day03, part2)]
pub fn part2(input: &str) -> i32 {
    generate_pt2(input)
        .into_iter()
        .filter(|x| x[0] + x[1] > x[2])
        .count() as i32
}
