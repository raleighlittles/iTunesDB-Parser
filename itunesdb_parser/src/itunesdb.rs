/**
 * File: itunesdb.rs
 *
 * Provides the iTunesDB constants, and associated functionality. See itunesdb-doc for more info.
 * Constants are split into "sections", ie 'Image List', 'Image Item'. Each section has a header,
 * and within that header are fields, which have two values associated with them:
 * (1) an offset -- where it is found in the header structure
 * (2) a length.
 *
 * Each section of constants has a "last offset" value that indicates the offset of the last item in the header.
 */


 use std::fmt::Write;

use chrono::{DateTime, NaiveDateTime, Utc};

pub const DEFAULT_SUBSTRUCTURE_SIZE: usize = 4;

pub mod photo_database {

    // ----- IMAGE LIST ----- //
    pub const IMAGE_LIST_KEY: &str = "mhli";

    pub const IMAGE_LIST_KEY_ASCII : &[u8] = IMAGE_ITEM_KEY.as_bytes();

    pub const IMAGE_LIST_NUM_IMAGES_OFFSET: usize = 8; // 4 + 4
    pub const IMAGE_LIST_NUM_IMAGES_LEN: usize = 4;

    pub const IMAGE_LIST_LAST_OFFSET: usize = 12;

    // ----- IMAGE ITEM ----- //
    pub const IMAGE_ITEM_KEY: &str = "mhii";

    pub const IMAGE_ITEM_RATING_OFFSET: usize = 32; // 4 * 8
    pub const IMAGE_ITEM_RATING_LEN: usize = 4;

    pub const IMAGE_ITEM_ORIG_DATE_OFFSET: usize =
        IMAGE_ITEM_RATING_OFFSET + IMAGE_ITEM_RATING_LEN + 4;
    pub const IMAGE_ITEM_ORIG_DATE_LEN: usize = 4;

    pub const IMAGE_ITEM_DIGITIZED_DATE_OFFSET: usize =
        IMAGE_ITEM_ORIG_DATE_OFFSET + IMAGE_ITEM_ORIG_DATE_LEN;
    pub const IMAGE_ITEM_DIGITIZED_DATE_LEN: usize = 4;

    pub const IMAGE_ITEM_SOURCE_IMG_SIZE_OFFSET: usize =
        IMAGE_ITEM_DIGITIZED_DATE_OFFSET + IMAGE_ITEM_DIGITIZED_DATE_LEN;
    pub const IMAGE_ITEM_SOURCE_IMG_SIZE_LEN: usize = 4;

    pub const IMAGE_ITEM_LAST_OFFSET: usize = 52; // 4 * 13

    // ----- IMAGE NAME ----- //
    pub const IMAGE_NAME_KEY: &str = "mhni";

    // TODO #1 ~ There's 2 size fields in this key list, and I don't understand what the difference between the two is.
    // There's also another image size field in the "Image Item" key list. I don't know the difference between this
    // and that one either.
    // There's also a table that indicates you can determine the format of the image itself (eg UYVY, RGB, etc) from the size,
    // but I don't know which size field it's referring to, so I can't implement that functionality.
    pub const IMAGE_NAME_IMG_SIZE_OFFSET: usize = 24; // 4 * 6
    pub const IMAGE_NAME_IMG_SIZE_LEN: usize = 4;

    pub const IMAGE_NAME_IMG_HEIGHT_OFFSET: usize = 32; // 4 * 8
    pub const IMAGE_NAME_IMG_HEIGHT_LEN: usize = 2;

    pub const IMAGE_NAME_IMG_WIDTH_OFFSET: usize =
        IMAGE_NAME_IMG_HEIGHT_OFFSET + IMAGE_NAME_IMG_HEIGHT_LEN;
    pub const IMAGE_NAME_IMG_WIDTH_LEN: usize = IMAGE_NAME_IMG_HEIGHT_LEN;

    pub const IMAGE_NAME_LAST_OFFSET: usize = 44; // 4 * 11

    // ----- PHOTO ALBUM ----- //
    pub const PHOTO_ALBUM_KEY: &str = "mhba";

    pub const PHOTO_ALBUM_ALBUM_ITEM_CNT_OFFSET: usize = 16; // 4 * 4
    pub const PHOTO_ALBUM_ALBUM_ITEM_CNT_LEN: usize = 4;

