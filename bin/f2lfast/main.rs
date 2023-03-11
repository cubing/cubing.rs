use cubing::puzzles::{get_puzzle, PuzzleID};

pub fn main() {
    println!("{:?}", get_puzzle(PuzzleID::Cube3x3x3))
}
