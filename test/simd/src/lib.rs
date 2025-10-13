#[cfg(test)]
mod tests {
    use cubing::{alg::parse_alg, puzzles::cube3x3x3_kpuzzle};

    #[test]
    fn my_test() -> Result<(), String> {
        let kpuzzle = cube3x3x3_kpuzzle();
        let transformation = kpuzzle
            .transformation_from_alg(parse_alg!("R U2 R'"))
            .unwrap();

        let p1 = kpuzzle
            .default_pattern()
            .apply_transformation(&transformation);
        assert_ne!(p1, kpuzzle.default_pattern());
        let p2 = p1.apply_transformation(&transformation);
        assert_eq!(p2, kpuzzle.default_pattern());

        Ok(())
    }
}
