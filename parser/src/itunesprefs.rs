
/**
 * File: itunesprefs.rs
 *
 * Provides functionality around working with the Photo Database internals file. Analogue of 'itunesdb.rs'
 * 
 */

 #[derive(Debug)] 
 pub enum SyncSetting {
    MANUAL = 0,
    AUTO = 1
 }

 #[derive(Debug)] 
 pub enum SyncSelection {
    EntireLibrary = 1,
    SelectedPlayListsOnly = 2
 }

/// https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
impl std::fmt::Display for SyncSetting{
    
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

// TODO: how to implement the same trait across multiple, unshared enums?
impl std::fmt::Display for SyncSelection{
    
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

 pub fn has_ipod_been_initialized(ipod_init_value_raw : u32) -> bool {
    
    if ipod_init_value_raw == 0 {
        return false;
    }
    else if ipod_init_value_raw == 1 {
        return true;
    }
    else {
        panic!("Invalid iPod initialization value '{}'", ipod_init_value_raw);
    }
 }

 pub fn auto_open_itunes_enabled(open_itunes_setting_raw : u32) -> bool {

    if open_itunes_setting_raw == 1 {
        return true;
    }
    else {
        return false;
    }
 }

 pub fn decode_sync_automation_level(sync_setting_raw : u32) -> String {
    // Rust doesn't allow for the implicit conversion between int and enum types, like C++ does
    if sync_setting_raw == 0 {
        return SyncSetting::MANUAL.to_string();
    }

    else if sync_setting_raw == 1 {
        return SyncSetting::AUTO.to_string();
    }

    else {
        panic!("Unable to determine sync type for value '{}'", sync_setting_raw);
    }
 }

 pub fn decode_sync_selection(sync_selection_setting_raw : u32) -> String {
    if sync_selection_setting_raw == 1 {
        return SyncSelection::EntireLibrary.to_string();
    }
    else if sync_selection_setting_raw == 2 {
        return SyncSelection::SelectedPlayListsOnly.to_string();
    }
    else {
        panic!("Unable to determine sync selection for value '{}'", sync_selection_setting_raw);
    }
 }

 pub fn disk_use_enabled(disk_use_setting_raw : u32) -> bool {

    if disk_use_setting_raw == 01 {
        return true;
    }
    else if disk_use_setting_raw == 1 {
        return false;
    }
    else {
        panic!("Unable to decode disk use setting '{}'", disk_use_setting_raw);
    }
 }

 pub fn should_show_artwork(show_artwork_setting_raw : u32) -> bool {

    if show_artwork_setting_raw == 1 {
        return true;
    }
    else {
        return false;
    }
 }

