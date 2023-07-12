pub const DEFAULT_SUBSTRUCTURE_SIZE: usize = 4;

pub mod itunesdb_constants {

    // ----- DATABASE OBJECT ----- //
    pub const DATABASE_OBJECT_KEY: &str = "mhbd";

    pub const DATABASE_OBJECT_VERSION_NUMBER_OFFSET: usize = 4;
    pub const DATABASE_OBJECT_VERSION_NUMBER_LEN: usize = 4;

    pub const DATABASE_OBJECT_LANGUAGE_OFFSET: usize = 70;
    pub const DATABASE_OBJECT_LANGUAGE_LEN: usize = 2;

    pub const DATABASE_OBJECT_LAST_OFFSET: usize = 108;

    // ----- DATASET ----- //
    pub const DATASET_KEY: &str = "mhsd";

    pub const DATASET_TYPE_OFFSET: usize = 12;
    pub const DATASET_TYPE_LEN: usize = 12;

    pub const DATASET_LAST_OFFSET: usize = 16;

    // ----- TRACKLIST ----- //
    pub const TRACKLIST_KEY: &str = "mhlt";

    pub const TRACKLIST_NUM_SONGS_OFFSET: usize = 8;
    pub const TRACKLIST_NUM_SONGS_LEN: usize = 4;

    pub const TRACKLIST_LAST_OFFSET: usize = 12;

    // ----- TRACK ITEM ----- //
    pub const TRACK_ITEM_KEY: &str = "mhit";

    pub const TRACK_ITEM_TRACK_FILETYPE_OFFSET: usize = 24;
    pub const TRACK_ITEM_TRACK_FILETYPE_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_BITRATE_SETTING_OFFSET: usize = 28;
    pub const TRACK_ITEM_TRACK_BITRATE_SETTING_LEN: usize = 1;

    pub const TRACK_ITEM_IS_COMPILATION_SETTING_OFFSET: usize = 30;
    pub const TRACK_ITEM_IS_COMPILATION_SETTING_LEN: usize = 1;

    pub const TRACK_ITEM_TRACK_RATING_OFFSET: usize = 31;
    pub const TRACK_ITEM_TRACK_RATING_LEN: usize = 1;

    pub const TRACK_ITEM_TRACK_MODIFIED_TIME_OFFSET: usize = 32;
    pub const TRACK_ITEM_TRACK_MODIFIED_TIME_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_FILE_SIZE_BYTES_OFFSET: usize = 36;
    pub const TRACK_ITEM_TRACK_FILE_SIZE_BYTES_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_OFFSET: usize = 40;
    pub const TRACK_ITEM_TRACK_LENGTH_MILLISECONDS_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_NUMBER_OFFSET: usize = 44;
    pub const TRACK_ITEM_TRACK_NUMBER_LEN: usize = 4;

    pub const TRACK_ITEM_NUM_TRACKS_IN_ALBUM_OFFSET: usize = 48;
    pub const TRACK_ITEM_NUM_TRACKS_IN_ALBUM_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_YEAR_PUBLISHED_OFFSET: usize = 52;
    pub const TRACK_ITEM_TRACK_YEAR_PUBLISHED_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_BITRATE_OFFSET: usize = 56;
    pub const TRACK_ITEM_TRACK_BITRATE_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_SAMPLE_RATE_OFFSET: usize = 60;
    pub const TRACK_ITEM_TRACK_SAMPLE_RATE_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_VOLUME_OFFSET: usize = 64;
    pub const TRACK_ITEM_TRACK_VOLUME_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_START_TIME_OFFSET: usize = 68;
    pub const TRACK_ITEM_TRACK_START_TIME_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_STOP_TIME_OFFSET: usize = 72;
    pub const TRACK_ITEM_TRACK_STOP_TIME_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_PLAY_COUNT_OFFSET: usize = 80;
    pub const TRACK_ITEM_TRACK_PLAY_COUNT_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_OFFSET: usize = 88;
    pub const TRACK_ITEM_TRACK_LAST_PLAYED_TIMESTAMP_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_DISC_NUMBER_OFFSET: usize = 92;
    pub const TRACK_ITEM_TRACK_DISC_NUMBER_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_OFFSET: usize = 96;
    pub const TRACK_ITEM_TRACK_TOTAL_NUM_DISCS_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_USER_ID_OFFSET: usize = 100;
    pub const TRACK_ITEM_TRACK_USER_ID_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_ADDED_TIMESTAMP_OFFSET: usize = 104;
    pub const TRACK_ITEM_TRACK_ADDED_TIMESTAMP_LEN: usize = 4;

    // pub const TRACK_ITEM_TRACK_BOOKMARK_TIME_MILLISECONDS_OFFSET : usize = 108;
    // pub const TRACK_ITEM_TRACK_BOOKMARK_TIME_MILLISECONDS_LEN : usize = 4;

    pub const TRACK_ITEM_TRACK_PREVIOUS_RATING_OFFSET: usize = 121;
    pub const TRACK_ITEM_TRACK_PREVIOUS_RATING_LEN: usize = 1;

    pub const TRACK_ITEM_TRACK_BPM_OFFSET: usize = 122;
    pub const TRACK_ITEM_TRACK_BPM_LEN: usize = 2;

    pub const TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_OFFSET: usize = 128;
    pub const TRACK_ITEM_TRACK_ARTWORK_SIZE_BYTES_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_OFFSET: usize = 164;
    pub const TRACK_ITEM_TRACK_HAS_ARTWORK_SETTING_LEN: usize = 1;

    pub const TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_OFFSET: usize = 140;
    pub const TRACK_ITEM_TRACK_RELEASED_TIMESTAMP_LEN: usize = 4;

