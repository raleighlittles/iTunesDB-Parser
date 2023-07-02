mod itunesdb;
mod helpers;

use crate::itunesdb::*;

fn main() {
    let itunesdb_filename: String = std::env::args()
        .nth(1)
        .expect("Missing parameter: iTunes DB filename");

    let itunesdb_file_type : String = std::env::args().nth(2).expect("Missing parameter: iTunes DB file type. Values are: 'music', 'photo' ");

    let db_file_as_bytes: Vec<u8> = std::fs::read(&itunesdb_filename).unwrap();

    // Photo Database counters
    let mut num_image_lists = 0;
    let mut num_image_items = 0;
    let mut num_image_names = 0;
    let mut num_photo_albums = 0;
    let mut num_photo_data_objects = 0;

    if itunesdb_file_type == "photo" {

        println!("Parsing photo file: {}", itunesdb_filename);

        let mut idx = 0;

        while idx < db_file_as_bytes.len() {

            let potential_photo_section_heading = &db_file_as_bytes[idx .. idx + DEFAULT_SUBSTRUCTURE_SIZE];

            // println!("Comparing slice {:?} against {:?}", potential_photo_section_heading, photo_database::IMAGE_LIST_KEY.as_bytes());

            // if (db_file_as_bytes[idx] == photo_database::IMAGE_LIST_KEY_ASCII[0])
            //     && (db_file_as_bytes[idx + 1] == photo_database::IMAGE_LIST_KEY_ASCII[1])
            //     && (db_file_as_bytes[idx + 2] == photo_database::IMAGE_LIST_KEY_ASCII[2])
            //     && (db_file_as_bytes[idx + 3] == photo_database::IMAGE_LIST_KEY_ASCII[3])
            if potential_photo_section_heading == photo_database::IMAGE_LIST_KEY.as_bytes()
            {
                let image_list_num_images_raw = &db_file_as_bytes[idx + photo_database::IMAGE_LIST_NUM_IMAGES_OFFSET
                    ..idx + photo_database::IMAGE_LIST_NUM_IMAGES_OFFSET + photo_database::IMAGE_LIST_NUM_IMAGES_LEN];

                let image_list_num_images: u32 =
                    helpers::helpers::build_le_u32_from_bytes(image_list_num_images_raw);

                println!(
                    "ImageList#{} info : NumImages={}",
                    num_image_lists, image_list_num_images
                );
                println!("==========");
                num_image_lists += 1;

                // Done parsing the header, move the index forward up to the end of it
                idx += photo_database::IMAGE_LIST_LAST_OFFSET;
            }
            // Parse Image Item
            else if potential_photo_section_heading == photo_database::IMAGE_ITEM_KEY.as_bytes()
            {
                let image_item_rating_raw: &[u8] = &db_file_as_bytes[idx + photo_database::IMAGE_ITEM_RATING_OFFSET
                    ..idx + photo_database::IMAGE_ITEM_RATING_OFFSET + photo_database::IMAGE_ITEM_RATING_LEN];

                let image_item_rating: u32 =
                    helpers::helpers::build_le_u32_from_bytes(image_item_rating_raw);

                let image_item_orig_date_raw = &db_file_as_bytes[idx + photo_database::IMAGE_ITEM_ORIG_DATE_OFFSET
                    ..idx + photo_database::IMAGE_ITEM_ORIG_DATE_OFFSET + photo_database::IMAGE_ITEM_ORIG_DATE_LEN];

                let image_item_orig_date_timestamp =
                    helpers::helpers::build_le_u32_from_bytes(image_item_orig_date_raw);

                let image_item_orig_date_date = itunesdb_helpers::get_timestamp_as_mac(image_item_orig_date_timestamp as u64);

                let image_item_digitized_date_raw: &[u8] = &db_file_as_bytes[idx
                    + photo_database::IMAGE_ITEM_DIGITIZED_DATE_OFFSET
                    ..idx + photo_database::IMAGE_ITEM_DIGITIZED_DATE_OFFSET + photo_database::IMAGE_ITEM_DIGITIZED_DATE_LEN];

                let image_item_digitized_date_timestamp: u32 =
                    helpers::helpers::build_le_u32_from_bytes(image_item_digitized_date_raw);

                let image_item_digitized_date_date = itunesdb_helpers::get_timestamp_as_mac(image_item_digitized_date_timestamp as u64);

                let image_item_source_img_size_raw = &db_file_as_bytes[idx
                    + photo_database::IMAGE_ITEM_SOURCE_IMG_SIZE_OFFSET
                    ..idx + photo_database::IMAGE_ITEM_SOURCE_IMG_SIZE_OFFSET + photo_database::IMAGE_ITEM_SOURCE_IMG_SIZE_LEN];

                let image_item_source_img_size =
                    helpers::helpers::build_le_u32_from_bytes(image_item_source_img_size_raw);

                println!("ImageItem#{} info : Rating= {} , ImgSize= {}, OrigDateTS= {} , DigitizedDateTS= {}", num_image_items, image_item_rating, image_item_source_img_size, image_item_orig_date_date, image_item_digitized_date_date);

                println!("==========");
                num_image_items += 1;

                idx += photo_database::IMAGE_ITEM_LAST_OFFSET;
            }
            // Parse Image Name
            else if potential_photo_section_heading == photo_database::IMAGE_NAME_KEY.as_bytes()
            {
                let image_name_img_size_raw = &db_file_as_bytes[idx + photo_database::IMAGE_NAME_IMG_SIZE_OFFSET
                    ..idx + photo_database::IMAGE_NAME_IMG_SIZE_OFFSET + photo_database::IMAGE_NAME_IMG_SIZE_LEN];
                let image_name_img_size =
                    helpers::helpers::build_le_u32_from_bytes(image_name_img_size_raw);

                let image_name_img_height_raw = &db_file_as_bytes[idx + photo_database::IMAGE_NAME_IMG_HEIGHT_OFFSET
                    ..idx + photo_database::IMAGE_NAME_IMG_HEIGHT_OFFSET + photo_database::IMAGE_NAME_IMG_HEIGHT_LEN];
                
                // TODO: Figure out why the Image Height and Image Width are both 0
                let image_name_img_height =
                    helpers::helpers::build_le_u32_from_bytes(image_name_img_height_raw);

                let image_name_img_width_raw = &db_file_as_bytes[idx + photo_database::IMAGE_NAME_IMG_WIDTH_OFFSET
                    ..idx + photo_database::IMAGE_NAME_IMG_WIDTH_OFFSET + photo_database::IMAGE_NAME_IMG_WIDTH_LEN];
                let image_name_img_width =
                    helpers::helpers::build_le_u32_from_bytes(image_name_img_width_raw);

                println!(
                    "ImageName#{} info : Size(bytes)={} , Height={} , Width={}",
                    num_image_names,
                    image_name_img_size,
                    image_name_img_height,
                    image_name_img_width
                );

                println!("==========");
                num_image_names += 1;

                idx += photo_database::IMAGE_NAME_LAST_OFFSET;
            }
            // Parse Photo Album
            else if potential_photo_section_heading == photo_database::PHOTO_ALBUM_KEY.as_bytes()
            {
                let photo_album_item_cnt_raw = &db_file_as_bytes[idx
                    + photo_database::PHOTO_ALBUM_ALBUM_ITEM_CNT_OFFSET
                    ..idx + photo_database::PHOTO_ALBUM_ALBUM_ITEM_CNT_OFFSET + photo_database::PHOTO_ALBUM_ALBUM_ITEM_CNT_LEN];

                let photo_album_item_cnt =
                    helpers::helpers::build_le_u32_from_bytes(photo_album_item_cnt_raw);

                println!(
                    "PhotoAlbum#{} info : Item count#={}",
                    num_photo_albums, photo_album_item_cnt
                );

                println!("==========");
                num_photo_albums += 1;

                idx += photo_database::PHOTO_ALBUM_LAST_OFFSET;
            }
            // Parse Data Object
            else if potential_photo_section_heading == photo_database::DATA_OBJECT_KEY.as_bytes()
            {
                let data_object_type_raw = &db_file_as_bytes[idx + photo_database::DATA_OBJECT_TYPE_OFFSET
                    ..idx + photo_database::DATA_OBJECT_TYPE_OFFSET + photo_database::DATA_OBJECT_TYPE_LEN];

                let data_object_type =
                    helpers::helpers::build_le_u32_from_bytes(data_object_type_raw);

                if data_object_type == (photo_database::MhodType::ALBUM_NAME as u32) || data_object_type == (photo_database::MhodType::FILE_NAME as u32) {
                    let data_object_subcontainer_str_len =
                        helpers::helpers::build_le_u32_from_bytes(
                            &db_file_as_bytes[idx + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_OFFSET
                                ..idx
                                    + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_OFFSET
                                    + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_LEN],
                        );

                    let data_object_subcontainer_encoding =
                        helpers::helpers::build_le_u32_from_bytes(
                            &db_file_as_bytes[idx + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET
                                ..idx
                                    + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET
                                    + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_LEN],
                        );

                    if data_object_subcontainer_encoding == 0
                        || data_object_subcontainer_encoding == 1
                    {
                        // TODO: Figure out why I'm off by a width of 4 on the length.
                        // Same issue with UTF-16 encoding (below)

                        let data_object_subcontainer_data = std::str::from_utf8(
                            &db_file_as_bytes[idx + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET + 4
                                ..idx
                                    + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET
                                    + data_object_subcontainer_str_len as usize
                                    + 4],
                        )
                        .expect("Can't parse MHOD string");

                        println!("MHOD substring (UTF-8) = {}", data_object_subcontainer_data);
                    } else if data_object_subcontainer_encoding == 2 {
                        // Build UTF-16 array, out of UTF-8, by combining elements pairwise
                        // Maybe put this into its own function in helpers
                        let mut data_object_pairwise_combined: Vec<u16> = vec![];

                        for i in (idx + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET + 4
                            ..idx
                                + photo_database::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET
                                + data_object_subcontainer_str_len as usize
                                + 4)
                            .step_by(2)
                        {
                            let u16_elem: u16 = ((db_file_as_bytes[i + 1] as u16) << 8)
                                | db_file_as_bytes[i] as u16;
                            data_object_pairwise_combined.push(u16_elem);
                        }

                        let data_object_subcontainer_data =
                            String::from_utf16(&data_object_pairwise_combined)
                                .expect("Can't convert UTF-16 array to string");

                        println!("MHOD substring (UTF-16) = {}", data_object_subcontainer_data);
                    }

                    println!(
                        "String MHOD detected : Length={}, Encoding (raw)={}",
                        data_object_subcontainer_str_len, data_object_subcontainer_encoding
                    );
                }

                println!(
                    "DataObject#{} info : Type={}",
                    num_photo_data_objects, &photo_database::decode_mhod_type(data_object_type as u16)
                );

                println!("==========");
                num_photo_data_objects += 1;

                idx += photo_database::DATA_OBJECT_LAST_OFFSET;
            }

            idx += DEFAULT_SUBSTRUCTURE_SIZE;
        }
    } else if itunesdb_file_type == "music" {

        let mut idx = 0;

        while idx < db_file_as_bytes.len() {

            let potential_section_heading = &db_file_as_bytes[idx .. idx + DEFAULT_SUBSTRUCTURE_SIZE];

            // Parse Database Object
            if potential_section_heading == iTunesDB::DATABASE_OBJECT_KEY.as_bytes() {

                let db_language_raw = &db_file_as_bytes[idx + iTunesDB::DATABASE_OBJECT_LANGUAGE_OFFSET .. idx + iTunesDB::DATABASE_OBJECT_LANGUAGE_OFFSET + iTunesDB::DATABASE_OBJECT_LANGUAGE_LEN];

                let db_language = std::str::from_utf8(&db_language_raw).expect("Can't parse database language string");

                println!("File {} is using language: {}", itunesdb_file_type, db_language);

                idx += iTunesDB::DATABASE_OBJECT_LAST_OFFSET;
            }

            idx += DEFAULT_SUBSTRUCTURE_SIZE;
        }
        
    }
}
