use cubing::{parse_alg, puzzles::cube3x3x3_kpuzzle};

#[test]
fn it_works() -> Result<(), String> {
    let kpuzzle = cube3x3x3_kpuzzle();
    assert_eq!(
        kpuzzle.transformation_from_alg(&parse_alg!("R U R' F' U2")?)?,
        kpuzzle.transformation_from_str("(L' U' L F U2')'")?,
    );

    Ok(())
}
