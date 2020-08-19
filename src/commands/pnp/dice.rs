use rand::{thread_rng, Rng};

use super::types::{DiceResults, RollOptions};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_roll_works() {
        let (sides, num_dice) = (6, 100);
        let results = roll(sides, num_dice);
        assert!(results.iter().all(|&x| x <= sides));
        assert!(results.len() == num_dice as usize);
    }
    #[test]
    fn exploding_roll_works() {
        let (sides, num_dice, explode_on) = (6, 100, 6);
        let results = exploding_roll(sides, num_dice, explode_on);
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

    let mut results = roll(sides, num_dice);
    if !opts.is_empty() {
        opts.sort();
        for opt in opts.iter() {
            println!("Do: {:?}", opt);
            match opt {
                RollOptions::Nothing => break,
                RollOptions::ExplodeOn(val) => results = exploding_roll(sides, num_dice, *val),
                RollOptions::CountSuccesses(val) => results.set_succ_threshold(*val),
            }
        }
    }

    let result_str = format!("{}", results);
    Some(result_str)
}

/// Rolls a number of dice and returns the results.
fn roll(sides: u32, num_dice: u32) -> DiceResults {
    let mut rng = thread_rng();
    let mut rolls = DiceResults::new(num_dice as usize);
    for _ in 0..num_dice {
        let n: u32 = rng.gen_range(1, sides + 1);
        rolls.push(n);
    }
    rolls
}

/// Rolls a number of dice, rolls again when the roll matches the explode_on
/// argument, and restuns the results.,
fn exploding_roll(sides: u32, num_dice: u32, explode_on: u32) -> DiceResults {
    let mut rng = thread_rng();
    let mut rolls = DiceResults::new(num_dice as usize);
    for _ in 0..num_dice {
        let mut result: u32 = 0;
        let mut n;
        loop {
            n = rng.gen_range(1, sides + 1);
            result += n;
            if n != explode_on {
                break;
            };
        }
        rolls.push(result);
    }
    rolls
}
