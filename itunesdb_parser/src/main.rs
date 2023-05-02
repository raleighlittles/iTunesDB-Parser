mod constants;
mod helpers;

use crate::constants::itunesdb_constants::*;
use crate::helpers::helpers::build_le_u32_from_bytes;

use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    
    let itunesdb_filename: String = std::env::args()
        .nth(1)
        .expect("Missing parameter: iTunes DB filename");

    let itunesdb_file_type : String = std::env::args().nth(2).expect("Missing parameter: iTunes DB file type. Values are: 'itunes', 'playcounts', 'otg', 'eq', 'art', or 'photo'.");

    let db_file_as_bytes: Vec<u8> = std::fs::read(itunesdb_filename).unwrap();

    // Counters
    let mut num_image_lists = 0;
    let mut num_image_items = 0;
    let mut num_image_names = 0;
    let mut num_photo_albums = 0;
    let mut num_data_objects = 0;

    if itunesdb_file_type == "photo" {
        let mut idx = 0;

        while idx < db_file_as_bytes.len() {
            // TODO: Inside of each of the ifs, advance the index correctly

            // Parse Image List

            if (db_file_as_bytes[idx] == image_list_key_ascii[0])
                && (db_file_as_bytes[idx + 1] == image_list_key_ascii[1])
                && (db_file_as_bytes[idx + 2] == image_list_key_ascii[2])
                && (db_file_as_bytes[idx + 3] == image_list_key_ascii[3])
            {
                let image_list_num_images_raw = &db_file_as_bytes[idx + image_list_num_images_offset
                    ..idx + image_list_num_images_offset + image_list_num_images_len];
                println!("ImageList numImages [RAW] {:?}", image_list_num_images_raw);

                let image_list_num_images: u32 =
                    helpers::helpers::build_le_u32_from_bytes(image_list_num_images_raw);

                println!(
                    "ImageList#{} info... NumImages={}",
                    num_image_lists, image_list_num_images
                );
                println!("==========");
                num_image_lists += 1;

                // Done parsing the header, move the index forward up to the end of it
                idx = idx + image_list_last_offset;
            }
            // Parse Image Item
            else if (db_file_as_bytes[idx] == image_item_key_ascii[0])
                && (db_file_as_bytes[idx + 1] == image_item_key_ascii[1])
                && (db_file_as_bytes[idx + 2] == image_item_key_ascii[2])
                && (db_file_as_bytes[idx + 3] == image_item_key_ascii[3])
            {
                let image_item_rating_raw: &[u8] = &db_file_as_bytes[idx + image_item_rating_offset
                    ..idx + image_item_rating_offset + image_item_rating_len];

                let image_item_rating: u32 =
                    helpers::helpers::build_le_u32_from_bytes(image_item_rating_raw);

                let image_item_orig_date_raw = &db_file_as_bytes[idx + image_item_orig_date_offset
                    ..idx + image_item_orig_date_offset + image_item_orig_date_len];

                let image_item_orig_date_timestamp =
                    helpers::helpers::build_le_u32_from_bytes(image_item_orig_date_raw);

                let image_item_orig_date_date = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(
                        image_item_orig_date_timestamp as i64
                            - constants::mac_to_linux_epoch_conversion,
                        0,
                    )
                    .unwrap(),
                    Utc,
                );

                let image_item_digitized_date_raw: &[u8] = &db_file_as_bytes[idx
                    + image_item_digitized_date_offset
                    ..idx + image_item_digitized_date_offset + image_item_digitized_date_len];

                let image_item_digitized_date_timestamp: u32 =
                    helpers::helpers::build_le_u32_from_bytes(image_item_digitized_date_raw);
                let image_item_digitized_date_date = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(
                        image_item_digitized_date_timestamp as i64
                            - constants::mac_to_linux_epoch_conversion,
                        0,
                    )
                    .unwrap(),
                    Utc,
                );

                let image_item_source_img_size_raw = &db_file_as_bytes[idx
                    + image_item_source_img_size_offset
                    ..idx + image_item_source_img_size_offset + image_item_source_img_size_len];

                let image_item_source_img_size =
                    helpers::helpers::build_le_u32_from_bytes(image_item_source_img_size_raw);

                println!("ImageItem#{} info... Rating= {} , ImgSize= {}, OrigDateTS= {} , DigitizedDateTS= {}", num_image_items, image_item_rating, image_item_source_img_size, image_item_orig_date_date, image_item_digitized_date_date);

                println!("==========");
                num_image_items += 1;

                idx = idx + image_item_last_offset;
            }
            // Parse Image Name
            else if (db_file_as_bytes[idx] == image_name_key_ascii[0])
                && (db_file_as_bytes[idx + 1] == image_name_key_ascii[1])
                && (db_file_as_bytes[idx + 2] == image_name_key_ascii[2])
                && (db_file_as_bytes[idx + 3] == image_name_key_ascii[3])
            {
                let image_name_img_size_raw = &db_file_as_bytes[idx + image_name_img_size_offset
                    ..idx + image_name_img_size_offset + image_name_img_size_len];
                let image_name_img_size =
                    helpers::helpers::build_le_u32_from_bytes(image_name_img_size_raw);

                let image_name_img_height_raw = &db_file_as_bytes[idx + image_name_img_height_offset
                    ..idx + image_name_img_height_offset + image_name_img_height_len];
                let image_name_img_height =
                    helpers::helpers::build_le_u32_from_bytes(image_name_img_height_raw);
                println!(
                    "ImageName image height [RAW] {:?}",
                    image_name_img_height_raw
                );

                let image_name_img_width_raw = &db_file_as_bytes[idx + image_name_img_width_offset
                    ..idx + image_name_img_width_offset + image_name_img_width_len];
                let image_name_img_width =
                    helpers::helpers::build_le_u32_from_bytes(image_name_img_width_raw);
                println!("ImageName image width [RAW] {:?}", image_name_img_width_raw);

                println!(
                    "ImageName#{} info... Size(bytes)={} , Height={} , Width={}",
                    num_image_names,
                    image_name_img_size,
                    image_name_img_height,
                    image_name_img_width
                );

                println!("==========");
                num_image_names += 1;

                idx = idx + image_name_last_offset;
            }
            // Parse Photo Album
            else if (db_file_as_bytes[idx] == photo_album_key_ascii[0])
                && (db_file_as_bytes[idx + 1] == photo_album_key_ascii[1])
                && (db_file_as_bytes[idx + 2] == photo_album_key_ascii[2])
                && (db_file_as_bytes[idx + 3] == photo_album_key_ascii[3])
            {
                let photo_album_item_cnt_raw = &db_file_as_bytes[idx
                    + photo_album_album_item_cnt_offset
                    ..idx + photo_album_album_item_cnt_offset + photo_album_album_item_cnt_len];

                let photo_album_item_cnt =
                    helpers::helpers::build_le_u32_from_bytes(photo_album_item_cnt_raw);

                println!(
                    "PhotoAlbum#{} info... Item count#={}",
                    num_photo_albums, photo_album_item_cnt
                );

                println!("==========");
                num_photo_albums += 1;

                idx = idx + photo_album_last_offset;
            }
            // Parse Data Object
            else if (db_file_as_bytes[idx] == data_object_key_ascii[0])
                && (db_file_as_bytes[idx + 1] == data_object_key_ascii[1])
                && (db_file_as_bytes[idx + 2] == data_object_key_ascii[2])
                && (db_file_as_bytes[idx + 3] == data_object_key_ascii[3])
            {
                let data_object_type_raw = &db_file_as_bytes[idx + data_object_type_offset
                    ..idx + data_object_type_offset + data_object_type_len];

                let data_object_type =
                    helpers::helpers::build_le_u32_from_bytes(data_object_type_raw);

                if data_object_type == 1 || data_object_type == 3 {
                    let data_object_subcontainer_str_len =
                        helpers::helpers::build_le_u32_from_bytes(
                            &db_file_as_bytes[idx + data_object_string_subcontainer_length_offset
                                ..idx
                                    + data_object_string_subcontainer_length_offset
                                    + data_object_string_subcontainer_length_len],
                        );

                    let data_object_subcontainer_encoding =
                        helpers::helpers::build_le_u32_from_bytes(
                            &db_file_as_bytes[idx + data_object_string_subcontainer_encoding_offset
                                ..idx
                                    + data_object_string_subcontainer_encoding_offset
                                    + data_object_string_subcontainer_encoding_len],
                        );

                    if data_object_subcontainer_encoding == 0
                        || data_object_subcontainer_encoding == 1
                    {
                        // TODO: Figure out why I'm off by a width of 4 on the length.
                        // Same issue with UTF-16 encoding (below)

                        let data_object_subcontainer_data = std::str::from_utf8(
                            &db_file_as_bytes[idx + data_object_string_subcontainer_data_offset + 4
                                ..idx
                                    + data_object_string_subcontainer_data_offset
                                    + data_object_subcontainer_str_len as usize + 4],
                        )
                        .expect("Can't parse MHOD string");

                        println!("MHOD substring (UTF-8) = {}", data_object_subcontainer_data);
                    } else if data_object_subcontainer_encoding == 2 {
                        // Build UTF-16 array, out of UTF-8, by combining elements pairwise
                        let mut data_object_pairwise_combined: Vec<u16> = vec![];

                        for i in (idx + data_object_string_subcontainer_data_offset + 4
                            ..idx
                                + data_object_string_subcontainer_data_offset
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

                        println!("MHOD substring (UTF-16)= {}", data_object_subcontainer_data);
                    }

                    println!(
                        "String MHOD detected... Len={}, Encoding (raw)={}",
                        data_object_subcontainer_str_len, data_object_subcontainer_encoding
                    );
                }

                println!(
                    "DataObject#{} info... Type={}",
                    num_data_objects, data_object_type
                );

                println!("==========");
                num_data_objects += 1;

                idx = idx + data_object_last_offset;
            }

            idx = idx + SUBSTRUCTURE_SIZE;
        }
    } else {
        println!("ERROR: Only the 'photo' file parser is currently implemented.");
    }
}
