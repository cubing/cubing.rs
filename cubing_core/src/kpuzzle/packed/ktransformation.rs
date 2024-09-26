use std::fmt::Debug;

use more_asserts::assert_lt;

use crate::{
    alg::Amount,
    kpuzzle::{KTransformationData, KTransformationOrbitData},
};

use super::{
    kpuzzle::KPuzzleOrbitInfo, packed_orbit_data::PackedOrbitData, ConversionError, KPuzzle,
};

#[derive(Clone, Eq)]
pub struct KTransformation {
    pub(crate) packed_orbit_data: PackedOrbitData,
}

impl KTransformation {
    pub(crate) fn new_uninitialized(kpuzzle: KPuzzle) -> Self {
        Self {
            packed_orbit_data: unsafe { PackedOrbitData::new_with_uninitialized_bytes(kpuzzle) },
        }
    }

    // TODO: validation?
    pub fn try_from_data<T: Into<KPuzzle>>(
        kpuzzle: T,
        ktransformation_data: &KTransformationData,
    ) -> Result<Self, ConversionError> {
        let kpuzzle: KPuzzle = kpuzzle.into();
        let mut new_ktransformation = Self::new_uninitialized(kpuzzle.clone());
        for orbit_info in kpuzzle.orbit_info_iter() {
            for i in 0..orbit_info.num_pieces {
                let orbit = ktransformation_data.get(&orbit_info.name);
                let orbit = match orbit {
                    Some(default_orbit) => default_orbit,
                    None => panic!("Invalid default pattern"), // TODO: catch at construction time?
                };

                new_ktransformation.set_permutation_idx(
                    orbit_info,
                    i,
                    orbit.permutation[i as usize],
                );
                new_ktransformation.set_orientation_delta(
                    orbit_info,
                    i,
                    orbit.orientation_delta[i as usize],
                );
            }
        }
        Ok(new_ktransformation)
    }

    pub fn to_data(&self) -> KTransformationData {
        let mut data = KTransformationData::new();
        for orbit_info in self.kpuzzle().orbit_info_iter() {
            let mut permutation = Vec::with_capacity(orbit_info.num_pieces as usize);
            let mut orientation_delta = Vec::with_capacity(orbit_info.num_pieces as usize);

            for i in 0..orbit_info.num_pieces {
                permutation.push(self.get_permutation_idx(orbit_info, i));
                orientation_delta.push(self.get_orientation_delta(orbit_info, i));
            }
            data.insert(
                orbit_info.name.clone(),
                KTransformationOrbitData {
                    permutation,
                    orientation_delta,
                },
            );
        }
        data
    }

    pub fn try_from_json<T: Into<KPuzzle>>(
        kpuzzle: T,
        json_bytes: &[u8],
    ) -> Result<Self, ConversionError> {
        let kpuzzle: KPuzzle = kpuzzle.into();
        // TODO: implement this directly
        let ktransformation_data: KTransformationData = match serde_json::from_slice(json_bytes) {
            Ok(ktransformation_data) => ktransformation_data,
            Err(e) => {
                return Err(ConversionError::InvalidKPatternData(
                    super::InvalidKPatternDataError {
                        description: format!(
                            "Could not parse JSON for KTransformation data: {}",
                            e
                        ),
                    },
                ))
            }
        };
        Self::try_from_data(&kpuzzle, &ktransformation_data)
    }
    pub fn kpuzzle(&self) -> &KPuzzle {
        &self.packed_orbit_data.kpuzzle
    }

    /// # Safety
    /// `packed_orbit_data` implementation details are not a public API and implemented using `unsafe` themselves.
    pub unsafe fn packed_orbit_data(&mut self) -> &mut PackedOrbitData {
        &mut self.packed_orbit_data
    }

