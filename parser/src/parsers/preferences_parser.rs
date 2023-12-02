
use crate::constants::itunesprefs_constants;
use crate::constants::preferences_constants;
use crate::constants::itunesdb_constants;

use crate::itunesprefs;
use crate::preferences;

use crate::helpers::helpers;

pub fn parse_itunes_prefs_file(itunesdb_filename : String) {

    let db_file_as_bytes: Vec<u8> = std::fs::read(&itunesdb_filename).unwrap();

    println!("Parsing iTunesPrefs file: {}", itunesdb_filename);

    let mut idx : usize = 0;

    while idx < (db_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {

        let itunespref_file_heading : &[u8] = &db_file_as_bytes[idx .. idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

        if itunespref_file_heading == itunesprefs_constants::ITUNESPREF_OBJECT_KEY.as_bytes() {

            let ipod_is_setup : bool = itunesprefs::has_ipod_been_initialized(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::IPOD_SET_UP_YET_SETTING_OFFSET, itunesprefs_constants::IPOD_SET_UP_YET_SETTING_LEN));

            println!("iPod {} been setup yet", if ipod_is_setup {"has"} else { "has NOT" });

            let auto_open_itunes_setting : bool = itunesprefs::auto_open_itunes_enabled(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::AUTO_OPEN_ITUNES_SETTING_OFFSET, itunesprefs_constants::AUTO_OPEN_ITUNES_SETTING_LEN));

            println!("Automatically open iTunes when iPod is plugged in? {}", if auto_open_itunes_setting {" Yes "} else { "No" });

            let song_sync_type : String = itunesprefs::decode_sync_automation_level(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::SONG_SYNC_AUTOMATION_LEVEL_SETTING_OFFSET, itunesprefs_constants::SONG_SYNC_AUTOMATION_LEVEL_SETTING_LEN));

            let podcast_sync_type : String = itunesprefs::decode_sync_automation_level(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::PODCAST_SYNC_AUTOMATION_LEVEL_SETTING_OFFSET, itunesprefs_constants::PODCAST_SYNC_AUTOMATION_LEVEL_SETTING_LEN));

            print!("Podcast sync type: {} | Song sync type: {} ", podcast_sync_type, song_sync_type);

            let only_update_checked_songs_setting_raw : u32 = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::ONLY_UPDATE_CHECKED_SONGS_SETTING_OFFSET, itunesprefs_constants::ONLY_UPDATE_CHECKED_SONGS_SETTING_LEN);

            if only_update_checked_songs_setting_raw == 1 {
                println!("(Warning: only updating checked songs!)");
            } else {
                print!("\n");
            }

            let sync_selection_setting : String = itunesprefs::decode_sync_selection(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::SYNC_SELECTION_SETTING_OFFSET, itunesprefs_constants::SYNC_SELECTION_SETTING_LEN));

            println!("Sync Selection setting: {}", sync_selection_setting);

            let disk_use_setting : bool = itunesprefs::disk_use_enabled(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::ENABLE_DISK_USE_SETTING_OFFSET, itunesprefs_constants::ENABLE_DISK_USE_SETTING_LEN));

            println!("Allow disk use? {}", if disk_use_setting {" Yes "} else { "No" });

            let show_artwork_setting = itunesprefs::should_show_artwork(helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesprefs_constants::SHOW_ARTWORK_SETTING_OFFSET, itunesprefs_constants::SHOW_ARTWORK_SETTING_LEN));

            println!("Show album artwork? {}", if show_artwork_setting { "Yes" } else { "No" });

            idx += itunesprefs_constants::ITUNESPREFS_OBJECT_LAST_OFFSET;
            
        }

        idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE;
    }

}

pub fn parse_preferences_file(itunesdb_filename : String) {

    let db_file_as_bytes: Vec<u8> = std::fs::read(&itunesdb_filename).unwrap();

    println!("Parsing Preferences file: {}", itunesdb_filename);

    let idx : usize = 0;

    if idx < (db_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {

        let dst_setting_raw = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, preferences_constants::DST_SETTING_OFFSET, preferences_constants::DST_SETTING_LEN);

        println!("DST enabled?: {}", preferences::is_daylight_savings_enabled(dst_setting_raw as u8));

        let language_selection_idx = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, preferences_constants::LANGUAGE_SELECTION_OFFSET, preferences_constants::LANGUAGE_SELECTION_LEN);

        println!("Selected language idx: {}", language_selection_idx);

        let tz_info_raw = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, preferences_constants::TIMEZONE_INFO_OFFSET, preferences_constants::TIMEZONE_INFO_LEN);

        println!("Raw timezone value: {} | Calculated timezone : GMT+'{}'", tz_info_raw, preferences::decode_timezone(tz_info_raw as u8));

        let volume_limit = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, preferences_constants::VOLUME_LIMIT_OFFSET, preferences_constants::VOLUME_LIMIT_LEN);
        
        if volume_limit != 0 {
            println!("Volume limit (if enabled): {} ", volume_limit);
        }

        let region_info = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, preferences_constants::REGION_OFFSET, preferences_constants::REGION_LEN);

        println!("Raw region info: '{}'", region_info);
    }
}