    pub const PHOTO_ALBUM_LAST_OFFSET: usize = 64; // 4 * 16

    // ----- DATA OBJECT (for photosDB only!!) ----- //
    pub const DATA_OBJECT_KEY: &str = "mhod";

    pub const DATA_OBJECT_HEADER_LENGTH: usize = 0x18;

    pub const DATA_OBJECT_TYPE_OFFSET: usize = 12; // 4 + 8
    pub const DATA_OBJECT_TYPE_LEN: usize = 2;

    // As mentioned in the wiki, there are 2 categories of Data Objects. The regular container kind,
    // and the 'string' kind. See the 'String MHODs' section in the wiki.
    pub const DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_OFFSET: usize = DATA_OBJECT_HEADER_LENGTH;
    pub const DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_LEN: usize = 4;

    pub const DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET: usize =
        DATA_OBJECT_HEADER_LENGTH + 4;
    pub const DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_LEN: usize = 4;

    pub const DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET: usize =
        DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET + 4;

    pub const DATA_OBJECT_LAST_OFFSET: usize = 16; // 4 * 4

    pub enum MhodType {
        ALBUM_NAME = 1,
        THUMBNAIL_IMAGE = 2,
        FILE_NAME = 3,
        CONTAINER = 5
    }

    /// See "MHOD types" table in Photos Database section
    pub fn decode_mhod_type(mhod_type : u16) -> String {

        let mhod_type_name : String;
        
        if mhod_type == 1 {
            mhod_type_name = String::from("Album Name");
        }

        else if mhod_type == 2 {
            mhod_type_name = String::from("Thumbnail image");
        }

        else if mhod_type == 3 {
            mhod_type_name = String::from("File name");
        }

        else if mhod_type == 5 {
            mhod_type_name = String::from("Container (unused)");
        }

        else {
            // panic!("{} is not a supported mhod type", mhod_type);
            // I would normally have panicked here, since the wiki doesn't mention any other valid mhod types,
            // but in my testing I found that for some reason, I was seeing mhod type "6" in the photo database file,
            // which shouldn't be possible...
            mhod_type_name = String::from("Unsupported");
        }

        return mhod_type_name;
    }

}

pub mod iTunesDB {

    // ----- DATABASE OBJECT ----- //
    pub const DATABASE_OBJECT_KEY : &str = "mhbd";

    // 4×8+2+8+2+20
    pub const DATABASE_OBJECT_LANGUAGE_OFFSET : usize = 70;
    pub const DATABASE_OBJECT_LANGUAGE_LEN : usize = 2;

    pub const DATABASE_OBJECT_LAST_OFFSET : usize = 108;

    // ----- TRACKLIST ----- //
    pub const TRACKLIST_KEY : &str = "mhlt";

    pub const TRACKLIST_NUM_SONGS_OFFSET : usize = 8;
    pub const TRACKLIST_NUM_SONGS_LEN : usize = 4;

    pub const TRACKLIST_LAST_OFFSET : usize = 12;

    // ----- TRACK ITEM ----- //
    pub const TRACK_ITEM_KEY : &str = "mhit";

    pub const TRACK_ITEM_TRACK_FILETYPE_OFFSET : usize = 24;
    pub const TRACK_ITEM_TRACK_FILETYPE_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET : usize = 28;
    pub const TRACK_ITEM_TRACK_BITRATE_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_IS_COMPILATION_SETTING_OFFSET : usize = 30;
    pub const TRACK_ITEM_IS_COMPILATION_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_RATING_OFFSET : usize = 31;
    pub const TRACK_ITEM_TRACK_RATING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_MODIFIED_TIME_OFFSET : usize = 32;
    pub const TRACK_ITEM_TRACK_MODIFIED_TIME_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_FILE_SIZE_BYTES_OFFSET : usize = 36;
    pub const TRACK_ITEM_TRACK_FILE_SIZE_BYTES_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_OFFSET : usize = 40;
    pub const TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_NUMBER_OFFSET : usize = 44;
    pub const TRACK_ITEM_TRACK_NUMBER_LEN  : usize = 4;

