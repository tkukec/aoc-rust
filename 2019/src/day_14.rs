use std::collections::HashMap;

type Ingredient = (String, u64);
type RevRecipes = HashMap<Ingredient, Vec<Ingredient>>;

fn to_ingredient(x: &str) -> Ingredient {
    let (amount, name) = x.split_once(' ').unwrap();
    let amount = amount.parse().unwrap();
    let name = name.to_owned();
    (name, amount)
}

#[aoc_generator(day14)]
fn generate(input: &str) -> RevRecipes {
    let mut map = RevRecipes::new();
    for l in input.lines() {
        let (inp, out) = l.split_once(" => ").unwrap();
        let inp: Vec<Ingredient> = inp.split(", ").map(to_ingredient).collect();
        let out = to_ingredient(out);
        map.insert(out, inp);
    }
    map
}

fn ore_for(
    x: Ingredient,
    map: &RevRecipes,
    counts: &HashMap<String, u64>,
    storage: &mut HashMap<String, u64>,
) -> u64 {
    if x.0 == "ORE" {
        return x.1;
    }
    let in_storage = storage.entry(x.0.clone()).or_default();

    let to_make = if *in_storage != 0 {
        let tmp = *in_storage;
        *in_storage = in_storage.saturating_sub(x.1);
        x.1.saturating_sub(tmp)
    } else {
        x.1
    };

    if to_make == 0 {
        return 0;
    }

    let recipe_times = div_ceil(to_make, counts[&x.0]);
    let excess = counts[&x.0] * recipe_times - to_make;
    *in_storage += excess;
    let ingr = &map[&(x.0.clone(), counts[&x.0])];

    ingr.iter()
        .map(|(n, i)| (n.clone(), i * recipe_times))
        .map(|x| ore_for(x, map, counts, storage))
        .sum()
}

#[aoc(day14, part1)]
pub fn part1(map: &RevRecipes) -> u64 {
    let counts: HashMap<String, u64> = HashMap::from_iter(map.keys().cloned());
    let mut storage: HashMap<String, u64> = HashMap::new();

    ore_for(("FUEL".to_owned(), 1), map, &counts, &mut storage)
}

#[aoc(day14, part2)]
pub fn part2(map: &RevRecipes) -> u64 {
    let counts: HashMap<String, u64> = HashMap::from_iter(map.keys().cloned());
    let initial = 1000000000000 / part1(map);

    let mut best = 0;

    for i in (initial..).step_by(1000) {
        let mut storage: HashMap<String, u64> = HashMap::new();
        let cost = ore_for(("FUEL".to_owned(), i), map, &counts, &mut storage);
        if cost > 1000000000000 {
            best = i - 1;
            break;
        }
    }
    for i in (best - 1000..=best).rev() {
        let mut storage: HashMap<String, u64> = HashMap::new();
        let cost = ore_for(("FUEL".to_owned(), i), map, &counts, &mut storage);
        if cost <= 1000000000000 {
            return i;
        }
    }

    unreachable!()
}

fn div_ceil(a: u64, b: u64) -> u64 {
    let d = a / b;
    if a % b > 0 {
        d + 1
    } else {
        d
    }
}
