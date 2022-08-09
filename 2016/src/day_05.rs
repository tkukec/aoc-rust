#[aoc(day05, part1)]
pub fn part1(input: &str) -> String {
    let mut sol = String::new();
    for i in (1..).map(|x| x.to_string()) {
        if sol.len() == 8 {
            break;
        }
        let s = md5::compute(input.to_owned() + &i);
        if s[0] == 0 && s[1] == 0 && s[2] < 16 {
            let g = format!("{:?}", s);
            sol.push(g.chars().nth(5).unwrap());
        }
    }
    sol
}

#[aoc(day05, part2)]
pub fn part2(input: &str) -> String {
    let mut sol = ['-'; 8].to_vec();
    for i in (1..).map(|x| x.to_string()) {
        if !sol.contains(&'-') {
            break;
        }
        let s = md5::compute(input.to_owned() + &i);
        if s[0] == 0 && s[1] == 0 && s[2] < 16 {
            let g = format!("{:?}", s);
            if let Some(num) = g.chars().nth(5).unwrap().to_digit(10) {
                if num < 8 && sol[num as usize] == '-' {
                    sol[num as usize] = g.chars().nth(6).unwrap();
                }
            }
        }
    }
    sol.iter().collect::<String>()
}
