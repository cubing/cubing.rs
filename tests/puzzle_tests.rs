use cubing::{
    parse_alg,
    puzzles::{cube2x2x2_kpuzzle, cube3x3x3_kpuzzle},
};

#[test]
fn it_works() -> Result<(), String> {
    let kpuzzle = cube3x3x3_kpuzzle();
    assert_eq!(
        kpuzzle.transformation_from_alg(&parse_alg!("R U R' F' U2")?)?,
        kpuzzle.transformation_from_str("(L' U' L F U2')'")?,
    );
    assert_eq!(
        kpuzzle.transformation_from_alg(&parse_alg!("R U R' F' U2")?)?,
        (kpuzzle, "(L' U' L F U2')'").try_into()?,
    );

    Ok(())
}

#[test]
fn test_2x2x2() -> Result<(), String> {
    let kpuzzle = cube2x2x2_kpuzzle();
    assert_eq!(
        kpuzzle.transformation_from_alg(&parse_alg!("z")?)?,
        kpuzzle.transformation_from_str("z")?,
    );
    Ok(())
}
