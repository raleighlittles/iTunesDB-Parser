/*
 * File: itunesdb_helpers.rs
 * 
 * Contains helper methods for handling iTunes-specific features.
*/

/// Shows how many "stars" a song had in iTunes, based on the raw rating value.
/// The formula is: 'raw rating' / 20 = # of stars
/// and the max rating is 100, therefore the max # of stars is 5
/// Also, this isn't mentioned in the iTunesDB wiki, but the iTunes UI
/// makes it impossible to give a song 0 stars.
pub fn decode_itunes_stars(users_rating_raw: u8) -> String {

    if users_rating_raw > 100 {
        panic!("Invalid (raw) rating value of '{}' received", users_rating_raw);
    }

    let num_stars = users_rating_raw / 20;

    let rating: String;

    if num_stars == 0 {
        rating = "No rating".to_string();
    }
    else if num_stars > 5 {
        panic!("Error converting rating value")
    }
    else {
        rating = format!("{} / 5 ({})", num_stars, "⭐".repeat(num_stars as usize));
    }

    return rating;
}

// This doesn't seem to be explicitly mentioned in the iTunesDB wiki,
// but the iTunesDB files use colons instead of forward slashes for directories sometimes
// e.g. "E::DCIM:129CANON:IMG_2470.JPG", actually represents "E::DCIM/129CANON/IMG_2470.jpg"
// The character after the first set of double colons is the drive letter -- in this case 'E'
// but it sometimes doesn't appear; in these other cases (what I call 'Case 2'),
// the path just appears in Unix-style (no disk letter), e.g. ":F06:T359.ithmb"
// which, again, maps to "/F06/T359.ithmb"

const ITUNESDB_DIRECTORY_SEPARATOR: char = ':';

pub fn get_canonical_path(itunesdb_format_path: String) -> String {
    let string_to_sanitize: String;

    // Case 2
    if itunesdb_format_path.chars().nth(0).unwrap() == ITUNESDB_DIRECTORY_SEPARATOR {
        string_to_sanitize = itunesdb_format_path[1..].to_string();
    } else {
        // Case 1; the drive letter is present
        string_to_sanitize = itunesdb_format_path[3..].to_string();
    }

    return str::replace(&string_to_sanitize, ITUNESDB_DIRECTORY_SEPARATOR, "/");
}

pub fn is_song_in_vec(song_to_check: &crate::itunesdb::Song, songs_vec: &Vec<crate::itunesdb::Song>) -> bool {
    for song in songs_vec {
        if song == song_to_check {
            return true;
        }
    }
    false
}

pub fn is_podcast_in_vec(podcast_to_check: &crate::itunesdb::Podcast, podcast_vec: &Vec<crate::itunesdb::Podcast>) -> bool {
    for podcast in podcast_vec {
        if podcast == podcast_to_check {
            return true;
        }
    }
    false
}

use flate2::read::ZlibDecoder;
use std::io::Read;
use crate::constants::itunesdb_constants;
use crate::helpers::helpers;

/// Checks if an iTunesDB file buffer represents a compressed iTunesCDB file.
///
/// iTunesCDB files have an uncompressed `mhbd` header with a compression flag at offset 12:
/// - 1 = Uncompressed iTunesDB
/// - 2 = Compressed iTunesCDB
pub fn is_itunescdb_compressed(itunesdb_file_as_bytes: &[u8]) -> bool {
    let min_hdr_len = itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_OFFSET
        + itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_LEN;
    if itunesdb_file_as_bytes.len() < min_hdr_len {
        return false;
    }

    if &itunesdb_file_as_bytes[0..itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE]
        != itunesdb_constants::DATABASE_OBJECT_KEY.as_bytes()
    {
        return false;
    }

    let flag = helpers::get_slice_as_le_u32(
        0,
        itunesdb_file_as_bytes,
        itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_OFFSET,
        itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_LEN,
    );

    flag == itunesdb_constants::DATABASE_OBJECT_FLAG_COMPRESSED
}

