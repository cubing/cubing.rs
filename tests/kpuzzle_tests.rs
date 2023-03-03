#[test]
fn it_works() -> Result<(), String> {
    use std::collections::HashMap;

    use cubing::kpuzzle::{KPuzzleOrbitDefinition, KStateOrbitData};

    let def = cubing::kpuzzle::KPuzzleDefinition {
        name: "test".into(),
        orbits: HashMap::from([(
            "items".into(),
            KPuzzleOrbitDefinition {
                num_pieces: 12,
                num_orientations: 1,
            },
        )]),
        start_state_data: HashMap::from([(
            "items".into(),
            KStateOrbitData {
                pieces: (0..11).collect(),
                orientation: vec![0; 12],
            },
        )]),
        moves: HashMap::new(),
    };

    let kpuzzle = cubing::kpuzzle::KPuzzle {
        definition: def.into(),
    };

    assert_eq!(kpuzzle.definition.name, "test");
    assert_eq!(
        kpuzzle.definition.start_state_data["items"]
            .orientation
            .len(),
        12
    );
    assert_eq!(kpuzzle.definition.start_state_data["items"].pieces[4], 4);
    assert_eq!(
        kpuzzle.definition.start_state_data["items"].orientation[4],
        0
    );

    Ok(())
}
