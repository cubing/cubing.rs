use std::{fmt::Debug, hash::BuildHasher};

use crate::{alg::Amount, kpuzzle::KTransformationData};

use super::{
    byte_conversions::{u8_to_usize, usize_to_u8},
    kpuzzle::KPuzzleOrbitInfo,
    packed_orbit_data::PackedOrbitData,
    ConversionError, KPuzzle,
};

#[derive(Clone, Eq)]
pub struct KTransformation {
    pub packed_orbit_data: PackedOrbitData,
}

impl KTransformation {
    pub(crate) fn new_uninitialized(kpuzzle: KPuzzle) -> Self {
        Self {
            packed_orbit_data: PackedOrbitData::new_with_uninitialized_bytes(kpuzzle),
        }
    }

    // TODO: validation?
    pub fn try_from_data(
        kpuzzle: &KPuzzle,
        kpattern_data: &KTransformationData,
    ) -> Result<Self, ConversionError> {
        let mut new_packed_ktransformation = Self::new_uninitialized(kpuzzle.clone());
        for orbit_info in &kpuzzle.data.orbit_iteration_info {
            for i in 0..orbit_info.num_pieces {
                let orbit = kpattern_data.get(&orbit_info.name);
                let orbit = match orbit {
                    Some(default_orbit) => default_orbit,
                    None => panic!("Invalid default pattern"), // TODO: catch at construction time?
                };

                new_packed_ktransformation.set_permutation_idx(
                    orbit_info,
                    i,
                    usize_to_u8(orbit.permutation[i]),
                );
                new_packed_ktransformation.set_orientation_delta(
                    orbit_info,
                    i,
                    usize_to_u8(orbit.orientation_delta[i]),
                );
            }
        }
        Ok(new_packed_ktransformation)
    }
    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn get_permutation_idx(&self, orbit_info: &KPuzzleOrbitInfo, i: usize) -> u8 {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.pieces_or_permutations_offset + i)
                .read()
        }
    }

    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn get_orientation_delta(&self, orbit_info: &KPuzzleOrbitInfo, i: usize) -> u8 {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .read()
        }
    }

    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn set_permutation_idx(&mut self, orbit_info: &KPuzzleOrbitInfo, i: usize, value: u8) {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.pieces_or_permutations_offset + i)
                .write(value)
        }
    }

    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn set_orientation_delta(&mut self, orbit_info: &KPuzzleOrbitInfo, i: usize, value: u8) {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .write(value)
        }
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/transformation.rs#L32-L61
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    pub fn apply_transformation(&self, transformation: &KTransformation) -> KTransformation {
        let mut new_packed_ktransformation =
            KTransformation::new_uninitialized(self.packed_orbit_data.kpuzzle.clone());
        self.apply_transformation_into(transformation, &mut new_packed_ktransformation);
        new_packed_ktransformation
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/transformation.rs#L32-L61
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    // TODO: assign to self from another value, not into another
    pub fn apply_transformation_into(
        &self,
        transformation: &KTransformation,
        into_packed_ktransformation: &mut KTransformation,
    ) {
        for orbit_info in &self.packed_orbit_data.kpuzzle.data.orbit_iteration_info {
            // TODO: optimization when either value is the identity.
            for i in 0..orbit_info.num_pieces {
                let transformation_idx = transformation.get_permutation_idx(orbit_info, i);

                let new_piece_permutation =
                    self.get_permutation_idx(orbit_info, u8_to_usize(transformation_idx));
                into_packed_ktransformation.set_permutation_idx(
                    orbit_info,
                    i,
                    new_piece_permutation,
                );

                let previous_orientation_delta =
                    self.get_orientation_delta(orbit_info, u8_to_usize(transformation_idx));

                // TODO: lookup table?
                let new_orientation_delta = (previous_orientation_delta
                    + transformation.get_orientation_delta(orbit_info, i))
                    % orbit_info.num_orientations;
                into_packed_ktransformation.set_orientation_delta(
                    orbit_info,
                    i,
                    new_orientation_delta,
                );
            }
        }
    }

    pub fn byte_slice(&self) -> &[u8] {
        // yiss ☺️
        // https://stackoverflow.com/a/27150865
        unsafe {
            std::slice::from_raw_parts(
                self.packed_orbit_data.bytes,
                self.packed_orbit_data.kpuzzle.data.num_bytes,
            )
        }
    }

    pub fn hash(&self) -> u64 {
        let h = cityhasher::CityHasher::new();
        h.hash_one(self.byte_slice())
    }

    pub fn invert(&self) -> KTransformation {
        let mut new_packed_ktransformation =
            KTransformation::new_uninitialized(self.packed_orbit_data.kpuzzle.clone());
        for orbit_info in &self.packed_orbit_data.kpuzzle.data.orbit_iteration_info {
            let num_orientations = orbit_info.num_orientations;

            // TODO: optimization when either value is the identity.
            for i in 0..orbit_info.num_pieces {
                let from_idx = self.get_permutation_idx(orbit_info, i);
                new_packed_ktransformation.set_permutation_idx(
                    orbit_info,
                    u8_to_usize(from_idx),
                    usize_to_u8(i),
                );
                new_packed_ktransformation.set_orientation_delta(
                    orbit_info,
                    u8_to_usize(from_idx),
                    (num_orientations - self.get_orientation_delta(orbit_info, i))
                        .rem_euclid(num_orientations),
                )
            }
        }
        new_packed_ktransformation
    }

    pub(crate) fn self_multiply(&self, amount: Amount) -> Self {
        if amount == 1 {
            return self.clone();
        }
        if amount < 0 {
            return self.invert().self_multiply(-amount);
        }
        if amount == 0 {
            // TODO: use cached identity transformations from `KPuzzle`???
            return self.packed_orbit_data.kpuzzle.identity_transformation();
        }
        let twice_halfish = if amount == 2 {
            // We'd share this `apply_transformation` with the other branch, but that triggers a bug in the borrow checker(!)
            // https://github.com/rust-lang/rust/issues/54663
            self.apply_transformation(self)
        } else {
            println!("--{}--", amount / 2);
            let halfish = self.self_multiply(amount / 2);
            halfish.apply_transformation(&halfish)
        };
        if amount % 2 == 0 {
            twice_halfish
        } else {
            self.apply_transformation(&twice_halfish)
        }
    }
}