/// Decompresses an iTunesCDB file into a standard, uncompressed iTunesDB byte buffer.
///
/// The `mhbd` header (from offset 0 to header_length) remains uncompressed.
/// The compressed zlib payload starts at `header_length` and continues to the end of the file.
///
/// In the returned decompressed buffer:
/// - The header is preserved.
/// - The total length field (offset 8) is updated to the uncompressed file size.
/// - The compression flag field (offset 12) is updated to 1 (uncompressed).
/// - The decompressed payload is appended immediately following the header.
pub fn decompress_itunescdb(itunesdb_file_as_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let min_hdr_fields_len = itunesdb_constants::DATABASE_OBJECT_HEADER_LEN_OFFSET
        + itunesdb_constants::DATABASE_OBJECT_HEADER_LEN_LEN;

    if itunesdb_file_as_bytes.len() < min_hdr_fields_len {
        return Err("File is too small to contain an mhbd header".to_string());
    }

    if &itunesdb_file_as_bytes[0..itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE]
        != itunesdb_constants::DATABASE_OBJECT_KEY.as_bytes()
    {
        return Err("File does not begin with 'mhbd' database object key".to_string());
    }

    let header_length = helpers::get_slice_as_le_u32(
        0,
        itunesdb_file_as_bytes,
        itunesdb_constants::DATABASE_OBJECT_HEADER_LEN_OFFSET,
        itunesdb_constants::DATABASE_OBJECT_HEADER_LEN_LEN,
    ) as usize;

    let min_required_header_len = itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_OFFSET
        + itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_LEN;
    if header_length < min_required_header_len {
        return Err(format!(
            "Reported header length ({}) is smaller than minimum required ({})",
            header_length, min_required_header_len
        ));
    }

    if itunesdb_file_as_bytes.len() < header_length {
        return Err(format!(
            "File length ({}) is smaller than reported header length ({})",
            itunesdb_file_as_bytes.len(),
            header_length
        ));
    }

    let compressed_payload = &itunesdb_file_as_bytes[header_length..];
    if compressed_payload.is_empty() {
        return Err("No compressed payload found after mhbd header".to_string());
    }

    let mut decoder = ZlibDecoder::new(compressed_payload);
    let mut decompressed_payload = Vec::new();

    decoder
        .read_to_end(&mut decompressed_payload)
        .map_err(|e| format!("Failed to decompress iTunesCDB payload with zlib: {}", e))?;

    let uncompressed_total_len = header_length + decompressed_payload.len();
    let mut decompressed_db = Vec::with_capacity(uncompressed_total_len);

    // Copy original header
    decompressed_db.extend_from_slice(&itunesdb_file_as_bytes[..header_length]);

    // Update total length (offset 8..12) to full uncompressed size
    let total_len_bytes = (uncompressed_total_len as u32).to_le_bytes();
    decompressed_db[itunesdb_constants::DATABASE_OBJECT_TOTAL_LEN_OFFSET
        ..itunesdb_constants::DATABASE_OBJECT_TOTAL_LEN_OFFSET
            + itunesdb_constants::DATABASE_OBJECT_TOTAL_LEN_LEN]
        .copy_from_slice(&total_len_bytes);

    // Update compression flag (offset 12..16) to 1 (uncompressed)
    let flag_bytes = itunesdb_constants::DATABASE_OBJECT_FLAG_UNCOMPRESSED.to_le_bytes();
    decompressed_db[itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_OFFSET
        ..itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_OFFSET
            + itunesdb_constants::DATABASE_OBJECT_COMPRESSION_FLAG_LEN]
        .copy_from_slice(&flag_bytes);

    // Append decompressed payload
    decompressed_db.extend_from_slice(&decompressed_payload);

    Ok(decompressed_db)
}

#[cfg(test)]
mod itunesdb_helpers_tests {
    use super::*;
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;

