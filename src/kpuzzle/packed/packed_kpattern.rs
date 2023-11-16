use std::{fmt::Debug, hash::Hash};

use super::{
    byte_conversions::{u8_to_usize, PackedOrientationWithMod, usize_to_u8},
    packed_kpuzzle::{PackedKPuzzleOrbitInfo, InvalidAlgError},
    packed_orbit_data::PackedOrbitData,
    ConversionError, PackedKPuzzle, PackedKTransformation, orientation_packer::OrientationWithMod, InvalidPatternDataError,
};

use crate::{alg::{Alg, Move}, kpuzzle::KPatternData};


#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PackedKPattern {
    pub packed_orbit_data: PackedOrbitData,
}

impl PackedKPattern {
    pub(crate) fn new_unitialized(packed_kpuzzle: PackedKPuzzle) -> Self {
        Self {
            packed_orbit_data: PackedOrbitData::new_with_uninitialized_bytes(packed_kpuzzle),
        }
    }

    // TODO: validation?
    pub fn try_from_data(
        packed_kpuzzle: &PackedKPuzzle,
        kpattern_data: &KPatternData,
    ) -> Result<Self, ConversionError> {
        let mut new_packed_kpattern = Self::new_unitialized(packed_kpuzzle.clone());
        for orbit_info in &packed_kpuzzle.data.orbit_iteration_info {
            for i in 0..orbit_info.num_pieces {
                let default_orbit = kpattern_data.get(&orbit_info.name);
                let default_orbit = match default_orbit {
                    Some(default_orbit) => default_orbit,
                    None => panic!("Invalid default pattern"), // TODO: catch at construction time?
                };

                new_packed_kpattern.set_piece(
                    orbit_info,
                    i,
                    usize_to_u8(default_orbit.pieces[i]),
                );
                new_packed_kpattern.set_packed_orientation_with_mod(
                    orbit_info,
                    i,
                    match &default_orbit.orientation_mod {
                        None => usize_to_u8(default_orbit.orientation[i]),
                        Some(orientation_mod) => {
                                if orientation_mod[i] != 0 && u8_to_usize(orbit_info.num_orientations) % orientation_mod[i] != 0 {
                                    return Err(ConversionError::InvalidPatternData(InvalidPatternDataError { description: format!(
                                        "`orientation_mod` of {} seen for piece at index {} in orbit {} in the start pattern for puzzle {}. This must be a factor of `num_orientations` for the orbit ({}). See: https://js.cubing.net/cubing/api/interfaces/kpuzzle.KPatternOrbitData.html#orientationMod",
                                        orientation_mod[i],
                                        i,
                                        orbit_info.name,
                                        packed_kpuzzle.data.definition.name,
                                        orbit_info.num_orientations
                                    )}));
                                };
                                orbit_info.orientation_packer.pack(&OrientationWithMod {
                                    orientation: default_orbit.orientation[i],
                                    orientation_mod: orientation_mod[i],
                                })
                            
                        }
                    },
                );
            }
        };
        Ok(new_packed_kpattern)
    }

    pub fn try_from_json(
        packed_kpuzzle: &PackedKPuzzle,
        json_bytes: &[u8],
    ) -> Result<Self, ConversionError> {
        // TODO: implement this directly
        let kpattern_data: KPatternData = match serde_json::from_slice(json_bytes) {
            Ok(kpattern_data) => kpattern_data,
            Err(e) => {
                return Err(ConversionError::InvalidPatternData(
                    super::InvalidPatternDataError {
                        description: format!("Could not parse JSON for KPattern data: {}", e),
                    },
                ))
            }
        };
        Self::try_from_data(packed_kpuzzle, &kpattern_data)
    }

