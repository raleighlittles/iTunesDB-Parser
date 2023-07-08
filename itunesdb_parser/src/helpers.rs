use core::slice;
use std::array;

/**
 * File: helpers.rs
 *
 */

// TODO: Once Rust adds support for default arguments, add the following arguments:
//       * endianness
//       * radix
pub fn build_le_u32_from_bytes(bytes: &[u8]) -> u32 {
    let mut number: u32 = 0;
    const RADIX: u32 = 256;

    for (idx, item) in bytes.iter().enumerate() {
        let summand: u32 = RADIX
            .checked_pow(idx as u32)
            .unwrap_or_else(|| panic!("Can't raise {} to power {}", RADIX, idx))
            as u32;

        number += (summand as u32) * (*item as u32);
    }

    return number;
}

pub fn get_slice_from_offset_with_len(array_idx : usize, file_as_array: &[u8], file_offset : usize, slice_len : usize) -> Vec<u8> {

    return file_as_array[array_idx + file_offset .. array_idx + file_offset + slice_len].to_vec();
}

pub fn get_slice_as_le_u32(array_idx : usize, file_as_array: &[u8], file_offset : usize, slice_len : usize) -> u32 {

    if slice_len > 4 {
        panic!("Can't create u32 out of this large of a slice");
    }

    return build_le_u32_from_bytes(&get_slice_from_offset_with_len(array_idx, file_as_array, file_offset, slice_len));
}

/// // Build UTF-16 array, out of UTF-8, by combining elements pairwise
pub fn return_utf16_from_utf8(utf8_bytes : &[u8]) -> Vec<u16> {
    
    let mut arr_elements_pairwise_combined: Vec<u16> = vec![];

    for i in (0 .. utf8_bytes.len()).step_by(2)
    {
        let u16_elem: u16 = ((utf8_bytes[i + 1] as u16) << 8)
            | utf8_bytes[i] as u16;
            arr_elements_pairwise_combined.push(u16_elem);
    }

    return arr_elements_pairwise_combined;
}

// TODO: Add function to take in a raw number of bytes, and print it as either KB, or MB, depending on whichever is appropriate (leaving it between 1-10)

// TODO add function for converting time in seconds, to time in minutes and seconds
