pub mod alg {
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
