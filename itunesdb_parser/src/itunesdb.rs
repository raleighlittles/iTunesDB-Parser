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

use chrono::{DateTime, NaiveDateTime, Utc};

pub mod itunesdb_constants {

    pub const SUBSTRUCTURE_SIZE: usize = 4;

    // ----- IMAGE LIST ----- //
    pub const IMAGE_LIST_KEY: &str = "mhli";

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

    // ----- DATA OBJECT ----- //
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

    // ----- DATABASE OBJECT ----- //
    pub const DATABASE_OBJECT_KEY : &str = "mhbd";

    // 4×8+2+8+2+20
    pub const DATABASE_OBJECT_LANGUAGE_OFFSET : usize = 64;
    pub const DATABASE_OBJECT_LANGUAGE_LEN : usize = 2;

    // ----- TRACK LIST ----- //
    pub const TRACK_LIST_KEY : &str = "mhlt";

    pub const TRACK_LIST_NUM_SONGS_OFFSET : usize = 8;
    pub const TRACK_LIST_NUM_SONGS_LEN : usize = 4;

    pub const TRACK_LIST_LAST_OFFSET : usize = 12;

    // ----- TRACK_ITEM ----- //
    pub const TRACK_ITEM_KEY : &str = "mhit";

    pub const TRACK_ITEM_TRACK_FILETYPE_OFFSET : usize = 24;
    pub const TRACK_ITEM_TRACK_FILETYPE_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET : usize = 28;
    pub const TRACK_ITEM_TRACK_BITRATE_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_IS_COMPILATION_OFFSET : usize = 30;
    pub const TRACK_ITEM_IS_COMPILATION_LEN : usize = 1;

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
    pub const TRACK_ITEM_TRACK_LAST_PLAYED_LEN : usize = 4;





    pub const TRACK_ITEM_LAST_OFFSET : usize = 356;


}


pub mod itunesdb_helpers {

    pub fn get_timestamp_as_mac(mac_timestamp : u64) -> chrono::DateTime<chrono::Utc> {

        return chrono::DateTime::<chrono::Utc>::from_utc( chrono::NaiveDateTime::from_timestamp_opt((mac_timestamp as i64) - super::MAC_TO_LINUX_EPOCH_CONVERSION, 0).unwrap(), chrono::offset::Utc);
    }

}
/// Mac timestamps start on Jan 1 1904, whereas Linux timestamps
/// (which is what Rust's `chrono` library uses) start at Jan 1 1970,
/// hence this difference
 const MAC_TO_LINUX_EPOCH_CONVERSION: i64 = 2082844800;
