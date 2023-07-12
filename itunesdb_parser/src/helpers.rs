
/**
 * File: helpers.rs
 *
 */
use std::str;

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

// TODO: Use template function
pub fn build_le_u64_from_bytes(bytes: &[u8]) -> u64 {

    let mut number: u64 = 0;
    const RADIX: u64 = 256;

    for (idx, item) in bytes.iter().enumerate() {
        let summand: u64 = RADIX
            .checked_pow(idx as u32)
            .unwrap_or_else(|| panic!("Can't raise {} to power {}", RADIX, idx));

        number += (summand) * (*item as u64);
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

pub fn get_slice_as_le_u64(array_idx : usize, file_as_array: &[u8], file_offset : usize, slice_len : usize) -> u64 {

    if slice_len > 8 {
        panic!("Can't create u64 out of this large of a slice");
    }

    return build_le_u64_from_bytes(&get_slice_from_offset_with_len(array_idx, file_as_array, file_offset, slice_len));
}

pub fn get_slice_as_mac_timestamp(array_idx : usize, file_as_array : &[u8], file_offset : usize, slice_len : usize) -> chrono::DateTime<chrono::Utc> {


    let epoch_time : u64 = get_slice_as_le_u32(array_idx, file_as_array, file_offset, slice_len) as u64;

    if epoch_time == 0 {
        //panic!("Error! Epoch time converted was 0. Check the slice starting at idx {} with len {}, actually contains a valid timestamp", array_idx, slice_len);
        eprintln!("Error! Epoch time converted was 0. Check the slice starting at idx {} with len {}, actually contains a valid timestamp", array_idx, slice_len);
    }

    return get_timestamp_as_mac(epoch_time);
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

/// Apple (Mac OS X, iOS, iPadOS, et al) timestamps start on Jan 1 1904, whereas Linux timestamps
/// (which is what Rust's `chrono` library uses) start at Jan 1 1970,
/// hence this difference
const MAC_TO_LINUX_EPOCH_CONVERSION: i64 = 2082844800;

/// Converts a given Mac epoch time into an actual UTC timestamp
pub fn get_timestamp_as_mac(mac_timestamp : u64) -> chrono::DateTime<chrono::Utc> {

    return chrono::DateTime::<chrono::Utc>::from_utc( chrono::NaiveDateTime::from_timestamp_opt((mac_timestamp as i64) - MAC_TO_LINUX_EPOCH_CONVERSION, 0).unwrap(), chrono::offset::Utc);
}


// This doesn't seem to be explicitly mentioned in the iTunesDB wiki,
// but the iTunesDB files use colons instead of forward slashes for directories
// e.g. "E::DCIM:129CANON:IMG_2470.JPG" actually represents "E::DCIM/129CANON/IMG_2470.jpg"
// The first set of double colons is the disk letter (in this case 'E'), like in Windows,
// so we skip the drive letter and the double colons, only printing the actual path
// (e.g. starting with 'DCIM/...')
pub fn get_canonical_path(itunesdb_format_path : String) -> String {

    return str::replace(&itunesdb_format_path[3..], ":", "/");
}


// TODO add function for converting time in seconds, to time in minutes and seconds
