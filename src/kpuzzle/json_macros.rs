/// Load a `KPuzzle` from a JSON file in your source code. The file will only be
/// loaded on the first call, and all subsequent calls will use the cached
/// value
///
///  Example:
///
/// ```
/// use cubing::kpuzzle::KPuzzle;
/// use cubing::{kpuzzle_from_json_file};
///
/// kpuzzle_from_json_file!(pub(crate), example, "./example.kpuzzle.json");
/// let kpuzzle: &KPuzzle = example_kpuzzle();
/// ```
#[macro_export]
macro_rules! kpuzzle_from_json_file {
    ($visibility:vis, $kpuzzle_name: ident, $file:expr) => {
      $crate::kpuzzle::_reexported::_paste::paste! {
          #[allow(non_upper_case_globals)]
          static [<$kpuzzle_name _kpuzzle_cell>]: std::sync::OnceLock<$crate::kpuzzle::KPuzzle> = std::sync::OnceLock::new();
          $visibility fn [<$kpuzzle_name _kpuzzle>]() -> &'static $crate::kpuzzle::KPuzzle {
                [<$kpuzzle_name _kpuzzle_cell>].get_or_init(|| {
                    let def: $crate::kpuzzle::KPuzzleDefinition = serde_json::from_slice(include_bytes!($file)).unwrap();
                    let kpuzzle: $crate::kpuzzle::KPuzzle = def.try_into().unwrap();
                    kpuzzle
                })
            }
        }
    };
}

/// Load a `KPattern` from a JSON file in your source code. The file will only be
/// loaded on the first call, and all subsequent calls will use the cached
/// value
///
///  Example:
///
/// ```
/// use cubing::kpuzzle::{KPuzzle, KPattern};
/// use cubing::{kpattern_from_json_file, kpuzzle_from_json_file};
///
/// kpuzzle_from_json_file!(pub(crate), example, "./example.kpuzzle.json");
///
/// kpattern_from_json_file!(pub(crate), example, "./example.kpattern.json", example_kpuzzle());
/// let kpattern: &KPattern = example_kpattern();
/// ```
#[macro_export]
macro_rules! kpattern_from_json_file {
    ($visibility:vis, $kpattern_name: ident, $file:expr, $kpuzzle: expr) => {
      $crate::kpuzzle::_reexported::_paste::paste! {
          #[allow(non_upper_case_globals)]
          static [<$kpattern_name _kpattern_cell>]: std::sync::OnceLock<$crate::kpuzzle::KPattern> = std::sync::OnceLock::new();
          $visibility fn [<$kpattern_name _kpattern>]() -> &'static $crate::kpuzzle::KPattern {
            [<$kpattern_name _kpattern_cell>]
                  .get_or_init(|| {
                    $crate::kpuzzle::KPattern::try_from_json(
                          $kpuzzle,
                          include_bytes!($file),
                      )
                      .unwrap()
                  })

                }
          }
    };
}

/// Load a `KTransformation` from a JSON file in your source code. The file will only be
/// loaded on the first call, and all subsequent calls will use the cached
/// value
///
///  Example:
///
/// ```
/// use cubing::kpuzzle::{KPuzzle, KTransformation};
/// use cubing::{kpuzzle_from_json_file, ktransformation_from_json_file};
///
/// kpuzzle_from_json_file!(pub(crate), example, "./example.kpuzzle.json");
///
/// ktransformation_from_json_file!(pub(crate), example, "./example.ktransformation.json", example_kpuzzle());
/// let ktransformation: &KTransformation = example_ktransformation();
/// ```
#[macro_export]
macro_rules! ktransformation_from_json_file {
    ($visibility:vis, $ktransformation_name: ident, $file:expr, $kpuzzle: expr) => {
      $crate::kpuzzle::_reexported::_paste::paste! {
          #[allow(non_upper_case_globals)]
          static [<$ktransformation_name _ktransformation_cell>]: std::sync::OnceLock<$crate::kpuzzle::KTransformation> = std::sync::OnceLock::new();
          $visibility fn [<$ktransformation_name _ktransformation>]() -> &'static $crate::kpuzzle::KTransformation {
            [<$ktransformation_name _ktransformation_cell>]
                  .get_or_init(|| {
                    $crate::kpuzzle::KTransformation::try_from_json(
                          $kpuzzle,
                          include_bytes!($file),
                      )
                      .unwrap()
                  })

                }
          }
    };
}
