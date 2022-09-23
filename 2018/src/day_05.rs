#[aoc(day05, part1)]
pub fn part1(input: &str) -> usize {
    let mut p = input.to_owned();
    let mut changed = true;
    while changed {
        changed = false;
        let mut new = String::new();
        let mut old = p.chars().peekable();
        while let Some(c) = old.next() {
            if let Some(next) = old.peek() {
                if !(c.is_ascii_lowercase() != next.is_ascii_lowercase()
                    && c.to_ascii_lowercase() == next.to_ascii_lowercase())
                {
                    new.push(c);
                } else {
                    old.next();
                    changed = true;
                }
            } else {
                new.push(c);
            }
        }
        p = new;
    }
    p.len()
}

#[aoc(day05, part2)]
pub fn part2(input: &str) -> usize {
    let mut best = usize::MAX;
    for to_remove in 'a'..='z' {
        let mut p = input.to_owned();
        p.retain(|c| c.to_ascii_lowercase() != to_remove);
        let mut changed = true;
        while changed {
            changed = false;
            let mut new = String::new();
            let mut old = p.chars().peekable();
            while let Some(c) = old.next() {
                if let Some(next) = old.peek() {
                    if !(c.is_ascii_lowercase() != next.is_ascii_lowercase()
                        && c.to_ascii_lowercase() == next.to_ascii_lowercase())
                    {
                        new.push(c);
                    } else {
                        old.next();
                        changed = true;
                    }
                } else {
                    new.push(c);
                }
            }
            p = new;
        }
        best = std::cmp::min(best, p.len());
    }
    best
}