    pub const TRACK_ITEM_NUM_TRACKS_IN_ALBUM_OFFSET : usize = 48;
    pub const TRACK_ITEM_NUM_TRACKS_IN_ALBUM_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_YEAR_PUBLISHED_OFFSET : usize = 52;
    pub const TRACK_ITEM_TRACK_YEAR_PUBLISHED_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_BITRATE_OFFSET : usize = 56;
    pub const TRACK_ITEM_TRACK_BITRATE_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_SAMPLE_RATE_OFFSET : usize = 60;
    pub const TRACK_ITEM_TRACK_SAMPLE_RATE_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_VOLUME_OFFSET : usize = 64;
    pub const TRACK_ITEM_TRACK_VOLUME_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_START_TIME_OFFSET : usize = 68;
    pub const TRACK_ITEM_TRACK_START_TIME_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_STOP_TIME_OFFSET : usize = 72;
    pub const TRACK_ITEM_TRACK_STOP_TIME_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_PLAY_COUNT_OFFSET : usize = 80;
    pub const TRACK_ITEM_TRACK_PLAY_COUNT_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_OFFSET : usize = 88;
    pub const TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_DISC_NUMBER_OFFSET : usize = 92;
    pub const TRACK_ITEM_TRACK_DISC_NUMBER_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_OFFSET : usize = 96;
    pub const TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_USER_ID_OFFSET : usize = 100;
    pub const TRACK_ITEM_TRACK_USER_ID_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_ADDED_TIMESTAMP_OFFSET : usize = 104;
    pub const TRACK_ITEM_TRACK_ADDED_TIMESTAMP_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_BOOKMARK_TIME_MILLISECONDS_OFFSET : usize = 108;
    pub const TRACK_ITEM_TRACK_BOOKMARK_TIME_MILLISECONDS_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_PREVIOUS_RATING_OFFSET : usize = 121;
    pub const TRACK_ITEM_TRACK_PREVIOUS_RATING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_BPM_OFFSET : usize = 122;
    pub const TRACK_ITEM_TRACK_BPM_LEN : usize = 2;

    pub const TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_OFFSET : usize = 128;
    pub const TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_OFFSET : usize = 140;
    pub const TRACK_ITEM_TRACK_RELEASE_TIMESTAMP_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_SKIPPED_COUNT_OFFSET : usize = 156;
    pub const TRACK_ITEM_TRACK_SKIPPED_COUNT_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_OFFSET : usize =  160;
    pub const TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_OFFSET : usize = 165;
    pub const TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_REMEMBER_PLAYBACK_POSITION_SETTING_OFSET : usize = 166;
    pub const TRACK_ITEM_TRACK_REMEMBER_PLAYBACK_POSITION_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_OFFSET : usize = 176;
    pub const TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_OFFSET : usize = 177;
    pub const TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_OFFSET : usize = 184;
    pub const TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_NUM_SAMPLES_OFFSET : usize = 188;
    pub const TRACK_ITEM_TRACK_NUM_SAMPLES_LEN : usize = 8;

    pub const TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_OFFSET : usize = 200;
    pub const TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_MEDIA_TYPE_OFFSET : usize = 208;
    pub const TRACK_ITEM_TRACK_MEDIA_TYPE_LEN : usize = 4;

    // Parse this FIRST, before doing anything else
    pub const TRACK_ITEM_TRACK_SEASON_NUMBER_OFFSET : usize = 212;
    pub const TRACK_ITEM_TRACK_SEASON_NUMBER_LEN : usize = 4;
    
    pub const TRACK_ITEM_TRACK_EPISODE_NUMBER_OFFSET : usize = 216;
    pub const TRACK_ITEM_TRACK_EPISODE_NUMBER_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_OFFSET : usize = 256;
    pub const TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_LEN : usize = 2;

    pub const TRACK_ITEM_TRACK_CROSSFADING_SETTING_OFFSET : usize = 258;
    pub const TRACK_ITEM_TRACK_CROSSFADING_SETTING_LEN : usize = 2;

    pub const TRACK_ITEM_LAST_OFFSET : usize = 356;

