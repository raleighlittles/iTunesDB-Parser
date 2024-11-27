/**
 * File: itunesSD_constants.rs
 *
 * http://www.ipodlinux.org/ITunesDB/#iTunesSD_file
 *
 */

/// Header constants
pub const ITUNESSD_NUM_SONGS_OFFSET: usize = 0;
pub const ITUNESSD_NUM_SONGS_LEN: usize = 3;
pub const ITUNESSD_HEADER_SIZE_OFFSET: usize = 6;
pub const ITUNESSD_HEADER_SIZE_LEN: usize = 3;
pub const ITUNESSD_HEADER_SIZE_EXPECTED_VALUE: usize = 18; // 0x12

pub const ITUNESSD_ENTRY_SIZE: usize = 0x22E; // 558d
pub const ITUNESSD_ENTRY_SIZE_LEN : usize = 3;
pub const ITUNESSD_START_TIME_OFFSET: usize = 6;
pub const ITUNESSD_START_TIME_LEN: usize = 3;

pub const ITUNESSD_STOP_TIME_OFFSET: usize = 15; // 3 * 5
pub const ITUNESSD_STOP_TIME_LEN: usize = 3;

pub const ITUNESSD_VOLUME_OFFSET : usize = 24; // 3 * 8
pub const ITUNESSD_VOLUME_LEN : usize = 3;

pub const ITUNESSD_FILE_TYPE : usize = 27; // 3 * 9
pub const ITUNESSD_FILE_TYPE_LEN : usize = 3;

pub const ITUNESSD_SONG_ENTRY_FILENAME_OFFSET : usize = 33; // 3 * 11
pub const ITUNESSD_SONG_ENTRY_FILENAME_LEN : usize = 522; // 0x20A