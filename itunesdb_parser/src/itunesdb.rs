/**
 * File: itunesdb.rs
 *
 * Provides the iTunesDB constants, and associated functionality. See itunesdb-doc for more info.
 * Constants are split into "sections", ie 'Image List', 'Image Item'. Each section has a header,
 * and within that header are fields, which have two values associated with them:
 * (1) an offset -- where it is found in the header structure
 * (2) a length.
 *
 * Each section of constants has a "last offset" value that indicates the offset of the last item in the header.
 */


use crate::helpers;



pub mod iTunesDB {

    pub struct Song {
        pub file_extension : String,
        pub bitrate_kbps : String,
        pub sample_rate_hz : u64,
        pub file_size_bytes : u32, // iPod file systems use FAT
        pub song_length_s : u64,
        pub song_length : String,
        pub song_year : u16,
        pub song_title : String,
        pub song_artist : String,
        pub song_composer : String,
        pub song_album : String,
        pub song_genre : String,
        pub song_comment : String,
        /// As far as I can tell from looking at the output, this field
        /// is always the last one to get populated
        pub song_filename : String
    }

    pub fn parse_version_number(version_number : u32) -> String {

        let itunes_version : String;

        if version_number == 0x09 {
            itunes_version = "iTunes 4.2".to_string();
        }

        else if version_number == 0x0A {
            itunes_version = "iTunes 4.5".to_string();
        }

        else if version_number == 0x0B {
            itunes_version = "iTunes 4.7".to_string();
        }

        else if version_number == 0x0C {
            itunes_version = "iTunes 4.71 or 4.8".to_string();
        }

        else if version_number == 0x0D {
            itunes_version = "iTunes 4.9".to_string();
        }

        else if version_number == 0x0E {
            itunes_version = "iTunes 5".to_string();
        }

        else if version_number == 0x0F {
            itunes_version = "iTunes 6".to_string();
        }

        else if version_number == 0x10 {
            itunes_version = "iTunes 6.0.1".to_string();
        }

        else if version_number == 0x011 {
            itunes_version = "iTunes 6.0.2 - 6.0.4".to_string();
        }

        else if version_number == 0x12 {
            itunes_version = "iTunes 6.0.5".to_string();
        }

        else if version_number == 0x13 {
            itunes_version = "iTunes 7.0".to_string();
        }

        else if version_number == 0x14 {
            itunes_version = "iTunes 7.1".to_string();
        }

        else if version_number == 0x15 {
            itunes_version = "iTunes 7.2".to_string();
        }

        else if version_number == 0x17 {
            itunes_version = "iTunes 7.3.0".to_string();
        }

        else if version_number == 0x18 {
            itunes_version = "Tunes 7.3.1 - 7.3.2".to_string();
        }

        else if version_number == 0x19 {
            itunes_version = "iTunes 7.4".to_string();
        }

        else {
            itunes_version = format!("N/A ({})", version_number);
        }

        return itunes_version;
    }

    pub fn parse_dataset_type(dataset_type_raw : u32) -> String {
        
        let dataset_type : String;

        if dataset_type_raw == 1 {
            dataset_type = "Track List".to_string();
        }

        else if dataset_type_raw == 2 {
            dataset_type = "Playlist List".to_string();
        }

        else if dataset_type_raw == 3 {
            dataset_type = "Podcast List".to_string();
        }

        else if dataset_type_raw == 4 {
            dataset_type = "Album List".to_string();
        }

        else if dataset_type_raw == 5 {
            dataset_type = "New Playlist List (smart playlists)".to_string();
        }

        else {
            dataset_type = format!("N/A ({})", dataset_type_raw);
        }

        return dataset_type;
    }



    // From the wiki: "the file's type [..] an ANSI string padded with spaces"
    pub fn decode_track_item_filetype(file_type_raw : &[u8]) -> String {

        let mut filetype_str : String = String::from(std::str::from_utf8(&file_type_raw).expect("Can't parse Track Item file type raw string"));

        // The Track Item filetype has spaces in it, we obviously don't want that
        filetype_str.retain(|ch : char| !ch.is_whitespace());

        // Technically the characters are stored in reverse-endian order, but they're ASCII
        // (8-bit) so you can just achieve the same result by reversing the string
        return filetype_str.chars().rev().collect();
    }

    pub fn decode_track_bitrate_type_setting(bitrate_type_raw : &[u8]) -> String {

        let bitrate_type : String;

        if bitrate_type_raw[0] == 0x0 {
            bitrate_type = "Constant bitrate".to_string();
        }

        else if bitrate_type_raw[0] == 0x1 {
            bitrate_type = "Variable bitrate/AAC-encoded".to_string();
        }

        else {
            bitrate_type = "Unable to determine if constant or variable bitrate".to_string();
        }

        return bitrate_type;
    }

