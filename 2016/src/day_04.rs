use itertools::Itertools;

#[aoc(day04, part1)]
pub fn part1(input: &str) -> i32 {
    let mut rooms: Vec<(Vec<char>, String, i32)> = input
        .lines()
        .map(|x| {
            (
                x.split('-')
                    .take_while(|x| x.chars().next().unwrap().is_ascii_lowercase())
                    .join("")
                    .chars()
                    .collect(),
                x.chars().skip_while(|x| x != &'[').collect::<String>(),
                x.chars()
                    .skip_while(|c| !c.is_numeric())
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        })
        .collect();
    rooms.iter_mut().for_each(|(x, _, _)| {
        let a = x.clone();
        x.sort_by(|c1, c2| {
            a.iter()
                .filter(|c| c == &c2)
                .count()
                .cmp(&a.iter().filter(|c| c == &c1).count())
                .then(c1.cmp(c2))
        })
    });
    rooms
        .iter()
        .filter(|(s, best, _)| {
            let s_i: String = s.iter().unique().collect();

            s_i[0..5] == best[1..=5]
        })
        .map(|x| x.2)
        .sum()
}

#[aoc(day04, part2)]
pub fn part2(input: &str) -> u32 {
    let rooms: Vec<(Vec<char>, String, u32)> = input
        .lines()
        .map(|x| {
            (
                x.split('-')
                    .take_while(|x| x.chars().next().unwrap().is_ascii_lowercase())
                    .join("")
                    .chars()
                    .collect(),
                x.chars().skip_while(|x| x != &'[').collect::<String>(),
                x.chars()
                    .skip_while(|c| !c.is_numeric())
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        })
        .collect();
    rooms
        .iter()
        .find(|(s, _, i)| {
            s.iter()
                .map(|c| {
                    char::from_digit((c.to_digit(36).unwrap() - 10 + i) % 26 + 10, 36).unwrap()
                })
                .collect::<String>()
                == "northpoleobjectstorage"
        })
        .unwrap()
        .2
}
