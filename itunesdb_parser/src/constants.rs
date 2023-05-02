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
        pub const image_list_key : &str = "mhli";
        pub const image_list_key_ascii : &[u8] = image_list_key.as_bytes();
    
        pub const image_list_num_images_offset : usize = 8; // 4 + 4
        pub const image_list_num_images_len : usize = 4;

        pub const image_list_last_offset : usize = 12;
    
        // ----- IMAGE ITEM ----- //
        pub const image_item_key : &str = "mhii";
        pub const image_item_key_ascii : &[u8] = image_item_key.as_bytes();
    
        pub const image_item_rating_offset : usize = 32; // 4 * 8
        pub const image_item_rating_len : usize = 4;
    
        pub const image_item_orig_date_offset : usize = image_item_rating_offset + image_item_rating_len + 4;
        pub const image_item_orig_date_len : usize = 4;
    
        pub const image_item_digitized_date_offset : usize = image_item_orig_date_offset + image_item_orig_date_len;
        pub const image_item_digitized_date_len : usize = 4;
    
        pub const image_item_source_img_size_offset : usize = image_item_digitized_date_offset + image_item_digitized_date_len;
        pub const image_item_source_img_size_len : usize = 4;

        pub const image_item_last_offset : usize = 52; // 4 * 13
    
        // ----- IMAGE NAME ----- //
        pub const image_name_key : &str = "mhni";
        pub const image_name_key_ascii : &[u8] = image_name_key.as_bytes();
    
        // TODO #1 ~ There's 2 size fields in this key list, and I don't understand what the difference between the two is.
        // There's also another image size field in the "Image Item" key list. I don't know the difference between this
        // and that one either.
        // There's also a table that indicates you can determine the format of the image itself (eg UYVY, RGB, etc) from the size,
        // but I don't know which size field it's referring to, so I can't implement that functionality.
        pub const image_name_img_size_offset : usize = 24; // 4 * 6
        pub const image_name_img_size_len : usize = 4;
    
        pub const image_name_img_height_offset : usize = 32; // 4 * 8
        pub const image_name_img_height_len : usize = 2;
    
        pub const image_name_img_width_offset : usize = image_name_img_height_offset + image_name_img_height_len;
        pub const image_name_img_width_len : usize = image_name_img_height_len;
    
        pub const image_name_last_offset : usize = 44; // 4 * 11
    
        // ----- PHOTO ALBUM ----- //
        pub const photo_album_key : &str = "mhba";
        pub const photo_album_key_ascii : &[u8] = photo_album_key.as_bytes();
    
        pub const photo_album_album_item_cnt_offset : usize = 16; // 4 * 4
        pub const photo_album_album_item_cnt_len : usize = 4;

        pub const photo_album_last_offset : usize = 64; // 4 * 16
    
        // ----- DATA OBJECT ----- //
        pub const data_object_key : &str = "mhod";
        pub const data_object_key_ascii  : &[u8] = data_object_key.as_bytes();
    
        pub const data_object_header_length : usize = 0x18;
    
        pub const data_object_type_offset : usize = 12; // 4 + 8
        pub const data_object_type_len : usize = 2;
    
        // As mentioned in the wiki, there are 2 categories of Data Objects. The regular container kind,
        // and the 'string' kind. See the 'String MHODs' section in the wiki.
        pub const data_object_string_subcontainer_length_offset : usize = data_object_header_length;
        pub const data_object_string_subcontainer_length_len : usize = 4;
    
        pub const data_object_string_subcontainer_encoding_offset : usize = data_object_header_length + 4;
        pub const data_object_string_subcontainer_encoding_len : usize = 4;
    
        pub const data_object_string_subcontainer_data_offset : usize = data_object_string_subcontainer_encoding_offset + 4;

        pub const data_object_last_offset : usize = 16; // 4 * 4
}

/// Mac timestamps start on Jan 1 1904, whereas Linux timestamps (which is what Rust's library uses)
/// start at Jan 1 1970, hence this difference
pub const mac_to_linux_epoch_conversion : i64 = 2082844800;