struct KPuzzleDebug {
    kpuzzle: KPuzzle,
}

impl Debug for KPuzzleDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ … name: \"{}\" … }}",
            &self.kpuzzle.data.definition.name
        )
    }
}

impl Debug for KTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedKTransformation")
            .field(
                "kpuzzle",
                &KPuzzleDebug {
                    kpuzzle: self.packed_orbit_data.kpuzzle.clone(),
                },
            )
            .field("bytes", &self.byte_slice())
            .finish()
    }
}

impl PartialEq<KTransformation> for KTransformation {
    fn eq(&self, other: &Self) -> bool {
        self.byte_slice() == other.byte_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::alg::AlgParseError;
    use crate::kpuzzle::InvalidAlgError;
    use crate::kpuzzle::KTransformation;
    use crate::parse_move;
    use crate::puzzles::cube3x3x3_kpuzzle;

    #[test]
    fn compose() -> Result<(), String> {
        let kpuzzle = cube3x3x3_kpuzzle();

        let from_move = |move_str: &str| -> Result<KTransformation, String> {
            let r#move = parse_move!(move_str).map_err(|e: AlgParseError| e.description)?;
            kpuzzle
                .transformation_from_move(&r#move)
                .map_err(|e: InvalidAlgError| e.to_string())
        };

        let id = kpuzzle.identity_transformation();
        let t1 = from_move("R")?;
        let t2 = from_move("R2")?;
        let t2prime = from_move("R2'")?;
        let t4 = from_move("R4")?;
        let t5 = from_move("R5")?;

        assert_eq!(id, t4);
        assert_eq!(t1, t5);
        assert_eq!(t2, t2prime);

        assert_ne!(id, t1);
        assert_ne!(id, t2);
        assert_ne!(t1, t2);

        assert_eq!(id.apply_transformation(&t1), t1);
        assert_eq!(t1.apply_transformation(&t1), t2);
        assert_eq!(t2.apply_transformation(&t1).apply_transformation(&t2), t1);

        Ok(())
    }
}

pub struct KTransformationBuffer {
    a: KTransformation,
    b: KTransformation,
    // In some rough benchmarks, using a boolean to track the current pattern was just a tad faster than using `std::mem::swap(…)`.
    // TODO: measure this properly across devices, and updated `PackedKTransformationBuffer` to match.
    a_is_current: bool,
}

impl From<KTransformation> for KTransformationBuffer {
    fn from(initial: KTransformation) -> Self {
        Self {
            b: initial.clone(), // TODO?
            a: initial,
            a_is_current: true,
        }
    }
}

impl KTransformationBuffer {
    pub fn apply_transformation(&mut self, transformation: &KTransformation) {
        if self.a_is_current {
            self.a
                .apply_transformation_into(transformation, &mut self.b);
        } else {
            self.b
                .apply_transformation_into(transformation, &mut self.a);
        }
        self.a_is_current = !self.a_is_current
    }

    pub fn current(&self) -> &KTransformation {
        if self.a_is_current {
            &self.a
        } else {
            &self.b
        }
    }
}

impl PartialEq for KTransformationBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.current() == other.current()
    }
}
