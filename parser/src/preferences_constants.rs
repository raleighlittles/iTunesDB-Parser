/*
 * File: preferences_constants.rs
 * 
 * Contains information about offsets needed for 'Preferences' file.
 * 
 * Follows same format as itunesdb_constants.rs
 * 
*/

/// Daylight Savings Time
pub const DST_SETTING_OFFSET : usize = 1724; // 0x6BC
pub const DST_SETTING_LEN : usize = 1;

pub const LANGUAGE_SELECTION_OFFSET : usize = 2808; // 0xAF8
pub const LANGUAGE_SELECTION_LEN : usize = 1;

// This is the offset the documentation says to use, but on the Preferences
// file I received, this offset was empty (0), which, if the timezone calculation
// instructions in the wiki are correct, would give a bogus timezone of
// UTC-12 (a timezone that is completely uninhabited). When examining the file
// in a hex editor, I noticed that there's a value at offset 0xB22 (2850d)
// that matches the user's timezone.
// 0x1C - 0x19 = 3 , 3 / 2 = 1
// (user said they were in GMT+1)
//pub const TIMEZONE_INFO_OFFSET : usize = 2832; //0xB10
pub const TIMEZONE_INFO_OFFSET : usize = 2850;
pub const TIMEZONE_INFO_LEN : usize = 1;

pub const VOLUME_LIMIT_OFFSET : usize = 2896; // 0xB50
pub const VOLUME_LIMIT_LEN : usize = 1;

pub const REGION_OFFSET : usize = 2928; // 0xB70
pub const REGION_LEN : usize = 1;
