use cubing::{parse_alg, puzzles::get_kpuzzle};

#[test]
fn it_works() -> Result<(), String> {
    let kpuzzle = get_kpuzzle(cubing::puzzles::PuzzleID::Cube3x3x3);
    assert_eq!(
        kpuzzle.transformation_from_alg(&parse_alg!("R U R' F' U2")?)?,
        kpuzzle.transformation_from_alg(&parse_alg!("(L' U' L F U2')'")?)?,
    );

    Ok(())
}
