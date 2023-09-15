use std::fmt::Write;

/// Top-level declaration of modules, see: https://stackoverflow.com/questions/46829539
mod helpers;
mod itunesdb;
mod itunesdb_constants;
mod itunesdb_helpers;
mod photo_database;
mod photo_database_constants;
mod itunesprefs_constants;
mod itunesprefs;
mod playcounts_constants;

use crate::itunesdb_constants::*;

fn init_csv_writer(filename : &str) -> csv::Writer<std::fs::File> {

    // let mut csv_writer =
    //     csv::Writer::from_path(itunesdb_filename.clone() + ".csv").expect(&format!(
    //         "Can't initialize CSV file for output of '{}'",
    //         itunesdb_file_type
    //     ));

    let csv_writer =
    csv::Writer::from_path(filename).expect(&format!(
        "Can't initialize CSV file '{}'",
        &filename
    ));

    return csv_writer;
}

fn main() {
    let itunesdb_filename: String = std::env::args()
        .nth(1)
        .expect("Missing first parameter: iTunes DB filename");

    let itunesdb_file_type: String = std::env::args()
        .nth(2)
        .expect("Missing second parameter: iTunes DB file type. Supported values are: 'music', 'photo', 'itprefs'");

    let db_file_as_bytes: Vec<u8> = std::fs::read(&itunesdb_filename).unwrap();

    // Photo Database counters
    let mut num_image_lists = 0;
    let mut num_image_items = 0;
    let mut num_image_names = 0;
    let mut num_photo_albums = 0;
    let mut num_photo_data_objects = 0;

    // Setup CSV file

    let desired_csv_filename = itunesdb_filename.to_string() + ".csv";
    

    if itunesdb_file_type == "photo" {

        let mut photos_csv_writer = init_csv_writer(&desired_csv_filename);

        println!("Parsing photo file: {}", itunesdb_filename);

        let mut images_found: Vec<photo_database::Image> = Vec::new();

        let mut curr_img = photo_database::Image::default();

        let mut idx = 0;

        while idx < (db_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {
            let potential_photo_section_heading =
                &db_file_as_bytes[idx..idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

            if potential_photo_section_heading
                == photo_database_constants::IMAGE_LIST_KEY.as_bytes()
            {
                let image_list_num_images = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_LIST_NUM_IMAGES_OFFSET,
                    photo_database_constants::IMAGE_LIST_NUM_IMAGES_LEN,
                );

                println!(
                    "{} images found in file {}",
                    image_list_num_images, itunesdb_filename
                );
                println!("==========");
                num_image_lists += 1;

                // Done parsing the header, move the index forward up to the end of it
                idx += photo_database_constants::IMAGE_LIST_LAST_OFFSET;
            }
            // Parse Image Item
            else if potential_photo_section_heading
                == photo_database_constants::IMAGE_ITEM_KEY.as_bytes()
            {
                let image_item_rating = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_ITEM_RATING_OFFSET,
                    photo_database_constants::IMAGE_ITEM_RATING_LEN,
                );

                let image_item_orig_date_timestamp_raw = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_ITEM_ORIG_DATE_OFFSET,
                    photo_database_constants::IMAGE_ITEM_ORIG_DATE_LEN,
                );

                let image_item_digitized_timestamp_raw = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_ITEM_DIGITIZED_DATE_OFFSET,
                    photo_database_constants::IMAGE_ITEM_DIGITIZED_DATE_LEN,
                );

                let image_item_source_img_size = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_ITEM_SOURCE_IMG_SIZE_OFFSET,
                    photo_database_constants::IMAGE_ITEM_SOURCE_IMG_SIZE_LEN,
                );

                // println!("ImageItem#{} : {} , ImgSize= {}, OrigDateTS= {} , DigitizedDateTS= {}", num_image_items, itunesdb_helpers::decode_itunes_stars(image_item_rating), image_item_source_img_size, itunesdb_helpers::get_timestamp_as_mac(image_item_orig_date_timestamp_raw as u64), itunesdb_helpers::get_timestamp_as_mac(image_item_digitized_timestamp_raw as u64));

                //println!("==========");
                num_image_items += 1;

                idx += photo_database_constants::IMAGE_ITEM_LAST_OFFSET;

                // Populate existing image with properties
                curr_img.set_original_date(image_item_orig_date_timestamp_raw as u64);
                curr_img.set_digitized_date(image_item_digitized_timestamp_raw as u64);
            }
            // Parse Image Name
            else if potential_photo_section_heading
                == photo_database_constants::IMAGE_NAME_KEY.as_bytes()
            {
                let image_name_img_size = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_NAME_IMG_SIZE_OFFSET,
                    photo_database_constants::IMAGE_NAME_IMG_SIZE_LEN,
                );

                // TODO: Figure out why the Image Height and Image Width are both 0
                let image_name_img_height = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_NAME_IMG_HEIGHT_OFFSET,
                    photo_database_constants::IMAGE_NAME_IMG_HEIGHT_LEN,
                );

                let image_name_img_width = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::IMAGE_NAME_IMG_WIDTH_OFFSET,
                    photo_database_constants::IMAGE_NAME_IMG_WIDTH_LEN,
                );

                // println!(
                //     "ImageName#{} : Size= {} bytes, Height={} , Width={}",
                //     num_image_names,
                //     image_name_img_size,
                //     image_name_img_height,
                //     image_name_img_width
                // );
                // println!("==========");

                num_image_names += 1;

                idx += photo_database_constants::IMAGE_NAME_LAST_OFFSET;

                //curr_img.file_size_bytes = image_name_img_size as u64;
                curr_img.set_filesize(image_name_img_size);
            }
            // Parse Photo Album
            else if potential_photo_section_heading
                == photo_database_constants::PHOTO_ALBUM_KEY.as_bytes()
            {
                let photo_album_item_count = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::PHOTO_ALBUM_ALBUM_ITEM_CNT_OFFSET,
                    photo_database_constants::PHOTO_ALBUM_ALBUM_ITEM_CNT_LEN,
                );

                // println!(
                //     "PhotoAlbum#{} : Item count#={}",
                //     num_photo_albums, photo_album_item_count
                // );
                // println!("==========");

                num_photo_albums += 1;

                idx += photo_database_constants::PHOTO_ALBUM_LAST_OFFSET;
            }
            // Parse Data Object
            else if potential_photo_section_heading
                == photo_database_constants::DATA_OBJECT_KEY.as_bytes()
            {
                let data_object_type = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    photo_database_constants::DATA_OBJECT_TYPE_OFFSET,
                    photo_database_constants::DATA_OBJECT_TYPE_LEN,
                );

                if data_object_type == (photo_database::MhodType::AlbumName as u32)
                    || data_object_type == (photo_database::MhodType::FileName as u32)
                {
                    let data_object_subcontainer_str_len = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_OFFSET,
                        photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_LEN,
                    );

                    let data_object_subcontainer_encoding = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET,
                        photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_LEN,
                    );

                    if data_object_subcontainer_encoding == 0
                        || data_object_subcontainer_encoding == 1
                    {
                        // TODO: Figure out why I'm off by a width of 4 on the length.
                        // Same issue with UTF-16 encoding (below)

                        let data_object_string_bytes = helpers::get_slice_from_offset_with_len(
                            idx,
                            &db_file_as_bytes,
                            photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET,
                            data_object_subcontainer_str_len as usize,
                        );

                        let data_object_subcontainer_data =
                            std::str::from_utf8(&data_object_string_bytes)
                                .expect("Can't convert UTF-8 array to MHOD string");

                        //println!("MHOD substring = {}", data_object_subcontainer_data);

                        curr_img.set_filename(data_object_subcontainer_data.to_string());
                    } else if data_object_subcontainer_encoding == 2 {
                        let data_object_pairwise_combined = &helpers::return_utf16_from_utf8(&helpers::get_slice_from_offset_with_len(idx, &db_file_as_bytes, photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET + 4, (data_object_subcontainer_str_len) as usize));

                        let data_object_subcontainer_data =
                            String::from_utf16(&data_object_pairwise_combined)
                                .expect("Can't convert UTF-16 array to MHOD string");

                        // println!("MHOD substring = {}", data_object_subcontainer_data);

                        curr_img.set_filename(data_object_subcontainer_data.to_string());
                    }

                    // println!(
                    //     "String MHOD detected : Length= {}, Encoding (raw)= {}",
                    //     data_object_subcontainer_str_len, data_object_subcontainer_encoding
                    // );
                }

                let mhod_type = photo_database::decode_mhod_type(data_object_type as u16);

                // println!(
                //     "DataObject#{} info : Type={}",
                //     num_photo_data_objects, mhod_type
                // );
                // println!("==========");

                num_photo_data_objects += 1;

                idx += photo_database_constants::DATA_OBJECT_LAST_OFFSET;

                // Once you've parsed the data object, all properties for the "current" image have been set
                // so store the current one, then 'reset' it
                if curr_img.are_enough_fields_valid() {
                    images_found.push(curr_img);
                    curr_img = photo_database::Image::default();
                }
            }

            idx += DEFAULT_SUBSTRUCTURE_SIZE;
        } // end while

        // println!("{} images found", images_found.len());

        // Setup columns of CSV file
        // TODO see if there's a way to get the struct field names as strings?
        photos_csv_writer
            .write_record(&[
                "Filename",
                "File size (bytes)",
                "File size",
                "Original Date (Mac epoch)",
                "Original Date",
                "Digitized Date (Mac epoch)",
                "Digitized Date",
            ])
            .expect("Can't create CSV headers"); // TODO better log message

        for image in images_found.iter() {
            //println!("Image filename = {}, Image size (raw) = {}, Image size = {}", image.filename, image.file_size_bytes, image.file_size_human_readable);

            // Need quotes around filename in case there's spaces in it
            photos_csv_writer
                .write_record(&[
                    format!("'{}'", image.filename),
                    image.file_size_bytes.to_string(),
                    // Even though `file_size_human_readable` is already a String,
                    // you'll get this error: move occurs because `image.file_size_human_readable` has type `String`, which does not implement the `Copy` trait
                    // if you don't convert it to a string
                    image.file_size_human_readable.to_string(),
                    image.original_date_epoch.to_string(),
                    image.original_date_ts.to_string(),
                    image.digitized_date_epoch.to_string(),
                    image.digitized_date_ts.to_string(),
                ])
                .expect("Can't write row");
        }
    } else if itunesdb_file_type == "music" {

        let mut music_csv_writer = init_csv_writer(&desired_csv_filename);

        let mut songs_found: Vec<itunesdb::Song> = Vec::new();

        let mut curr_song = itunesdb::Song::default();

        let mut idx = 0;

        while idx < (db_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {
            let potential_section_heading = &db_file_as_bytes[idx..idx + DEFAULT_SUBSTRUCTURE_SIZE];

            // Parse Database Object
            if potential_section_heading == itunesdb_constants::DATABASE_OBJECT_KEY.as_bytes() {
                let db_language_raw = helpers::get_slice_from_offset_with_len(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::DATABASE_OBJECT_LANGUAGE_OFFSET,
                    itunesdb_constants::DATABASE_OBJECT_LANGUAGE_LEN,
                );

                let db_language = std::str::from_utf8(&db_language_raw)
                    .expect("Can't parse database language string");

                println!(
                    "File {} is using language: {}, and has iTunes version: {}",
                    itunesdb_filename,
                    db_language,
                    itunesdb::parse_version_number(helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::DATABASE_OBJECT_VERSION_NUMBER_OFFSET,
                        itunesdb_constants::DATABASE_OBJECT_VERSION_NUMBER_LEN
                    ))
                );

                idx += itunesdb_constants::DATABASE_OBJECT_LAST_OFFSET;
            }
            // Parse DataSet
            else if potential_section_heading == itunesdb_constants::DATASET_KEY.as_bytes() {

                let dataset_type_raw = helpers::get_slice_from_offset_with_len(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::DATASET_TYPE_OFFSET,
                    itunesdb_constants::DATASET_TYPE_LEN,
                );

                let dataset_type_parsed = itunesdb::parse_dataset_type(dataset_type_raw[0] as u32);

                // println!(
                //     "Dataset Type: {}",
                //     dataset_type_parsed
                // );

                idx += itunesdb_constants::DATASET_LAST_OFFSET;
            }
            // Parse TrackList
            else if potential_section_heading == itunesdb_constants::TRACKLIST_KEY.as_bytes() {
                let num_songs_in_db = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::TRACKLIST_NUM_SONGS_OFFSET,
                    itunesdb_constants::TRACKLIST_NUM_SONGS_LEN,
                );

                println!("{} songs in tracklist", num_songs_in_db);

                idx += itunesdb_constants::TRACKLIST_LAST_OFFSET;
            } else if potential_section_heading == itunesdb_constants::TRACK_ITEM_KEY.as_bytes() {
                let mut track_item_info: String = String::new();

                write!(
                    track_item_info,
                    "========== Track #{} of {} ",
                    helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_NUMBER_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_NUMBER_LEN
                    ),
                    helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_NUM_TRACKS_IN_ALBUM_OFFSET,
                        itunesdb_constants::TRACK_ITEM_NUM_TRACKS_IN_ALBUM_LEN
                    )
                )
                .unwrap();

                let num_discs = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_OFFSET,
                    itunesdb_constants::TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_LEN,
                );

                // Only print disc info if current song is part of multi-disc set
                if num_discs > 0 {
                    let tracks_current_disc_num = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_DISC_NUMBER_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_DISC_NUMBER_LEN,
                    );

                    write!(
                        track_item_info,
                        " | 💿 #{} of {}",
                        tracks_current_disc_num, num_discs
                    )
                    .unwrap();
                }

                write!(track_item_info, "==========\n").unwrap();

                let track_filetype_raw = &db_file_as_bytes[idx
                    + itunesdb_constants::TRACK_ITEM_TRACK_FILETYPE_OFFSET
                    ..idx
                        + itunesdb_constants::TRACK_ITEM_TRACK_FILETYPE_OFFSET
                        + itunesdb_constants::TRACK_ITEM_TRACK_FILETYPE_LEN];

                // TODO: encapsulate this logic elsewhere
                if helpers::build_le_u32_from_bytes(track_filetype_raw) == 0 {
                    println!("Track Item file type missing. Is this is a 1st - 4th gen iPod?");
                } else {
                    let track_item_extension =
                        itunesdb::decode_track_item_filetype(track_filetype_raw);
                    write!(
                        track_item_info,
                        "Track extension: '{}' | ",
                        track_item_extension
                    )
                    .unwrap();

                    curr_song.file_extension = track_item_extension;
                }

                let track_media_type_raw = &db_file_as_bytes[idx
                    + itunesdb_constants::TRACK_ITEM_TRACK_MEDIA_TYPE_OFFSET
                    ..idx
                        + itunesdb_constants::TRACK_ITEM_TRACK_MEDIA_TYPE_OFFSET
                        + itunesdb_constants::TRACK_ITEM_TRACK_MEDIA_TYPE_LEN];

                let track_movie_file_flag = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_OFFSET,
                    itunesdb_constants::TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_LEN,
                );

                let (track_media_type_name, track_media_type_enum) =
                    itunesdb::decode_track_media_type(track_media_type_raw);

                write!(
                    track_item_info,
                    "Movie file flag: {} | Media Type: {} \n",
                    (track_movie_file_flag == 1),
                    track_media_type_name
                )
                .unwrap();

                if matches!(
                    track_media_type_enum,
                    itunesdb::HandleableMediaType::Television
                ) {
                    let season_number = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_SEASON_NUMBER_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_SEASON_NUMBER_LEN,
                    );

                    let episode_number = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_EPISODE_NUMBER_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_EPISODE_NUMBER_LEN,
                    );

                    write!(
                        track_item_info,
                        "Season #{} Episode #{}",
                        season_number, episode_number
                    )
                    .unwrap();
                } else if matches!(
                    track_media_type_enum,
                    itunesdb::HandleableMediaType::SongLike
                ) {
                    let track_advanced_audio_type = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_ADVANCED_TRACK_TYPE_OFFSET,
                        itunesdb_constants::TRACK_ITEM_ADVANCED_TRACK_TYPE_LEN,
                    );

                    write!(
                        track_item_info,
                        "Experimental(!) advanced audio info: {} \n",
                        itunesdb::decode_track_audio_type(track_advanced_audio_type)
                    )
                    .unwrap();

                    let apple_user_id = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_USER_ID_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_USER_ID_LEN,
                    );

                    if apple_user_id != 0 {
                        write!(track_item_info, "Apple User ID: {} \n", apple_user_id).unwrap();
                    }

                    let track_bitrate_type_raw = &db_file_as_bytes[idx
                        + itunesdb_constants::TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET
                        ..idx
                            + itunesdb_constants::TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET
                            + itunesdb_constants::TRACK_ITEM_TRACK_BITRATE_SETTING_LEN];

                    let track_bitrate = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_BITRATE_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_BITRATE_LEN,
                    );

                    let track_sample_rate_raw = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_SAMPLE_RATE_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_SAMPLE_RATE_LEN,
                    );

                    let track_sample_rate_hz =
                        itunesdb::decode_track_samplerate_to_hz(track_sample_rate_raw);

                    let track_volume_setting = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_VOLUME_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_VOLUME_LEN,
                    );

                    let track_bpm = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_BPM_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_BPM_LEN,
                    );

                    write!(
                        track_item_info,
                        "[Audio info] {} kbps ({}) ~ {} Hz | {} bpm |  🔈 adj. {} \n",
                        track_bitrate,
                        itunesdb::decode_track_bitrate_type_setting(track_bitrate_type_raw),
                        track_sample_rate_hz,
                        track_bpm,
                        track_volume_setting
                    )
                    .unwrap();

                    curr_song.bitrate_kbps = track_bitrate;
                    curr_song.sample_rate_hz = track_sample_rate_hz;

                    let track_size_bytes = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_FILE_SIZE_BYTES_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_FILE_SIZE_BYTES_LEN,
                    );

                    if track_size_bytes < 1 {
                        panic!("Error: Track must have non-zero file size");
                    }

                    write!(track_item_info, "Track size: {} bytes | ", track_size_bytes).unwrap();

                    curr_song.set_song_filesize(track_size_bytes);

                    let track_length_raw = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_LEN,
                    );

                    let track_length_s = itunesdb::decode_raw_track_length_to_s(track_length_raw);

                    //println!("Raw track length (ms): {} | in seconds: {}", track_length_raw, track_length_s);

                    curr_song.set_song_duration(track_length_raw);

                    write!(
                        track_item_info,
                        "Track duration: {} (Raw = {} seconds)",
                        helpers::convert_seconds_to_human_readable_duration(track_length_s),
                        track_length_s
                    )
                    .unwrap();

                    let track_start_time_offset = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_START_TIME_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_START_TIME_LEN,
                    );

                    let track_stop_time_offset = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_STOP_TIME_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_STOP_TIME_LEN,
                    );

                    write!(
                        track_item_info,
                        "{} \n",
                        itunesdb::get_track_length_info(
                            track_length_raw,
                            track_start_time_offset,
                            track_stop_time_offset
                        )
                    )
                    .unwrap();

                    let track_play_count = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_PLAY_COUNT_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_PLAY_COUNT_LEN,
                    );

                    curr_song.num_plays = track_play_count;

                    let track_skipped_count = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_SKIPPED_COUNT_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_SKIPPED_COUNT_LEN,
                    );

                    // TODO: WHy are the last played timestamps zero sometimes?

                    let track_last_played_timestamp = helpers::get_slice_as_mac_timestamp(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_LEN,
                    );

                    let track_last_skipped_timestamp = helpers::get_slice_as_mac_timestamp(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_LEN,
                    );

                    let track_skip_when_shuffle_setting = &db_file_as_bytes[idx + itunesdb_constants::TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_OFFSET .. idx + itunesdb_constants::TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_OFFSET + itunesdb_constants::TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_LEN];

                    write!(track_item_info, "Play/Skip statistics: # of plays: {} , Last played on: {} | # of skips: {}, Last skipped on: {} (Skip when shuffling? {}) ", track_play_count, track_last_played_timestamp, track_skipped_count, track_last_skipped_timestamp, track_skip_when_shuffle_setting[0] ).unwrap();

                    let track_is_compilation_setting_raw = &db_file_as_bytes[idx
                        + itunesdb_constants::TRACK_ITEM_IS_COMPILATION_SETTING_OFFSET
                        ..idx
                            + itunesdb_constants::TRACK_ITEM_IS_COMPILATION_SETTING_OFFSET
                            + itunesdb_constants::TRACK_ITEM_IS_COMPILATION_SETTING_LEN];

                    let track_has_lyrics_setting_raw = &db_file_as_bytes[idx
                        + itunesdb_constants::TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_OFFSET
                        ..idx
                            + itunesdb_constants::TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_OFFSET
                            + itunesdb_constants::TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_LEN];

                    write!(
                        track_item_info,
                        " \n Is part of compilation? {} , Has lyrics? {}",
                        track_is_compilation_setting_raw[0], track_has_lyrics_setting_raw[0]
                    )
                    .unwrap();

                    let track_rating = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_RATING_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_RATING_LEN,
                    );

                    if track_rating > 0 {
                        curr_song.song_rating_raw = track_rating as u8;

                        let track_prev_rating = helpers::get_slice_as_le_u32(
                            idx,
                            &db_file_as_bytes,
                            itunesdb_constants::TRACK_ITEM_TRACK_PREVIOUS_RATING_OFFSET,
                            itunesdb_constants::TRACK_ITEM_TRACK_PREVIOUS_RATING_LEN,
                        );

                        write!(
                            track_item_info,
                            "\n Rating info: Current rating: {} | Previous rating: {} \n",
                            itunesdb_helpers::decode_itunes_stars(track_rating as u8),
                            itunesdb_helpers::decode_itunes_stars(track_prev_rating as u8)
                        )
                        .unwrap();
                    }

                    let gapless_playback_setting_for_track = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_LEN,
                    );

                    if gapless_playback_setting_for_track == 1 {
                        let num_beginning_silence_samples = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, itunesdb_constants::TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_OFFSET, itunesdb_constants::TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_LEN);

                        let num_ending_silence_samples = helpers::get_slice_as_le_u32(
                            idx,
                            &db_file_as_bytes,
                            itunesdb_constants::TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_OFFSET,
                            itunesdb_constants::TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_LEN,
                        );

                        // let num_total_samples = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_NUM_SAMPLES_OFFSET, iTunesDB::TRACK_ITEM_TRACK_NUM_SAMPLES_LEN);

                        let num_total_samples = helpers::get_slice_as_le_u64(
                            idx,
                            &db_file_as_bytes,
                            itunesdb_constants::TRACK_ITEM_TRACK_NUM_SAMPLES_OFFSET,
                            itunesdb_constants::TRACK_ITEM_TRACK_NUM_SAMPLES_LEN,
                        );

                        write!(track_item_info, "[Gapless playback info] # of silent samples ({} at start, {} at end) - Total {}\n", num_beginning_silence_samples, num_ending_silence_samples, num_total_samples).unwrap();
                    }

                    let track_crossfade_setting = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_CROSSFADING_SETTING_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_CROSSFADING_SETTING_LEN,
                    );

                    write!(
                        track_item_info,
                        "Crossfade: {} | ",
                        (if track_crossfade_setting == 1 {
                            "Enabled"
                        } else {
                            "Disabled"
                        })
                    )
                    .unwrap();

                    let track_has_artwork_setting = &db_file_as_bytes[idx
                        + itunesdb_constants::TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_OFFSET
                        ..idx
                            + itunesdb_constants::TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_OFFSET
                            + itunesdb_constants::TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_LEN];

                    // TODO: Encapsulate this logic elsewhere
                    if itunesdb::track_has_artwork(track_has_artwork_setting) {
                        let track_associated_artwork_size = helpers::get_slice_as_le_u32(
                            idx,
                            &db_file_as_bytes,
                            itunesdb_constants::TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_OFFSET,
                            itunesdb_constants::TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_LEN,
                        );

                        write!(
                            track_item_info,
                            "🎨 artwork size: {} bytes \n",
                            track_associated_artwork_size
                        )
                        .unwrap();
                    }

                    let track_year_released = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_YEAR_PUBLISHED_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_YEAR_PUBLISHED_LEN,
                    );

                    write!(track_item_info, "\n 🗓️  ").unwrap();

                    if track_year_released != 0 {
                        write!(
                            track_item_info,
                            "Track year (from title): {} ",
                            track_year_released
                        )
                        .unwrap();

                        curr_song.song_year = track_year_released as u16;
                    }

                    // let track_added_timestamp = helpers::get_slice_as_mac_timestamp(
                    //     idx,
                    //     &db_file_as_bytes,
                    //     itunesdb_constants::TRACK_ITEM_TRACK_ADDED_TIMESTAMP_OFFSET,
                    //     itunesdb_constants::TRACK_ITEM_TRACK_ADDED_TIMESTAMP_LEN,
                    // );

                    let track_added_epoch = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_ADDED_TIMESTAMP_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_ADDED_TIMESTAMP_LEN,
                    );

                    if track_added_epoch > 0 {
                        let track_added_timestamp =
                            helpers::get_timestamp_as_mac(track_added_epoch as u64);

                        curr_song.set_song_added_timestamp(track_added_epoch as u64);

                        write!(
                            track_item_info,
                            "Added to library on: {} ",
                            track_added_timestamp
                        )
                        .unwrap();
                    }

                    let track_modified_timestamp = helpers::get_slice_as_mac_timestamp(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::TRACK_ITEM_TRACK_MODIFIED_TIME_OFFSET,
                        itunesdb_constants::TRACK_ITEM_TRACK_MODIFIED_TIME_LEN,
                    );

                    let track_published_to_store_timestamp: chrono::DateTime<chrono::Utc> =
                        helpers::get_slice_as_mac_timestamp(
                            idx,
                            &db_file_as_bytes,
                            itunesdb_constants::TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_OFFSET,
                            itunesdb_constants::TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_LEN,
                        );

                    write!(
                        track_item_info,
                        "Last modified: {} Published to iTunes store: {}",
                        track_modified_timestamp, track_published_to_store_timestamp
                    )
                    .unwrap();

                    //println!("{} \n", track_item_info);
                }

                idx += itunesdb_constants::TRACK_ITEM_LAST_OFFSET;
            } else if potential_section_heading == itunesdb_constants::PLAYLIST_KEY.as_bytes() {
                let mut playlist_info: String = "==== ".to_string();

                let is_master_playlist_setting = &db_file_as_bytes[idx
                    + itunesdb_constants::PLAYLIST_IS_MASTER_PLAYLIST_SETTING_OFFSET
                    ..idx
                        + itunesdb_constants::PLAYLIST_IS_MASTER_PLAYLIST_SETTING_OFFSET
                        + itunesdb_constants::PLAYLIST_IS_MASTER_PLAYLIST_SETTING_LEN];

                if is_master_playlist_setting[0] == 1 {
                    write!(playlist_info, "Master ").unwrap();
                }

                write!(playlist_info, "Playlist found!").unwrap();

                let playlist_created_timestamp = helpers::get_slice_as_mac_timestamp(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::PLAYLIST_CREATED_TIMESTAMP_OFFSET,
                    itunesdb_constants::PLAYLIST_CREATED_TIMESTAMP_LEN,
                );

                write!(
                    playlist_info,
                    " | Playlist created at: {} ",
                    playlist_created_timestamp
                )
                .unwrap();

                let playlist_sort_order = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::PLAYLIST_PLAYLIST_SORT_ORDER_OFFSET,
                    itunesdb_constants::PLAYLIST_PLAYLIST_SORT_ORDER_LEN,
                );

                write!(
                    playlist_info,
                    "| {} \n",
                    itunesdb::decode_playlist_sort_order(playlist_sort_order)
                )
                .unwrap();

                //println!("{} ====", playlist_info);

                idx += itunesdb_constants::PLAYLIST_LAST_OFFSET;
            } else if potential_section_heading == itunesdb_constants::PLAYLIST_ITEM_KEY.as_bytes()
            {
                let mut playlist_item_info: String = "-----".to_string();

                let playlist_item_added_timestamp = helpers::get_slice_as_mac_timestamp(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::PLAYLIST_ITEM_ADDED_TIMESTAMP_OFFSET,
                    itunesdb_constants::PLAYLIST_ITEM_ADDED_TIMESTAMP_LEN,
                );

                write!(
                    playlist_item_info,
                    " | Date added to playlist: {}",
                    playlist_item_added_timestamp
                )
                .unwrap();

                //println!("{}  -----\n", playlist_item_info);

                idx += itunesdb_constants::PLAYLIST_ITEM_LAST_OFFSET;
            } else if potential_section_heading == itunesdb_constants::ALBUM_LIST_KEY.as_bytes() {
                let mut album_list_info: String = "~~~~~~~".to_string();

                let album_item_total_num_songs = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::ALBUM_LIST_TOTAL_NUM_SONGS_OFFSET,
                    itunesdb_constants::ALBUM_LIST_TOTAL_NUM_SONGS_LEN,
                );

                write!(
                    album_list_info,
                    " {} songs in Album List",
                    album_item_total_num_songs
                )
                .unwrap();

                //println!("{}  ~~~~~~~\n", album_list_info);

                idx += itunesdb_constants::ALBUM_LIST_LAST_OFFSET;
            }
            // else if potential_section_heading == iTunesDB::ALBUM_ITEM_KEY.as_bytes() {

            //     let album_item_info : String = "######## Album item found! | ".to_string();

            //     // write!(album_item_info, " {} ########\n", itunesdb_helpers::get_timestamp_as_mac(helpers::build_le_u32_from_bytes(album_item_unknown_timestamp_raw) as u64)).unwrap();

            //     println!("{} ########\n", album_item_info);

            //     idx += iTunesDB::ALBUM_ITEM_LAST_OFFSET;

            // }
            else if potential_section_heading == itunesdb_constants::DATA_OBJECT_KEY.as_bytes() {
                let mut data_object_info: String = "%%%%%%% Data Object found!\n".to_string();

                let data_object_type_raw = helpers::get_slice_as_le_u32(
                    idx,
                    &db_file_as_bytes,
                    itunesdb_constants::DATA_OBJECT_TYPE_OFFSET,
                    itunesdb_constants::DATA_OBJECT_TYPE_LEN,
                );

                write!(
                    data_object_info,
                    "Type (raw) = {}, Decoded= '{}' | ",
                    data_object_type_raw,
                    itunesdb::decode_data_object_type(data_object_type_raw)
                )
                .unwrap();

                if itunesdb::is_data_object_type_string(data_object_type_raw) {
                    let data_object_string_len = helpers::get_slice_as_le_u32(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::DATA_OBJECT_STRING_LENGTH_OFFSET,
                        itunesdb_constants::DATA_OBJECT_STRING_LENGTH_LEN,
                    );

                    let data_object_str_bytes = helpers::get_slice_from_offset_with_len(
                        idx,
                        &db_file_as_bytes,
                        itunesdb_constants::DATA_OBJECT_STRING_LOCATION_OFFSET,
                        data_object_string_len as usize,
                    );

                    // let data_object_str = std::str::from_utf8(&data_object_str_bytes).expect("Can't parse string data object!");
                    let data_object_str = String::from_utf16(&helpers::return_utf16_from_utf8(
                        &data_object_str_bytes,
                    ))
                    .expect("Can't decode string to UTF-16");

                    write!(
                        data_object_info,
                        "Length= {} | Value: '{}'",
                        data_object_string_len, data_object_str
                    )
                    .unwrap();

                    // Store the now-decoded string in the appropriate struct field

                    if data_object_type_raw == itunesdb::HandleableDataObjectType::SongTitle as u32
                    {
                        curr_song.song_title = data_object_str;
                    } else if data_object_type_raw
                        == itunesdb::HandleableDataObjectType::SongAlbum as u32
                    {
                        curr_song.song_album = data_object_str;
                    } else if data_object_type_raw
                        == itunesdb::HandleableDataObjectType::SongArtist as u32
                    {
                        curr_song.song_artist = data_object_str;
                    } else if data_object_type_raw
                        == itunesdb::HandleableDataObjectType::SongGenre as u32
                    {
                        curr_song.song_genre = data_object_str;
                    } else if data_object_type_raw
                        == itunesdb::HandleableDataObjectType::SongComment as u32
                    {
                        curr_song.song_comment = data_object_str;
                    } else if data_object_type_raw
                        == itunesdb::HandleableDataObjectType::SongComposer as u32
                    {
                        curr_song.song_composer = data_object_str;
                    } else if data_object_type_raw
                        == itunesdb::HandleableDataObjectType::FileLocation as u32
                    {
                        curr_song.set_song_filename(data_object_str);

                        if curr_song.are_enough_fields_valid() {
                            songs_found.push(curr_song);
                            curr_song = itunesdb::Song::default();

                        } // why wouldn't current song be valid?
                    }
                }
                // Non-string MHODs
                else {
                    if (data_object_type_raw
                        == itunesdb::HandleableDataObjectType::PodcastEnclosureURL as u32)
                        || (data_object_type_raw
                            == itunesdb::HandleableDataObjectType::Podcast_RSS_URL as u32)
                    {
                        let podcast_url = itunesdb::decode_podcast_urls(idx, &db_file_as_bytes);

                        write!(
                            data_object_info,
                            "Podcast discovered, with URL: {}",
                            podcast_url
                        )
                        .unwrap();
                    }
                }

                //println!("{} %%%%%%% \r\n", data_object_info);

                idx += itunesdb_constants::DATA_OBJECT_LAST_OFFSET;
            }

            idx += DEFAULT_SUBSTRUCTURE_SIZE;
        }

        println!("{} songs found", songs_found.len());

        // Setup columns of CSV file

        music_csv_writer
            .write_record(&[
                "Song Title",
                "Artist",
                "Album",
                "Year released",
                "File size",
                "Song Duration",
                "Filename",
                "Genre",
                "File extension",
                "Bitrate (kbps)",
                "Sample Rate (Hz)",
                "File size (bytes)",
                "Song duration (seconds)",
                "Play count",
                "Rating",
                "Added to library on (timestamp)",
                "Added to library on (epoch)",
                "Composer",
                "Comment",
            ])
            .expect("Can't create CSV headers");

        for song in songs_found.iter() {
            // the duplicate `to_string()` calls are to avoid this error:
            // cannot move out of `song.song_title` which is behind a shared reference
            // move occurs because `song.song_title` has type `String`, which does not implement the `Copy` trait

            music_csv_writer
                .write_record(&[
                    song.song_title.to_string(),
                    song.song_artist.to_string(),
                    song.song_album.to_string(),
                    song.song_year.to_string(),
                    song.file_size_friendly.to_string(),
                    song.song_duration_friendly.to_string(),
                    song.song_filename.to_string(),
                    song.song_genre.to_string(),
                    song.file_extension.to_string(),
                    song.bitrate_kbps.to_string(),
                    song.sample_rate_hz.to_string(),
                    song.file_size_bytes.to_string(),
                    song.song_duration_s.to_string(),
                    song.num_plays.to_string(),
                    itunesdb_helpers::decode_itunes_stars(song.song_rating_raw),
                    song.song_added_to_library_ts.to_string(),
                    song.song_added_to_library_epoch.to_string(),
                    song.song_composer.to_string(),
                    song.song_comment.to_string(),
                ])
                .expect("Can't write row to CSV");
        }
    } // End music parser

    else if itunesdb_file_type == "itprefs" {

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

    else if itunesdb_file_type == "playcounts" {
        
        println!("Parsing PlayCounts file: {}", itunesdb_filename);

        let mut playcount_csv_writer = init_csv_writer(&desired_csv_filename);

        let mut idx = 0;

        while idx < (db_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {

            let playcount_file_heading : &[u8] = &db_file_as_bytes[idx .. idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

            if playcount_file_heading == playcounts_constants::PLAYCOUNTS_OBJECT_KEY.as_bytes() {

                let pc_entry_len = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, playcounts_constants::PLAYCOUNTS_ENTRY_LENGTH_OFFSET, playcounts_constants::PLAYCOUNTS_ENTRY_LENGTH_LEN);

                let num_entries = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, playcounts_constants::PLAYCOUNTS_NUM_ENTRIES_OFFSET, playcounts_constants::PLAYCOUNTS_NUM_ENTRIES_LEN);

                println!("Playcounts file has {} songs, and each entry has length {}", num_entries, pc_entry_len);

                println!("===========");

                if num_entries > 1 {

                    playcount_csv_writer.write_record(
                        &["# of times played (since last sync)",
                    "# of times skipped (since last sync)",
                    "Rating",
                    "Last played timestamp",
                    "Last played (epoch)",
                    "Audio playback bookmark (ms)"]).expect("Error creating header column in CSV");
                }

                for track_idx in 0 .. (num_entries as usize) {

                    let pc_starting_idx = (track_idx * pc_entry_len as usize) + playcounts_constants::PLAYCOUNTS_FILE_HEADER_LENGTH;

                    // Use the PC_ENTRY__????? fields here

                    let num_plays = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &db_file_as_bytes, playcounts_constants::PC_ENTRY_NUM_PLAYS_OFFSET, playcounts_constants::PC_ENTRY_NUM_PLAYS_LEN);

                    let num_skips : u32 = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &db_file_as_bytes, playcounts_constants::PC_ENTRY_NUM_SKIPS_OFFSET, playcounts_constants::PC_ENTRY_NUM_SKIPS_LEN);
                    
                    let raw_rating = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &db_file_as_bytes, playcounts_constants::PC_ENTRY_RATING_OFFSET, playcounts_constants::PC_ENTRY_RATING_LEN) as u8;

                    let last_played_timestamp = helpers::get_slice_as_le_u64(idx + pc_starting_idx, &db_file_as_bytes, playcounts_constants::PC_ENTRY_AUDIO_BOOKMARK_MS_OFFSET, playcounts_constants::PC_ENTRY_AUDIO_BOOKMARK_MS_LEN);
                    
                    let audio_bookmark_ms = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &db_file_as_bytes, playcounts_constants::PC_ENTRY_LAST_SKIPPED_TIMESTAMP_OFFSET, playcounts_constants::PC_ENTRY_LAST_SKIPPED_TIMESTAMP_LEN);

                    //println!("Song ID #{} of {} has been played {} times, skipped {} times, and has rating {} ", track_idx, num_entries, num_plays, num_skips, itunesdb_helpers::decode_itunes_stars(raw_track_rating as u8));

                    // Vectors have to be all the same type, hence the `to_string()`
                    playcount_csv_writer.write_record(
                        &[num_plays.to_string(),
                         num_skips.to_string(),
                         itunesdb_helpers::decode_itunes_stars(raw_rating),
                           helpers::get_timestamp_as_mac(last_played_timestamp).to_string(),
                            last_played_timestamp.to_string(),
                            audio_bookmark_ms.to_string()]).expect("Unable to write row");
                }

                

            }

            idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE;
        }

    }
    else {
        println!("'{}' is not a supported iTunesDB file type!", itunesdb_file_type);
    }
}