    pub fn get_permutation_idx(&self, orbit_info: &KPuzzleOrbitInfo, i: u8) -> u8 {
        assert_lt!(i, orbit_info.num_pieces);
        unsafe { self.get_permutation_idx_unchecked(orbit_info, i) }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    pub unsafe fn get_permutation_idx_unchecked(&self, orbit_info: &KPuzzleOrbitInfo, i: u8) -> u8 {
        // TODO: dedup with PackedKTransformation, or at least implement as a trait?
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit_info.pieces_or_permutations_offset, i)
                .read()
        }
    }

    pub fn get_orientation_delta(&self, orbit_info: &KPuzzleOrbitInfo, i: u8) -> u8 {
        assert_lt!(i, orbit_info.num_pieces);
        unsafe { self.get_orientation_delta_unchecked(orbit_info, i) }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    pub unsafe fn get_orientation_delta_unchecked(
        &self,
        orbit_info: &KPuzzleOrbitInfo,
        i: u8,
    ) -> u8 {
        // TODO: dedup with PackedKTransformation, or at least implement as a trait?
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit_info.orientations_offset, i)
                .read()
        }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    pub fn set_permutation_idx(&mut self, orbit_info: &KPuzzleOrbitInfo, i: u8, value: u8) {
        assert_lt!(i, orbit_info.num_pieces);
        unsafe { self.set_permutation_idx_unchecked(orbit_info, i, value) }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    pub unsafe fn set_permutation_idx_unchecked(
        &mut self,
        orbit_info: &KPuzzleOrbitInfo,
        i: u8,
        value: u8,
    ) {
        // TODO: dedup with PackedKTransformation, or at least implement as a trait?
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit_info.pieces_or_permutations_offset, i)
                .write(value)
        }
    }

    pub fn set_orientation_delta(
        &mut self,
        orbit_info: &KPuzzleOrbitInfo,
        i: u8,
        orientation_delta: u8,
    ) {
        assert_lt!(i, orbit_info.num_pieces);
        assert_lt!(orientation_delta, orbit_info.num_orientations);
        unsafe { self.set_orientation_delta_unchecked(orbit_info, i, orientation_delta) }
    }

    /// # Safety
    /// This version does not check whether `i` or `orientation_delta` are in the correct ranges.
    pub unsafe fn set_orientation_delta_unchecked(
        &mut self,
        orbit_info: &KPuzzleOrbitInfo,
        i: u8,
        orientation_delta: u8,
    ) {
        // TODO: dedup with PackedKTransformation, or at least implement as a trait?
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit_info.orientations_offset, i)
                .write(orientation_delta)
        }
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/transformation.rs#L32-L61
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    pub fn apply_transformation(&self, transformation: &KTransformation) -> KTransformation {
        let mut new_ktransformation = KTransformation::new_uninitialized(self.kpuzzle().clone());
        self.apply_transformation_into(transformation, &mut new_ktransformation);
        new_ktransformation
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/transformation.rs#L32-L61
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    // TODO: assign to self from another value, not into another
    pub fn apply_transformation_into(
        &self,
        transformation: &KTransformation,
        into_ktransformation: &mut KTransformation,
    ) {
        for orbit_info in self.kpuzzle().orbit_info_iter() {
            // TODO: optimization when either value is the identity.
            for i in 0..orbit_info.num_pieces {
                let transformation_idx =
                    unsafe { transformation.get_permutation_idx_unchecked(orbit_info, i) };

                let new_piece_permutation =
                    self.get_permutation_idx(orbit_info, transformation_idx);
                unsafe {
                    into_ktransformation.set_permutation_idx_unchecked(
                        orbit_info,
                        i,
                        new_piece_permutation,
                    )
                };

                let previous_orientation_delta =
                    unsafe { self.get_orientation_delta_unchecked(orbit_info, transformation_idx) };

                // TODO: lookup table?
                let new_orientation_delta = (previous_orientation_delta
                    + unsafe { transformation.get_orientation_delta_unchecked(orbit_info, i) })
                    % orbit_info.num_orientations;
                unsafe {
                    into_ktransformation.set_orientation_delta_unchecked(
                        orbit_info,
                        i,
                        new_orientation_delta,
                    )
                };
            }
        }
    }

    /// # Safety
    /// The internal structure of bytes is not yet stable.
    pub unsafe fn byte_slice(&self) -> &[u8] {
        self.packed_orbit_data.byte_slice()
    }

    pub fn invert(&self) -> KTransformation {
        let mut new_ktransformation = KTransformation::new_uninitialized(self.kpuzzle().clone());
        for orbit_info in self.kpuzzle().orbit_info_iter() {
            let num_orientations = orbit_info.num_orientations;

            // TODO: optimization when either value is the identity.
            for i in 0..orbit_info.num_pieces {
                let from_idx = self.get_permutation_idx(orbit_info, i);
                new_ktransformation.set_permutation_idx(orbit_info, from_idx, i);
                new_ktransformation.set_orientation_delta(
                    orbit_info,
                    from_idx,
                    (num_orientations - self.get_orientation_delta(orbit_info, i))
                        .rem_euclid(num_orientations),
                )
            }
        }
        new_ktransformation
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
            return self.kpuzzle().identity_transformation();
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
        write!(f, "{{ … name: \"{}\" … }}", &self.kpuzzle.definition().name)
    }
}

impl Debug for KTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedKTransformation")
            .field(
                "kpuzzle",
                &KPuzzleDebug {
                    kpuzzle: self.kpuzzle().clone(),
                },
            )
            .field("bytes", &unsafe { self.byte_slice() })
            .finish()
    }
}

impl PartialEq<KTransformation> for KTransformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.byte_slice() == other.byte_slice() }
    }
}

#[cfg(test)]
mod tests {
    use crate::alg::AlgParseError;
    use crate::alg::Move;
    use crate::kpuzzle::InvalidAlgError;
    use crate::kpuzzle::KTransformation;
    use crate::puzzles::cube3x3x3_kpuzzle;

    #[test]
    fn compose() -> Result<(), String> {
        let kpuzzle = cube3x3x3_kpuzzle();

        let from_move = |move_str: &str| -> Result<KTransformation, String> {
            let r#move = (move_str)
                .parse::<Move>()
                .map_err(|e: AlgParseError| e.description)?;
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
