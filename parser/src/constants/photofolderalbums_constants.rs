#![allow(dead_code)]
/**
 * File: photofolderalbums_constants.rs
 * 
 * Provides the different constants needed to index info inside of the PhotosFolderAlbums file.
 * 
 * Follows same format as itunesdb_constants.rs
 * 
 * See: http://www.ipodlinux.org/ITunesDB/#PhotosFolderAlbums
 * 
 */

 pub const PHOTOFOLDERALBUMS_OBJECT_KEY : &str = "frpd";

 pub const PFA_NUM_FOLDERS_OFFSET : usize = 12;
 pub const PFA_NUM_FOLDERS_LEN : usize = 4;

 pub const PHOTOFOLDERALBUMS_LAST_HEADER_OFFSET : usize = 100;

 pub const PFA_FOLDER_IS_SYNCED_OFFSET : usize = 0;
 pub const PFA_FOLDER_IS_SYNCED_LEN : usize = 4;

 pub const PFA_FOLDER_NAME_STR_LENGTH_OFFSET : usize = 4;
 pub const PFA_FOLDER_NAME_STR_LENGTH_LEN : usize = 2;

 pub const PFA_FOLDER_NAME_OFFSET : usize = 6; // 4 + 2
 pub const PFA_FOLDER_NAME_LEN : usize = 510;

