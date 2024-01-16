/**
 * 
 * File: preferences.rs
 * 
 * Provides functionality for working with the Preferences file
 */

use std::num::Wrapping;

pub fn is_daylight_savings_enabled(raw_dst_setting : u8) -> bool {

    let dst_enabled_val : u8 = 0x3C; // 60d

    return raw_dst_setting == dst_enabled_val;
}

pub fn decode_timezone(raw_timezone_info : u8) -> u8 {

    let gmt_timezone_const = 0x19; // 25d, supposed to represent GMT (UTC+0)

    let timezone_hour = (Wrapping(raw_timezone_info)) - Wrapping(gmt_timezone_const);

    return timezone_hour.0 / 2;
}

/// This was a little bit tricky to figure out. The index comes from when the device starts up for the first time, and
/// asks you to choose the language list.
/// I couldn't find anywhere this was documented so I ended up having to find an old iPod myself, reset it, and scroll
/// through the list of available languages.
pub fn decode_language_from_idx(lang_idx : u8) -> String {

    let selected_lang : Option<isolang::Language> = match lang_idx {
        0_u8 | 1_u8 => isolang::Language::from_639_1("en"),
        2_u8 => isolang::Language::from_639_1("ja"),
        3_u8 => isolang::Language::from_639_1("cs"),
        4_u8 => isolang::Language::from_639_1("da"),
        5_u8 => isolang::Language::from_639_1("de"),
        6_u8 => isolang::Language::from_639_1("es"),
        7_u8 => isolang::Language::from_639_1("fr"),
        8_u8 => isolang::Language::from_639_1("el"),
        9_u8 => isolang::Language::from_639_1("hr"),
        
    };

    return selected_lang.unwrap().to_name().to_string();
}