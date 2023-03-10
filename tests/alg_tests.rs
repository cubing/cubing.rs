use cubing::alg::{Alg, Move, MoveLayer, MovePrefix, MoveRange, QuantumMove};

#[test]
fn it_works() -> Result<(), String> {
    assert_eq!(
        "R",
        format!(
            "{}",
            Move {
                quantum: QuantumMove::new("R", None).into(),
                amount: 1
            }
        )
    );

    assert_eq!("F2", format!("{}", Move::try_from("F2").unwrap()));
    assert_eq!("F2", format!("{}", "F2".parse::<Move>().unwrap()));
    assert_eq!("F", format!("{}", "F1".parse::<Move>().unwrap()));
    assert_eq!("F", format!("{}", "F".parse::<Move>().unwrap()));
    assert_eq!("F'", format!("{}", "F1'".parse::<Move>().unwrap()));
    assert_eq!("F0", format!("{}", "F0".parse::<Move>().unwrap()));
    assert_eq!("F2'", format!("{}", "F2'".parse::<Move>().unwrap()));
    assert_eq!("U_R", format!("{}", "U_R".parse::<Move>().unwrap()));
    assert_eq!("4R2'", format!("{}", "4R2'".parse::<Move>().unwrap()));
    assert_eq!("3-7R2'", format!("{}", "3-7R2'".parse::<Move>().unwrap()));

    assert_eq!(
        "3-7R2'",
        format!(
            "{}",
            Move {
                quantum: QuantumMove::new("R", MoveRange::new(3, 7).into()).into(),
                amount: -2
            }
        )
    );

    assert_eq!(MoveLayer { layer: 7 }, MoveLayer::try_from("7")?);
    assert_eq!(Ok(MoveLayer { layer: 7 }), "7".try_into());
    assert_eq!(MoveLayer { layer: 7 }, "7".parse()?);
    assert_eq!(MoveLayer::from(7), MoveLayer { layer: 7 });

    assert_eq!(
        MoveRange {
            outer_layer: 2,
            inner_layer: 4
        },
        "2-4".parse()?
    );
    assert_eq!(
        MovePrefix::Range(MoveRange {
            outer_layer: 2,
            inner_layer: 4
        }),
        "2-4".parse()?
    );

    let single_move = "R2'".parse::<Move>().unwrap();
    assert_eq!(single_move.quantum.prefix, None);
    assert_eq!(single_move.quantum.family, "R");
    assert_eq!(single_move.amount, -2);

    let face_move = "R2'".parse::<Move>().unwrap();
    assert_eq!(face_move.quantum.prefix, None);
    assert_eq!(face_move.quantum.family, "R");
    assert_eq!(face_move.amount, -2);

    let block_move = "7R2'".parse::<Move>().unwrap();
    assert_eq!(block_move.quantum.prefix, MoveLayer::new(7).into());
    assert_eq!(block_move.quantum.family, "R");
    assert_eq!(block_move.amount, -2);

    let range_move = "3-7R2'".parse::<Move>().unwrap();
    assert_eq!(range_move.quantum.prefix, MoveRange::new(3, 7).into());
    assert_eq!(range_move.quantum.family, "R");
    assert_eq!(range_move.amount, -2);

    assert_eq!(
        "R2".parse::<Move>().unwrap(),
        Move {
            quantum: QuantumMove {
                family: "R".into(),
                prefix: None
            }
            .into(),
            amount: 2
        }
    );

    assert_eq!("F2'", format!("{}", "F2".parse::<Move>().unwrap().invert()));

    assert!("2".parse::<Move>().is_err());
    assert!("U-R".parse::<Move>().is_err());
    let mv: Move = "UR43".try_into()?;
    println!("Display: {}", mv);
    println!("Debug: {:?}", mv);

    let a1 = Alg {
        nodes: vec![Move::try_from("F2").unwrap(), Move::try_from("R").unwrap()],
    };
    let a2 = Alg {
        nodes: vec![
            Move::try_from("R'").unwrap(),
            Move::try_from("F2'").unwrap(),
        ],
    };
    assert!(a1 == a2.invert());

    assert_eq!("R U' R'".parse::<Alg>()?, "R U R'".parse::<Alg>()?.invert());
    assert_eq!(" R   U'  R'  ".parse::<Alg>()?, "R U' R'".parse::<Alg>()?);
    assert_eq!("R U' R'", " R   U'  R'  ".parse::<Alg>()?.to_string());

    Ok(())
}
