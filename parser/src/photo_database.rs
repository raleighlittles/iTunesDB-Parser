/**
 * File: photo_database.rs
 *
 * Provides functionality around working with the Photo Database internals file. Photo analogue of 'itunesdb.rs'
 * http://www.ipodlinux.org/ITunesDB/#Photo_Database
 */
use crate::helpers::helpers;
use crate::helpers::itunesdb_helpers;

pub struct Image {
    pub filename: String,
    /// iPod's filesystem is FAT
    pub file_size_bytes: u32,
    pub file_size_human_readable: String,
    pub original_date_epoch: u64,
    pub original_date_ts: chrono::DateTime<chrono::Utc>,
    pub digitized_date_epoch: u64,
    pub digitized_date_ts: chrono::DateTime<chrono::Utc>,
    pub ithmb_offset: u32
}

/// Allows instantiation of a "default" Image,
/// since each property/field of the image struct will be populated
/// at a different time
impl Default for Image {
    fn default() -> Image {
        return Image {
            filename: "".to_string(),
            file_size_bytes: 0,
            file_size_human_readable: "".to_string(),
            original_date_epoch: 0,
            original_date_ts: helpers::get_timestamp_as_mac(0),
            digitized_date_epoch: 0,
            digitized_date_ts: helpers::get_timestamp_as_mac(0),
            ithmb_offset : 0
        };
    }
}

impl Image {
    pub fn set_original_date(&mut self, orig_date_epoch: u64) {
        self.original_date_epoch = orig_date_epoch;
        self.original_date_ts = helpers::get_timestamp_as_mac(orig_date_epoch);
    }

    pub fn set_filesize(&mut self, filesize_in_bytes: u32) {
        self.file_size_bytes = filesize_in_bytes;

        self.file_size_human_readable =
            helpers::convert_bytes_to_human_readable_size(filesize_in_bytes as u64);
    }

    pub fn set_digitized_date(&mut self, digitized_date_epoch: u64) {
        self.digitized_date_epoch = digitized_date_epoch;
        self.digitized_date_ts = helpers::get_timestamp_as_mac(digitized_date_epoch);
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = itunesdb_helpers::get_canonical_path(filename);
    }

    fn are_dates_valid(&self) -> bool {
        return (self.original_date_epoch > 0) && (self.digitized_date_epoch > 0);
    }

    pub fn are_enough_fields_valid(&self) -> bool {
        return (!self.filename.is_empty())
            && (self.file_size_bytes > 0)
            && (self.are_dates_valid());
    }
}

#[allow(dead_code)]
pub enum MhodType {
    AlbumName = 1,
    ThumbNailImage = 2,
    FileName = 3,
    Container = 5,
}

/// See "MHOD types" table in Photos Database section
pub fn decode_mhod_type(mhod_type: u16) -> String {
    let mhod_type_name: String;

    if mhod_type == 1 {
        mhod_type_name = String::from("Album Name");
    } else if mhod_type == 2 {
        mhod_type_name = String::from("Thumbnail image");
    } else if mhod_type == 3 {
        mhod_type_name = String::from("File name");
    } else if mhod_type == 5 {
        mhod_type_name = String::from("Container (unused)");
    } else {
        // panic!("{} is not a supported mhod type", mhod_type);
        // I would normally have panicked here, since the wiki doesn't mention any other valid mhod types,
        // but in my testing I found that for some reason, I was seeing mhod type "6" in the photo database file,
        // which shouldn't be possible...
        mhod_type_name = format!("Unsupported ({})", mhod_type);
    }

    return mhod_type_name;
}
