pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut answer_char_count = [0u8; 26];
    for c in answer.bytes() {
        answer_char_count[(c - b'a') as usize] += 1;
    }

    let mut history = Vec::new();
    // Wordle only allows six guesses.
    // We allow more to avoid chopping off the score distrubution for stats purposes.
    for i in 1..=32 {
        let guess = guesser.guess(&history[..]);
        if guess == answer {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, &guess, &answer_char_count);
        history.push(Guess {
            word: guess,
            mask: correctness,
        })
    }
    None
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
    fn compute(answer: &str, guess: &str, answer_char_count: &[u8; 26]) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Self::Wrong; 5];
        let mut freq = answer_char_count.clone();
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
