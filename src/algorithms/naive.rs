use crate::{DICTIONARY, Guess, Guesser};
use std::collections::HashMap;

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is work + space + frequency");
                let count: usize = count.parse().expect("every count is a number");
                (word, count)
            })),
        }
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            // TODO: update self.remaining base on history
            self.remaining.retain(|word, _| last.matches(word));
        }

        let mut best: Option<Candidate> = None;
        for (&word, &count) in &self.remaining {
            // TODO: how do we compute this?
            // - SUM_i p_i * log_2(p_i)
            let goodness = 0.0;
            if let Some(c) = &best {
                // Is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness })
                }
            } else {
                best = Some(Candidate { word, goodness })
            }
        }
        best.unwrap().word.to_string()
    }
}