    // From the wiki: "the file's type [..] an ANSI string padded with spaces"
    pub fn decode_track_item_filetype(file_type_raw : &[u8]) -> String {

        let mut filetype_str : String = String::from(std::str::from_utf8(&file_type_raw).expect("Can't parse Track Item file type raw string"));

        // The Track Item filetype has spaces in it, we obviously don't want that
        filetype_str.retain(|ch : char| !ch.is_whitespace());

        // Technically the characters are stored in reverse-endian order, but they're ASCII
        // (8-bit) so you can just achieve the same result by reversing the string
        return filetype_str.chars().rev().collect();
    }

    pub fn decode_track_bitrate_type_setting(bitrate_type_raw : &[u8]) -> String {

        let bitrate_type : String;

        if bitrate_type_raw[0] == 0x0 {
            bitrate_type = "Constant bitrate".to_string();
        }

        else if bitrate_type_raw[0] == 0x1 {
            bitrate_type = "Variable bitrate/AAC-encoded".to_string();
        }

        else {
            bitrate_type = "Unable to determine if constant or variable bitrate".to_string();
        }

        return bitrate_type;
    }

    // ----- PLAYLIST ----- //
    pub const PLAYLIST_KEY : &str = "mhyp";
    pub const PLAYLIST_IS_MASTER_PLAYLIST_SETTING_OFFSET : usize = 20;
    pub const PLAYLIST_IS_MASTER_PLAYLIST_SETTING_LEN : usize = 1;

    pub const PLAYLIST_CREATED_TIMESTAMP_OFFSET : usize = 24;
    pub const PLAYLIST_CREATED_TIMESTAMP_LEN : usize = 4;

    pub const PLAYLIST_PLAYLIST_SORT_ORDER_OFFSET : usize = 44;
    pub const PLAYLIST_PLAYLIST_SORT_ORDER_LEN : usize = 4;

    pub const PLAYLIST_LAST_OFFSET : usize = 48;

    // ----- PLAYLIST ITEM ----- //
    pub const PLAYLIST_ITEM_KEY : &str = "mhip";
    
    pub const PLAYLIST_ITEM_SONG_ADDED_TIMESTAMP_OFFSET : usize = 28;
    pub const PLAYLIST_ITEM_SONG_ADDED_TIMESTAMP_LEN : usize = 4;

    pub const PLAYLIST_ITEM_LAST_OFFSET : usize = 36;

    // Add data object (for music only)

    // ----- ALBUM LIST ----- //
    pub const ALBUM_LIST_KEY : &str = "mhla";

    pub const ALBUM_LIST_TOTAL_NUM_SONGS_OFFSET : usize = 8;
    pub const ALBUM_LIST_TOTAL_NUM_SONGS_LEN : usize = 4;

    pub const ALBUM_LIST_LAST_OFFSET : usize = 12;

    // ----- ALBUM ITEM ----- //
    pub const ALBUM_ITEM_KEY : &str = "mhia";

    pub const ALBUM_ITEM_UNKNOWN_TIMESTAMP_OFFSET : usize = 20;
    pub const ALBUM_ITEM_UNKNOWN_TIMESTAMP_LEN : usize = 8;

    pub const ALBUM_ITEM_LAST_OFFSET : usize = 32;
}


pub mod itunesdb_helpers {

    /// Mac timestamps start on Jan 1 1904, whereas Linux timestamps
    /// (which is what Rust's `chrono` library uses) start at Jan 1 1970,
    /// hence this difference
    const MAC_TO_LINUX_EPOCH_CONVERSION: i64 = 2082844800;

    pub fn get_timestamp_as_mac(mac_timestamp : u64) -> chrono::DateTime<chrono::Utc> {

        return chrono::DateTime::<chrono::Utc>::from_utc( chrono::NaiveDateTime::from_timestamp_opt((mac_timestamp as i64) - MAC_TO_LINUX_EPOCH_CONVERSION, 0).unwrap(), chrono::offset::Utc);
    }

    // Shows how many "stars" a song had in iTunes, based on the raw rating value.
    // The formula is: rating / 20 = stars
    // and the max rating is 100, so stars are out of 5
    pub fn decode_itunes_stars(users_rating_raw : u32) -> String {

        let num_stars = users_rating_raw / 20;

        let rating : String;
        
        if num_stars != 0 {
            rating = format!("{}/5 ⭐", num_stars);
        } else {
            rating = "N/A".to_string();
        }

        return rating
    }

}