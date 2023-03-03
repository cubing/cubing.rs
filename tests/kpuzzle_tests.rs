#[cfg(test)]
#[test]
fn it_works() -> Result<(), String> {
    use std::collections::HashMap;

    use cubing::kpuzzle::{KPuzzleOrbitDefinition, KStateOrbitData};

    let def = cubing::kpuzzle::KPuzzleDefinition {
        name: "test".into(),
        orbits: HashMap::from([(
            "pieces".into(),
            KPuzzleOrbitDefinition {
                num_pieces: 12,
                num_orientations: 1,
            },
        )]),
        start_state_data: HashMap::from([(
            "pieces".into(),
            KStateOrbitData {
                pieces: (1..12).collect(),
                orientation: vec![0; 12],
            },
        )]),
        moves: HashMap::new(),
    };

    let kpuzzle = cubing::kpuzzle::KPuzzle {
        definition: def.into(),
    };

    assert_eq!(kpuzzle.definition.name, "test");

    Ok(())
}