    /// Each track has both a real duration (how long the actual song is), and, also:
    /// (1) a 'start' time, which is where the song starts playing
    /// (2) a 'end' time, which is where the song starts playing
    /// This function shows both the "experimental" song duration (which takes into account the difference above) 
    /// and the "theoretical" song duration
    pub fn get_track_length_info(track_length_ms : u32, start_time_offset_ms : u32, stop_time_offset_ms : u32) -> String {

        let mut formatted_track_length_info : String = String::new();

        // // Track length is stored in milliseconds, but we want seconds
        let track_length_s = track_length_ms / 1000;

        formatted_track_length_info.push_str(&format!("Track length: {} seconds", track_length_s).to_owned());

        let played_track_length_ms = stop_time_offset_ms - start_time_offset_ms;

        if (played_track_length_ms != track_length_ms) && ((start_time_offset_ms != 0) || (stop_time_offset_ms != 0)) {

            formatted_track_length_info.push_str(&format!(" | w/ offset: {} seconds (Start ~ {}s, Stop ~{}s)", played_track_length_ms / 1000, start_time_offset_ms / 1000, stop_time_offset_ms / 1000).to_owned());
        }

        return formatted_track_length_info;
        
    }

    pub fn decode_track_samplerate(track_samplerate_raw : u32) -> String {

        // Divide by 0x10000 (65536d) to get the actual sample rate

        return format!("{} Hz", track_samplerate_raw / 65536 );
    }

    pub fn decode_track_audio_type(track_type_unk14_1 : u32) -> String {

        let suspected_track_type : String;

        if track_type_unk14_1 == 0x0 {
            suspected_track_type = "WAV (Waveform Audio File Format)".to_string();
        }

        else if track_type_unk14_1 == 0x000c /* 12d */ {
            
            suspected_track_type = "MPEG-1 Layer-3".to_string();
        }

        else if track_type_unk14_1 == 0x0016 /* 22d */ {

            suspected_track_type = "MPEG-2 Layer 3".to_string();
        }

        else if track_type_unk14_1 == 0x0020 /* 32d */ {

            suspected_track_type = "MPEG-2.5 Layer 3".to_string();
        }

        else if track_type_unk14_1 == 0x0029 /* 41d */ {
            suspected_track_type = "Audible (audio book)".to_string();
        }

        else if track_type_unk14_1 == 0x0033 /* 51d */ {
            suspected_track_type = "AAC (Advanced Audio Codec)".to_string();
        }

        else {
            suspected_track_type = "N/A".to_string();
        }

        return suspected_track_type;
    }

    pub enum MediaType {
        
        AUDIO_VISUAL = 1,
        PODCAST = 2,
        TELEVISION = 3
    }

    pub fn decode_track_media_type(track_media_type_raw : &[u8]) -> (String, MediaType) {

        let media_type_name : String;
        let mut media_type = MediaType::AUDIO_VISUAL;

        let conditional_byte = track_media_type_raw[0];

        if conditional_byte == 0x00 {
            media_type_name = "Audio/Video".to_string();
        }

        else if conditional_byte == 0x01 {
            media_type_name = "Audio".to_string();
        }

        else if conditional_byte == 0x02 {
            media_type_name = "Video".to_string();
        }

        else if conditional_byte == 0x04 {
            media_type_name = "Podcast".to_string();
            media_type = MediaType::PODCAST;
        }

        else if conditional_byte == 0x06 {
            media_type_name = "Video Podcast".to_string();
            media_type = MediaType::PODCAST;
        }

        else if conditional_byte == 0x08 {
            media_type_name = "Audiobook".to_string();
        }

        else if conditional_byte == 0x20 /* 32d */ {
            media_type_name = "Music Video".to_string();
        }

        else if conditional_byte == 0x40 /* 64d */ {
            media_type_name = "TV Show (only!)".to_string();
            media_type = MediaType::TELEVISION;
        }

        else if conditional_byte == 0x60 /* 96d */ {
            media_type_name = "TV show (hybrid w/ Music)".to_string();
            media_type = MediaType::TELEVISION;
        }

        else {
            //media_type_name = "N/A".to_string();
            media_type_name = format!("Unknown {}", conditional_byte);
        }

        return (media_type_name, media_type);
    }


