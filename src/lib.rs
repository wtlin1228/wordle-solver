use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line is word + space + frequency")
                    .1
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let answer_letter_freq = get_letter_freq(answer);
        let mut history = Vec::new();
        // Wordle only allows six guesses.
        // We allow more to avoid chopping off the score distrubution for stats purposes.
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&guess[..]));
            let correctness = Correctness::compute(answer, &guess, &answer_letter_freq);
            history.push(Guess {
                word: guess,
                mask: correctness,
            })
        }
        None
    }
}

fn get_letter_freq(s: &str) -> [u8; 26] {
    let mut result = [0u8; 26];
    for c in s.bytes() {
        result[(c - b'a') as usize] += 1;
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str, answer_letter_freq: &[u8; 26]) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Self::Wrong; 5];
        let mut freq = answer_letter_freq.clone();
        for (i, (a, g)) in answer.bytes().zip(guess.bytes()).enumerate() {
            if a == g {
                c[i] = Self::Correct;
                freq[(a - b'a') as usize] -= 1;
            }
        }
        for (i, g) in guess.bytes().enumerate() {
            if c[i] == Self::Correct {
                // Already marked as green.
                continue;
            }
            let index = (g - b'a') as usize;
            if freq[index] > 0 {
                freq[index] -= 1;
                c[i] = Self::Misplaced;
            }
        }
        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[cfg(test)]
mod tests {
    mod compute {
        use crate::{Correctness, get_letter_freq};

        macro_rules! mask {
            (C) => {{ Correctness::Correct }};
            (M) => {{ Correctness::Misplaced }};
            (W) => {{ Correctness::Wrong }};
            ($($c:tt)+) => {{[ $(mask!($c)),+ ]}}
        }

        #[test]
        fn all_green() {
            assert_eq!(
                Correctness::compute("abcde", "abcde", &get_letter_freq("abcde")),
                mask!(C C C C C)
            );
        }

        #[test]
        fn all_yellow() {
            assert_eq!(
                Correctness::compute("abcde", "eabcd", &get_letter_freq("abcde")),
                mask!(M M M M M)
            );
        }

        #[test]
        fn all_grey() {
            assert_eq!(
                Correctness::compute("abcde", "fghij", &get_letter_freq("abcde")),
                mask!(W W W W W)
            );
        }

        #[test]
        fn repeat_green() {
            assert_eq!(
                Correctness::compute("aabbb", "aaccc", &get_letter_freq("aabbb")),
                mask!(C C W W W)
            );
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(
                Correctness::compute("aabbb", "cccaa", &get_letter_freq("aabbb")),
                mask!(W W W M M)
            );
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(
                Correctness::compute("aabbb", "accca", &get_letter_freq("aabbb")),
                mask!(C W W W M)
            );
        }

        #[test]
        fn already_taken() {
            assert_eq!(
                Correctness::compute("aabbb", "ccaaa", &get_letter_freq("aabbb")),
                mask!(W W M M W)
            );
        }

        #[test]
        fn should_not_complete_with_correct_one() {
            assert_eq!(
                Correctness::compute("babbb", "aaccc", &get_letter_freq("babbb")),
                mask!(W C W W W)
            );
        }
    }
}
