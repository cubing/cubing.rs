#[cfg(test)]
mod tests {
    use cubing_core::alg::{Alg, Move};
    use cubing_macros::{parse_alg, parse_move};

    #[test]
    fn parse_move() {
        let r#move: Move = parse_move!("R");
        assert_eq!("R".parse::<Move>().unwrap(), r#move);
    }

    #[test]
    fn parse_alg() {
        let alg: Alg = parse_alg!("R");
        assert_eq!("R".parse::<Alg>().unwrap(), alg);

        let alg: Alg = parse_alg!("R U R'");
        assert_eq!("R U R'".parse::<Alg>().unwrap(), alg);
    }

    #[test]
    fn parse_move_trailing_underscores() {
        let r#move: Move = parse_move!("R"_____); // TODO: Make this fail to compile.
        assert_eq!("R".parse::<Move>().unwrap(), r#move);
    }
}
