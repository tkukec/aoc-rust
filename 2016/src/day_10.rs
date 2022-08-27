#[derive(Debug, Clone, Copy)]
enum BotType {
    Output(i32),
    Bot(i32),
}
#[derive(Debug, Clone)]
struct Bot {
    id: i32,
    low: BotType,
    high: BotType,
    values: (i32, i32),
}

fn generate(input: &str) -> Vec<Bot> {
    let mut botlist = vec![];
    let mut input: Vec<&str> = input.lines().collect();
    input.sort(); // initialize all bots before giving them values
    for i in input {
        let mut iter = i.split(' ');
        match iter.next().unwrap() {
            "bot" => {
                let id = iter.next().unwrap().parse().unwrap();
                let low = match iter.nth(3).unwrap() {
                    "bot" => BotType::Bot(iter.next().unwrap().parse().unwrap()),
                    "output" => BotType::Output(iter.next().unwrap().parse().unwrap()),
                    _ => unreachable!(),
                };
                let high = match iter.nth(3).unwrap() {
                    "bot" => BotType::Bot(iter.next().unwrap().parse().unwrap()),
                    "output" => BotType::Output(iter.next().unwrap().parse().unwrap()),
                    _ => unreachable!(),
                };

                botlist.push(Bot {
                    id,
                    low,
                    high,
                    values: (0, 0),
                });
            }
            "value" => {
                botlist.sort_by(|x, y| x.id.cmp(&y.id));
                let val = iter.next().unwrap().parse().unwrap();
                let bot_id = iter.last().unwrap().parse().unwrap();
                assert_eq!(botlist[(bot_id as usize)].id, bot_id);
                botlist[(bot_id as usize)].values = if botlist[(bot_id as usize)].values.0 == 0 {
                    (val, 0)
                } else {
                    assert_eq!(botlist[(bot_id as usize)].values.1, 0);
                    (botlist[(bot_id as usize)].values.0, val)
                }
            }
            _ => unreachable!(),
        }
    }
    botlist
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut botlist = generate(input);
    while let Some(bot) = botlist
        .clone()
        .iter_mut()
        .find(|x| x.values.0 != 0 && x.values.1 != 0)
    {
        let low = bot.values.0.min(bot.values.1);
        let high = bot.values.0.max(bot.values.1);
        if low == 17 && high == 61 {
            return bot.id;
        }
        match bot.low {
            BotType::Output(_x) => {}
            BotType::Bot(x) => {
                botlist[(x as usize)].values = if botlist[(x as usize)].values.0 == 0 {
                    (low, 0)
                } else {
                    assert_eq!(botlist[(x as usize)].values.1, 0);
                    (botlist[(x as usize)].values.0, low)
                }
            }
        }
        match bot.high {
            BotType::Output(_x) => {}
            BotType::Bot(x) => {
                botlist[(x as usize)].values = if botlist[(x as usize)].values.0 == 0 {
                    (high, 0)
                } else {
                    assert_eq!(botlist[(x as usize)].values.1, 0);
                    (botlist[(x as usize)].values.0, high)
                }
            }
        }

        // remove the current values from the bot, because they moved to other bots/outputs
        botlist[(bot.id as usize)].values = (0, 0);
    }
    todo!()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let mut botlist = generate(input);
    let mut outputs = [0; 3];
    while let Some(bot) = botlist
        .clone()
        .iter_mut()
        .find(|x| x.values.0 != 0 && x.values.1 != 0)
    {
        let low = bot.values.0.min(bot.values.1);
        let high = bot.values.0.max(bot.values.1);
        if outputs[0] != 0 && outputs[1] != 0 && outputs[2] != 0 {
            return outputs[0] * outputs[1] * outputs[2];
        }
        match bot.low {
            BotType::Output(x) => {
                if (0..3).contains(&x) {
                    outputs[x as usize] = low;
                }
            }
            BotType::Bot(x) => {
                botlist[(x as usize)].values = if botlist[(x as usize)].values.0 == 0 {
                    (low, 0)
                } else {
                    assert_eq!(botlist[(x as usize)].values.1, 0);
                    (botlist[(x as usize)].values.0, low)
                }
            }
        }
        match bot.high {
            BotType::Output(x) => {
                if (0..3).contains(&x) {
                    outputs[x as usize] = high;
                }
            }
            BotType::Bot(x) => {
                botlist[(x as usize)].values = if botlist[(x as usize)].values.0 == 0 {
                    (high, 0)
                } else {
                    assert_eq!(botlist[(x as usize)].values.1, 0);
                    (botlist[(x as usize)].values.0, high)
                }
            }
        }

        botlist[(bot.id as usize)].values = (0, 0);
    }
    outputs[0] * outputs[1] * outputs[2]
}
