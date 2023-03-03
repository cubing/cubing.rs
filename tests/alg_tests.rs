#[cfg(test)]
use std::convert::TryInto;

#[test]
fn it_works() -> Result<(), String> {
    assert_eq!(
        "R",
        format!(
            "{}",
            cubing::alg::Move {
                quantum: cubing::alg::QuantumMove::new("R", None, None).into(),
                amount: 1
            }
        )
    );

    assert_eq!(
        "F2",
        format!("{}", cubing::alg::Move::try_from("F2").unwrap())
    );
    assert_eq!("F2", format!("{}", cubing::alg::Move::parse("F2").unwrap()));
    assert_eq!("F", format!("{}", cubing::alg::Move::parse("F1").unwrap()));
    assert_eq!("F", format!("{}", cubing::alg::Move::parse("F").unwrap()));
    assert_eq!(
        "F'",
        format!("{}", cubing::alg::Move::parse("F1'").unwrap())
    );
    assert_eq!("F0", format!("{}", cubing::alg::Move::parse("F0").unwrap()));
    assert_eq!(
        "F2'",
        format!("{}", cubing::alg::Move::parse("F2'").unwrap())
    );
    assert_eq!(
        "U_R",
        format!("{}", cubing::alg::Move::parse("U_R").unwrap())
    );
    assert_eq!(
        "4R2'",
        format!("{}", cubing::alg::Move::parse("4R2'").unwrap())
    );
    assert_eq!(
        "3-7R2'",
        format!("{}", cubing::alg::Move::parse("3-7R2'").unwrap())
    );

    assert_eq!(
        "3-7R2'",
        format!(
            "{}",
            cubing::alg::Move {
                quantum: cubing::alg::QuantumMove::new("R", Some(3), Some(7)).into(),
                amount: -2
            }
        )
    );

    let single_move = cubing::alg::Move::parse("R2'").unwrap();
    assert_eq!(single_move.quantum.outer_layer, None);
    assert_eq!(single_move.quantum.inner_layer, None);
    assert_eq!(single_move.quantum.family, "R");
    assert_eq!(single_move.amount, -2);

    let face_move = cubing::alg::Move::parse("R2'").unwrap();
    assert_eq!(face_move.quantum.outer_layer, None);
    assert_eq!(face_move.quantum.inner_layer, None);
    assert_eq!(face_move.quantum.family, "R");
    assert_eq!(face_move.amount, -2);

    let block_move = cubing::alg::Move::parse("7R2'").unwrap();
    assert_eq!(block_move.quantum.outer_layer, None);
    assert_eq!(block_move.quantum.inner_layer, Some(7));
    assert_eq!(block_move.quantum.family, "R");
    assert_eq!(block_move.amount, -2);

    let range_move = cubing::alg::Move::parse("3-7R2'").unwrap();
    assert_eq!(range_move.quantum.outer_layer, Some(3));
    assert_eq!(range_move.quantum.inner_layer, Some(7));
    assert_eq!(range_move.quantum.family, "R");
    assert_eq!(range_move.amount, -2);

    assert_eq!(
        cubing::alg::Move::parse("R2").unwrap(),
        cubing::alg::Move {
            quantum: cubing::alg::QuantumMove {
                family: "R".into(),
                outer_layer: None,
                inner_layer: None
            }
            .into(),
            amount: 2
        }
    );

    assert_eq!(
        "F2'",
        format!("{}", cubing::alg::Move::parse("F2").unwrap().invert())
    );

    assert!(cubing::alg::Move::parse("2").is_err());
    assert!(cubing::alg::Move::parse("U-R").is_err());
    let mv: cubing::alg::Move = "UR43".try_into()?;
    println!("Display: {}", mv);
    println!("Debug: {:?}", mv);
    Ok(())
}
