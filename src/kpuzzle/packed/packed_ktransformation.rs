use std::{fmt::Debug, hash::BuildHasher, mem::swap};

use crate::{alg::Amount, kpuzzle::KTransformationData};

use super::{
    byte_conversions::{u8_to_usize, usize_to_u8},
    packed_kpuzzle::PackedKPuzzleOrbitInfo,
    packed_orbit_data::PackedOrbitData,
    ConversionError, PackedKPuzzle,
};

#[derive(Clone, Eq)]
pub struct PackedKTransformation {
    pub packed_orbit_data: PackedOrbitData,
}

impl PackedKTransformation {
    pub(crate) fn new_uninitialized(packed_kpuzzle: PackedKPuzzle) -> Self {
        Self {
            packed_orbit_data: PackedOrbitData::new_with_uninitialized_bytes(packed_kpuzzle),
        }
    }

    // TODO: validation?
    pub fn try_from_data(
        packed_kpuzzle: &PackedKPuzzle,
        kpattern_data: &KTransformationData,
    ) -> Result<Self, ConversionError> {
        let mut new_packed_ktransformation = Self::new_uninitialized(packed_kpuzzle.clone());
        for orbit_info in &packed_kpuzzle.data.orbit_iteration_info {
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
    pub fn get_permutation_idx(&self, orbit_info: &PackedKPuzzleOrbitInfo, i: usize) -> u8 {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.pieces_or_pemutations_offset + i)
                .read()
        }
    }

    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn get_orientation_delta(&self, orbit_info: &PackedKPuzzleOrbitInfo, i: usize) -> u8 {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .read()
        }
    }

    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn set_permutation_idx(
        &mut self,
        orbit_info: &PackedKPuzzleOrbitInfo,
        i: usize,
        value: u8,
    ) {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.pieces_or_pemutations_offset + i)
                .write(value)
        }
    }

    // TODO: dedup with PackedKTransformation, or at least implement as a trait?
    pub fn set_orientation_delta(
        &mut self,
        orbit_info: &PackedKPuzzleOrbitInfo,
        i: usize,
        value: u8,
    ) {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .write(value)
        }
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/transformation.rs#L32-L61
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    pub fn apply_transformation(
        &self,
        transformation: &PackedKTransformation,
    ) -> PackedKTransformation {
        let mut new_packed_ktransformation =
            PackedKTransformation::new_uninitialized(self.packed_orbit_data.packed_kpuzzle.clone());
        self.apply_transformation_into(transformation, &mut new_packed_ktransformation);
        new_packed_ktransformation
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/transformation.rs#L32-L61
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    // TODO: assign to self from another value, not into another
    pub fn apply_transformation_into(
        &self,
        transformation: &PackedKTransformation,
        into_packed_ktransformation: &mut PackedKTransformation,
    ) {
        for orbit_info in &self
            .packed_orbit_data
            .packed_kpuzzle
            .data
            .orbit_iteration_info
        {
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
                self.packed_orbit_data.packed_kpuzzle.data.num_bytes,
            )
        }
    }

    pub fn hash(&self) -> u64 {
        let h = cityhasher::CityHasher::new();
        h.hash_one(self.byte_slice())
    }

    pub fn invert(&self) -> PackedKTransformation {
        let mut new_packed_ktransformation =
            PackedKTransformation::new_uninitialized(self.packed_orbit_data.packed_kpuzzle.clone());
        for orbit_info in &self
            .packed_orbit_data
            .packed_kpuzzle
            .data
            .orbit_iteration_info
        {
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
            return self
                .packed_orbit_data
                .packed_kpuzzle
                .identity_transformation();
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
    kpuzzle: PackedKPuzzle,
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

impl Debug for PackedKTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedKTransformation")
            .field(
                "packed_kpuzzle",
                &KPuzzleDebug {
                    kpuzzle: self.packed_orbit_data.packed_kpuzzle.clone(),
                },
            )
            .field("bytes", &self.byte_slice())
            .finish()
    }
}

impl PartialEq<PackedKTransformation> for PackedKTransformation {
    fn eq(&self, other: &Self) -> bool {
        self.byte_slice() == other.byte_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::alg::AlgParseError;
    use crate::kpuzzle::packed::packed_kpuzzle::InvalidAlgError;
    use crate::kpuzzle::PackedKTransformation;
    use crate::parse_move;
    use crate::puzzles::cube3x3x3_kpuzzle;

    #[test]
    fn compose() -> Result<(), String> {
        let kpuzzle = cube3x3x3_kpuzzle();

        let from_move = |move_str: &str| -> Result<PackedKTransformation, String> {
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

pub struct PackedKTransformationBuffer {
    pub current: PackedKTransformation,
    scratch_space: PackedKTransformation,
}

impl From<PackedKTransformation> for PackedKTransformationBuffer {
    fn from(initial: PackedKTransformation) -> Self {
        Self {
            scratch_space: initial.clone(), // TODO?
            current: initial,
        }
    }
}

impl PackedKTransformationBuffer {
    pub fn apply_transformation(&mut self, transformation: &PackedKTransformation) {
        self.current
            .apply_transformation_into(transformation, &mut self.scratch_space);
        swap(&mut self.current, &mut self.scratch_space);
    }
}

impl PartialEq for PackedKTransformationBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}