    pub fn decode_playlist_sort_order(playlist_sort_order_raw : u32) -> String {

        let mut playlist_sort_order : String = "Playlist sort order: ".to_string();

        if playlist_sort_order_raw == 1 {
            playlist_sort_order.push_str("Manual (user sorted)");
        }

        else if playlist_sort_order_raw == 3 {
            playlist_sort_order.push_str("Song title");
        }

        else if playlist_sort_order_raw == 4 {
            playlist_sort_order.push_str("Album");
        }

        else if playlist_sort_order_raw == 5 {
            playlist_sort_order.push_str("Artist");
        }

        else if playlist_sort_order_raw == 6 {
            playlist_sort_order.push_str("Bitrate");
        }
        
        else if playlist_sort_order_raw == 7 {
            playlist_sort_order.push_str("Genre");
        }

        else if playlist_sort_order_raw == 8 {
            playlist_sort_order.push_str("Kind");
        }

        else if playlist_sort_order_raw == 9 {
            playlist_sort_order.push_str("Date Modified");
        }

        else if playlist_sort_order_raw == 10 {
            playlist_sort_order.push_str("Track number");
        }
        
        else if playlist_sort_order_raw == 11 {
            playlist_sort_order.push_str("Size");
        }

        else if playlist_sort_order_raw == 12 {
            playlist_sort_order.push_str("Time");
        }
        
        else if playlist_sort_order_raw == 13 {
            playlist_sort_order.push_str("Year");
        }

        else if playlist_sort_order_raw == 14 {
            playlist_sort_order.push_str("Sample Rate");
        }

        else if playlist_sort_order_raw == 15 {
            playlist_sort_order.push_str("Comment");
        }

        else if playlist_sort_order_raw == 16 {
            playlist_sort_order.push_str("Date Added");
        }

        else if playlist_sort_order_raw == 17 {
            playlist_sort_order.push_str("Equalizer");
        }

        else if playlist_sort_order_raw == 18 {
            playlist_sort_order.push_str("Composer");
        }

        else if playlist_sort_order_raw == 20 {
            playlist_sort_order.push_str("Play count");
        }
        
        else if playlist_sort_order_raw == 21 {
            playlist_sort_order.push_str("Last played");
        }

        else if playlist_sort_order_raw == 22 {
            playlist_sort_order.push_str("Disc number");
        }

        else if playlist_sort_order_raw == 23 {
            playlist_sort_order.push_str("My rating (# of stars)");
        }

        else if playlist_sort_order_raw == 24 {
            playlist_sort_order.push_str("Release Date (?)");
        }

        else if playlist_sort_order_raw == 25 {
            playlist_sort_order.push_str("BPM");
        }

        else if playlist_sort_order_raw == 26 {
            playlist_sort_order.push_str("Grouping");
        }

        else if playlist_sort_order_raw == 27 {
            playlist_sort_order.push_str("Category");
        }

        else if playlist_sort_order_raw == 28 {
            playlist_sort_order.push_str("Description");
        }

        else if playlist_sort_order_raw == 29 {
            playlist_sort_order.push_str("Show (Television)");
        }

        else if playlist_sort_order_raw == 30 {
            playlist_sort_order.push_str("Season (Television)");
        }

        else if playlist_sort_order_raw == 31 {
            playlist_sort_order.push_str("Episode # (Television)");
        }

        else {
            playlist_sort_order.push_str(&format!("N/A ({} ?)", playlist_sort_order_raw));
        }

        return playlist_sort_order;
    }

    pub enum HandleableDataObjectType {

        PODCAST_ENCLOSURE_URL = 15,
        PODCAST_RSS_URL = 16
    }

    pub fn is_data_object_type_string(data_object_raw : u32) -> bool {

        return data_object_raw < 15;
    }

    pub fn decode_podcast_urls(mhod_start_idx : usize, file_as_bytes : &[u8]) -> String {
        
        //let mut podcast_url : String;

        let header_len_offset = 4;
        let total_length_offset = 8;
        let element_header_length = super::helpers::get_slice_as_le_u32(mhod_start_idx, file_as_bytes, header_len_offset, super::DEFAULT_SUBSTRUCTURE_SIZE);
        let total_length = super::helpers::get_slice_as_le_u32(mhod_start_idx, file_as_bytes, total_length_offset, super::DEFAULT_SUBSTRUCTURE_SIZE);

        let podcast_url_length = total_length - element_header_length;
        let podcast_url_offset : usize = 24;

        let podcast_url = std::str::from_utf8(&file_as_bytes[mhod_start_idx + podcast_url_offset .. mhod_start_idx + podcast_url_offset + podcast_url_length as usize]).expect("Can't decode podcast URL");

        return podcast_url.to_string();
    }

