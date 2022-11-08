use regex::Regex;
use std::collections::HashSet;

#[aoc_generator(day24)]
fn generate(input: &str) -> (Vec<Group>, Vec<Group>) {
    let (immune_system, infection) = input.split_once("\n\n").expect("Bad input");
    let immune_system_data = immune_system.lines().skip(1);
    let infection_data = infection.lines().skip(1);

    // If it works, it works
    let group_regex = Regex::new(r"(\d+) units each with (\d+) hit points(?: \()?(?:(\w+?) to (\w+?)(?:, (\w+?))?(?:, (\w+?))?)?(?:; )?(?:(\w+?) to (\w+?)(?:, (\w+?))?(?:, (\w+?))?)?\)? with an attack that does (\d+) (\w+?) damage at initiative (\d+)").unwrap();

    let mut immune_system = vec![];
    let mut infection = vec![];
    for (side, data) in [
        (&mut immune_system, immune_system_data),
        (&mut infection, infection_data),
    ] {
        for l in data {
            let cap = group_regex.captures(l).expect("Bad line");
            let units = cap[1].parse().unwrap();
            let unit_health = cap[2].parse().unwrap();
            let mut weak_to_vec = vec![];
            let mut immune_to_vec = vec![];
            let first = match &cap.get(3).map(|x| x.as_str()) {
                Some("weak") => Some(&mut weak_to_vec),
                Some("immune") => Some(&mut immune_to_vec),
                Some(_) => unreachable!(),
                None => None,
            };
            if let Some(first) = first {
                if let Some(dmg) = cap.get(4).map(|x| x.as_str()) {
                    first.push(Damage::from(dmg));
                }
                if let Some(dmg) = cap.get(5).map(|x| x.as_str()) {
                    first.push(Damage::from(dmg));
                }
                if let Some(dmg) = cap.get(6).map(|x| x.as_str()) {
                    first.push(Damage::from(dmg));
                }
            }

            let second = match &cap.get(7).map(|x| x.as_str()) {
                Some("weak") => Some(&mut weak_to_vec),
                Some("immune") => Some(&mut immune_to_vec),
                Some(_) => unreachable!(),
                None => None,
            };

            if let Some(second) = second {
                if let Some(dmg) = cap.get(8).map(|x| x.as_str()) {
                    second.push(Damage::from(dmg));
                }
                if let Some(dmg) = cap.get(9).map(|x| x.as_str()) {
                    second.push(Damage::from(dmg));
                }
                if let Some(dmg) = cap.get(10).map(|x| x.as_str()) {
                    second.push(Damage::from(dmg));
                }
            }
            let immune_to = if immune_to_vec.is_empty() {
                None
            } else {
                Some(immune_to_vec)
            };
            let weak_to = if weak_to_vec.is_empty() {
                None
            } else {
                Some(weak_to_vec)
            };

            let attack = cap[11].parse().unwrap();
            let attack_type = Damage::from(&cap[12]);
            let initiative = cap[13].parse().unwrap();
            side.push(Group {
                units,
                unit_health,
                weak_to,
                immune_to,
                attack,
                attack_type,
                initiative,
            })
        }
    }

    (immune_system, infection)
}

