use std::{fmt::Debug, hash::Hash};

use more_asserts::assert_lt;

use super::{
    kpuzzle::{InvalidAlgError, KPuzzleOrbitInfo},
    orientation_packer::{OrientationWithMod, PackedOrientationWithMod},
    packed_orbit_data::PackedOrbitData,
    ConversionError, InvalidKPatternDataError, KPuzzle, KTransformation,
};

use crate::{
    alg::{Alg, Move},
    kpuzzle::{KPatternData, KPatternOrbitData},
};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct KPattern {
    pub(crate) packed_orbit_data: PackedOrbitData,
}

impl KPattern {
    pub(crate) fn new_unitialized<T: Into<KPuzzle>>(kpuzzle: T) -> Self {
        let kpuzzle: KPuzzle = kpuzzle.into();
        Self {
            packed_orbit_data: unsafe { PackedOrbitData::new_with_uninitialized_bytes(kpuzzle) },
        }
    }

    // TODO: validation?
    pub fn try_from_data<T: Into<KPuzzle>>(
        kpuzzle: T,
        kpattern_data: &KPatternData,
    ) -> Result<Self, ConversionError> {
        let kpuzzle: KPuzzle = kpuzzle.into();
        let mut new_kpattern = Self::new_unitialized(&kpuzzle);
        for orbit_info in kpuzzle.orbit_info_iter() {
            for i in 0..orbit_info.num_pieces {
                let default_orbit = kpattern_data.get(&orbit_info.name);
                let default_orbit = match default_orbit {
                    Some(default_orbit) => default_orbit,
                    None => panic!("Invalid default pattern"), // TODO: catch at construction time?
                };

                new_kpattern.set_piece(orbit_info, i, default_orbit.pieces[i as usize]);
                new_kpattern.set_orientation_with_mod(
                    orbit_info,
                    i,
                    &match &default_orbit.orientation_mod {
                        None => OrientationWithMod::new_using_default_orientation_mod( default_orbit.orientation[i as usize]),
                        Some(orientation_mod) => {
                            if orientation_mod[i as usize] != 0 && orbit_info.num_orientations % orientation_mod[i as usize] != 0 {
                                return Err(ConversionError::InvalidKPatternData(InvalidKPatternDataError { description: format!(
                                    "`orientation_mod` of {} seen for piece at index {} in orbit {} in the start pattern for puzzle {}. This must be a factor of `num_orientations` for the orbit ({}). See: https://js.cubing.net/cubing/api/interfaces/kpuzzle.KPatternOrbitData.html#orientationMod",
                                    orientation_mod[i as usize],
                                    i,
                                    orbit_info.name,
                                    kpuzzle.definition().name,
                                    orbit_info.num_orientations
                                )}));
                            };
                            OrientationWithMod {
                                orientation: default_orbit.orientation[i as usize],
                                orientation_mod: orientation_mod[i as usize],
                            }
                        }
                    },
                );
            }
        }
        Ok(new_kpattern)
    }

    pub fn to_data(&self) -> KPatternData {
        let mut data = KPatternData::new();
        for orbit_info in self.kpuzzle().orbit_info_iter() {
            let mut pieces = Vec::with_capacity(orbit_info.num_pieces as usize);
            let mut orientation = Vec::with_capacity(orbit_info.num_pieces as usize);
            let mut orientation_mod = Vec::with_capacity(orbit_info.num_pieces as usize);

            for i in 0..orbit_info.num_pieces {
                pieces.push(self.get_piece(orbit_info, i));

                let orientation_with_mod = self.get_orientation_with_mod(orbit_info, i);
                orientation.push(orientation_with_mod.orientation);
                orientation_mod.push(orientation_with_mod.orientation_mod);
            }
            let orientation_mod = Some(orientation_mod);
            data.insert(
                orbit_info.name.clone(),
                KPatternOrbitData {
                    pieces,
                    orientation,
                    orientation_mod,
                },
            );
        }
        data
    }

    pub fn kpuzzle(&self) -> &KPuzzle {
        &self.packed_orbit_data.kpuzzle
    }
    /// # Safety
    /// `packed_orbit_data` implementation details are not a public API and implemented using `unsafe` themselves.
    pub unsafe fn packed_orbit_data(&self) -> &PackedOrbitData {
        &self.packed_orbit_data
    }

    /// # Safety
    /// `packed_orbit_data` implementation details are not a public API and implemented using `unsafe` themselves.
    pub unsafe fn packed_orbit_data_mut(&mut self) -> &mut PackedOrbitData {
        &mut self.packed_orbit_data
    }

