use std::fmt::Write;

/// Top-level declaration of modules, see: https://stackoverflow.com/questions/46829539
mod helpers;
mod itunesdb;
mod itunesdb_constants;
mod itunesdb_helpers;
mod itunesprefs;
mod itunesprefs_constants;
mod photo_database;
mod photo_database_constants;
mod photofolderalbums_constants;
mod playcounts_constants;
mod preferences;
mod preferences_constants;

// Parsers
mod itunesdb_parser;
mod photo_type_parser;
mod playcounts_parser;
mod preferences_parser;

fn init_csv_writer(filename: &str) -> csv::Writer<std::fs::File> {
    let csv_writer = csv::Writer::from_path(filename)
        .expect(&format!("Can't initialize CSV file '{}'", &filename));

    return csv_writer;
}

fn main() {
    let itunesdb_filename: String = std::env::args()
        .nth(1)
        .expect("Missing first parameter: iTunes DB filename");

    let itunesdb_file_type: String = std::env::args()
        .nth(2)
        .expect("Missing second parameter: iTunes DB file type. Supported values are: 'music', 'photo', 'itprefs'");

    // Setup CSV file

    let desired_csv_filename = itunesdb_filename.to_string() + ".csv";

    if itunesdb_file_type == "photo" {

        let photos_csv_writer = init_csv_writer(&desired_csv_filename);
        photo_type_parser::parse_photo_type_file(itunesdb_filename, photos_csv_writer);

    } else if itunesdb_file_type == "itunes" {

        let itunesdb_csv_writer = init_csv_writer(&desired_csv_filename);
        itunesdb_parser::parse_itunesdb_file(itunesdb_filename, itunesdb_csv_writer);

    } else if itunesdb_file_type == "itprefs" {

        preferences_parser::parse_itunes_prefs_file(itunesdb_filename);

    } else if itunesdb_file_type == "playcounts" {

        let playcounts_csv_writer = init_csv_writer(&desired_csv_filename);
        playcounts_parser::parse_playcounts(itunesdb_filename, playcounts_csv_writer);

    } else if itunesdb_file_type == "pfalbums" {

        photo_type_parser::parse_photofolder_albums_file(itunesdb_filename);

    } else if itunesdb_file_type == "preferences" {

        preferences_parser::parse_preferences_file(itunesdb_filename);

    } else {
        println!(
            "'{}' is not a supported iTunesDB file type!",
            itunesdb_file_type
        );
    }
}
