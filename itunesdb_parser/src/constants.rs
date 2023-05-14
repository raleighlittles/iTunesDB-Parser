/**
 * File: constants.rs
 *
 * Provides the iTunesDB constants. See itunesdb-doc for more info.
 * Constants are split into "sections", ie 'Image List', 'Image Item'. Each section has a header,
 * and within that header are fields, which have two values associated with them:
 * (1) an offset -- where it is found in the header structure
 * (2) a length.
 *
 * Each section has a "last offset" value that indicates the offset of the last item in the header.
 */

pub mod itunesdb_constants {

    pub const SUBSTRUCTURE_SIZE: usize = 4;

    // ----- IMAGE LIST ----- //
    pub const IMAGE_LIST_KEY: &str = "mhli";
    pub const IMAGE_LIST_KEY_ASCII: &[u8] = IMAGE_LIST_KEY.as_bytes();

    pub const IMAGE_LIST_NUM_IMAGES_OFFSET: usize = 8; // 4 + 4
    pub const IMAGE_LIST_NUM_IMAGES_LEN: usize = 4;

    pub const IMAGE_LIST_LAST_OFFSET: usize = 12;

    // ----- IMAGE ITEM ----- //
    pub const IMAGE_ITEM_KEY: &str = "mhii";
    pub const IMAGE_ITEM_KEY_ASCII: &[u8] = IMAGE_ITEM_KEY.as_bytes();

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
    pub const IMAGE_NAME_KEY_ASCII: &[u8] = IMAGE_NAME_KEY.as_bytes();

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
    pub const PHOTO_ALBUM_KEY_ASCII: &[u8] = PHOTO_ALBUM_KEY.as_bytes();

    pub const PHOTO_ALBUM_ALBUM_ITEM_CNT_OFFSET: usize = 16; // 4 * 4
    pub const PHOTO_ALBUM_ALBUM_ITEM_CNT_LEN: usize = 4;

    pub const PHOTO_ALBUM_LAST_OFFSET: usize = 64; // 4 * 16

    // ----- DATA OBJECT ----- //
    pub const DATA_OBJECT_KEY: &str = "mhod";
    pub const DATA_OBJECT_KEY_ASCII: &[u8] = DATA_OBJECT_KEY.as_bytes();

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
}

/// Mac timestamps start on Jan 1 1904, whereas Linux timestamps (which is what Rust's library uses)
/// start at Jan 1 1970, hence this difference
pub const MAC_TO_LINUX_EPOCH_CONVERSION: i64 = 2082844800;
