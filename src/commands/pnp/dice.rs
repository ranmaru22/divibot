use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt;

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

    #[test]
    fn ordering_roll_options_works() {
        let nothing = RollOptions::Nothing;
        let explode = RollOptions::ExplodeOn(1);
        let successes = RollOptions::CountSuccesses(1);
        assert!(nothing < explode);
        assert!(successes > explode);
        assert!(successes == successes);
        let mut v = vec![explode, successes, nothing];
        v.sort();
        assert_eq!(v[0], RollOptions::Nothing);
        assert_eq!(v[1], RollOptions::ExplodeOn(1));
        assert_eq!(v[2], RollOptions::CountSuccesses(1));
    }
}

/// A printable collection of dice rolls.
struct DiceResults {
    rolls: Vec<u32>,
    succ_threshold: Option<u32>,
    successes: Option<u32>,
}

impl DiceResults {
    pub fn new(size: usize) -> Self {
        DiceResults {
            rolls: Vec::with_capacity(size),
            succ_threshold: None,
            successes: None,
        }
    }

    pub fn push(&mut self, item: u32) {
        self.rolls.push(item);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.rolls.iter()
    }

    pub fn len(&self) -> usize {
        self.rolls.len()
    }

    pub fn set_succ_threshold(&mut self, threshold: u32) {
        self.succ_threshold = Some(threshold);
        self.set_successes();
    }

    fn set_successes(&mut self) {
        let mut successes: u32 = 0;
        for roll in &self.rolls {
            if *roll >= self.succ_threshold.unwrap() {
                successes += 1;
            }
            self.successes = Some(successes);
        }
    }
}

impl fmt::Display for DiceResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_string = String::new();

        for roll in &self.rolls {
            match &self.succ_threshold {
                Some(val) if roll >= val => {
                    result_string.push_str(&format!("**{}**", &roll.to_string()))
                }
                _ => result_string.push_str(&roll.to_string()),
            }
            result_string.push_str(", ");
        }

        result_string = result_string
            .chars()
            .into_iter()
            .take(result_string.len() - 2)
            .collect();

        match &self.successes {
            None => write!(f, "[{}]", result_string),
            Some(val) if *val == 1 => write!(f, "[{}] - 1 success!", result_string),
            Some(val) => write!(f, "[{}] - {} successes!", result_string, val),
        }
    }
}

/// A variant type of different options for rolling dice.
#[derive(Debug, PartialEq, Eq)]
pub enum RollOptions {
    ExplodeOn(u32),
    CountSuccesses(u32),
    Nothing,
}

impl PartialOrd for RollOptions {
    #[inline]
    fn partial_cmp(&self, other: &RollOptions) -> Option<Ordering> {
        match self {
            RollOptions::Nothing => match other {
                RollOptions::Nothing => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            RollOptions::CountSuccesses(_) => match other {
                RollOptions::CountSuccesses(_) => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            RollOptions::ExplodeOn(_) => match other {
                RollOptions::ExplodeOn(_) => Some(Ordering::Equal),
                RollOptions::CountSuccesses(_) => Some(Ordering::Less),
                RollOptions::Nothing => Some(Ordering::Greater),
            },
        }
    }
}

impl Ord for RollOptions {
    #[inline]
    fn cmp(&self, other: &RollOptions) -> Ordering {
        match self {
            RollOptions::Nothing => match other {
                RollOptions::Nothing => Ordering::Equal,
                _ => Ordering::Less,
            },
            RollOptions::CountSuccesses(_) => match other {
                RollOptions::CountSuccesses(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            RollOptions::ExplodeOn(_) => match other {
                RollOptions::ExplodeOn(_) => Ordering::Equal,
                RollOptions::CountSuccesses(_) => Ordering::Less,
                RollOptions::Nothing => Ordering::Greater,
            },
        }
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