    pub fn try_from_json<T: Into<KPuzzle>>(
        kpuzzle: T,
        json_bytes: &[u8],
    ) -> Result<Self, ConversionError> {
        let kpuzzle: KPuzzle = kpuzzle.into();
        // TODO: implement this directly
        let kpattern_data: KPatternData = match serde_json::from_slice(json_bytes) {
            Ok(kpattern_data) => kpattern_data,
            Err(e) => {
                return Err(ConversionError::InvalidKPatternData(
                    super::InvalidKPatternDataError {
                        description: format!("Could not parse JSON for KPattern data: {}", e),
                    },
                ))
            }
        };
        Self::try_from_data(&kpuzzle, &kpattern_data)
    }

    pub fn get_piece(&self, orbit: &KPuzzleOrbitInfo, i: u8) -> u8 {
        assert_lt!(i, orbit.num_pieces);
        unsafe { self.get_piece_unchecked(orbit, i) }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    unsafe fn get_piece_unchecked(&self, orbit: &KPuzzleOrbitInfo, i: u8) -> u8 {
        self.packed_orbit_data
            .bytes_offset(orbit.pieces_or_permutations_offset, i)
            .read()
    }

    pub fn get_orientation_with_mod<'a>(
        &self,
        orbit: &'a KPuzzleOrbitInfo,
        i: u8,
    ) -> &'a OrientationWithMod {
        assert_lt!(i, orbit.num_pieces);
        unsafe { self.get_orientation_with_mod_unchecked(orbit, i) }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    pub unsafe fn get_orientation_with_mod_unchecked<'a>(
        &self,
        orbit: &'a KPuzzleOrbitInfo,
        i: u8,
    ) -> &'a OrientationWithMod {
        let packed_orientation_with_mod = &self.get_packed_orientation_with_mod_unchecked(orbit, i);
        orbit.orientation_packer.unpack(packed_orientation_with_mod)
    }

    fn get_packed_orientation_with_mod_unchecked(
        &self,
        orbit: &KPuzzleOrbitInfo,
        i: u8,
    ) -> PackedOrientationWithMod {
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit.orientations_offset, i)
                .read()
        }
    }

    pub fn set_piece(&mut self, orbit: &KPuzzleOrbitInfo, i: u8, value: u8) {
        assert_lt!(i, orbit.num_pieces);
        unsafe { self.set_piece_unchecked(orbit, i, value) }
    }

    /// # Safety
    /// This version does not check whether `i` is in the correct range.
    pub unsafe fn set_piece_unchecked(&mut self, orbit: &KPuzzleOrbitInfo, i: u8, value: u8) {
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit.pieces_or_permutations_offset, i)
                .write(value)
        }
    }

    pub fn set_orientation_with_mod(
        &mut self,
        orbit: &KPuzzleOrbitInfo,
        i: u8,
        orientation_with_mod: &OrientationWithMod,
    ) {
        assert_lt!(i, orbit.num_pieces);
        if orientation_with_mod.orientation_mod == 0 {
            assert_lt!(orientation_with_mod.orientation, orbit.num_orientations)
        } else {
            assert_lt!(
                orientation_with_mod.orientation,
                orientation_with_mod.orientation_mod
            );
            assert_eq!(
                orbit.num_orientations % orientation_with_mod.orientation_mod,
                0
            );
        }
        unsafe { self.set_orientation_with_mod_unchecked(orbit, i, orientation_with_mod) }
    }

    /// # Safety
    /// This version does not check whether `i` or `orientation_with_mod` are in the correct ranges.
    pub unsafe fn set_orientation_with_mod_unchecked(
        &mut self,
        orbit: &KPuzzleOrbitInfo,
        i: u8,
        orientation_with_mod: &OrientationWithMod,
    ) {
        let packed_orientation_with_mod = orbit.orientation_packer.pack(orientation_with_mod);
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit.orientations_offset, i)
                .write(packed_orientation_with_mod)
        }
    }

    pub(crate) fn set_packed_orientation_with_mod_unchecked(
        &mut self,
        orbit: &KPuzzleOrbitInfo,
        i: u8,
        value: PackedOrientationWithMod,
    ) {
        unsafe {
            self.packed_orbit_data
                .bytes_offset(orbit.orientations_offset, i)
                .write(value)
        }
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/pattern.rs#L31-L82
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    pub fn apply_transformation(&self, transformation: &KTransformation) -> KPattern {
        let mut new_kpattern = KPattern::new_unitialized(self.kpuzzle().clone());
        self.apply_transformation_into(transformation, &mut new_kpattern);
        new_kpattern
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/pattern.rs#L31-L82
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    // TODO: assign to self from another value, not into another
    pub fn apply_transformation_into(
        &self,
        transformation: &KTransformation,
        into_kpattern: &mut KPattern,
    ) {
        for orbit_info in self.kpuzzle().orbit_info_iter() {
            // TODO: optimization when either value is the identity.
            for i in 0..orbit_info.num_pieces {
                let transformation_idx =
                    unsafe { transformation.get_permutation_idx_unchecked(orbit_info, i) };

                let new_piece_value =
                    unsafe { self.get_piece_unchecked(orbit_info, transformation_idx) };
                unsafe { into_kpattern.set_piece_unchecked(orbit_info, i, new_piece_value) };

                let previous_packed_orientation_with_mod =
                    self.get_packed_orientation_with_mod_unchecked(orbit_info, transformation_idx);

                let new_packed_orientation_with_mod = {
                    orbit_info
                        .orientation_packer
                        .transform(previous_packed_orientation_with_mod, unsafe {
                            transformation.get_orientation_delta_unchecked(orbit_info, i)
                        })
                };
                into_kpattern.set_packed_orientation_with_mod_unchecked(
                    orbit_info,
                    i,
                    new_packed_orientation_with_mod,
                );
            }
        }
    }

    pub fn apply_alg(&self, alg: &Alg) -> Result<KPattern, InvalidAlgError> {
        let transformation = self
            .packed_orbit_data
            .kpuzzle
            .transformation_from_alg(alg)?;
        Ok(self.apply_transformation(&transformation))
    }

    pub fn apply_move(&self, m: &Move) -> Result<KPattern, InvalidAlgError> {
        let transformation = self.kpuzzle().transformation_from_move(m)?;
        Ok(self.apply_transformation(&transformation))
    }

    /// # Safety
    /// The internal structure of bytes is not yet stable.
    pub unsafe fn byte_slice(&self) -> &[u8] {
        self.packed_orbit_data.byte_slice()
    }
}

impl From<&KPattern> for KPattern {
    fn from(value: &KPattern) -> Self {
        value.clone()
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

impl Debug for KPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KPattern")
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

#[cfg(test)]
mod tests {

    use crate::alg::{AlgParseError, Move};
    use crate::kpuzzle::packed::kpuzzle::InvalidAlgError;
    use crate::kpuzzle::{KPattern, KPatternData, KTransformation};
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

        let start_pattern_data: KPatternData = serde_json::from_str(
            /* Cross */
            r#"
{
    "EDGES": {
        "pieces": [0, 0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 0],
        "orientation": [1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1]
    },
    "CORNERS": {
        "pieces": [0, 0, 0, 0, 0, 0, 0, 0],
        "orientation": [1, 1, 1, 1, 1, 1, 1, 1]
    },
    "CENTERS": {
        "pieces": [0, 1, 2, 3, 4, 5],
        "orientation": [0, 0, 0, 0, 0, 0],
        "orientationMod": [1, 1, 1, 1, 1, 1]
    }
}"#,
        )
        .unwrap();
        let start_pattern = KPattern::try_from_data(kpuzzle, &start_pattern_data).unwrap();

        let t1 = from_move("R")?;

        assert_eq!(
            unsafe { start_pattern.apply_transformation(&t1).byte_slice() },
            vec![
                /* EP */ 0, 0, 0, 0, 1, 0, 3, 4, 2, 0, 0, 0, /* EO */ 1, 1, 1, 1, 0, 1,
                0, 0, 0, 1, 1, 1, /* CP */ 0, 0, 0, 0, 0, 0, 0, 0, /* CO */ 0, 2, 1, 1,
                2, 1, 1, 0, /* MP */ 0, 1, 2, 3, 4, 5, /* MO */ 4, 4, 4, 4, 4, 4
            ]
        );
        assert_eq!(
            unsafe { start_pattern.apply_transformation(&t1).byte_slice() },
            unsafe {
                start_pattern
                    .apply_move(&("R").parse::<Move>().unwrap())
                    .unwrap()
                    .byte_slice()
            }
        );

        Ok(())
    }
}

pub struct KPatternBuffer {
    a: KPattern,
    b: KPattern,
    // In some rough benchmarks, using a boolean to track the current pattern was just a tad faster than using `std::mem::swap(…)`.
    // TODO: measure this properly across devices, and updated `PackedKTransformationBuffer` to match.
    a_is_current: bool,
}

impl From<KPattern> for KPatternBuffer {
    fn from(initial: KPattern) -> Self {
        Self {
            b: initial.clone(), // TODO?
            a: initial,
            a_is_current: true,
        }
    }
}

impl KPatternBuffer {
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

    pub fn current(&self) -> &KPattern {
        if self.a_is_current {
            &self.a
        } else {
            &self.b
        }
    }
}

impl PartialEq for KPatternBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.current() == other.current()
    }
}
