const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let wordle = wordle_solver::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = wordle_solver::algorithms::Naive::new();
        wordle.play(answer, guesser);
    }
}
