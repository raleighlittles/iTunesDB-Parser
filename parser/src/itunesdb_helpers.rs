/// Code that is shared across the different iTunesDB file types

// Shows how many "stars" a song had in iTunes, based on the raw rating value.
// The formula is: rating / 20 = stars
// and the max rating is 100, so stars are out of 5
pub fn decode_itunes_stars(users_rating_raw: u8) -> String {
    let num_stars = users_rating_raw / 20;

    let rating: String;

    if num_stars != 0 {
        rating = format!("{} ⭐ / 5", num_stars);
    } else {
        rating = "N/A".to_string();
    }

    return rating;
}

// This doesn't seem to be explicitly mentioned in the iTunesDB wiki,
// but the iTunesDB files use colons instead of forward slashes for directories sometimes
// e.g. "E::DCIM:129CANON:IMG_2470.JPG", actually represents "E::DCIM/129CANON/IMG_2470.jpg"
// The first set of double colons is the disk letter -- in this case 'E' -- like in Windows
// but it sometimes doesn't appear; in these cases (what I call 'Case 2'),
// the path just appears in Unix-style (no disk letter), e.g. ":F06:T359.ithmb"

const ITUNESDB_DIRECTORY_SEPARATOR: char = ':';

pub fn get_canonical_path(itunesdb_format_path: String) -> String {
    let string_to_sanitize: String;

    // Case 2
    if itunesdb_format_path.chars().nth(0).unwrap() == ITUNESDB_DIRECTORY_SEPARATOR {
        string_to_sanitize = itunesdb_format_path[1..].to_string();
    } else {
        // Case 1; the disk letter is present
        string_to_sanitize = itunesdb_format_path[3..].to_string();
    }

    return str::replace(&string_to_sanitize, ITUNESDB_DIRECTORY_SEPARATOR, "/");
}
