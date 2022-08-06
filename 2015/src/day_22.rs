// m/d - mana / damage dealt
// m/h - mana / hp healed
// Magic missile => 13.25 m/d
// Drain => 36.5 m/d + 36.5 m/h (never use, it's bad)
// Shield => 6t 2.7 m/h (use when not enough health left to kill the boss)
//        => 1t 16.14 m/h
// Poison => 6t 9.61 m/d
//        => 5t 11.53 m/d
//        => 4t 14.42 m/d (MM is better at 4 turns and less)
// Recharge => use sparingly, when there isn't enough mana left to kill the boss
//
// Strategy - Poison every 6 turns, then MM. This kills the boss in 18
// turns (Poison turns 0, 6, 12) deals 54 dmg, a single MM last turn kills the boss.
// Mana cost - 3*173+53=572
//
// in 18 turns, boss deals 9*17=153 dmg, i have 50 hp, so i have to shield for at least 103 dmg.
// which i can do with shield at turns 1, 7, 13 to block for 17 turns = 119 dmg shielded
// (blocking only on turns 1, 7 shields from 12*7=84 dmg, so i would have to drain 10 times, which is expensive)
// Mana cost - 3*113=339
//
// Total mana to kill and survive = 911
//
// Mana needed to recharge = 411
// Recharging once gets me 505-229=276 Mana
// recharging on turns 2, 8 gives me a total of 1010-458=552 Mana, which is enough
//
// Total mana cost = 911 + 458 = 1369 (too high)
// ==============================================================================================================
// we have to deal at least 58 dmg to kill the boss, so 3 Poisons and one MM is the cheapest, but
// killing it before with more in between MM's might bring the survive cost down. To use only 2
// rounds of shield I would shield 7*12=84 dmg, which allows me to survive for {(50+84)/9=14.9} 15 turns
// 2 rounds of shield cost 226 Mana
//
// to kill the boss in 15 turns, i need 2 rounds of Poison (226 Mana, 36 damage) and 6 MM's (318 mana, 24 damage)
// that costs a total of 544 mana to kill the boss
//
// total mana used is now 770
//
// i now need 1 recharge to get the 270 mana missing
//
// total cost = 770+229=999 (too low)
// ==============================================================================================================
// that would be too low only if i miscalculated something
// turn 1, 6 poison deals 36 dmg in 12 enemy turns, costs               346 mana
// turn 2, 7 shield shields me from 84 damage in 12 enemy turns, costs  226 mana
// turn 4,5,9,10,11,12 MM deals 24 damage, the boss dies on turn 12, c. 318 mana
// turn 3, 8 recharge gets me enough mana, costs                        458 mana
// total cost is now 1348 (too high)
// ==============================================================================================================
//
// THAT'S NOT HOW THE GAME WORKS NOOOO
// what i counted as one turn is actually 2 turns, because the player has one turn, and the boss
// has one turn
//
// Player turn (m - 500, ph - 50, bh - 58)
// Cast Poison (-173 mana)
// --
// Boss turn (m - 327, ph - 50, bh - 58)
// Poison deals 3 dmg, timer is now 5
// boss hits for 9
// --
// Player turn (m - 327, ph - 41, bh - 55)
// Poison deals 3, timer is now 4
// Cast Shield (-113 mana)
// --
// Boss turn (m - 214, ph - 41, bh - 52)
// Poison deals 3, timer is now 3
// Shield timer is now 5
// Boss attacks for 2
// --
// Player turn (m - 214, ph - 39, bh - 49)
// Poison deals 3, t2
// Shield t4
// (Can't cast recharge)
// 40 dmg costs 530 more mana, game is dead

// attempt a brute force, can't do it by hand anymore
#[derive(Copy, Clone, PartialEq, Eq)]
enum Spell {
    MM,
    Drain,
    Shield(i32),
    Poison(i32),
    Recharge(i32),
}

#[derive(Clone)]
struct Game {
    turn: i32,
    player_turn: bool,
    health: i32,
    mana: i32,
    b_health: i32,
    effects_active: Vec<Spell>,
    total_mana_spent: i32,
}

