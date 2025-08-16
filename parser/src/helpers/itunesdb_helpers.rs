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

pub fn is_song_in_vec(song_to_check: &crate::itunesdb::Song, songs_vec: &Vec<crate::itunesdb::Song>) -> bool {
    for song in songs_vec {
        if song == song_to_check {
            return true;
        }
    }
    false
}

pub fn is_podcast_in_vec(podcast_to_check: &crate::itunesdb::Podcast, podcast_vec: &Vec<crate::itunesdb::Podcast>) -> bool {
    for podcast in podcast_vec {
        if podcast == podcast_to_check {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod itunesdb_helpers_tests {
    use super::*;

    #[test]
    fn test_decode_itunes_stars() {
        assert_eq!(decode_itunes_stars(0), "No rating");
        assert_eq!(decode_itunes_stars(20), "1 / 5 (⭐)");
        assert_eq!(decode_itunes_stars(40), "2 / 5 (⭐⭐)");
        assert_eq!(decode_itunes_stars(60), "3 / 5 (⭐⭐⭐)");
        assert_eq!(decode_itunes_stars(80), "4 / 5 (⭐⭐⭐⭐)");
        assert_eq!(decode_itunes_stars(100), "5 / 5 (⭐⭐⭐⭐⭐)");
    }

    #[test]
    fn test_get_canonical_path() {
        assert_eq!(get_canonical_path("E::DCIM:129CANON:IMG_2470.JPG".to_string()), "DCIM/129CANON/IMG_2470.JPG");
        assert_eq!(get_canonical_path(":F06:T359.ithmb".to_string()), "F06/T359.ithmb");
    }
}