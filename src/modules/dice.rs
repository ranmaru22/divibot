use rand::{thread_rng, Rng};
use std::fmt;

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

pub fn is_delimiter(c: char) -> bool {
    let delimiters = ['d', 'w'];
    delimiters.contains(&c)
}

pub fn roll(args: Vec<u32>) -> Option<String> {
    let (sides, num_dice) = match args.len() {
        1 => (args[0], 1),
        2 => (args[1], args[0]),
        _ => return None,
    };

    if sides == 0 {
        return None;
    }

    let mut rng = thread_rng();
    let mut rolls = DiceResults::new(num_dice as usize);

    for _ in 0..num_dice {
        let n: u32 = rng.gen_range(1, sides + 1);
        rolls.push(n);
    }

    Some(format!("{}", rolls))
}