    // Called "unk14/1"
    pub const TRACK_ITEM_ADVANCED_TRACK_TYPE_OFFSET: usize = 144;
    pub const TRACK_ITEM_ADVANCED_TRACK_TYPE_LEN: usize = 2;

    pub const TRACK_ITEM_TRACK_SKIPPED_COUNT_OFFSET: usize = 156;
    pub const TRACK_ITEM_TRACK_SKIPPED_COUNT_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_OFFSET: usize = 160;
    pub const TRACK_ITEM_TRACK_LAST_SKIPPED_TIMESTAMP_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_OFFSET: usize = 165;
    pub const TRACK_ITEM_TRACK_SKIP_WHEN_SHUFFLING_SETTING_LEN: usize = 1;

    // pub const TRACK_ITEM_TRACK_REMEMBER_PLAYBACK_POSITION_SETTING_OFSET : usize = 166;
    // pub const TRACK_ITEM_TRACK_REMEMBER_PLAYBACK_POSITION_SETTING_LEN : usize = 1;

    pub const TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_OFFSET: usize = 176;
    pub const TRACK_ITEM_TRACK_LYRICS_AVAILABLE_SETTING_LEN: usize = 1;

    pub const TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_OFFSET: usize = 177;
    pub const TRACK_ITEM_TRACK_MOVIE_FLAG_SETTING_LEN: usize = 1;

    pub const TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_OFFSET: usize = 184;
    pub const TRACK_ITEM_TRACK_BEGINNING_SILENCE_SAMPLE_COUNT_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_NUM_SAMPLES_OFFSET: usize = 188;
    pub const TRACK_ITEM_TRACK_NUM_SAMPLES_LEN: usize = 8;

    pub const TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_OFFSET: usize = 200;
    pub const TRACK_ITEM_TRACK_ENDING_SILENCE_SAMPLE_COUNT_LEN: usize = 4;

    /// Formerly known as unk28
    pub const TRACK_ITEM_TRACK_MEDIA_TYPE_OFFSET: usize = 208;
    pub const TRACK_ITEM_TRACK_MEDIA_TYPE_LEN: usize = 4;

    // Parse this FIRST, before doing anything else
    pub const TRACK_ITEM_TRACK_SEASON_NUMBER_OFFSET: usize = 212;
    pub const TRACK_ITEM_TRACK_SEASON_NUMBER_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_EPISODE_NUMBER_OFFSET: usize = 216;
    pub const TRACK_ITEM_TRACK_EPISODE_NUMBER_LEN: usize = 4;

    pub const TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_OFFSET: usize = 256;
    pub const TRACK_ITEM_TRACK_GAPLESS_PLAYBACK_SETTING_LEN: usize = 2;

    pub const TRACK_ITEM_TRACK_CROSSFADING_SETTING_OFFSET: usize = 258;
    pub const TRACK_ITEM_TRACK_CROSSFADING_SETTING_LEN: usize = 2;

    pub const TRACK_ITEM_LAST_OFFSET: usize = 356;

    // ----- PLAYLIST ----- //
    pub const PLAYLIST_KEY: &str = "mhyp";

    pub const PLAYLIST_IS_MASTER_PLAYLIST_SETTING_OFFSET: usize = 20;
    pub const PLAYLIST_IS_MASTER_PLAYLIST_SETTING_LEN: usize = 1;

    pub const PLAYLIST_CREATED_TIMESTAMP_OFFSET: usize = 24;
    pub const PLAYLIST_CREATED_TIMESTAMP_LEN: usize = 4;

    pub const PLAYLIST_PLAYLIST_SORT_ORDER_OFFSET: usize = 44;
    pub const PLAYLIST_PLAYLIST_SORT_ORDER_LEN: usize = 4;

    pub const PLAYLIST_LAST_OFFSET: usize = 48;

        // ----- PLAYLIST ITEM ----- //
        pub const PLAYLIST_ITEM_KEY : &str = "mhip";
    
        pub const PLAYLIST_ITEM_ADDED_TIMESTAMP_OFFSET : usize = 28;
        pub const PLAYLIST_ITEM_ADDED_TIMESTAMP_LEN : usize = 4;
    
        pub const PLAYLIST_ITEM_LAST_OFFSET : usize = 36;
    
        // ----- DATA OBJECT ----- //
    
        pub const DATA_OBJECT_KEY : &str = "mhod";
    
        // Must parse this first, to decide how to handle the rest of the object
    
        pub const DATA_OBJECT_TYPE_OFFSET : usize = 12;
        pub const DATA_OBJECT_TYPE_LEN : usize = 4;
    
        pub const DATA_OBJECT_STRING_LENGTH_OFFSET : usize = 28;
        pub const DATA_OBJECT_STRING_LENGTH_LEN : usize = 4;
    
        // This is where the actual string (for string mhod-types only! is held)
        // Use the length you derived above to index it
        pub const DATA_OBJECT_STRING_LOCATION_OFFSET : usize = 40;
    
        pub const DATA_OBJECT_LAST_OFFSET : usize = 0x18; // 24d

            // ----- ALBUM LIST ----- //
    pub const ALBUM_LIST_KEY : &str = "mhla";

    pub const ALBUM_LIST_TOTAL_NUM_SONGS_OFFSET : usize = 8;
    pub const ALBUM_LIST_TOTAL_NUM_SONGS_LEN : usize = 4;

    pub const ALBUM_LIST_LAST_OFFSET : usize = 12;

    // ----- ALBUM ITEM ----- //
    pub const ALBUM_ITEM_KEY : &str = "mhia";

    pub const ALBUM_ITEM_LAST_OFFSET : usize = 32;
}
