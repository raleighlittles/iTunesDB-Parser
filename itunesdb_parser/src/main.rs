mod endian_helpers;

use crate::endian_helpers::endian_helpers::build_le_u32_from_bytes;
use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    
    let itunesdb_filename : String = std::env::args().nth(1).expect("Missing parameter: iTunes DB filename");

    // TODO: Convert to enum
    let itunesdb_file_type : String = std::env::args().nth(2).expect("Missing parameter: iTunes DB file type. Values are: 'itunes', 'playcounts', 'otg', 'eq', 'art', or 'photo'.");

    let db_file_as_bytes : Vec<u8> = std::fs::read(itunesdb_filename).unwrap();

    const SUBSTRUCTURE_SIZE : usize = 4;

    // TODO: Move these to another module
    // ==================================
    // ----- IMAGE LIST -----
    const image_list_key : &str = "mhli";
    const image_list_key_ascii : &[u8] = image_list_key.as_bytes();

    const image_list_num_images_offset : usize = 8; // 4 + 4
    const image_list_num_images_len : usize = 4;

    // ----- IMAGE ITEM -----
    const image_item_key : &str = "mhii";
    const image_item_key_ascii : &[u8] = image_item_key.as_bytes();

    const image_item_rating_offset : usize = 32; // 4 * 8
    const image_item_rating_len : usize = 4;

    const image_item_orig_date_offset : usize = image_item_rating_offset + image_item_rating_len + 4;
    const image_item_orig_date_len : usize = 4;

    const image_item_digitized_date_offset : usize = image_item_orig_date_offset + image_item_orig_date_len;
    const image_item_digitized_date_len : usize = 4;

    const image_item_source_img_size_offset : usize = image_item_digitized_date_offset + image_item_digitized_date_len;
    const image_item_source_img_size_len : usize = 4;

    // ----- IMAGE NAME -----
    const image_name_key : &str = "mhni";
    const image_name_key_ascii : &[u8] = image_name_key.as_bytes();

    // TODO ~ There's 2 size fields in this key list, and I don't understand what the difference between the two is.
    // There's also another image size field in the "Image Item" key list. I don't know the difference between this
    // and that one either.
    // There's also a table that indicates you can determine the format of the image itself (eg UYVY, RGB, etc) from the size,
    // but I don't know which size field it's referring to.
    const image_name_img_size_offset : usize = 24; // 4 * 6
    const image_name_img_size_len : usize = 4;

    const image_name_img_height_offset : usize = 32; // 4 * 8
    const image_name_img_height_len : usize = 2;

    const image_name_img_width_offset : usize = image_name_img_height_offset + image_name_img_height_len;
    const image_name_img_width_len : usize = image_name_img_height_len;


    // ----- PHOTO ALBUM -----
    const photo_album_key : &str = "mhba";
    const photo_album_key_ascii : &[u8] = photo_album_key.as_bytes();

    const photo_album_album_item_cnt_offset : usize = 16; // 4 * 4
    const photo_album_album_item_cnt_len : usize = 4;

    // ----- Data Object -----
    const data_object_key : &str = "mhod";
    const data_object_key_ascii  : &[u8] = data_object_key.as_bytes();

    const data_object_header_length : usize = 0x18;

    const data_object_type_offset : usize = 12; // 4 + 8
    const data_object_type_len : usize = 2;

    // See the 'String MHODs' section
    const data_object_string_subcontainer_length_offset : usize = data_object_header_length;
    const data_object_string_subcontainer_length_len : usize = 4;

    const data_object_string_subcontainer_encoding_offset : usize = data_object_header_length + 4;
    const data_object_string_subcontainer_encoding_len : usize = 4;

    const data_object_string_subcontainer_data_offset : usize = data_object_string_subcontainer_encoding_offset + 4;


    // ----------------------- End key name definitions

    // Counters
    let mut num_image_lists = 0;
    let mut num_image_items = 0;
    let mut num_image_names = 0;
    let mut num_photo_albums = 0;
    let mut num_data_objects = 0;

    const mac_to_linux_epoch_conversion : i64 = 2082844800;


    if itunesdb_file_type == "photo" {

        let mut idx = 0;

        while idx < db_file_as_bytes.len() { // TODO: Inside of each of the ifs, advance the index correctly
            
            // Parse Image List

            if (db_file_as_bytes[idx] == image_list_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == image_list_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == image_list_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == image_list_key_ascii[3]) {

                let image_list_num_images_raw = &db_file_as_bytes[idx + image_list_num_images_offset .. idx + image_list_num_images_offset + image_list_num_images_len];
                println!("ImageList numImages [RAW] {:?}", image_list_num_images_raw);

                let image_list_num_images : u32 = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_list_num_images_raw);

                println!("ImageList#{} info... NumImages={}", num_image_lists, image_list_num_images);
                println!("==========");

                num_image_lists += 1;
            }

            // Parse Image Item

            else if (db_file_as_bytes[idx] == image_item_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == image_item_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == image_item_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == image_item_key_ascii[3]) {

                let image_item_rating_raw :&[u8] = &db_file_as_bytes[idx + image_item_rating_offset .. idx + image_item_rating_offset + image_item_rating_len];

                let image_item_rating : u32 = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_item_rating_raw);

                let image_item_orig_date_raw = &db_file_as_bytes[idx + image_item_orig_date_offset .. idx + image_item_orig_date_offset + image_item_orig_date_len];

                let image_item_orig_date_timestamp = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_item_orig_date_raw);

                let image_item_orig_date_date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(image_item_orig_date_timestamp as i64 - mac_to_linux_epoch_conversion, 0).unwrap(), Utc);

                let image_item_digitized_date_raw : &[u8] = &db_file_as_bytes[idx + image_item_digitized_date_offset .. idx + image_item_digitized_date_offset + image_item_digitized_date_len];
        
                let image_item_digitized_date_timestamp : u32 = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_item_digitized_date_raw);
                let image_item_digitized_date_date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(image_item_digitized_date_timestamp as i64 - mac_to_linux_epoch_conversion, 0).unwrap(), Utc);

                let image_item_source_img_size_raw  = &db_file_as_bytes[idx + image_item_source_img_size_offset .. idx + image_item_source_img_size_offset + image_item_source_img_size_len];

                let image_item_source_img_size = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_item_source_img_size_raw);

                println!("ImageItem#{} info... Rating= {} , ImgSize= {}, OrigDateTS= {} , DigitizedDateTS= {}", num_image_items, image_item_rating, image_item_source_img_size, image_item_orig_date_date, image_item_digitized_date_date);

                println!("==========");

                num_image_items += 1;
            }

            // Parse Image Name

            else if (db_file_as_bytes[idx] == image_name_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == image_name_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == image_name_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == image_name_key_ascii[3]) {

                let image_name_img_size_raw = &db_file_as_bytes[idx + image_name_img_size_offset .. idx + image_name_img_size_offset + image_name_img_size_len];
                let image_name_img_size = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_name_img_size_raw);

                let image_name_img_height_raw = &db_file_as_bytes[idx + image_name_img_height_offset .. idx + image_name_img_height_offset + image_name_img_height_len];
                let image_name_img_height = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_name_img_height_raw);
                println!("ImageName image height [RAW] {:?}", image_name_img_height_raw);

                let image_name_img_width_raw = &db_file_as_bytes[idx + image_name_img_width_offset .. idx + image_name_img_width_offset + image_name_img_width_len];
                let image_name_img_width = endian_helpers::endian_helpers::build_le_u32_from_bytes(image_name_img_width_raw);
                println!("ImageName image width [RAW] {:?}", image_name_img_width_raw);

                println!("ImageName#{} info... Size(bytes)={} , Height={} , Width={}", num_image_names, image_name_img_size, image_name_img_height, image_name_img_width);

                println!("==========");

                num_image_names += 1;
            }

            // Parse Photo Album

            else if (db_file_as_bytes[idx] == photo_album_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == photo_album_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == photo_album_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == photo_album_key_ascii[3]) {

                let photo_album_item_cnt_raw = &db_file_as_bytes[idx + photo_album_album_item_cnt_offset .. idx + photo_album_album_item_cnt_offset + photo_album_album_item_cnt_len];

                let photo_album_item_cnt = endian_helpers::endian_helpers::build_le_u32_from_bytes(photo_album_item_cnt_raw);

                println!("PhotoAlbum#{} info... Item count#={}", num_photo_albums, photo_album_item_cnt);

                println!("==========");

                num_photo_albums += 1;
            }


            // Parse Data Object

            else if (db_file_as_bytes[idx] == data_object_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == data_object_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == data_object_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == data_object_key_ascii[3]) {

                let data_object_type_raw = &db_file_as_bytes[idx + data_object_type_offset .. idx + data_object_type_offset + data_object_type_len];

                let data_object_type = endian_helpers::endian_helpers::build_le_u32_from_bytes(data_object_type_raw);

                if data_object_type == 1 || data_object_type == 3 {

                    let data_object_subcontainer_str_len = endian_helpers::endian_helpers::build_le_u32_from_bytes(&db_file_as_bytes[idx + data_object_string_subcontainer_length_offset .. idx + data_object_string_subcontainer_length_offset + data_object_string_subcontainer_length_len]);

                    let data_object_subcontainer_encoding = endian_helpers::endian_helpers::build_le_u32_from_bytes(&db_file_as_bytes[idx + data_object_string_subcontainer_encoding_offset .. idx + data_object_string_subcontainer_encoding_offset + data_object_string_subcontainer_encoding_len]);

                    if data_object_subcontainer_encoding == 0 || data_object_subcontainer_encoding == 1 {
                        // use UTF 8
                        let data_object_subcontainer_data = std::str::from_utf8(&db_file_as_bytes[idx + data_object_string_subcontainer_data_offset .. idx + data_object_string_subcontainer_data_offset + data_object_subcontainer_str_len as usize]).expect("Can't parse MHOD string");
                        
                        println!("MHOD substring (UTF-8) = {}", data_object_subcontainer_data);

                    } else if data_object_subcontainer_encoding == 2 {

                        // Build UTF-16 array, out of UTF-8, by combining elements pairwise
                        let mut data_object_pairwise_combined : Vec<u16> = vec![];
                        
                        // TODO: Figure out why I'm off by a width of 4 on the length (below)
                        for i in (idx + data_object_string_subcontainer_data_offset + 4 .. idx + data_object_string_subcontainer_data_offset + data_object_subcontainer_str_len as usize + 4).step_by(2) {

                            let u16_elem : u16 = ((db_file_as_bytes[i+1] as u16) << 8) | db_file_as_bytes[i] as u16;
                            data_object_pairwise_combined.push(u16_elem);
                        }

                        let data_object_subcontainer_data = String::from_utf16(&data_object_pairwise_combined).expect("Can't convert UTF-16 array to string");

                        println!("MHOD substring (UTF-16)= {}", data_object_subcontainer_data);

                    }

                    println!("String MHOD detected... Len={}, Encoding (raw)={}", data_object_subcontainer_str_len, data_object_subcontainer_encoding);
                }

                println!("DataObject#{} info... Type={}", num_data_objects, data_object_type);

                println!("==========");

                num_data_objects += 1;
            }

            idx = idx + SUBSTRUCTURE_SIZE;
        }
    } else {
        println!("ERROR: Only the 'photo' file parser is currently implemented.");
    }



}
