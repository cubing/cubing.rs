use std::sync::Arc;

use cubing::{
    alg::Move,
    kpuzzle::{KPuzzle, KPuzzleOrbitName, KTransformationOrbitData},
};

#[test]
fn it_works() -> Result<(), String> {
    use std::collections::HashMap;

    use cubing::kpuzzle::{KPuzzleOrbitDefinition, KStateOrbitData};

    let items_orbit_name = &KPuzzleOrbitName("items".to_owned());
    let def = cubing::kpuzzle::KPuzzleDefinition {
        name: "topsy_turvy".into(),
        orbits: HashMap::from([(
            items_orbit_name.clone(),
            KPuzzleOrbitDefinition {
                num_pieces: 12,
                num_orientations: 1,
            },
        )]),
        start_state_data: HashMap::from([(
            items_orbit_name.clone(),
            KStateOrbitData {
                pieces: (0..11).collect(),
                orientation: vec![0; 12],
            },
        )])
        .into(),
        moves: HashMap::from([
            (
                "L".try_into()?,
                Arc::new(HashMap::from([(
                    items_orbit_name.clone(),
                    KTransformationOrbitData {
                        permutation: vec![10, 8, 6, 4, 2, 0, 1, 3, 5, 7, 9, 11], // TODO: is this actually L'?
                        orientation: vec![0; 12],
                    },
                )])),
            ),
            (
                "R".try_into()?,
                Arc::new(HashMap::from([(
                    items_orbit_name.clone(),
                    KTransformationOrbitData {
                        permutation: vec![1, 3, 5, 7, 9, 11, 10, 8, 6, 4, 2, 0], // TODO: is this actually R'?
                        orientation: vec![0; 12],
                    },
                )])),
            ),
        ]),
        experimental_derived_moves: None,
    };

    let kpuzzle: KPuzzle = def.try_into()?;
    let items_orbit_name = &KPuzzleOrbitName("items".to_owned());

    assert_eq!(kpuzzle.definition().name, "topsy_turvy");
    assert_eq!(
        kpuzzle.definition().start_state_data[items_orbit_name]
            .orientation
            .len(),
        12
    );
    assert_eq!(
        kpuzzle.definition().start_state_data[items_orbit_name].pieces[4],
        4
    );
    assert_eq!(
        kpuzzle.definition().start_state_data[items_orbit_name].orientation[4],
        0
    );

    assert_eq!(
        kpuzzle
            .transformation_from_move(&("L").parse::<Move>()?)?
            .transformation_data[items_orbit_name]
            .permutation[0],
        10
    );

    let t = kpuzzle.transformation_from_move(&("R").parse::<Move>()?)?;
    let mut current = t.clone(); // TODO: start with solved.
    for _ in 1..10 {
        assert_ne!(
            current.transformation_data[items_orbit_name].permutation[0],
            0
        );
        current = current.apply_transformation(&t);
    }
    assert_eq!(
        current.transformation_data[items_orbit_name].permutation[0],
        0
    );

    assert_eq!(
        t.apply_transformation(&t).transformation_data,
        kpuzzle.transformation_from_str("R2")?.transformation_data
    );
    assert_ne!(t.apply_transformation(&t), (&kpuzzle, "L R").try_into()?);
    assert_eq!(
        t.apply_transformation(&t).apply_transformation(&t),
        (&kpuzzle, "R3").try_into()?
    );
    assert_eq!(
        kpuzzle.identity_transformation(),
        (&kpuzzle, "R10").try_into()?
    );
    assert_ne!(
        kpuzzle.identity_transformation(),
        (&kpuzzle, "R5").try_into()?
    );
    Ok(())
}
