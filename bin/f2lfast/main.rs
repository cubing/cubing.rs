use cubing::puzzles::{get_kpuzzle, PuzzleID};

pub fn main() {
    println!("{:?}", get_kpuzzle(PuzzleID::Cube3x3x3))
}
