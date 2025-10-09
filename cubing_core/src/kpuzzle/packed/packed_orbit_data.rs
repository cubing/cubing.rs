use std::{
    alloc::{alloc, dealloc},
    fmt::Debug,
    hash::Hash,
};

use more_asserts::debug_assert_le;

use super::{kpuzzle::KPuzzleOrbitInfo, KPuzzle};

pub struct PackedOrbitData {
    /// Use `.kpuzzle()` directly on `KPattern` or `KTransformation` instead, when possible.
    pub(crate) kpuzzle: KPuzzle,
    pub bytes: *mut u8,
}

impl Drop for PackedOrbitData {
    fn drop(&mut self) {
        unsafe { dealloc(self.bytes, self.kpuzzle.data.layout) }
    }
}

impl PackedOrbitData {
    pub fn kpuzzle(&self) -> &KPuzzle {
        &self.kpuzzle
    }

    pub(crate) unsafe fn new_with_uninitialized_bytes(kpuzzle: KPuzzle) -> Self {
        let bytes = unsafe { alloc(kpuzzle.data.layout) };
        Self { kpuzzle, bytes }
    }

    pub(crate) unsafe fn bytes_offset(&self, main_offset: usize, second_offset: u8) -> *mut u8 {
        self.bytes.add(main_offset + (second_offset as usize))
    }

    pub unsafe fn get_raw_piece_or_permutation_value(&self, orbit: &KPuzzleOrbitInfo, i: u8) -> u8 {
        unsafe {
            self.bytes_offset(orbit.pieces_or_permutations_offset, i)
                .read()
        }
    }

    /// Note: to get orientation with mod, call functions on `PackedKPattern` instead.
    pub unsafe fn get_raw_orientation_value(&self, orbit: &KPuzzleOrbitInfo, i: u8) -> u8 {
        unsafe { self.bytes_offset(orbit.orientations_offset, i).read() }
    }

    pub unsafe fn set_raw_piece_or_permutation_value(
        &mut self,
        orbit: &KPuzzleOrbitInfo,
        i: u8,
        value: u8,
    ) {
        unsafe {
            self.bytes_offset(orbit.pieces_or_permutations_offset, i)
                .write(value)
        }
    }

    /// Note: to set orientation with mod, call functions on `KPattern` instead.
    pub unsafe fn set_raw_orientation_value(&mut self, orbit: &KPuzzleOrbitInfo, i: u8, value: u8) {
        unsafe { self.bytes_offset(orbit.orientations_offset, i).write(value) }
    }

    pub unsafe fn byte_slice(&self) -> &[u8] {
        self.byte_slice_offset(0, self.kpuzzle.data.num_bytes)
    }

    pub unsafe fn byte_slice_mut(&mut self) -> &mut [u8] {
        self.byte_slice_offset_mut(0, self.kpuzzle.data.num_bytes)
    }

    pub(crate) unsafe fn byte_slice_offset(&self, offset: usize, len: usize) -> &[u8] {
        debug_assert_le!(offset, self.kpuzzle.data.num_bytes);
        debug_assert_le!(offset + len, self.kpuzzle.data.num_bytes);
        // yiss ☺️
        // https://stackoverflow.com/a/27150865
        unsafe { std::slice::from_raw_parts(self.bytes.add(offset), len) }
    }

    pub(crate) unsafe fn byte_slice_offset_mut(&mut self, offset: usize, len: usize) -> &mut [u8] {
        debug_assert_le!(offset, self.kpuzzle.data.num_bytes);
        debug_assert_le!(offset + len, self.kpuzzle.data.num_bytes);
        // yiss ☺️
        // https://stackoverflow.com/a/27150865
        unsafe { std::slice::from_raw_parts_mut(self.bytes.add(offset), len) }
    }
}

impl Debug for PackedOrbitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedOrbitData")
            .field("kpuzzle", &self.kpuzzle)
            .field("bytes", &unsafe { self.byte_slice() })
            .finish()
    }
}

impl Hash for PackedOrbitData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.byte_slice() }.hash(state); // TODO: would hashing the kpuzzle significantly affect performance?
    }
}

impl PartialEq for PackedOrbitData {
    fn eq(&self, other: &Self) -> bool {
        // TODO: would comparing the kpuzzles significantly affect performance?
        unsafe { self.byte_slice() == other.byte_slice() }
    }
}

impl Eq for PackedOrbitData {}

impl Clone for PackedOrbitData {
    fn clone(&self) -> Self {
        let new_packed_orbit_data =
            unsafe { PackedOrbitData::new_with_uninitialized_bytes(self.kpuzzle.clone()) };
        unsafe {
            std::ptr::copy(
                self.bytes,
                new_packed_orbit_data.bytes,
                self.kpuzzle.data.num_bytes,
            )
        };
        new_packed_orbit_data
    }
}

// TODO
unsafe impl Send for PackedOrbitData {}
unsafe impl Sync for PackedOrbitData {}
