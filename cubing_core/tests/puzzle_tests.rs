use std::collections::HashMap;

use cubing_core::{
    alg::Alg,
    kpuzzle::{
        InvalidAlgError, InvalidDefinitionError, KPatternData, KPatternOrbitData, KPuzzle,
        KPuzzleDefinition, KPuzzleOrbitDefinition, KTransformationData, KTransformationOrbitData,
    },
    puzzles::{cube2x2x2_kpuzzle, cube3x3x3_kpuzzle},
};

#[test]
fn it_works() -> Result<(), InvalidAlgError> {
    let kpuzzle = cube3x3x3_kpuzzle();
    assert_eq!(
        &kpuzzle.transformation_from_alg(&("R U R' F' U2").parse::<Alg>()?)?,
        &kpuzzle.transformation_from_alg(&("(L' U' L F U2')'").parse::<Alg>()?)?,
    );
    assert_ne!(
        &kpuzzle.transformation_from_alg(&("(R U R' U)5").parse::<Alg>()?)?,
        &kpuzzle.transformation_from_alg(&("").parse::<Alg>()?)?
    );
    assert_eq!(
        &kpuzzle
            .default_pattern()
            .apply_alg(&("(R U R' U)5").parse::<Alg>()?)?,
        &kpuzzle.default_pattern().apply_alg(&("").parse::<Alg>()?)?
    );

    Ok(())
}

#[test]
fn test_2x2x2() -> Result<(), InvalidAlgError> {
    let kpuzzle = cube2x2x2_kpuzzle();
    assert_eq!(
        kpuzzle.transformation_from_alg(&("z").parse::<Alg>()?)?,
        kpuzzle.transformation_from_alg(&("[x: y]").parse::<Alg>()?)?,
    );
    assert_eq!(
        kpuzzle.transformation_from_alg(&("L").parse::<Alg>()?)?,
        kpuzzle.transformation_from_alg(&("x' R").parse::<Alg>()?)?
    );
    Ok(())
}

#[test]
fn avoids_recursion() -> Result<(), InvalidDefinitionError> {
    let def = KPuzzleDefinition {
        name: "uh-oh".to_owned(),
        orbits: vec![KPuzzleOrbitDefinition {
            orbit_name: "SOLVE_ORBIT".into(),
            num_pieces: 2,
            num_orientations: 1,
        }],
        default_pattern: (KPatternData::from([(
            "SOLVE_ORBIT".into(),
            KPatternOrbitData {
                pieces: vec![1, 0],
                orientation: vec![0; 2],
                orientation_mod: None,
            },
        )])),
        moves: HashMap::from([(
            "A".try_into().unwrap(),
            (KTransformationData::from([(
                "SOLVE_ORBIT".into(),
                KTransformationOrbitData {
                    permutation: vec![1, 0],
                    orientation_delta: vec![0; 2],
                },
            )])),
        )]),
        derived_moves: Some(HashMap::from([
            ("B".try_into().unwrap(), "C".try_into().unwrap()),
            ("C".try_into().unwrap(), "A B".try_into().unwrap()),
        ])),
    };
    assert!(KPuzzle::try_new(def)
        .expect_err("Expected recursive KPuzzle to fail instantiation.")
        .description
        .starts_with("Recursive derived move definition for: "));
    Ok(())
}
