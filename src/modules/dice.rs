use rand::{thread_rng, Rng};
use std::fmt;

/// A printable collection of dice rolls.
struct DiceResults {
    rolls: Vec<u32>,
}

impl DiceResults {
    pub fn new(size: usize) -> Self {
        DiceResults {
            rolls: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, item: u32) {
        self.rolls.push(item);
    }
}

impl fmt::Display for DiceResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_string = String::new();

        for roll in &self.rolls {
            result_string.push_str(&roll.to_string());
            result_string.push_str(", ");
        }

        result_string = result_string
            .chars()
            .into_iter()
            .take(result_string.len() - 2)
            .collect();

        write!(f, "[{}]", result_string)
    }
}

/// A variant type of different options for rolling dice.
pub enum RollOptions {
    ExplodeOn(u32),
    Nothing,
}

/// Returns whether a character is recognizes as an argument delimiter.
pub fn is_delimiter(c: char) -> bool {
    let delimiters = ['d', 'w'];
    delimiters.contains(&c)
}

/// Rolls dice and respects options passed in as arguments.
pub fn roll_dice(args: Vec<u32>, opts: RollOptions) -> Option<String> {
    let (sides, num_dice) = match args.len() {
        1 => (args[0], 1),
        2 => (args[1], args[0]),
        _ => return None,
    };

    if sides == 0 {
        return None;
    }

    let results = match opts {
        RollOptions::ExplodeOn(val) => exploding_roll(sides, num_dice, val),
        RollOptions::Nothing => roll(sides, num_dice),
    };

    Some(format!("{}", results))
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