    pub fn get_piece(&self, orbit_info: &PackedKPuzzleOrbitInfo, i: usize) -> u8 {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.pieces_or_pemutations_offset + i)
                .read()
        }
    }

    pub fn get_orientation_with_mod<'a>(
        &self,
        orbit_info: &'a PackedKPuzzleOrbitInfo,
        i: usize,
    ) -> &'a OrientationWithMod {
        let packed_orientation_with_mod = &self.get_packed_orientation_with_mod(orbit_info, i);
        orbit_info.orientation_packer.unpack(packed_orientation_with_mod)
    }

    pub(crate) fn get_packed_orientation_with_mod(
        &self,
        orbit_info: &PackedKPuzzleOrbitInfo,
        i: usize,
    ) -> PackedOrientationWithMod {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .read()
        }
    }

    pub fn set_piece(
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

    pub fn set_orientation_with_mod(
        &mut self,
        orbit_info: &PackedKPuzzleOrbitInfo,
        i: usize,
        orientation_with_mod: &OrientationWithMod,
    ) {
        let packed_orientation_with_mod = orbit_info.orientation_packer.pack(orientation_with_mod);
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .write(packed_orientation_with_mod)
        }
    }

    pub(crate) fn set_packed_orientation_with_mod(
        &mut self,
        orbit_info: &PackedKPuzzleOrbitInfo,
        i: usize,
        value: PackedOrientationWithMod,
    ) {
        unsafe {
            self.packed_orbit_data
                .bytes
                .add(orbit_info.orientations_offset + i)
                .write(value)
        }
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/pattern.rs#L31-L82
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    pub fn apply_transformation(&self, transformation: &PackedKTransformation) -> PackedKPattern {
        let mut new_packed_kpattern =
            PackedKPattern::new_unitialized(self.packed_orbit_data.packed_kpuzzle.clone());
        self.apply_transformation_into(transformation, &mut new_packed_kpattern);
        new_packed_kpattern
    }

    // Adapted from https://github.com/cubing/cubing.rs/blob/b737c6a36528e9984b45b29f9449a9a330c272fb/src/kpuzzle/pattern.rs#L31-L82
    // TODO: dedup the implementation (but avoid runtime overhead for the shared abstraction).
    // TODO: assign to self from another value, not into another
    pub fn apply_transformation_into(
        &self,
        transformation: &PackedKTransformation,
        into_packed_kpattern: &mut PackedKPattern,
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

                let new_piece_value =
                    self.get_piece(orbit_info, u8_to_usize(transformation_idx));
                into_packed_kpattern.set_piece(orbit_info, i, new_piece_value);

                let previous_packed_orientation_with_mod =
                    self.get_packed_orientation_with_mod(orbit_info, u8_to_usize(transformation_idx));

                let new_packed_orientation_with_mod = {
                    orbit_info.orientation_packer.transform(
                        previous_packed_orientation_with_mod,
                        u8_to_usize(transformation.get_orientation_delta(orbit_info, i)),
                    )
                };
                into_packed_kpattern.set_packed_orientation_with_mod(orbit_info, i, new_packed_orientation_with_mod);
            }
        }
    }

    pub fn apply_alg(&self, alg: &Alg) -> Result<PackedKPattern, InvalidAlgError> {
        let transformation = self.packed_orbit_data.packed_kpuzzle.transformation_from_alg(alg)?;
        Ok(self.apply_transformation(&transformation))
    }

    pub fn apply_move(&self, m: &Move) -> Result<PackedKPattern, InvalidAlgError> {
        let transformation = self.packed_orbit_data.packed_kpuzzle.transformation_from_move(m)?;
        Ok(self.apply_transformation(&transformation))
    }

    pub fn byte_slice(&self) -> &[u8] {
        self.packed_orbit_data.byte_slice()
    }

    pub fn hash(&self) -> u64 {
        self.packed_orbit_data.hash()
    }
}

struct KPuzzleDebug {
    kpuzzle: PackedKPuzzle,
}

impl Debug for KPuzzleDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ … name: \"{}\" … }}", &self.kpuzzle.data.definition.name)
    }
}

impl Debug for PackedKPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedKPattern")
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

#[cfg(test)]
mod tests {
    

    use crate::alg::AlgParseError;
    use crate::kpuzzle::{PackedKTransformation, KPatternData, PackedKPattern};
    use crate::kpuzzle::packed::packed_kpuzzle::InvalidAlgError;
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
        let start_pattern = PackedKPattern::try_from_data(&kpuzzle, &start_pattern_data).unwrap();


        let t1 = from_move("R")?;

        assert_eq!(
            start_pattern.apply_transformation(&t1).byte_slice(),
            vec![
                /* EP */ 0, 0, 0, 0, 1, 0, 3, 4, 2, 0, 0, 0, /* EO */ 1, 1, 1, 1, 0, 1,
                0, 0, 0, 1, 1, 1, /* CP */ 0, 0, 0, 0, 0, 0, 0, 0, /* CO */ 0, 2, 1, 1,
                2, 1, 1, 0, /* MP */ 0, 1, 2, 3, 4, 5, /* MO */ 4, 4, 4, 4, 4, 4
            ]
        );
        assert_eq!(
            start_pattern.apply_transformation(&t1).byte_slice(),
            start_pattern.apply_move(&parse_move!("R").unwrap()).unwrap().byte_slice()
        );

        Ok(())
    }
}

pub struct PackedKPatternBuffer {
    a: PackedKPattern,
    b: PackedKPattern,
    // In some rough benchmarks, using a boolean to track the current pattern was just a tad faster than using `std::mem::swap(…)`.
    // TODO: measure this properly across devices, and updated `PackedKTransformationBuffer` to match.
    a_is_current: bool,
}

impl From<PackedKPattern> for PackedKPatternBuffer {
    fn from(initial: PackedKPattern) -> Self {
        Self {
            b: initial.clone(), // TODO?
            a: initial,
            a_is_current: true,
        }
    }
}

impl PackedKPatternBuffer {
    pub fn apply_transformation(&mut self, transformation: &PackedKTransformation) {
        if self.a_is_current {
            self.a
                .apply_transformation_into(transformation, &mut self.b);
        } else {
            self.b
                .apply_transformation_into(transformation, &mut self.a);
        }
        self.a_is_current = !self.a_is_current
    }

    pub fn current(&self) -> &PackedKPattern {
        if self.a_is_current {
            &self.a
        } else {
            &self.b
        }
    }
}

impl PartialEq for PackedKPatternBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.current() == other.current()
    }
}
