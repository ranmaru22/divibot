use rand::{thread_rng, Rng};

use super::types::{DiceResults, RollOptions};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_roll_works() {
        let (sides, num_dice) = (6, 100);
        let results = roll(sides, num_dice, None);
        assert!(results.iter().all(|&x| x <= sides));
        assert!(results.len() == num_dice as usize);
    }

    #[test]
    fn reroll_works() {
        let (sides, num_dice, reroll) = (6, 100, 3);
        let results = roll(sides, num_dice, Some(reroll));
        assert!(results.iter().all(|&x| x != reroll));
        assert!(results.len() == num_dice as usize);
    }

    #[test]
    fn exploding_roll_works() {
        let (sides, num_dice, explode_on) = (6, 100, 6);
        let results = exploding_roll(sides, num_dice, explode_on, None);
        assert!(results.iter().all(|&x| x % 6 != 0));
        assert!(results.len() == num_dice as usize);
    }

    #[test]
    fn arg_parsing_works() {
        let (sides, num_dice) = (6, 100);
        let dice_args = vec![num_dice, sides];
        let opts = vec![RollOptions::CountSuccesses(1)];
        let expected = format!("{} successes!", num_dice);
        match roll_dice(dice_args, opts) {
            Some(results) => {
                println!("{}", results);
                assert!(results.contains(&expected));
            }
            None => (),
        };
    }
}

/// Returns whether a character is recognizes as an argument delimiter.
pub fn is_delimiter(c: char) -> bool {
    let delimiters = ['d', 'w'];
    delimiters.contains(&c)
}

/// Rolls dice and respects options passed in as arguments.
pub fn roll_dice(args: Vec<u32>, mut opts: Vec<RollOptions>) -> Option<String> {
    let (sides, num_dice) = match args.len() {
        1 => (args[0], 1),
        2 => (args[1], if args[0] == 0 { 1 } else { args[0] }),
        _ => return None,
    };
    if sides == 0 {
        return None;
    }

    let mut reroll: Option<u32> = None;
    let mut results = roll(sides, num_dice, None);
    if !opts.is_empty() {
        opts.sort();
        for opt in opts.iter() {
            match opt {
                RollOptions::RerollOn(val) => reroll = Some(*val),
                RollOptions::Nothing => match reroll {
                    None => continue,
                    Some(_) => results = roll(sides, num_dice, reroll),
                },
                RollOptions::ExplodeOn(val) => {
                    results = exploding_roll(sides, num_dice, *val, reroll)
                }
                RollOptions::CountSuccesses(val) => results.set_succ_threshold(*val),
                RollOptions::KeepBest(val) => results.keep_best(*val),
                RollOptions::DropLowest(val) => results.drop_lowest(*val),
            }
        }
    }
    let result_str = format!("{}", results);
    Some(result_str)
}

/// Rolls a number of dice and returns the results.
fn roll(sides: u32, num_dice: u32, reroll: Option<u32>) -> DiceResults {
    let mut rng = thread_rng();
    let mut rolls = DiceResults::new(num_dice as usize);
    for _ in 0..num_dice {
        let n: u32 = match reroll {
            None => rng.gen_range(1, sides + 1),
            Some(val) => {
                let mut candidate = val;
                while candidate == val {
                    candidate = rng.gen_range(1, sides + 1);
                }
                candidate
            }
        };
        rolls.push(n);
    }
    rolls
}

/// Rolls a number of dice, rolls again when the roll matches the explode_on
/// argument, and restuns the results.,
fn exploding_roll(sides: u32, num_dice: u32, explode_on: u32, reroll: Option<u32>) -> DiceResults {
    let mut rng = thread_rng();
    let mut rolls = DiceResults::new(num_dice as usize);
    for _ in 0..num_dice {
        let mut result: u32 = 0;
        let mut n = match reroll {
            None => rng.gen_range(1, sides + 1),
            Some(val) => {
                let mut candidate = val;
                while candidate == val {
                    candidate = rng.gen_range(1, sides + 1);
                }
                candidate
            }
        };
        result += n;
        while n == explode_on {
            n = rng.gen_range(1, sides + 1);
            result += n;
        }
        rolls.push(result);
    }
    rolls
}
