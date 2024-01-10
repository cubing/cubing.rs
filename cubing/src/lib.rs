pub mod alg {
    /// A representation of a cubing alg, the equivalent of <https://js.cubing.net/cubing/api/classes/alg.Alg.html>
    ///
    /// To create a fresh alg, it is often useful to use [`parse_alg`] macro:
    ///
    /// ```
    /// # pub mod cubing {
    /// #   pub mod alg {
    /// #     pub use cubing_macros::parse_alg;
    /// #     pub use cubing_core::alg::Alg;
    /// #   }
    /// # }
    /// use cubing::alg::parse_alg;
    ///
    /// let alg = parse_alg!("F U R");
    /// assert_eq!(alg.invert(), parse_alg!("R' U' F'"));
    /// ```
    pub use cubing_core::alg::Alg;
    pub use cubing_core::alg::*;
    pub use cubing_macros::{parse_alg, parse_move};
}

pub mod kpuzzle {
    pub use cubing_core::kpuzzle::*;
    pub use cubing_core::{
        kpattern_from_json_file, kpuzzle_from_json_file, ktransformation_from_json_file,
    };
}

pub mod puzzles {
    pub use cubing_core::puzzles::*;
}
