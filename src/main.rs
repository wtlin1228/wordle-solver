const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = wordle_solver::algorithms::Naive::new();
    for answer in GAMES.split_whitespace() {
        wordle_solver::play(answer, guesser);
    }
}
