use std::cmp::Ordering;
use std::fmt;

#[cfg(test)]
mod test {
    use super::*;

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
pub struct DiceResults {
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