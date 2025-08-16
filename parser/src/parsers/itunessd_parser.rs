use crate::constants::itunessd_constants;
use crate::helpers::helpers;
use crate::itunessd;

pub fn parse_itunessd_file(itunessd_file_as_bytes: Vec<u8>) {
    let num_songs = helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
        0,
        &itunessd_file_as_bytes,
        itunessd_constants::ITUNESSD_NUM_SONGS_OFFSET,
        itunessd_constants::ITUNESSD_NUM_SONGS_LEN,
    ));

    println!("iTunesSD file has {} songs", num_songs);

    let itunessd_header_size =
        helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
            0,
            &itunessd_file_as_bytes,
            itunessd_constants::ITUNESSD_HEADER_SIZE_OFFSET,
            itunessd_constants::ITUNESSD_HEADER_SIZE_LEN,
        ));

    if itunessd_header_size != itunessd_constants::ITUNESSD_HEADER_SIZE_EXPECTED_VALUE as u32 {
        panic!(
            "Invalid iTunesSD header size value of '{}', expected '{}'",
            itunessd_header_size,
            itunessd_constants::ITUNESSD_HEADER_SIZE_EXPECTED_VALUE
        );
    }

    println!("==========");

    let mut file_idx: usize = itunessd_header_size as usize;
    let mut num_songs_parsed : usize = 1;

    while file_idx < itunessd_file_as_bytes.len() - itunessd_constants::ITUNESSD_ENTRY_SIZE {
        // Now parse the individual song entries... start by checking that the size of the entry object matches the known value

        println!("---------- Song {} of {} ----------", num_songs_parsed, num_songs);

        let entry_size =
            helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                file_idx,
                &itunessd_file_as_bytes,
                0,
                itunessd_constants::ITUNESSD_ENTRY_SIZE_LEN,
            ));

        if entry_size != itunessd_constants::ITUNESSD_ENTRY_SIZE as u32 {
            panic!("Invalid iTunesSD entry size value of '{}'", entry_size);
        }

        let start_time =
            helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                file_idx,
                &itunessd_file_as_bytes,
                itunessd_constants::ITUNESSD_START_TIME_OFFSET,
                itunessd_constants::ITUNESSD_START_TIME_LEN,
            ));

        if start_time != 0 {
            println!("Start Time: {}", start_time);
        }

        let stop_time = helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
            file_idx,
            &itunessd_file_as_bytes,
            itunessd_constants::ITUNESSD_STOP_TIME_OFFSET,
            itunessd_constants::ITUNESSD_STOP_TIME_LEN,
        ));

        if stop_time != 0 {
            println!("Stop Time: {}", stop_time);
        }

        let _volume_raw =
            helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                file_idx,
                &itunessd_file_as_bytes,
                itunessd_constants::ITUNESSD_VOLUME_OFFSET,
                itunessd_constants::ITUNESSD_VOLUME_LEN,
            ));

        let file_type_raw =
            helpers::build_be_u32_from_bytes(&helpers::get_slice_from_offset_with_len(
                file_idx,
                &itunessd_file_as_bytes,
                itunessd_constants::ITUNESSD_FILE_TYPE,
                itunessd_constants::ITUNESSD_FILE_TYPE_LEN,
            ));

        println!(
            "File Type: {}",
            itunessd::ITunesSdFileType::try_from(file_type_raw).unwrap_or_else(|err| {
                panic!("Error parsing iTunesSD file type: {}", err)
            })
        );

        let song_filename = String::from_utf16(&helpers::return_utf16_from_utf8(
            &helpers::get_slice_from_offset_with_len(
                file_idx,
                &itunessd_file_as_bytes,
                itunessd_constants::ITUNESSD_SONG_ENTRY_FILENAME_OFFSET,
                itunessd_constants::ITUNESSD_SONG_ENTRY_FILENAME_LEN,
            ),
        ))
        .unwrap();

        // This string contains null bytes at the end (we don't know it's length),
        // which look like: "/iPod_Control/Music/F00/XZYL.m4a\0\0\0\0\0\"
        // so we need to trim that before printing it
        // https://stackoverflow.com/questions/49406517/how-to-remove-trailing-null-characters-from-string
        println!(
            "Song Filename: {:?}",
            song_filename.trim_matches(char::from(0))
        );

        println!("--------------------");
        
        num_songs_parsed += 1;

        file_idx += itunessd_constants::ITUNESSD_ENTRY_SIZE;
    }
}
