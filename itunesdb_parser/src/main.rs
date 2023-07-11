use std::fmt::Write;

/// Top-level declaration of modules, see: https://stackoverflow.com/questions/46829539
mod helpers;
mod itunesdb;

//use csv::Writer;

use crate::itunesdb::*;
use crate::helpers::*;

fn main() {
    let itunesdb_filename: String = std::env::args()
        .nth(1)
        .expect("Missing parameter: iTunes DB filename");

    let itunesdb_file_type : String = std::env::args().nth(2).expect("Missing parameter: iTunes DB file type. Supported values are: 'music', 'photo'");

    let db_file_as_bytes: Vec<u8> = std::fs::read(&itunesdb_filename).unwrap();

    // Photo Database counters
    let mut num_image_lists = 0;
    let mut num_image_items = 0;
    let mut num_image_names = 0;
    let mut num_photo_albums = 0;
    let mut num_photo_data_objects = 0;

    if itunesdb_file_type == "photo" {

        println!("Parsing photo file: {}", itunesdb_filename);

        // Setup CSV file
        let mut csv_writer = csv::Writer::from_path(itunesdb_filename.clone() + ".csv");

        let mut images_found : Vec<itunesdb::Image> = Vec::new();

        let mut curr_img = itunesdb::Image::default();

        let mut idx = 0;

        while idx < db_file_as_bytes.len() {

            let potential_photo_section_heading = &db_file_as_bytes[idx .. idx + DEFAULT_SUBSTRUCTURE_SIZE];

            if potential_photo_section_heading == photo_database::IMAGE_LIST_KEY.as_bytes()
            {
                let image_list_num_images = get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_LIST_NUM_IMAGES_OFFSET, photo_database::IMAGE_LIST_NUM_IMAGES_LEN);

                println!(
                    "{} images found in file {}", image_list_num_images, itunesdb_filename
                );
                println!("==========");
                num_image_lists += 1;

                // Done parsing the header, move the index forward up to the end of it
                idx += photo_database::IMAGE_LIST_LAST_OFFSET;
            }
            // Parse Image Item
            else if potential_photo_section_heading == photo_database::IMAGE_ITEM_KEY.as_bytes()
            {

                let image_item_rating = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_ITEM_RATING_OFFSET, photo_database::IMAGE_ITEM_RATING_LEN);


                let image_item_orig_date_timestamp_raw = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_ITEM_ORIG_DATE_OFFSET, photo_database::IMAGE_ITEM_ORIG_DATE_LEN);


                let image_item_digitized_timestamp_raw = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_ITEM_DIGITIZED_DATE_OFFSET, photo_database::IMAGE_ITEM_DIGITIZED_DATE_LEN);

                let image_item_source_img_size = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_ITEM_SOURCE_IMG_SIZE_OFFSET, photo_database::IMAGE_ITEM_SOURCE_IMG_SIZE_LEN);

                // println!("ImageItem#{} : {} , ImgSize= {}, OrigDateTS= {} , DigitizedDateTS= {}", num_image_items, itunesdb_helpers::decode_itunes_stars(image_item_rating), image_item_source_img_size, itunesdb_helpers::get_timestamp_as_mac(image_item_orig_date_timestamp_raw as u64), itunesdb_helpers::get_timestamp_as_mac(image_item_digitized_timestamp_raw as u64));

                //println!("==========");
                num_image_items += 1;

                idx += photo_database::IMAGE_ITEM_LAST_OFFSET;

                // Populate existing image with properties
                //curr_img.original_date_epoch = image_item_orig_date_timestamp_raw as u64;
                curr_img.set_original_date(image_item_orig_date_timestamp_raw as u64);
                //curr_img.digitized_date_epoch = image_item_digitized_timestamp_raw as u64;
                curr_img.set_digitized_date(image_item_digitized_timestamp_raw as u64);
            }
            // Parse Image Name
            else if potential_photo_section_heading == photo_database::IMAGE_NAME_KEY.as_bytes()
            {

                let image_name_img_size = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_NAME_IMG_SIZE_OFFSET, photo_database::IMAGE_NAME_IMG_SIZE_LEN);
                
                // TODO: Figure out why the Image Height and Image Width are both 0
                let image_name_img_height = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_NAME_IMG_HEIGHT_OFFSET, photo_database::IMAGE_NAME_IMG_HEIGHT_LEN);

                let image_name_img_width = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::IMAGE_NAME_IMG_WIDTH_OFFSET, photo_database::IMAGE_NAME_IMG_WIDTH_LEN);

                // println!(
                //     "ImageName#{} : Size= {} bytes, Height={} , Width={}",
                //     num_image_names,
                //     image_name_img_size,
                //     image_name_img_height,
                //     image_name_img_width
                // );
                // println!("==========");

                num_image_names += 1;

                idx += photo_database::IMAGE_NAME_LAST_OFFSET;

                //curr_img.file_size_bytes = image_name_img_size as u64;
                curr_img.set_filesize(image_name_img_size as u64);
            }
            // Parse Photo Album
            else if potential_photo_section_heading == photo_database::PHOTO_ALBUM_KEY.as_bytes()
            {

                let photo_album_item_count = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::PHOTO_ALBUM_ALBUM_ITEM_CNT_OFFSET, photo_database::PHOTO_ALBUM_ALBUM_ITEM_CNT_LEN);

                // println!(
                //     "PhotoAlbum#{} : Item count#={}",
                //     num_photo_albums, photo_album_item_count
                // );
                // println!("==========");

                num_photo_albums += 1;

                idx += photo_database::PHOTO_ALBUM_LAST_OFFSET;
            }
            // Parse Data Object
            else if potential_photo_section_heading == photo_database::DATA_OBJECT_KEY.as_bytes()
            {
                let data_object_type = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::DATA_OBJECT_TYPE_OFFSET, photo_database::DATA_OBJECT_TYPE_LEN);

                if data_object_type == (photo_database::MhodType::ALBUM_NAME as u32) || data_object_type == (photo_database::MhodType::FILE_NAME as u32) {

                    let data_object_subcontainer_str_len = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_OFFSET, photo_database::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_LEN);

                    let data_object_subcontainer_encoding = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, photo_database::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET, photo_database::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_LEN);

                    if data_object_subcontainer_encoding == 0
                        || data_object_subcontainer_encoding == 1
                    {
                        // TODO: Figure out why I'm off by a width of 4 on the length.
                        // Same issue with UTF-16 encoding (below)

                        // let data_object_subcontainer_data = std::str::from_utf8(
                        //     &db_file_as_bytes[idx + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET + 4
                        //         ..idx
                        //             + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET
                        //             + data_object_subcontainer_str_len as usize
                        //             + 4],
                        // )
                        // .expect("Can't parse MHOD string");

                        let data_object_string_bytes = get_slice_from_offset_with_len(idx, &db_file_as_bytes, photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET, data_object_subcontainer_str_len as usize);

                        let data_object_subcontainer_data = std::str::from_utf8(&data_object_string_bytes).expect("Can't convert UTF-8 array to MHOD string");

                        //println!("MHOD substring = {}", data_object_subcontainer_data);

                        //curr_img.filename = data_object_subcontainer_data.to_string();
                        curr_img.set_filename(data_object_subcontainer_data.to_string());

                    } else if data_object_subcontainer_encoding == 2 {

                        let data_object_pairwise_combined = &helpers::return_utf16_from_utf8(&helpers::get_slice_from_offset_with_len(idx, &db_file_as_bytes, photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET, (data_object_subcontainer_str_len + 4) as usize));

                        let data_object_subcontainer_data =
                            String::from_utf16(&data_object_pairwise_combined)
                                .expect("Can't convert UTF-16 array to MHOD string");

                        //println!("MHOD substring = {}", data_object_subcontainer_data);

                        curr_img.filename = data_object_subcontainer_data;
                    }

                    // println!(
                    //     "String MHOD detected : Length= {}, Encoding (raw)= {}",
                    //     data_object_subcontainer_str_len, data_object_subcontainer_encoding
                    // );
                }

                // println!(
                //     "DataObject#{} info : Type={}",
                //     num_photo_data_objects, &photo_database::decode_mhod_type(data_object_type as u16)
                // );
                // println!("==========");

                num_photo_data_objects += 1;

                idx += photo_database::DATA_OBJECT_LAST_OFFSET;


                // Once you've parsed the data object, all properties for the "current" image have been set
                // so store the current one, then 'reset' it

                if (curr_img.filename.len() > 0) && (curr_img.file_size_bytes > 0) && (curr_img.original_date_epoch != 0) && (curr_img.digitized_date_epoch != 0) {
                    images_found.push(curr_img);
                    curr_img = itunesdb::Image::default();
                }
            }

            idx += DEFAULT_SUBSTRUCTURE_SIZE;

        } // end while

        println!("{} images found", images_found.len());

        for image in images_found.iter() {
            println!("Image filename = {}, Image size (raw) = {}, Image size = {}", image.filename, image.file_size_bytes, image.file_size_human_readable);
        }

    } else if itunesdb_file_type == "music" {

        let mut idx = 0;

        while idx < db_file_as_bytes.len() {

            let potential_section_heading = &db_file_as_bytes[idx .. idx + DEFAULT_SUBSTRUCTURE_SIZE];

            // Parse Database Object
            if potential_section_heading == iTunesDB::DATABASE_OBJECT_KEY.as_bytes() {

                let db_language_raw = get_slice_from_offset_with_len(idx, &db_file_as_bytes, iTunesDB::DATABASE_OBJECT_LANGUAGE_OFFSET, iTunesDB::DATABASE_OBJECT_LANGUAGE_LEN);

                let db_language = std::str::from_utf8(&db_language_raw).expect("Can't parse database language string");

                println!("File {} is using language: {}, and has iTunes version: {}", itunesdb_filename, db_language, iTunesDB::parse_version_number(get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::DATABASE_OBJECT_VERSION_NUMBER_OFFSET, iTunesDB::DATABASE_OBJECT_VERSION_NUMBER_LEN)));

                idx += iTunesDB::DATABASE_OBJECT_LAST_OFFSET;
            }

            
            // Parse DataSet
            else if potential_section_heading == iTunesDB::DATASET_KEY.as_bytes() {

                let dataset_type_raw = get_slice_from_offset_with_len(idx, &db_file_as_bytes, iTunesDB::DATASET_TYPE_OFFSET, iTunesDB::DATASET_TYPE_LEN);
                
                println!("Dataset Type: {}", iTunesDB::parse_dataset_type(dataset_type_raw[0] as u32));

                idx += iTunesDB::DATASET_LAST_OFFSET;
            }

            // Parse TrackList
            else if potential_section_heading == iTunesDB::TRACKLIST_KEY.as_bytes() {

                let num_songs_in_db = get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACKLIST_NUM_SONGS_OFFSET, iTunesDB::TRACKLIST_NUM_SONGS_LEN);

                println!("{} songs total", num_songs_in_db);
                
                idx += iTunesDB::TRACKLIST_LAST_OFFSET;

            }

            else if potential_section_heading == iTunesDB::TRACK_ITEM_KEY.as_bytes() {

                let mut track_item_info : String = String::new();

                write!(track_item_info, "========== Track #{} of {} ", helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_NUMBER_OFFSET, iTunesDB::TRACK_ITEM_TRACK_NUMBER_LEN), helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_NUM_TRACKS_IN_ALBUM_OFFSET, iTunesDB::TRACK_ITEM_NUM_TRACKS_IN_ALBUM_LEN) ).unwrap();

                let num_discs = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_OFFSET, iTunesDB::TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_LEN);

                // Only print disc info if current song is part of multi-disc set
                if num_discs > 0 {

                    let tracks_current_disc_num = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_DISC_NUMBER_OFFSET, iTunesDB::TRACK_ITEM_TRACK_DISC_NUMBER_LEN);

                    write!(track_item_info, " | 💿 #{} of {}", tracks_current_disc_num, num_discs).unwrap();
                }

                write!(track_item_info, "==========\n").unwrap();
                
                let track_filetype_raw = &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_TRACK_FILETYPE_OFFSET .. idx + iTunesDB::TRACK_ITEM_TRACK_FILETYPE_OFFSET + iTunesDB::TRACK_ITEM_TRACK_FILETYPE_LEN];

                // TODO: encapsulate this logic elsewhere
                if helpers::build_le_u32_from_bytes(track_filetype_raw) == 0 {

                    println!("Track Item file type missing. Is this is a 1st - 4th gen iPod?");
                } else {

                    let track_item_extension = iTunesDB::decode_track_item_filetype(track_filetype_raw);
                    write!(track_item_info, "Track extension: '{}' | ", track_item_extension).unwrap();
                }

                let track_media_type_raw =  &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_TRACK_MEDIA_TYPE_OFFSET .. idx + iTunesDB::TRACK_ITEM_TRACK_MEDIA_TYPE_OFFSET + iTunesDB::TRACK_ITEM_TRACK_MEDIA_TYPE_LEN];

                let track_movie_file_flag = get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_OFFSET, iTunesDB::TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_LEN);

                let (track_media_type_name, track_media_type_enum) = iTunesDB::decode_track_media_type(track_media_type_raw);

                write!(track_item_info, "Movie file flag: {} | Media Type: {} \n", (track_movie_file_flag == 1), track_media_type_name).unwrap();

                //if track_media_type_enum == iTunesDB::MediaType::TELEVISION {
                if matches!(track_media_type_enum, iTunesDB::MediaType::TELEVISION) {

                    let season_number = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_SEASON_NUMBER_OFFSET, iTunesDB::TRACK_ITEM_TRACK_SEASON_NUMBER_LEN);

                    let episode_number = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_EPISODE_NUMBER_OFFSET, iTunesDB::TRACK_ITEM_TRACK_EPISODE_NUMBER_LEN);

                    write!(track_item_info, "Season #{} Episode #{}", season_number, episode_number).unwrap();
                }

                let track_advanced_audio_type = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_ADVANCED_TRACK_TYPE_OFFSET, iTunesDB::TRACK_ITEM_ADVANCED_TRACK_TYPE_LEN);

                write!(track_item_info, "Experimental(!) advanced audio info: {} \n", iTunesDB::decode_track_audio_type(track_advanced_audio_type)).unwrap();

                let apple_user_id = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_USER_ID_OFFSET, iTunesDB::TRACK_ITEM_TRACK_USER_ID_LEN);

                if apple_user_id != 0 {
                    write!(track_item_info, "Apple User ID: {} \n", apple_user_id).unwrap();
                }

                let track_bitrate_type_raw = &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET .. idx + iTunesDB::TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET + iTunesDB::TRACK_ITEM_TRACK_BITRATE_SETTING_LEN];

                let track_bitrate = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_BITRATE_OFFSET, iTunesDB::TRACK_ITEM_TRACK_BITRATE_LEN);

                let track_sample_rate_raw = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_SAMPLE_RATE_OFFSET, iTunesDB::TRACK_ITEM_TRACK_SAMPLE_RATE_LEN);

                let track_volume_setting = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_VOLUME_OFFSET, iTunesDB::TRACK_ITEM_TRACK_VOLUME_LEN);

                let track_bpm = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_BPM_OFFSET, iTunesDB::TRACK_ITEM_TRACK_BPM_LEN);

                write!(track_item_info, "[Audio info] {} kbps ({}) ~ {} | {} bpm |  🔈 adj. {} \n", track_bitrate, iTunesDB::decode_track_bitrate_type_setting(track_bitrate_type_raw), iTunesDB::decode_track_samplerate(track_sample_rate_raw), track_bpm, track_volume_setting).unwrap();

                let track_size_bytes = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_FILE_SIZE_BYTES_OFFSET, iTunesDB::TRACK_ITEM_TRACK_FILE_SIZE_BYTES_LEN);

                write!(track_item_info, "Track size: {} bytes | ", track_size_bytes).unwrap();

                let track_length_raw = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_OFFSET, iTunesDB::TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_LEN);

                let track_start_time_offset = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_START_TIME_OFFSET, iTunesDB::TRACK_ITEM_TRACK_START_TIME_LEN);


                let track_stop_time_offset = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_STOP_TIME_OFFSET, iTunesDB::TRACK_ITEM_TRACK_STOP_TIME_LEN);

                write!(track_item_info, "{} \n", iTunesDB::get_track_length_info(track_length_raw, track_start_time_offset, track_stop_time_offset)).unwrap();


                let track_play_count = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_PLAY_COUNT_OFFSET, iTunesDB::TRACK_ITEM_TRACK_PLAY_COUNT_LEN);


                let track_skipped_count = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_SKIPPED_COUNT_OFFSET, iTunesDB::TRACK_ITEM_TRACK_SKIPPED_COUNT_LEN);

                // TODO: WHy are the last played timestamps zero??

                let track_last_played_timestamp = get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_OFFSET, iTunesDB::TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_LEN);

                let track_last_skipped_timestamp = get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_OFFSET, iTunesDB::TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_LEN);

                let track_skip_when_shuffle_setting = &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_OFFSET .. idx + iTunesDB::TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_OFFSET + iTunesDB::TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_LEN];

                write!(track_item_info, "Play/Skip statistics: # of plays: {} , Last played on: {} | # of skips: {}, Last skipped on: {} (Skip when shuffling? {}) ", track_play_count, track_last_played_timestamp, track_skipped_count, track_last_skipped_timestamp, track_skip_when_shuffle_setting[0] ).unwrap();

                let track_is_compilation_setting_raw = &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_IS_COMPILATION_SETTING_OFFSET .. idx + iTunesDB::TRACK_ITEM_IS_COMPILATION_SETTING_OFFSET + iTunesDB::TRACK_ITEM_IS_COMPILATION_SETTING_LEN];

                let track_has_lyrics_setting_raw = &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_OFFSET .. idx + iTunesDB::TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_OFFSET + iTunesDB::TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_LEN];

                write!(track_item_info, " \n Is part of compilation? {} , Has lyrics? {}", track_is_compilation_setting_raw[0], track_has_lyrics_setting_raw[0]).unwrap();

                let track_rating = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_RATING_OFFSET, iTunesDB::TRACK_ITEM_TRACK_RATING_LEN);

                let track_prev_rating = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_PREVIOUS_RATING_OFFSET, iTunesDB::TRACK_ITEM_TRACK_PREVIOUS_RATING_LEN);

                write!(track_item_info, "\n Rating info: Current rating: {} | Previous rating: {} \n", itunesdb_helpers::decode_itunes_stars(track_rating), itunesdb_helpers::decode_itunes_stars(track_prev_rating)).unwrap();

                let gapless_playback_setting_for_track = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_OFFSET, iTunesDB::TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_LEN);

                if gapless_playback_setting_for_track == 1 {

                    let num_beginning_silence_samples = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_OFFSET, iTunesDB::TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_LEN);

                    let num_ending_silence_samples = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_OFFSET, iTunesDB::TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_LEN);

                    // let num_total_samples = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_NUM_SAMPLES_OFFSET, iTunesDB::TRACK_ITEM_TRACK_NUM_SAMPLES_LEN);

                    let num_total_samples = helpers::get_slice_as_le_u64(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_NUM_SAMPLES_OFFSET, iTunesDB::TRACK_ITEM_TRACK_NUM_SAMPLES_LEN);

                    write!(track_item_info, "[Gapless playback info] # of silent samples ({} at start, {} at end) - Total {}\n", num_beginning_silence_samples, num_ending_silence_samples, num_total_samples).unwrap();
                }

                let track_crossfade_setting = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_CROSSFADING_SETTING_OFFSET , iTunesDB::TRACK_ITEM_TRACK_CROSSFADING_SETTING_LEN);

                write!(track_item_info, "Crossfade: {} | ", (if track_crossfade_setting == 1 {"Enabled"} else {"Disabled"})).unwrap();

                let track_has_artwork_setting = &db_file_as_bytes[idx + iTunesDB::TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_OFFSET .. idx + iTunesDB::TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_OFFSET + iTunesDB::TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_LEN];

                // TODO: Encapsulate this logic elsewhere
                if track_has_artwork_setting[0] == 0x01 {
                    
                    let track_associated_artwork_size = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_OFFSET, iTunesDB::TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_LEN);

                    write!(track_item_info, "🎨 artwork size: {} bytes \n", track_associated_artwork_size).unwrap();
                }

                let track_year_released = get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_YEAR_PUBLISHED_OFFSET, iTunesDB::TRACK_ITEM_TRACK_YEAR_PUBLISHED_LEN);

                let track_modified_timestamp = get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_MODIFIED_TIME_OFFSET, iTunesDB::TRACK_ITEM_TRACK_MODIFIED_TIME_LEN);

                let track_added_timestamp = get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_ADDED_TIMESTAMP_OFFSET, iTunesDB::TRACK_ITEM_TRACK_ADDED_TIMESTAMP_LEN);

                let track_published_to_store_timestamp: chrono::DateTime<chrono::Utc> = get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_OFFSET, iTunesDB::TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_LEN);

                write!(track_item_info, "\n 🗓️ | Track year (from title): {} | Published to iTunes on: {} | Added to library on: {} | Last modified: {} \n ", track_year_released, track_published_to_store_timestamp, track_added_timestamp, track_modified_timestamp).unwrap();

                println!("{} \n", track_item_info);

                idx += iTunesDB::TRACK_ITEM_LAST_OFFSET;
            }

            else if potential_section_heading == iTunesDB::PLAYLIST_KEY.as_bytes() {

                let mut playlist_info : String = "==== ".to_string();

                let is_master_playlist_setting = &db_file_as_bytes[idx + iTunesDB::PLAYLIST_IS_MASTER_PLAYLIST_SETTING_OFFSET .. idx + iTunesDB::PLAYLIST_IS_MASTER_PLAYLIST_SETTING_OFFSET + iTunesDB::PLAYLIST_IS_MASTER_PLAYLIST_SETTING_LEN];

                if is_master_playlist_setting[0] == 1 {
                    write!(playlist_info, "Master ").unwrap();
                }
                
                write!(playlist_info, "Playlist found!").unwrap();
                
                let playlist_created_timestamp = helpers::get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::PLAYLIST_CREATED_TIMESTAMP_OFFSET, iTunesDB::PLAYLIST_CREATED_TIMESTAMP_LEN);

                write!(playlist_info, " | Playlist created at: {} ", playlist_created_timestamp).unwrap();

                let playlist_sort_order = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::PLAYLIST_PLAYLIST_SORT_ORDER_OFFSET, iTunesDB::PLAYLIST_PLAYLIST_SORT_ORDER_LEN);

                write!(playlist_info, "| {} \n", iTunesDB::decode_playlist_sort_order(playlist_sort_order)).unwrap();

                println!("{} ====", playlist_info);

                idx += iTunesDB::PLAYLIST_LAST_OFFSET;
            }

            else if potential_section_heading == iTunesDB::PLAYLIST_ITEM_KEY.as_bytes() {

                let mut playlist_item_info : String = "-----".to_string();

                let playlist_item_added_timestamp = helpers::get_slice_as_mac_timestamp(idx, &db_file_as_bytes, iTunesDB::PLAYLIST_ITEM_ADDED_TIMESTAMP_OFFSET, iTunesDB::PLAYLIST_ITEM_ADDED_TIMESTAMP_LEN);

                write!(playlist_item_info, " | Date added to playlist: {}", playlist_item_added_timestamp).unwrap();

                println!("{}  -----\n", playlist_item_info);

                idx += iTunesDB::PLAYLIST_ITEM_LAST_OFFSET;
            }

            else if potential_section_heading == iTunesDB::ALBUM_LIST_KEY.as_bytes() {

                let mut album_list_info : String = "~~~~~~~".to_string();

                let album_item_total_num_songs = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::ALBUM_LIST_TOTAL_NUM_SONGS_OFFSET, iTunesDB::ALBUM_LIST_TOTAL_NUM_SONGS_LEN);

                write!(album_list_info, " {} songs in Album List", album_item_total_num_songs).unwrap();

                println!("{}  ~~~~~~~\n", album_list_info);

                idx += iTunesDB::ALBUM_LIST_LAST_OFFSET;

            }

            // else if potential_section_heading == iTunesDB::ALBUM_ITEM_KEY.as_bytes() {

            //     let album_item_info : String = "######## Album item found! | ".to_string();

            //     // write!(album_item_info, " {} ########\n", itunesdb_helpers::get_timestamp_as_mac(helpers::build_le_u32_from_bytes(album_item_unknown_timestamp_raw) as u64)).unwrap();

            //     println!("{} ########\n", album_item_info);

            //     idx += iTunesDB::ALBUM_ITEM_LAST_OFFSET;

            // }
            
            else if potential_section_heading == iTunesDB::DATA_OBJECT_KEY.as_bytes() {

                let mut data_object_info : String = "%%%%%%% Data Object found!\n".to_string();

                let data_object_type_raw = helpers::get_slice_as_le_u32(idx,  &db_file_as_bytes, iTunesDB::DATA_OBJECT_TYPE_OFFSET, iTunesDB::DATA_OBJECT_TYPE_LEN);

                write!(data_object_info, "Type (raw) = {}, Decoded= '{}' | ", data_object_type_raw, iTunesDB::decode_data_object_type(data_object_type_raw)).unwrap();

                if iTunesDB::is_data_object_type_string(data_object_type_raw) { // Means its a 'string' MHOD
        
                    let data_object_string_len = helpers::get_slice_as_le_u32(idx, &db_file_as_bytes, iTunesDB::DATA_OBJECT_STRING_LENGTH_OFFSET, iTunesDB::DATA_OBJECT_STRING_LENGTH_LEN);

                    let data_object_str_bytes = helpers::get_slice_from_offset_with_len(idx, &db_file_as_bytes, iTunesDB::DATA_OBJECT_STRING_LOCATION_OFFSET, data_object_string_len as usize);

                    // let data_object_str = std::str::from_utf8(&data_object_str_bytes).expect("Can't parse string data object!");
                    let data_object_str = String::from_utf16(&helpers::return_utf16_from_utf8(&data_object_str_bytes)).expect("Can't decode string to UTF-16");

                    write!(data_object_info, "Length= {} | Value: '{}'", data_object_string_len, data_object_str).unwrap();
                }

                else {

                    if (data_object_type_raw == iTunesDB::HandleableDataObjectType::PODCAST_ENCLOSURE_URL as u32) || (data_object_type_raw == iTunesDB::HandleableDataObjectType::PODCAST_RSS_URL as u32) {

                        let podcast_url = iTunesDB::decode_podcast_urls(idx, &db_file_as_bytes);

                        write!(data_object_info, "Podcast discovered, with URL: {}", podcast_url).unwrap();
                    }
                }

                println!("{} %%%%%%% \r\n", data_object_info);

                idx += iTunesDB::DATA_OBJECT_LAST_OFFSET;
            }


            idx += DEFAULT_SUBSTRUCTURE_SIZE;
        } // End 'music' parser
        
    }

}