    #[test]
    fn test_decode_itunes_stars() {
        assert_eq!(decode_itunes_stars(0), "No rating");
        assert_eq!(decode_itunes_stars(20), "1 / 5 (⭐)");
        assert_eq!(decode_itunes_stars(40), "2 / 5 (⭐⭐)");
        assert_eq!(decode_itunes_stars(60), "3 / 5 (⭐⭐⭐)");
        assert_eq!(decode_itunes_stars(80), "4 / 5 (⭐⭐⭐⭐)");
        assert_eq!(decode_itunes_stars(100), "5 / 5 (⭐⭐⭐⭐⭐)");
    }

    #[test]
    fn test_get_canonical_path() {
        assert_eq!(get_canonical_path("E::DCIM:129CANON:IMG_2470.JPG".to_string()), "DCIM/129CANON/IMG_2470.JPG");
        assert_eq!(get_canonical_path(":F06:T359.ithmb".to_string()), "F06/T359.ithmb");
    }

    #[test]
    fn test_is_itunescdb_compressed_detection() {
        assert!(!is_itunescdb_compressed(&[]));
        assert!(!is_itunescdb_compressed(b"mhb"));

        // Header with uncompressed flag (1)
        let mut uncompressed_hdr = vec![0u8; 108];
        uncompressed_hdr[0..4].copy_from_slice(b"mhbd");
        uncompressed_hdr[12..16].copy_from_slice(&1u32.to_le_bytes());
        assert!(!is_itunescdb_compressed(&uncompressed_hdr));

        // Header with compressed flag (2)
        let mut compressed_hdr = vec![0u8; 108];
        compressed_hdr[0..4].copy_from_slice(b"mhbd");
        compressed_hdr[12..16].copy_from_slice(&2u32.to_le_bytes());
        assert!(is_itunescdb_compressed(&compressed_hdr));
    }

    #[test]
    fn test_decompress_itunescdb_synthetic() {
        let payload = b"mhsd\x60\x00\x00\x00synthetic_dataset_payload";
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(payload).unwrap();
        let compressed_payload = encoder.finish().unwrap();

        let header_len: u32 = 32;
        let mut file_bytes = vec![0u8; header_len as usize];
        file_bytes[0..4].copy_from_slice(b"mhbd");
        file_bytes[4..8].copy_from_slice(&header_len.to_le_bytes());
        let initial_total_len = (header_len as usize + compressed_payload.len()) as u32;
        file_bytes[8..12].copy_from_slice(&initial_total_len.to_le_bytes());
        file_bytes[12..16].copy_from_slice(&2u32.to_le_bytes()); // compressed
        file_bytes.extend_from_slice(&compressed_payload);

        assert!(is_itunescdb_compressed(&file_bytes));

        let decompressed = decompress_itunescdb(&file_bytes).expect("Decompression should succeed");
        assert!(!is_itunescdb_compressed(&decompressed));
        assert_eq!(&decompressed[0..4], b"mhbd");
        assert_eq!(&decompressed[header_len as usize..], payload);

        // Check total length field updated
        let decompressed_total_len = helpers::get_slice_as_le_u32(0, &decompressed, 8, 4);
        assert_eq!(decompressed_total_len as usize, decompressed.len());

        // Check flag updated to 1
        let flag = helpers::get_slice_as_le_u32(0, &decompressed, 12, 4);
        assert_eq!(flag, 1);
    }

    #[test]
    fn test_decompress_itunescdb_sample_file() {
        let sample_path = "../samples/input/2024-10-21_iTunesCDB";
        if let Ok(sample_bytes) = std::fs::read(sample_path) {
            assert!(is_itunescdb_compressed(&sample_bytes));
            let decompressed = decompress_itunescdb(&sample_bytes).expect("Sample decompression failed");
            assert!(!is_itunescdb_compressed(&decompressed));
            assert_eq!(&decompressed[0..4], b"mhbd");
            // Dataset header starts immediately after header
            let hdr_len = helpers::get_slice_as_le_u32(0, &decompressed, 4, 4) as usize;
            assert_eq!(&decompressed[hdr_len..hdr_len + 4], b"mhsd");
        }
    }
}