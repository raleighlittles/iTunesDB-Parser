use crate::constants::itunesdb_constants;
use crate::constants::itunessd_3g_constants;
use crate::helpers::helpers;
use crate::itunessd;

pub fn parse_itunessd_3rdgen_file(
    itunessd_file_as_bytes: Vec<u8>,
    mut csv_writer_obj: csv::Writer<std::fs::File>,
) {
    let mut idx: usize = 0;

    while idx < itunessd_file_as_bytes.len() {
        // Ensure at least 4 bytes are available before slicing
        if idx + 4 > itunessd_file_as_bytes.len() {
            println!("Insufficient bytes available for parsing iTunesSD header.");
            break;
        }
        let itunessd_3rd_gen_heading: &[u8] = &itunessd_file_as_bytes[idx..idx + 4];

        if itunessd_3rd_gen_heading
            == itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_HEADER_ID_KEY.as_bytes()
        {
            let shuffle_db_header_length =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_HEADER_LENGTH_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_HEADER_LENGTH_LEN,
                ));

            if shuffle_db_header_length
                != itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_HEADER_EXPECTED_LENGTH as u32
            {
                panic!(
                    "Invalid iTunesSD header length value of '{}', expected '{}'",
                    shuffle_db_header_length,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_HEADER_EXPECTED_LENGTH
                );
            }

            let volume_lock_value =  helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                idx,
                &itunessd_file_as_bytes,
                itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_MAX_VOLUME_OFFSET,
                itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_MAX_VOLUME_LEN,
            ));

            if volume_lock_value != 0 {
                println!(
                    "Warning! Volume lock is enabled (value: {})",
                    volume_lock_value
                );
            }

            let num_tracks_total =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_NUM_TRACKS_TOTAL_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_NUM_TRACKS_TOTAL_LEN,
                ));

            let num_tracks_music_only =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_NUM_MUSIC_ONLY_TRACKS_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_NUM_MUSIC_ONLY_TRACKS_LEN,
                ));

            let num_playlists =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_NUM_PLAYLISTS_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_SHUFFLE_DB_NUM_PLAYLISTS_LEN,
                ));

            if num_tracks_total == 0 {
                println!("No tracks found in this iTunesSD file.");
                return;
            }

            println!(
                "iTunesSD file has {} tracks overall ({} music-only), {} playlists",
                num_tracks_total, num_tracks_music_only, num_playlists
            );

            // Write CSV header row
            csv_writer_obj
                .write_record(&[
                    "Filename",
                    "TrackNum",
                    "FileType",
                    "StartTimeMS",
                    "StopTimeMS",
                    "BookmarkMS",
                    "NumSamples",
                    "DiscNum",
                    "AlbumID",
                    "ArtistID",
                    "VolumeGain",
                    "DatabaseID",
                ])
                .unwrap();

            println!("--------------------");

        } else if itunessd_3rd_gen_heading
            == itunessd_3g_constants::ITUNESSD_3RDGEN_TRACKS_HEADER_ID_KEY.as_bytes()
        {
            let num_tracks_in_group =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACKS_NUM_TRACKS_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACKS_NUM_TRACKS_LEN,
                ));

            println!("Found {} tracks in this group", num_tracks_in_group);

        } else if itunessd_3rd_gen_heading
            == itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_HEADER_ID_KEY.as_bytes()
        {
            let track_entry_header_length =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_HEADER_SIZE_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_HEADER_SIZE_LEN,
                ));

            if track_entry_header_length
                != itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_HEADER_EXPECTED_SIZE as u32
            {
                panic!(
                    "Invalid iTunesSD track entry header length value of '{}', expected '{}'",
                    track_entry_header_length,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_HEADER_EXPECTED_SIZE
                );
            }

            let start_time_ms =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_START_TIME_MS_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_START_TIME_MS_LEN,
                ));

            let stop_time_ms =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_STOP_TIME_MS_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_STOP_TIME_MS_LEN,
                ));

            let volume_gain =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_VOLUME_GAIN,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_VOLUME_GAIN_LEN,
                ));

            let file_type =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_FILE_TYPE_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_FILE_TYPE_LEN,
                ));

            let filename_bytes = &helpers::get_slice_from_offset_with_len(
                idx,
                &itunessd_file_as_bytes,
                itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_FILENAME_OFFSET,
                itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_FILENAME_LEN,
            );

            let filename = String::from_utf8_lossy(filename_bytes)
                .trim_end_matches('\0')
                .to_string();

            let bookmark_ms =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_BOOKMARK_MS_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_BOOKMARK_MS_LEN,
                ));
            let _pregap =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_PREGAP_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_PREGAP_LEN,
                ));
            let _postgap =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_POSTGAP_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_POSTGAP_LEN,
                ));
            let num_samples =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_NUM_SAMPLES_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_NUM_SAMPLES_LEN,
                ));
            let _gapless_data =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_GAPLESS_DATA_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_GAPLESS_DATA_LEN,
                ));
            let album_id =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_ALBUM_ID_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_ALBUM_ID_LEN,
                ));
            let track_num =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_TRACK_NUM_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_TRACK_NUM_LEN,
                ));
            let disc_num =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_DISC_NUM_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_DISC_NUM_LEN,
                ));

            let database_id: String = helpers::get_slice_from_offset_with_len(
                idx,
                &itunessd_file_as_bytes,
                itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_DATABASEID_OFFSET,
                itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_DATABASEID_LEN,
            )
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

            let artist_id =
                helpers::build_le_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                    idx,
                    &itunessd_file_as_bytes,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_ARTIST_ID_OFFSET,
                    itunessd_3g_constants::ITUNESSD_3RDGEN_TRACK_ENTRY_ARTIST_ID_LEN,
                ));

            // println!("Track entry found - Start Time: {} ms, Stop Time: {} ms, Volume Gain: {}, File Type: {}, Filename: '{}', Bookmark: {} ms, Pregap: {}, Postgap: {}, Num Samples: {}, Gapless Data: {}, Album ID: {}, Track Num: {}, Disc Num: {}, Database ID: {}, Artist ID: {}",
            //     start_time_ms,
            //     stop_time_ms,
            //     volume_gain,
            //     file_type,
            //     filename,
            //     bookmark_ms,
            //     pregap,
            //     postgap,
            //     num_samples,
            //     gapless_data,
            //     album_id,
            //     track_num,
            //     disc_num,
            //     database_id,
            //     artist_id
            // );

            csv_writer_obj
                .write_record(&[
                    filename,
                    track_num.to_string(),
                    itunessd::ITunesSdFileType::try_from(file_type)
                        .unwrap_or_else(|err| panic!("Error parsing iTunesSD file type: {}", err))
                        .to_string(),
                    start_time_ms.to_string(),
                    stop_time_ms.to_string(),
                    bookmark_ms.to_string(),
                    num_samples.to_string(),
                    disc_num.to_string(),
                    album_id.to_string(),
                    artist_id.to_string(),
                    volume_gain.to_string(),
                    database_id.to_uppercase(),
                ])
                .unwrap();
        } // end else-if

        idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE;
    }
}