    pub fn decode_data_object_type(data_object_type_raw : u32) -> String {

        let mut data_object_type : String = String::new();

        if data_object_type_raw == 1 {

            data_object_type = "Song title".to_string();
        }

        else if data_object_type_raw == 2 {
            data_object_type = "File location".to_string();
        }

        else if data_object_type_raw == 3 {
            data_object_type = "Album".to_string();
        }

        else if data_object_type_raw == 4 {
            data_object_type = "Artist".to_string();
        }

        else if data_object_type_raw == 5 {
            data_object_type = "Genre".to_string();
        }

        else if data_object_type_raw == 6 {
            data_object_type = "Filetype".to_string();
        }

        else if data_object_type_raw == 7 {
            data_object_type = "EQ Setting".to_string();
        }

        else if data_object_type_raw == 8 {
            data_object_type = "Comment".to_string();
        }

        else if data_object_type_raw == 9 {
            data_object_type = "Podcast Category".to_string();
        }

        else if data_object_type_raw == 12 {
            data_object_type = "Composer".to_string();
        }

        else if data_object_type_raw == 13 {
            data_object_type = "Grouping".to_string();
        }

        else if data_object_type_raw == 14 {
            data_object_type = "Description text (?)".to_string();
        }

        else if data_object_type_raw == 15 {
            data_object_type = "Podcast Enclosure URL".to_string();
        }

        else if data_object_type_raw == 16 {
            data_object_type = "Podcdast RSS URL".to_string();
        }

        else if data_object_type_raw == 17 {
            data_object_type = "Chapter data (?)".to_string();
        }

        else if data_object_type_raw == 18 {
            data_object_type = "Subtitle".to_string();
        }

        else if data_object_type_raw == 19 {
            data_object_type = "Television Show".to_string();
        }

        else if data_object_type_raw == 20 {
            data_object_type = "Episode #".to_string();
        }

        else if data_object_type_raw == 21 {
            data_object_type = "TV Show network".to_string();
        }

        else if data_object_type_raw == 22 {
            data_object_type = "Album Artist".to_string();
        }

        else if data_object_type_raw == 23 {
            data_object_type = "Artist name".to_string();
        }

        else if data_object_type_raw == 24 {
            data_object_type = "Track keywords (?)".to_string();
        }

        else if data_object_type_raw == 25 {
            data_object_type = "TV Show locale".to_string();
        }

        else if data_object_type_raw == 27 {
            data_object_type = "Title (for sorting)".to_string();
        }

        else if data_object_type_raw == 28 {
            data_object_type = "Album (for sorting)".to_string();
        }

        else if data_object_type_raw == 29 {
            data_object_type = "Album artist (for sorting)".to_string();
        }

        else if data_object_type_raw == 30 {
            data_object_type = "Composer (for sorting)".to_string();
        }

        else if data_object_type_raw == 31 {
            data_object_type = "Television Show (for sorting)".to_string();
        }

        else if data_object_type_raw == 32 {
            data_object_type = format!("Unknown video track field (#{}), iTunes 7.1+", data_object_type_raw);
        }

        else if data_object_type_raw == 50 {
            data_object_type = "Smart Playlist data".to_string();
        }

        else if data_object_type_raw == 51 {
            data_object_type = "Smart Playlist rules".to_string();
        }

        else if data_object_type_raw == 52 {
            data_object_type = "Library Playlist index".to_string();
        }

        else if data_object_type_raw == 53 {
            data_object_type = format!("Unknown type (#{}), iTunes7.2+", data_object_type_raw);
        }

        else if data_object_type_raw == 100 {
            data_object_type = format!("Indeterminate field (#{}), either column sizing or order indicator", data_object_type_raw);
        }

        else if data_object_type_raw == 200 {
            data_object_type = "Album (from Album List, iTunes 7.1+ only)".to_string();
        }

        // TODO what is the difference between this and the next entry (202) ???
        else if data_object_type_raw == 201 {
            data_object_type = "Artist (in Album List, iTunes 7.1)".to_string();
        }

        else if data_object_type_raw == 202 {
            data_object_type = "Artist (for sorting in Album List) - iTunes 7.1+ only".to_string();
        }

        else if data_object_type_raw == 203 {
            data_object_type = "Podcast URL (in Album List, iTunes 7.1)".to_string();
        }

        else if data_object_type_raw == 204 {
            data_object_type = "TV Show (in Album List)".to_string();
        }

        else {
            eprintln!("Unable to decode data object with type #{}", data_object_type_raw);
        }


        return data_object_type;

    }
}
