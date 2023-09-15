/*
 * File: itunesdb_helpers.rs
 * 
 * Contains helper methods for handling iTunes-specific features.
*/

/// Shows how many "stars" a song had in iTunes, based on the raw rating value.
/// The formula is: 'raw rating' / 20 = # of stars
/// and the max rating is 100, therefore the max # of stars is 5
/// Also, this isn't mentioned in the iTunesDB wiki, but the iTunes UI
/// makes it impossible to give a song 0 stars.
pub fn decode_itunes_stars(users_rating_raw: u8) -> String {

    if users_rating_raw > 100 {
        panic!("Invalid (raw) rating value of '{}' received", users_rating_raw);
    }

    let num_stars = users_rating_raw / 20;

    let rating: String;

    if num_stars == 0 {
        rating = "No rating".to_string();
    }
    else if num_stars > 5 {
        panic!("Error converting rating value")
    }
    else {
        rating = format!("{} / 5 ({})", num_stars, "⭐".repeat(num_stars as usize));
    }

    return rating;
}

// This doesn't seem to be explicitly mentioned in the iTunesDB wiki,
// but the iTunesDB files use colons instead of forward slashes for directories sometimes
// e.g. "E::DCIM:129CANON:IMG_2470.JPG", actually represents "E::DCIM/129CANON/IMG_2470.jpg"
// The character after the first set of double colons is the drive letter -- in this case 'E'
// but it sometimes doesn't appear; in these other cases (what I call 'Case 2'),
// the path just appears in Unix-style (no disk letter), e.g. ":F06:T359.ithmb"
// which, again, maps to "/F06/T359.ithmb"

const ITUNESDB_DIRECTORY_SEPARATOR: char = ':';

pub fn get_canonical_path(itunesdb_format_path: String) -> String {
    let string_to_sanitize: String;

    // Case 2
    if itunesdb_format_path.chars().nth(0).unwrap() == ITUNESDB_DIRECTORY_SEPARATOR {
        string_to_sanitize = itunesdb_format_path[1..].to_string();
    } else {
        // Case 1; the drive letter is present
        string_to_sanitize = itunesdb_format_path[3..].to_string();
    }

    return str::replace(&string_to_sanitize, ITUNESDB_DIRECTORY_SEPARATOR, "/");
}
