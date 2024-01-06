

use crate::constants::playcounts_constants;
use crate::constants::itunesdb_constants;

use crate::helpers::itunesdb_helpers;
use crate::helpers::helpers;

pub fn parse_playcounts(itunesdb_file_as_bytes: Vec<u8>, mut csv_writer_obj : csv::Writer<std::fs::File>) {


    let mut idx = 0;

    while idx < (itunesdb_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {

        let playcount_file_heading : &[u8] = &itunesdb_file_as_bytes[idx .. idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

        if playcount_file_heading == playcounts_constants::PLAYCOUNTS_OBJECT_KEY.as_bytes() {

            let pc_entry_len = helpers::get_slice_as_le_u32(idx, &itunesdb_file_as_bytes, playcounts_constants::PLAYCOUNTS_ENTRY_LENGTH_OFFSET, playcounts_constants::PLAYCOUNTS_ENTRY_LENGTH_LEN);

            let num_entries = helpers::get_slice_as_le_u32(idx, &itunesdb_file_as_bytes, playcounts_constants::PLAYCOUNTS_NUM_ENTRIES_OFFSET, playcounts_constants::PLAYCOUNTS_NUM_ENTRIES_LEN);

            println!("Playcounts file has {} songs, and each entry has length {}", num_entries, pc_entry_len);

            println!("===========");

            if num_entries > 1 {

                csv_writer_obj.write_record(
                    &["# of times played (since last sync)",
                "# of times skipped (since last sync)",
                "Rating",
                "Last played timestamp",
                "Last played (epoch)",
                "Audio playback bookmark (ms)"]).expect("Error creating header column in CSV");
            }

            for track_idx in 0 .. (num_entries as usize) {

                let pc_starting_idx = (track_idx * pc_entry_len as usize) + playcounts_constants::PLAYCOUNTS_FILE_HEADER_LENGTH;

                let num_plays = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &itunesdb_file_as_bytes, playcounts_constants::PC_ENTRY_NUM_PLAYS_OFFSET, playcounts_constants::PC_ENTRY_NUM_PLAYS_LEN);

                let num_skips : u32 = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &itunesdb_file_as_bytes, playcounts_constants::PC_ENTRY_NUM_SKIPS_OFFSET, playcounts_constants::PC_ENTRY_NUM_SKIPS_LEN);
                
                let raw_rating = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &itunesdb_file_as_bytes, playcounts_constants::PC_ENTRY_RATING_OFFSET, playcounts_constants::PC_ENTRY_RATING_LEN) as u8;

                let last_played_timestamp = helpers::get_slice_as_le_u64(idx + pc_starting_idx, &itunesdb_file_as_bytes, playcounts_constants::PC_ENTRY_AUDIO_BOOKMARK_MS_OFFSET, playcounts_constants::PC_ENTRY_AUDIO_BOOKMARK_MS_LEN);
                
                let audio_bookmark_ms = helpers::get_slice_as_le_u32(idx + pc_starting_idx, &itunesdb_file_as_bytes, playcounts_constants::PC_ENTRY_LAST_SKIPPED_TIMESTAMP_OFFSET, playcounts_constants::PC_ENTRY_LAST_SKIPPED_TIMESTAMP_LEN);

                //println!("Song ID #{} of {} has been played {} times, skipped {} times, and has rating {} ", track_idx, num_entries, num_plays, num_skips, itunesdb_helpers::decode_itunes_stars(raw_track_rating as u8));

                // Vectors have to be all the same type, hence the `to_string()`
                csv_writer_obj.write_record(
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