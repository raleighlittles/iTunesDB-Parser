/**
 * File: itunesprefs_constants.rs
 * 
 * Provides the different constants needed to index info inside of the iTunesPrefs file.
 * 
 * Follows same format as itunesdb_constants.rs
 * 
 */

 pub const ITUNESPREF_OBJECT_KEY : &str = "frpd";
 
 pub const IPOD_SET_UP_YET_SETTING_OFFSET : usize = 8;
 pub const IPOD_SET_UP_YET_SETTING_LEN : usize = 1;

 pub const AUTO_OPEN_ITUNES_SETTING_OFFSET : usize = 9;
 pub const AUTO_OPEN_ITUNES_SETTING_LEN : usize = 1;

 pub const SONG_SYNC_AUTOMATION_LEVEL_SETTING_OFFSET : usize = 10;
 pub const SONG_SYNC_AUTOMATION_LEVEL_SETTING_LEN : usize = 1;

 pub const SYNC_SELECTION_SETTING_OFFSET : usize = 11;
 pub const SYNC_SELECTION_SETTING_LEN : usize = 1;

 pub const ENABLE_DISK_USE_SETTING_OFFSET : usize = 31;
 pub const ENABLE_DISK_USE_SETTING_LEN : usize = 1;

 pub const ONLY_UPDATE_CHECKED_SONGS_SETTING_OFFSET : usize = 34;
 pub const ONLY_UPDATE_CHECKED_SONGS_SETTING_LEN : usize = 1;

 pub const SHOW_ARTWORK_SETTING_OFFSET : usize = 49;
 pub const SHOW_ARTWORK_SETTING_LEN : usize = 1;

 pub const PODCAST_SYNC_AUTOMATION_LEVEL_SETTING_OFFSET : usize = 90;
 pub const PODCAST_SYNC_AUTOMATION_LEVEL_SETTING_LEN : usize = 1;

 pub const ITUNESPREFS_OBJECT_LAST_OFFSET : usize = 125;