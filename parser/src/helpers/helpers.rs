/**
 * File: helpers.rs
 *
 * General helper methods. For iTunes-specific helper methods, see itunesdb_helpers.rs
 *
 */
use std::fmt::Write;

// TODO: Once Rust adds support for default arguments, add the following arguments:
//       * endianness
//       * radix
pub fn build_le_u16_from_bytes(bytes: &[u8]) -> u16 {
    let mut number: u16 = 0;
    const RADIX: u16 = 256;

    for (idx, item) in bytes.iter().enumerate() {
        let summand: u16 = RADIX
            .checked_pow(idx as u32)
            .unwrap_or_else(|| panic!("Can't raise {} to power {}", RADIX, idx))
            as u16;

        number += (summand as u16) * (*item as u16);
    }

    return number;
}

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

pub fn build_be_u32_from_bytes(bytes: &[u8]) -> u32 {
    let mut number: u32 = 0;
    const RADIX: u32 = 256;

    for (idx, item) in bytes.iter().rev().enumerate() {
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
    let mut array = [0u8; 8];
    let len = bytes.len().min(8);

    array[..len].copy_from_slice(&bytes[..len]);

    u64::from_le_bytes(array)
}

pub fn get_slice_from_offset_with_len(
    array_idx: usize,
    file_as_array: &[u8],
    file_offset: usize,
    slice_len: usize,
) -> Vec<u8> {
    return file_as_array[array_idx + file_offset..array_idx + file_offset + slice_len].to_vec();
}

pub fn get_slice_as_le_u32(
    array_idx: usize,
    file_as_array: &[u8],
    file_offset: usize,
    slice_len: usize,
) -> u32 {
    if slice_len > 4 {
        panic!("Can't create u32 out of this large of a slice");
    }

    return build_le_u32_from_bytes(&get_slice_from_offset_with_len(
        array_idx,
        file_as_array,
        file_offset,
        slice_len,
    ));
}

pub fn get_slice_as_le_u64(
    array_idx: usize,
    file_as_array: &[u8],
    file_offset: usize,
    slice_len: usize,
) -> u64 {
    if slice_len > 8 {
        panic!("Can't create u64 out of this large of a slice");
    }

    return build_le_u64_from_bytes(&get_slice_from_offset_with_len(
        array_idx,
        file_as_array,
        file_offset,
        slice_len,
    ));
}

pub fn get_slice_as_mac_timestamp(
    array_idx: usize,
    file_as_array: &[u8],
    file_offset: usize,
    slice_len: usize,
) -> chrono::DateTime<chrono::Utc> {
    let epoch_time: u64 =
        get_slice_as_le_u32(array_idx, file_as_array, file_offset, slice_len) as u64;

    if epoch_time == 0 {
        println!("Warning: Epoch time converted was 0 (slice at idx {} with len {}). This usually means the timestamp was not set in the iTunesDB file.", array_idx, slice_len);
    }

    return get_timestamp_as_mac(epoch_time);
}

/// // Build UTF-16 array, out of UTF-8, by combining elements pairwise
pub fn return_utf16_from_utf8(utf8_bytes: &[u8]) -> Vec<u16> {
    let mut arr_elements_pairwise_combined: Vec<u16> = vec![];

    for i in (0..utf8_bytes.len()).step_by(2) {
        let u16_elem: u16 = ((utf8_bytes[i + 1] as u16) << 8) | utf8_bytes[i] as u16;
        arr_elements_pairwise_combined.push(u16_elem);
    }

    return arr_elements_pairwise_combined;
}

/// Apple (Mac OS X, iOS, iPadOS, et al) timestamps start on Jan 1 1904, whereas Linux timestamps
/// (which is what Rust's `chrono` library uses) start at Jan 1 1970,
/// hence this difference
const MAC_TO_LINUX_EPOCH_CONVERSION: i64 = 2082844800;

/// Converts a given Mac epoch time into an actual UTC timestamp
pub fn get_timestamp_as_mac(mac_timestamp: u64) -> chrono::DateTime<chrono::Utc> {
    return chrono::DateTime::<chrono::Utc>::from_timestamp(
        (mac_timestamp as i64) - MAC_TO_LINUX_EPOCH_CONVERSION,
        0,
    )
    .unwrap();
}

/// Converts an integer seconds into a string representing that time in hours, minutes, and seconds
/// only showing the time units appropriately (ie not showing "hours" if the time is less than 1 hour)
/// and with correct pluralization (ie "1 second" vs "2 seconds")
/// e.g. 787 -> "13 minutes, 7 seconds"
/// Linear rewrite of this algorithm: https://github.com/raleighlittles/SMS-backup-and-restore-extractor/blob/a0d940a7aaac7add3c090b8341285b5eb2a162b0/call_log_generator.py#L6
pub fn convert_seconds_to_human_readable_duration(duration_in_s: u32) -> String {
    let mut formatted_duration_str: String = String::new();

    const TIME_DIVISOR: usize = 60;

    let mut minutes = duration_in_s / (TIME_DIVISOR as u32);
    let seconds = duration_in_s % (TIME_DIVISOR as u32);

    let hours = minutes / (TIME_DIVISOR as u32);
    minutes = minutes % (TIME_DIVISOR as u32);

    if hours > 0 {
        write!(formatted_duration_str, "{} hour", hours).unwrap();

        if hours > 1 {
            write!(formatted_duration_str, "s").unwrap();
        }
    }

    if minutes > 0 {
        if !formatted_duration_str.is_empty() {
            write!(formatted_duration_str, ", ").unwrap();
        }

        write!(formatted_duration_str, "{} minute", minutes).unwrap();

        if minutes > 1 {
            write!(formatted_duration_str, "s").unwrap();
        }
    }

    if (seconds > 0) || ((seconds == 0) && formatted_duration_str.is_empty()) {
        if !formatted_duration_str.is_empty() {
            write!(formatted_duration_str, ", ").unwrap();
        }

        write!(formatted_duration_str, "{} second", seconds).unwrap();

        if seconds > 1 {
            write!(formatted_duration_str, "s").unwrap();
        }
    }

    return formatted_duration_str;
}

/// Shows the size of the size of the image, in whatever the most
/// appropriate is, specifically, chooses the largest possible unit
/// that still results in a greater-than-1 value
/// ie. "1245916" will be shown as "1.245 MB", because
/// with KB, the value would be 1245.916 KB, but with GB
/// it would be smaller than 1
pub fn convert_bytes_to_human_readable_size(num_bytes: u64) -> String {
    let human_readable_size: String;

    const ONE_MB_AS_BYTES: f64 = 1000000_f64;
    const ONE_KB_AS_BYTES: f64 = 1000_f64;

    let size_in_kb = num_bytes as f64 / ONE_KB_AS_BYTES;
    let size_in_mb = num_bytes as f64 / ONE_MB_AS_BYTES;

    if size_in_mb < 1.0f64 {
        // Megabytes was too small, choose the next smallest unit

        human_readable_size = format!("{:.2} KB", size_in_kb);
    } else {
        human_readable_size = format!("{:.2} MB", size_in_mb);
    }

    return human_readable_size;
}

/// Initialize an object to write to a CSV file, given a CSV filename
pub fn init_csv_writer(filename: &str) -> csv::Writer<std::fs::File> {
    let csv_writer = csv::Writer::from_path(filename)
        .expect(&format!("Can't initialize CSV file '{}'", &filename));

    return csv_writer;
}

#[cfg(test)]
mod helpers_tests {
    use super::*;

    #[test]
    fn test_build_le_u16_from_bytes() {
        let bytes: [u8; 2] = [0x34, 0x12];
        assert_eq!(build_le_u16_from_bytes(&bytes), 0x1234);
    }

    #[test]
    fn test_build_le_u32_from_bytes() {
        let bytes: [u8; 4] = [0x78, 0x56, 0x34, 0x12];
        assert_eq!(build_le_u32_from_bytes(&bytes), 0x12345678);
    }

    #[test]
    fn test_build_le_u64_from_bytes() {
        let bytes: [u8; 8] = [0xAB, 0xBC, 0xCD, 0xDE, 0xEF, 0x00, 0x00, 0x00];
        assert_eq!(build_le_u64_from_bytes(&bytes), 0xefdecdbcab);
    }
    #[test]
    fn test_get_slice_from_offset_with_len() {
        let file_as_array: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let slice = get_slice_from_offset_with_len(0, &file_as_array, 2, 4);
        assert_eq!(slice, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_get_slice_as_le_u32() {
        let file_as_array: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let value = get_slice_as_le_u32(0, &file_as_array, 2, 4);
        assert_eq!(value, 0x05040302);
    }

    #[test]
    fn test_get_slice_as_le_u64() {
        let file_as_array: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let value = get_slice_as_le_u64(0, &file_as_array, 1, 8);
        assert_eq!(value, 0x0807060504030201);
    }

    #[test]
    fn test_return_utf16_from_utf8() {
        let utf8_bytes: [u8; 6] = [0x61, 0x00, 0x62, 0x00, 0x63, 0x00]; // "abc" in UTF-16
        let utf16_array = return_utf16_from_utf8(&utf8_bytes);
        assert_eq!(utf16_array, vec![0x61, 0x62, 0x63]);
    }

    #[test]
    fn test_get_timestamp_as_mac() {
        let mac_timestamp: u64 = 0x00000000;
        let timestamp = get_timestamp_as_mac(mac_timestamp);
        assert_eq!(timestamp.to_string(), "1904-01-01 00:00:00 UTC");
    }

    #[test]
    fn test_convert_seconds_to_human_readable_duration() {
        assert_eq!(
            convert_seconds_to_human_readable_duration(787),
            "13 minutes, 7 seconds"
        );
        assert_eq!(convert_seconds_to_human_readable_duration(3600), "1 hour");
        assert_eq!(
            convert_seconds_to_human_readable_duration(61),
            "1 minute, 1 second"
        );
    }
}
