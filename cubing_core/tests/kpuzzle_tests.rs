use std::thread::spawn;

use cubing_core::{
    alg::{Alg, Move},
    kpuzzle::{
        InvalidAlgError, KPatternData, KPuzzle, KPuzzleOrbitName, KTransformationData,
        KTransformationOrbitData,
    },
    puzzles::cube3x3x3_kpuzzle,
};
use once_cell::sync::Lazy;

#[test]
fn it_works() -> Result<(), InvalidAlgError> {
    use std::collections::HashMap;

    use cubing_core::kpuzzle::{KPatternOrbitData, KPuzzleOrbitDefinition};

    let items_orbit_name = &KPuzzleOrbitName("items".to_owned());
    let def = cubing_core::kpuzzle::KPuzzleDefinition {
        name: "topsy_turvy".into(),
        orbits: vec![KPuzzleOrbitDefinition {
            orbit_name: items_orbit_name.clone(),
            num_pieces: 12,
            num_orientations: 1,
        }],
        default_pattern: KPatternData::from([(
            items_orbit_name.clone(),
            KPatternOrbitData {
                pieces: (0..11).collect(),
                orientation: vec![0; 12],
                orientation_mod: None,
            },
        )]),
        moves: HashMap::from([
            (
                "L".try_into()?,
                (KTransformationData::from([(
                    items_orbit_name.clone(),
                    KTransformationOrbitData {
                        permutation: vec![10, 8, 6, 4, 2, 0, 1, 3, 5, 7, 9, 11], // TODO: is this actually L'?
                        orientation_delta: vec![0; 12],
                    },
                )])),
            ),
            (
                "R".try_into()?,
                (KTransformationData::from([(
                    items_orbit_name.clone(),
                    KTransformationOrbitData {
                        permutation: vec![1, 3, 5, 7, 9, 11, 10, 8, 6, 4, 2, 0], // TODO: is this actually R'?
                        orientation_delta: vec![0; 12],
                    },
                )])),
            ),
        ]),
        derived_moves: None,
    };

    let kpuzzle: KPuzzle = KPuzzle::try_new(def).unwrap();
    let items_orbit_name = &KPuzzleOrbitName("items".to_owned());
    let items_orbit = kpuzzle.lookup_orbit(items_orbit_name).unwrap();

    assert_eq!(kpuzzle.definition().name, "topsy_turvy");
    assert_eq!(
        kpuzzle.definition().default_pattern[items_orbit_name]
            .orientation
            .len(),
        12
    );
    assert_eq!(
        kpuzzle.definition().default_pattern[items_orbit_name].pieces[4],
        4
    );
    assert_eq!(
        kpuzzle.definition().default_pattern[items_orbit_name].orientation[4],
        0
    );

    assert_eq!(
        kpuzzle
            .transformation_from_move(&("L").parse::<Move>()?)?
            .get_permutation_idx(items_orbit, 0),
        10
    );

    let t = kpuzzle.transformation_from_move(&("R").parse::<Move>()?)?;
    let mut current = t.clone(); // TODO: start with solved.
    for _ in 1..10 {
        assert_ne!(current.get_permutation_idx(items_orbit, 0), 0);
        current = current.apply_transformation(&t);
    }
    assert_eq!(current.get_permutation_idx(items_orbit, 0), 0);

    assert_eq!(
        t.apply_transformation(&t),
        kpuzzle.transformation_from_alg(&("R2").parse::<Alg>().unwrap())?
    );
    assert_ne!(
        t.apply_transformation(&t),
        kpuzzle.transformation_from_alg(&("L R".parse::<Alg>()).unwrap())?
    );
    assert_eq!(
        t.apply_transformation(&t).apply_transformation(&t),
        kpuzzle.transformation_from_alg(&("R3").parse::<Alg>().unwrap())?
    );
    assert_eq!(
        kpuzzle.identity_transformation(),
        kpuzzle.transformation_from_alg(&("R10".parse::<Alg>()).unwrap())?
    );
    assert_ne!(
        kpuzzle.identity_transformation(),
        kpuzzle.transformation_from_alg(&("R5").parse::<Alg>().unwrap())?
    );
    Ok(())
}

#[test]
fn ktransformation_can_be_sent_to_and_returned_from_threads() -> Result<(), String> {
    let transformation = cube3x3x3_kpuzzle()
        .transformation_from_alg(&"R U R'".parse().unwrap())
        .unwrap();
    let inverse = transformation.invert();
    let inverse_clone = inverse.clone();
    let result = spawn(move || inverse_clone.invert()).join().unwrap();
    assert_eq!(transformation, result);
    assert_ne!(inverse, result);
    Ok(())
}

static SUPERFLIP: Lazy<Alg> = Lazy::new(|| ("((M' U')4 x y)3").parse::<Alg>().unwrap());
static TRIGGER: Lazy<Alg> = Lazy::new(|| ("[R: U]").parse::<Alg>().unwrap());

#[test]
fn static_kpattern_can_be_sent_to_and_returned_from_threads() -> Result<(), InvalidAlgError> {
    let default_pattern = cube3x3x3_kpuzzle().default_pattern();

    let superflip_first = default_pattern.apply_alg(&SUPERFLIP)?;
    let trigger_second_handle = spawn(move || superflip_first.apply_alg(&TRIGGER).unwrap());

    let trigger_first = default_pattern.apply_alg(&TRIGGER)?;
    let superflip_second_handle = spawn(move || trigger_first.apply_alg(&SUPERFLIP).unwrap());

    let trigger_second = trigger_second_handle.join().unwrap();
    let superflip_second = superflip_second_handle.join().unwrap();
    assert_eq!(trigger_second, superflip_second);
    assert_ne!(default_pattern, trigger_second);
    assert_ne!(default_pattern, superflip_second);
    Ok(())
}
