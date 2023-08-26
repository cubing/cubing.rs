use std::{collections::HashMap, sync::Arc};

use cubing::{
    kpuzzle::{
        InvalidAlgError, InvalidDefinitionError, KPatternData, KPatternOrbitData, KPuzzle,
        KPuzzleDefinition, KPuzzleOrbitDefinition, KTransformationData, KTransformationOrbitData,
    },
    parse_alg,
    puzzles::{cube2x2x2_kpuzzle, cube3x3x3_kpuzzle},
};

#[test]
fn it_works() -> Result<(), InvalidAlgError> {
    let kpuzzle = cube3x3x3_kpuzzle();
    assert_eq!(
        &kpuzzle.transformation_from_alg(&parse_alg!("R U R' F' U2")?)?,
        &kpuzzle.transformation_from_str("(L' U' L F U2')'")?,
    );
    assert_eq!(
        kpuzzle.transformation_from_alg(&parse_alg!("R U R' F' U2")?)?,
        (&kpuzzle, "(L' U' L F U2')'").try_into()?,
    );
    assert_ne!(
        &kpuzzle.transformation_from_alg(&parse_alg!("(R U R' U)5")?)?,
        &kpuzzle.transformation_from_alg(&parse_alg!("")?)?
    );
    assert_eq!(
        &kpuzzle
            .default_pattern()
            .apply_alg(&parse_alg!("(R U R' U)5")?)?
            .kpattern_data,
        &kpuzzle
            .default_pattern()
            .apply_alg(&parse_alg!("")?)?
            .kpattern_data
    );

    Ok(())
}

#[test]
fn test_2x2x2() -> Result<(), InvalidAlgError> {
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
        kpuzzle.transformation_from_str("L")?.ktransformation_data,
        kpuzzle
            .transformation_from_str("x' R")?
            .ktransformation_data,
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
        default_pattern: Arc::new(KPatternData::from([(
            "SOLVE_ORBIT".into(),
            KPatternOrbitData {
                pieces: vec![1, 0],
                orientation: vec![0; 2],
                orientation_mod: None,
            },
        )])),
        moves: HashMap::from([(
            "A".try_into().unwrap(),
            Arc::new(KTransformationData::from([(
                "SOLVE_ORBIT".into(),
                KTransformationOrbitData {
                    permutation: vec![1, 0],
                    orientation_delta: vec![0; 2],
                },
            )])),
        )]),
        experimental_derived_moves: Some(HashMap::from([
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