fn solve(input: &(Vec<Group>, Vec<Group>)) -> Option<(Side, u32)> {
    let (mut immune_system, mut infection) = input.clone();

    loop {
        // tie detection
        let old = (immune_system.clone(), infection.clone());

        immune_system.sort_by(|a, b| {
            a.effective_power()
                .cmp(&b.effective_power())
                .then(a.initiative.cmp(&b.initiative))
                .reverse()
        });
        infection.sort_by(|a, b| {
            a.effective_power()
                .cmp(&b.effective_power())
                .then(a.initiative.cmp(&b.initiative))
                .reverse()
        });

        let mut immune_sys_attacks: Vec<(Group, Side, Option<Group>)> = immune_system
            .iter()
            .cloned()
            .map(|x| (x, Side::ImmuneSystem, None))
            .collect();
        let mut infection_attacks: Vec<(Group, Side, Option<Group>)> = infection
            .iter()
            .cloned()
            .map(|x| (x, Side::Infection, None))
            .collect();

        let mut chosen: HashSet<Group> = HashSet::new();
        immune_sys_attacks.iter_mut().for_each(|(x, _, at)| {
            *at = infection
                .iter()
                .filter(|x| !chosen.contains(x))
                .max_by(|a, b| {
                    a.priority(x.attack_type)
                        .cmp(&b.priority(x.attack_type))
                        .then(a.effective_power().cmp(&b.effective_power()))
                        .then(a.initiative.cmp(&b.initiative))
                })
                .cloned();
            if let Some(at_new) = at {
                if !at_new
                    .immune_to
                    .as_ref()
                    .map_or(false, |a| a.contains(&x.attack_type))
                {
                    chosen.insert(at_new.clone());
                } else {
                    *at = None;
                }
            }
        });

        infection_attacks.iter_mut().for_each(|(x, _, at)| {
            *at = immune_system
                .iter()
                .filter(|x| !chosen.contains(x))
                .max_by(|a, b| {
                    a.priority(x.attack_type)
                        .cmp(&b.priority(x.attack_type))
                        .then(a.effective_power().cmp(&b.effective_power()))
                        .then(a.initiative.cmp(&b.initiative))
                })
                .cloned();
            if let Some(at) = at {
                chosen.insert(at.clone());
            }
        });

        let mut attack_order = Vec::new();
        attack_order.extend(&mut immune_sys_attacks);
        attack_order.extend(&mut infection_attacks);
        attack_order.sort_by_key(|x| x.0.initiative);

        while let Some((attacker, attacker_side, target)) = attack_order.pop() {
            // there might not be a target
            if let Some(target) = target {
                // attacker might have died before he got to his turn
                if let Some(attacker) = match attacker_side {
                    Side::ImmuneSystem => &immune_system,
                    Side::Infection => &infection,
                }
                .iter()
                // assume no two groups share the same unit health, because it doesn't happen in my
                // input
                .find(|x| x.unit_health == attacker.unit_health)
                // i do a lot of clones but it's still fast. idk if they get optimized out or sth
                .cloned()
                {
                    let target: (usize, &mut Group) = match attacker_side {
                        Side::ImmuneSystem => &mut infection,
                        Side::Infection => &mut immune_system,
                    }
                    .iter_mut()
                    .enumerate()
                    .find(|(_, x)| *x == target)
                    .expect("Target not found");
                    // deal damage, if it killed then
                    let to_delete = target
                        .1
                        .deal_damage(attacker.effective_power(), attacker.attack_type);

                    // to satisfy the borrowck
                    let index = target.0;
                    if to_delete {
                        match attacker_side {
                            Side::ImmuneSystem => &mut infection,
                            Side::Infection => &mut immune_system,
                        }
                        .swap_remove(index);
                    }
                }
            }
        }

        if (&immune_system, &infection) == (&old.0, &old.1) {
            return None;
        }
        if immune_system.is_empty() || infection.is_empty() {
            return Some((
                if immune_system.is_empty() {
                    Side::Infection
                } else {
                    Side::ImmuneSystem
                },
                immune_system.iter().map(|x| x.units).sum::<u32>()
                    + infection.iter().map(|x| x.units).sum::<u32>(),
            ));
        }
    }
}
#[aoc(day24, part1)]
pub fn part1(input: &(Vec<Group>, Vec<Group>)) -> u32 {
    solve(input).expect("got a tie").1
}
#[aoc(day24, part2)]
pub fn part2(input: &(Vec<Group>, Vec<Group>)) -> u32 {
    let mut input = input.clone();
    while let Some((Side::Infection, _)) = solve(&input) {
        input.0.iter_mut().for_each(|x| x.attack += 1);
    }
    while solve(&input).is_none() {
        input.0.iter_mut().for_each(|x| x.attack += 1);
    }

    solve(&input).unwrap().1
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Damage {
    Slashing,
    Radiation,
    Bludgeoning,
    Fire,
    Cold,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Group {
    units: u32,
    unit_health: u32,
    weak_to: Option<Vec<Damage>>,
    immune_to: Option<Vec<Damage>>,
    attack: u32,
    attack_type: Damage,
    initiative: u8,
}

impl From<&str> for Damage {
    fn from(value: &str) -> Self {
        match value {
            "slashing" => Damage::Slashing,
            "fire" => Damage::Fire,
            "cold" => Damage::Cold,
            "bludgeoning" => Damage::Bludgeoning,
            "radiation" => Damage::Radiation,
            _ => unreachable!(),
        }
    }
}

impl Group {
    fn effective_power(&self) -> u32 {
        assert_ne!(self.units, 0);
        self.units * self.attack
    }

    fn priority(&self, damage_type: Damage) -> u32 {
        if self
            .immune_to
            .as_ref()
            .map_or(false, |s| s.contains(&damage_type))
        {
            0
        } else if self
            .weak_to
            .as_ref()
            .map_or(false, |s| s.contains(&damage_type))
        {
            2
        } else {
            1
        }
    }
    // returns if it killed the group or not
    fn deal_damage(&mut self, damage_amount: u32, damage_type: Damage) -> bool {
        if self
            .immune_to
            .as_ref()
            .map_or(false, |s| s.contains(&damage_type))
        {
            // the attacker did no damage, so it shouldn't be dead i hope
            return false;
        } else if self
            .weak_to
            .as_ref()
            .map_or(false, |s| s.contains(&damage_type))
        {
            self.units = self
                .units
                .saturating_sub(damage_amount * 2 / self.unit_health)
        } else {
            self.units = self.units.saturating_sub(damage_amount / self.unit_health)
        }
        self.units == 0
    }
}