fn cost(spell: Spell) -> i32 {
    match spell {
        Spell::MM => 53,
        Spell::Drain => 73,
        Spell::Shield(_) => 113,
        Spell::Poison(_) => 173,
        Spell::Recharge(_) => 229,
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i32 {
    let mut boss = input
        .lines()
        .map(|x| x.split(' ').find_map(|n| n.parse::<i32>().ok()).unwrap());
    let boss_health = boss.next().unwrap();
    let boss_attack = boss.next().unwrap();

    let mut games = vec![Game {
        turn: 1,
        player_turn: true,
        health: 50,
        mana: 500,
        b_health: boss_health,
        effects_active: vec![],
        total_mana_spent: 0,
    }];
    let mut mana_to_win = vec![];
    let possible_moves = vec![
        Spell::MM,
        Spell::Drain,
        Spell::Shield(6),
        Spell::Poison(6),
        Spell::Recharge(5),
    ];
    while let Some(mut game) = games.pop() {
        let mut damage_to_deal = boss_attack;
        let mut new_effects = vec![];
        let mut new_possible_moves = possible_moves.clone();
        game.turn += 1;
        if game.turn > 40 {
            // remove the game, it went on for too long
            continue;
        }

        if game.health <= 0 || game.mana < 53 || game.total_mana_spent > 1348 {
            // remove the game, it's a loss
            continue;
        } else if game.b_health <= 0 {
            // we won, mr. stark
            mana_to_win.push(game.total_mana_spent);
            continue;
        }
        new_possible_moves.retain(|x| cost(*x) <= game.mana);
        for effect in &game.effects_active {
            match effect {
                Spell::Shield(x) => {
                    damage_to_deal -= 7;
                    if x > &1 {
                        new_possible_moves.retain(|x| x != &Spell::Shield(6));
                        new_effects.push(Spell::Shield(x - 1));
                    }
                }
                Spell::Poison(x) => {
                    game.b_health -= 3;
                    if x > &1 {
                        new_possible_moves.retain(|x| x != &Spell::Poison(6));
                        new_effects.push(Spell::Poison(x - 1));
                    }
                }
                Spell::Recharge(x) => {
                    game.mana += 101;
                    if x > &1 {
                        new_possible_moves.retain(|x| x != &Spell::Recharge(5));
                        new_effects.push(Spell::Recharge(x - 1));
                    }
                }
                _ => unreachable!(),
            }
        }
        if game.player_turn {
            // player turn
            for spell in &new_possible_moves {
                let mut new_game = game.clone();
                new_game.effects_active = new_effects.clone();
                new_game.mana -= cost(*spell);
                new_game.total_mana_spent += cost(*spell);
                match spell {
                    Spell::MM => new_game.b_health -= 4,
                    Spell::Drain => {
                        new_game.b_health -= 2;
                        new_game.health += 2;
                    }
                    x => new_game.effects_active.push(*x),
                }
                new_game.player_turn = !new_game.player_turn;
                games.push(new_game);
            }
        } else {
            // boss turn
            game.health -= damage_to_deal;
            games.push(Game {
                effects_active: new_effects,
                player_turn: !game.player_turn,
                ..game
            });
        }
    }
    mana_to_win.into_iter().min_by(|x, y| x.cmp(y)).unwrap()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i32 {
    let mut boss = input
        .lines()
        .map(|x| x.split(' ').find_map(|n| n.parse::<i32>().ok()).unwrap());
    let boss_health = boss.next().unwrap();
    let boss_attack = boss.next().unwrap();

    let mut games = vec![Game {
        turn: 1,
        player_turn: true,
        health: 50,
        mana: 500,
        b_health: boss_health,
        effects_active: vec![],
        total_mana_spent: 0,
    }];
    let mut mana_to_win = vec![];
    let possible_moves = vec![
        Spell::MM,
        Spell::Drain,
        Spell::Shield(6),
        Spell::Poison(6),
        Spell::Recharge(5),
    ];
    while let Some(mut game) = games.pop() {
        //println!("{:?}", game);
        let mut damage_to_deal = boss_attack;
        let mut new_effects = vec![];
        let mut new_possible_moves = possible_moves.clone();
        game.turn += 1;
        if game.turn > 50 {
            // remove the game, it went on for too long
            continue;
        }

        if game.health <= 0 || game.mana < 53 || game.total_mana_spent > 1348 {
            // remove the game, it's a loss
            continue;
        } else if game.b_health <= 0 {
            // we won, mr. stark
            mana_to_win.push(game.total_mana_spent);
            continue;
        }
        new_possible_moves.retain(|x| cost(*x) <= game.mana);
        for effect in &game.effects_active {
            match effect {
                Spell::Shield(x) => {
                    damage_to_deal -= 7;
                    if x > &1 {
                        new_possible_moves.retain(|x| x != &Spell::Shield(6));
                        new_effects.push(Spell::Shield(x - 1));
                    }
                }
                Spell::Poison(x) => {
                    game.b_health -= 3;
                    if x > &1 {
                        new_possible_moves.retain(|x| x != &Spell::Poison(6));
                        new_effects.push(Spell::Poison(x - 1));
                    }
                }
                Spell::Recharge(x) => {
                    game.mana += 101;
                    if x > &1 {
                        new_possible_moves.retain(|x| x != &Spell::Recharge(5));
                        new_effects.push(Spell::Recharge(x - 1));
                    }
                }
                _ => unreachable!(),
            }
        }
        if game.player_turn {
            // player turn
            // the only change
            game.health -= 1;
            for spell in &new_possible_moves {
                //println!("Used {:?}", spell);
                let mut new_game = game.clone();
                new_game.effects_active = new_effects.clone();
                new_game.mana -= cost(*spell);
                new_game.total_mana_spent += cost(*spell);
                match spell {
                    Spell::MM => new_game.b_health -= 4,
                    Spell::Drain => {
                        new_game.b_health -= 2;
                        new_game.health += 2;
                    }
                    x => new_game.effects_active.push(*x),
                }
                new_game.player_turn = !new_game.player_turn;
                games.push(new_game);
            }
        } else {
            // boss turn
            game.health -= damage_to_deal;
            games.push(Game {
                effects_active: new_effects,
                player_turn: !game.player_turn,
                ..game
            });
        }
    }
    mana_to_win.into_iter().min_by(|x, y| x.cmp(y)).unwrap()
}
