#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::Move;

    #[test]
    fn it_works() -> Result<(), String> {
        assert_eq!(
            "R",
            format!(
                "{}",
                crate::Move {
                    quantum: crate::QuantumMove::new("R", None, None).into(),
                    amount: 1
                }
            )
        );

        assert_eq!("F2", format!("{}", crate::Move::try_from("F2").unwrap()));
        assert_eq!("F2", format!("{}", alg_move!("F2")));
        assert_eq!("F2", format!("{}", crate::Move::parse("F2").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F1").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F").unwrap()));
        assert_eq!("F'", format!("{}", crate::Move::parse("F1'").unwrap()));
        assert_eq!("F0", format!("{}", crate::Move::parse("F0").unwrap()));
        assert_eq!("F2'", format!("{}", crate::Move::parse("F2'").unwrap()));
        assert_eq!("U_R", format!("{}", crate::Move::parse("U_R").unwrap()));
        assert_eq!("4R2'", format!("{}", crate::Move::parse("4R2'").unwrap()));
        assert_eq!(
            "3-7R2'",
            format!("{}", crate::Move::parse("3-7R2'").unwrap())
        );

        assert_eq!(
            "3-7R2'",
            format!(
                "{}",
                crate::Move {
                    quantum: crate::QuantumMove::new("R", Some(3), Some(7)).into(),
                    amount: -2
                }
            )
        );

        let single_move = crate::Move::parse("R2'").unwrap();
        assert_eq!(single_move.quantum.outer_layer, None);
        assert_eq!(single_move.quantum.inner_layer, None);
        assert_eq!(single_move.quantum.family, "R");
        assert_eq!(single_move.amount, -2);

        let face_move = crate::Move::parse("R2'").unwrap();
        assert_eq!(face_move.quantum.outer_layer, None);
        assert_eq!(face_move.quantum.inner_layer, None);
        assert_eq!(face_move.quantum.family, "R");
        assert_eq!(face_move.amount, -2);

        let block_move = crate::Move::parse("7R2'").unwrap();
        assert_eq!(block_move.quantum.outer_layer, None);
        assert_eq!(block_move.quantum.inner_layer, Some(7));
        assert_eq!(block_move.quantum.family, "R");
        assert_eq!(block_move.amount, -2);

        let range_move = crate::Move::parse("3-7R2'").unwrap();
        assert_eq!(range_move.quantum.outer_layer, Some(3));
        assert_eq!(range_move.quantum.inner_layer, Some(7));
        assert_eq!(range_move.quantum.family, "R");
        assert_eq!(range_move.amount, -2);

        assert_eq!(
            crate::Move::parse("R2").unwrap(),
            crate::Move {
                quantum: crate::QuantumMove {
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
            format!("{}", crate::Move::parse("F2").unwrap().invert())
        );

        assert!(crate::Move::parse("2").is_err());
        assert!(crate::Move::parse("U-R").is_err());
        let mv: Move = "UR43".try_into()?;
        println!("Display: {}", mv);
        println!("Debug: {:?}", mv);
        Ok(())
    }
}
