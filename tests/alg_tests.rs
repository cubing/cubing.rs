use cubing::alg::{Alg, Move, MoveLayer, MoveRange, QuantumMove};

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
    assert_eq!("F2", format!("{}", Move::parse("F2").unwrap()));
    assert_eq!("F", format!("{}", Move::parse("F1").unwrap()));
    assert_eq!("F", format!("{}", Move::parse("F").unwrap()));
    assert_eq!("F'", format!("{}", Move::parse("F1'").unwrap()));
    assert_eq!("F0", format!("{}", Move::parse("F0").unwrap()));
    assert_eq!("F2'", format!("{}", Move::parse("F2'").unwrap()));
    assert_eq!("U_R", format!("{}", Move::parse("U_R").unwrap()));
    assert_eq!("4R2'", format!("{}", Move::parse("4R2'").unwrap()));
    assert_eq!("3-7R2'", format!("{}", Move::parse("3-7R2'").unwrap()));

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

    assert_eq!(MoveLayer::from(7), MoveLayer { layer: 7 });

    let single_move = Move::parse("R2'").unwrap();
    assert_eq!(single_move.quantum.layers, None);
    assert_eq!(single_move.quantum.family, "R");
    assert_eq!(single_move.amount, -2);

    let face_move = Move::parse("R2'").unwrap();
    assert_eq!(face_move.quantum.layers, None);
    assert_eq!(face_move.quantum.family, "R");
    assert_eq!(face_move.amount, -2);

    let block_move = Move::parse("7R2'").unwrap();
    assert_eq!(block_move.quantum.layers, MoveLayer::new(7).into());
    assert_eq!(block_move.quantum.family, "R");
    assert_eq!(block_move.amount, -2);

    let range_move = Move::parse("3-7R2'").unwrap();
    assert_eq!(range_move.quantum.layers, MoveRange::new(3, 7).into());
    assert_eq!(range_move.quantum.family, "R");
    assert_eq!(range_move.amount, -2);

    assert_eq!(
        Move::parse("R2").unwrap(),
        Move {
            quantum: QuantumMove {
                family: "R".into(),
                layers: None
            }
            .into(),
            amount: 2
        }
    );

    assert_eq!("F2'", format!("{}", Move::parse("F2").unwrap().invert()));

    assert!(Move::parse("2").is_err());
    assert!(Move::parse("U-R").is_err());
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
    Ok(())
}
