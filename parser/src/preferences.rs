/**
 * 
 * File: preferences.rs
 * 
 * Provides functionality for working with the Preferences file
 */


pub fn is_daylight_savings_enabled(raw_dst_setting : u8) -> bool {

    let dst_enabled_val : u8 = 0x3C; // 60d

    return raw_dst_setting == dst_enabled_val;
}

/// This function is a little bit dubious, the logic for it was added in a comment on the original documentation
/// and I get invalid values with it sometimes.
pub fn decode_timezone(raw_timezone_info : u8) -> u8 {

    let gmt_timezone_const = 0x19; // 25d, supposed to represent GMT (UTC+0)

    let timezone_hour = (std::num::Wrapping(raw_timezone_info)) - std::num::Wrapping(gmt_timezone_const);

    return timezone_hour.0 / 2;
}

/// This was a little bit tricky to figure out. The index comes from when the device starts up for the first time, and
/// asks you to choose the language list.
/// I couldn't find anywhere this was documented so I ended up having to find an old iPod myself, reset it, and scroll
/// through the list of available languages.
/// Note that interestingly, the string tables on the iPod (stored in the UISS_combined.plist file) have the language strings
/// installed for "Thai" (`th-TH`), even though in the settings you can't actually change the device to that language.
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
        10_u8 =>isolang::Language::from_639_1("it"),
        11_u8 => isolang::Language::from_639_1("hu"),
        12_u8 => isolang::Language::from_639_1("nl"),
        13_u8 => isolang::Language::from_639_1("no"),
        14_u8 | 15_u8 => isolang::Language::from_639_1("pt"),
        16_u8 => isolang::Language::from_639_1("ru"),
        17_u8 => isolang::Language::from_639_1("pl"),
        18_u8 => isolang::Language::from_639_1("ro"),
        19_u8 => isolang::Language::from_639_1("sk"),
        20_u8 => isolang::Language::from_639_1("fi"),
        21_u8 => isolang::Language::from_639_1("sv"),
        22_u8 => isolang::Language::from_639_1("tr"),
        23_u8 => isolang::Language::from_639_1("ar"),
        24_u8 => isolang::Language::from_639_1("ko"),
        25_u8 | 26_u8 | 27_u8 => isolang::Language::from_639_1("zh"),
        28_u8 => isolang::Language::from_639_1("he"),
        unknown_lang_code => {
            panic!("'{}' is not a valid ISO-639-1 language code", unknown_lang_code);
        }
        
    };

    return selected_lang.unwrap().to_name().to_string();
}