use std::{sync::Arc, thread::spawn};

use cubing::{
    alg::{Alg, AlgBuilder, AlgNode, Move, MoveLayer, MovePrefix, MoveRange, Newline, QuantumMove},
    kpuzzle::InvalidAlgError,
    parse_alg, parse_move,
};

#[test]
fn it_works() -> Result<(), InvalidAlgError> {
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
    assert_eq!("R++", format!("{}", "R++".parse::<Move>().unwrap()));
    assert_eq!("D--", format!("{}", "D--".parse::<Move>().unwrap()));
    assert_eq!("/", format!("{}", "/".parse::<Move>().unwrap()));
    assert_eq!("R1+", format!("{}", "R1+".parse::<Move>().unwrap()));
    assert_eq!("DR3-", format!("{}", "DR3-".parse::<Move>().unwrap()));
    assert_eq!("U0+", format!("{}", "U0+".parse::<Move>().unwrap()));
    assert!("R+".parse::<Move>().is_err());
    assert!("DR-".parse::<Move>().is_err());
    assert!("DR+3".parse::<Move>().is_err());
    assert!("DR+3'".parse::<Move>().is_err());
    assert!("DR-3".parse::<Move>().is_err());
    assert!("DR-3'".parse::<Move>().is_err());
    assert!("DR++1".parse::<Move>().is_err());
    assert!("DR++3".parse::<Move>().is_err());

    assert_eq!("\n", Newline {}.to_string());

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

    let range_move = "R++".parse::<Move>().unwrap();
    assert_eq!(range_move.quantum.prefix, None);
    assert_eq!(range_move.quantum.family, "R_PLUSPLUS_");
    assert_eq!(range_move.amount, 1);

    let range_move = "D--".parse::<Move>().unwrap();
    assert_eq!(range_move.quantum.prefix, None);
    assert_eq!(range_move.quantum.family, "D_PLUSPLUS_");
    assert_eq!(range_move.amount, -1);

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

    assert_eq!(
        parse_move!("2R3'")?.quantum.as_ref(),
        &QuantumMove {
            prefix: Some(MovePrefix::Layer(MoveLayer { layer: 2 },)),
            family: "R".into()
        }
    );
    assert_eq!(parse_move!("2R3'")?.amount, -3);

    assert!("2".parse::<Move>().is_err());
    assert!("U-R".parse::<Move>().is_err());
    std::convert::TryInto::<Move>::try_into("UR43")?;
    // println!("Display: {}", mv);
    // println!("Debug: {:?}", mv);

    let a1 = Alg {
        nodes: vec![
            Move::try_from("F2").unwrap().into(),
            Move::try_from("R").unwrap().into(),
        ],
    };
    let a2 = Alg {
        nodes: vec![
            Move::try_from("R'").unwrap().into(),
            Move::try_from("F2'").unwrap().into(),
        ],
    };
    assert!(a1 == a2.invert());

    assert_eq!("R U' R'".parse::<Alg>()?, "R U R'".parse::<Alg>()?.invert());
    assert_eq!(" R   U'  R'  ".parse::<Alg>()?, "R U' R'".parse::<Alg>()?);
    assert_eq!("R U' R'", " R   U'  R'  ".parse::<Alg>()?.to_string());
    assert_eq!(Alg { nodes: vec![] }, "".parse::<Alg>()?);
    assert_eq!(Alg { nodes: vec![] }, "  ".parse::<Alg>()?);
    assert_eq!(
        Alg {
            nodes: vec!["R'".parse::<Move>()?.into()]
        },
        " R' ".parse::<Alg>()?
    );
    assert_eq!("(R U' R')", "(R   U'  R' )".parse::<Alg>()?.to_string());
    assert_eq!(
        "[R, U]",
        "  [ U  , R ]  ".parse::<Alg>()?.invert().to_string()
    );
    assert_eq!(
        "[R: U']",
        "  [ R  : U ]  ".parse::<Alg>()?.invert().to_string()
    );

    assert_eq!("R'".parse::<Move>()?, parse_move!("R'")?);
    assert_eq!("R U R'".parse::<Alg>()?, parse_alg!("R U R'")?);
    assert_eq!(
        "[R', F]3",
        format!("{}", "([R', F])3".parse::<Alg>().unwrap())
    );

    let wr = "y x' // inspection
U R2 U' F' L F' U' L' // XX-Cross + EO
U' R U R' // 3rd slot
R' U R U2' R' U R // 4th slot
U R' U' R U' R' U2 R // OLL / ZBLL
U // AUF

// from http://cubesolv.es/solve/5757";
    wr.parse::<Alg>()?;
    assert_eq!(wr, wr.parse::<Alg>()?.to_string()); // TODO: newline and line comment handling
    assert_eq!("\n\n", "\n\n".parse::<Alg>()?.to_string()); // TODO: newline and line comment handling
    assert_eq!("\n", "\n".parse::<Alg>()?.to_string()); // TODO: newline and line comment handling
    assert_eq!(
        "R\nB // comment\n\n",
        "R\nB // comment\n\n".parse::<Alg>()?.to_string()
    );
    assert_eq!(
        "R\nB // comment\n",
        "R\nB // comment\n".parse::<Alg>()?.to_string()
    ); // TODO: newline and line comment handling

    Ok(())
}

#[test]
fn it_can_build_and_parse_long_strings() -> Result<(), InvalidAlgError> {
    let mut builder = AlgBuilder::default();
    let quantum = Arc::new(QuantumMove {
        family: "R".into(),
        prefix: None,
    });
    for amount in 1..1000 {
        let r#move = Move {
            quantum: quantum.clone(),
            amount,
        };
        let alg_node: AlgNode = r#move.into();
        builder.push(&alg_node);
    }
    let alg = builder.to_alg();
    let s = alg.to_string();
    let re_parsed = s.parse::<Alg>()?;
    assert_eq!(alg, re_parsed);

    Ok(())
}

#[test]
fn mixed_puzzle_notation() -> Result<(), InvalidAlgError> {
    // Eventual parsing goal: `(R 2-5r3' (5, -24234) R++)' / [ UR1+   UR , F2 ]`
    // From: https://github.com/cubing/cubing.js/blob/4ca170732f9b178bb9af4e04135447f23acfa8d8/src/sites/experiments.cubing.net/cubing.js/alg/inspector.html#L16
    assert!("(R 2-5r3' R++)' / [ UR , F2 ]".parse::<Alg>().is_ok());

    Ok(())
}

#[test]
fn it_handles_crowding() -> Result<(), String> {
    assert!("R'U".parse::<Alg>().is_err());
    assert!("R//F".parse::<Alg>().is_err());
    Ok(())
}

#[test]
fn alg_can_be_sent_to_and_returned_from_threads() -> Result<(), InvalidAlgError> {
    let alg = "R U R'".parse::<Alg>()?;
    let inverse = alg.invert();
    let inverse_clone = alg.invert();
    let result = spawn(move || inverse_clone.invert()).join().unwrap();
    assert_eq!(alg, result);
    assert_ne!(inverse, result);
    Ok(())
}
