use std::{collections::HashMap, rc::Rc};

use cubing::{
    kpuzzle::{
        KPuzzle, KPuzzleDefinition, KPuzzleOrbitDefinition, KStateOrbitData,
        KTransformationOrbitData,
    },
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
    assert_eq!(
        kpuzzle.transformation_from_str("z")?,
        kpuzzle.transformation_from_str("[x: y]")?,
    );
    assert_eq!(
        kpuzzle.transformation_from_str("L")?.transformation_data,
        kpuzzle.transformation_from_str("x' R")?.transformation_data,
    );
    Ok(())
}

#[test]
fn avoids_recursion() -> Result<(), String> {
    let def = KPuzzleDefinition {
        name: "uh-oh".to_owned(),
        orbits: HashMap::from([(
            "SOLVE_ORBIT".into(),
            KPuzzleOrbitDefinition {
                num_pieces: 2,
                num_orientations: 1,
            },
        )]),
        start_state_data: Rc::new(HashMap::from([(
            "SOLVE_ORBIT".into(),
            KStateOrbitData {
                pieces: vec![1, 0],
                orientation: vec![0; 2],
            },
        )])),
        moves: HashMap::from([(
            "A".try_into()?,
            Rc::new(HashMap::from([(
                "SOLVE_ORBIT".into(),
                KTransformationOrbitData {
                    permutation: vec![1, 0],
                    orientation: vec![0; 2],
                },
            )])),
        )]),
        experimental_derived_moves: Some(HashMap::from([
            ("B".try_into()?, "C".try_into()?),
            ("C".try_into()?, "A B".try_into()?),
        ])),
    };
    assert!(KPuzzle::try_new(def)
        .expect_err("Expected recursive KPuzzle to fail instantiation.")
        .starts_with("Recursive derived move definition for: "));
    Ok(())
}
