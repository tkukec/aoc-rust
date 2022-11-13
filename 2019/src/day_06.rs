use std::collections::HashMap;

#[aoc_generator(day06)]
fn generate(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (parent, child) in input.lines().filter_map(|l| l.split_once(')')) {
        map.entry(parent.to_owned())
            .or_default()
            .push(child.to_owned());
    }

    map
}
#[aoc(day06, part1)]
pub fn part1(input: &HashMap<String, Vec<String>>) -> u32 {
    let mut rev_map: HashMap<String, String> = HashMap::new();
    input.iter().for_each(|(k, v)| {
        v.iter().for_each(|e| {
            rev_map.insert(e.clone(), k.clone());
        })
    });
    fn count_indirect(map: &HashMap<String, String>, elem: String) -> u32 {
        if elem == "COM" {
            0
        } else {
            1 + count_indirect(map, map[&elem].clone())
        }
    }

    rev_map
        .keys()
        .map(|k| count_indirect(&rev_map, k.clone()))
        .sum()
}

#[aoc(day06, part2)]
pub fn part2(input: &HashMap<String, Vec<String>>) -> usize {
    let mut rev_map: HashMap<String, String> = HashMap::new();
    input.iter().for_each(|(k, v)| {
        v.iter().for_each(|e| {
            rev_map.insert(e.clone(), k.clone());
        })
    });
    fn get_paren(map: &HashMap<String, String>, elem: String) -> Vec<String> {
        let mut out = vec![];
        let mut cur = elem;
        while let Some(par) = map.get(&cur).cloned() {
            out.push(par.clone());
            cur = par;
        }
        out
    }

    // just get the parents of each one and
    let me_paren = get_paren(&rev_map, "YOU".to_owned());
    let santa_paren = get_paren(&rev_map, "SAN".to_owned());

    let (me_index, first_together) = me_paren
        .iter()
        .enumerate()
        .find(|(_, x)| santa_paren.contains(x))
        .expect("No together node (bad)");
    let (santa_index, _) = santa_paren
        .iter()
        .enumerate()
        .find(|(_, x)| x == &first_together)
        .unwrap();

    me_index + santa_index
}
