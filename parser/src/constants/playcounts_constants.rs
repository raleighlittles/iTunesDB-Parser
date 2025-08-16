#![allow(dead_code)]
/*
 * File: playcounts_constants.rs
 * 
 * Provides the different constants needed for PlayCounts files.
 * 
 * Follows same format as itunesdb_constants.rs
 * 
 * Glossary:
 * 
 * "PC" = Play Counts
 * 
*/

pub const PLAYCOUNTS_OBJECT_KEY : &str = "mhdp";

pub const PLAYCOUNTS_ENTRY_LENGTH_OFFSET : usize = 8;
pub const PLAYCOUNTS_ENTRY_LENGTH_LEN : usize = 4;

pub const PLAYCOUNTS_NUM_ENTRIES_OFFSET : usize = 12;
pub const PLAYCOUNTS_NUM_ENTRIES_LEN : usize = 4;

/// Hardcoded from documentation
pub const PLAYCOUNTS_FILE_HEADER_LENGTH : usize = 96; // 0x60

pub const PLAYCOUNTS_HEADER_LAST_OFFSET : usize = 16;

/// These are the offsets for the individual entries themselves. Note that, per the
/// documentation, unless stated otherwise, the statistics are measured _from the last sync.
/// 
pub const PC_ENTRY_NUM_PLAYS_OFFSET : usize = 0;
pub const PC_ENTRY_NUM_PLAYS_LEN : usize = 4;

pub const PC_ENTRY_LAST_PLAYED_TIMESTAMP_OFFSET : usize = 4;
pub const PC_ENTRY_LAST_PLAYED_TIMESTAMP_LEN : usize = 4;

pub const PC_ENTRY_AUDIO_BOOKMARK_MS_OFFSET : usize = 8;
pub const PC_ENTRY_AUDIO_BOOKMARK_MS_LEN : usize = 4;

pub const PC_ENTRY_RATING_OFFSET : usize = 12;
pub const PC_ENTRY_RATING_LEN : usize = 4;

pub const PC_ENTRY_NUM_SKIPS_OFFSET : usize = 20;
pub const PC_ENTRY_NUM_SKIPS_LEN : usize = 4;

pub const PC_ENTRY_LAST_SKIPPED_TIMESTAMP_OFFSET : usize = 24;
pub const PC_ENTRY_LAST_SKIPPED_TIMESTAMP_LEN : usize = 4;