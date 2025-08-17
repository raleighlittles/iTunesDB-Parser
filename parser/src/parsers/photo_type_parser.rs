use crate::helpers::helpers;
use crate::helpers::itunesdb_helpers;

use crate::photo_database;

use crate::constants::itunesdb_constants;
use crate::constants::photo_database_constants;
use crate::constants::photofolderalbums_constants;

pub fn parse_photofolder_albums_file(itunesdb_file_as_bytes: Vec<u8>) {
    let mut idx: usize = 0;

    while idx < (itunesdb_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {
        let photofolderalbums_file_heading: &[u8] =
            &itunesdb_file_as_bytes[idx..idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

        if photofolderalbums_file_heading
            == photofolderalbums_constants::PHOTOFOLDERALBUMS_OBJECT_KEY.as_bytes()
        {
            let num_folders = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photofolderalbums_constants::PFA_NUM_FOLDERS_OFFSET,
                photofolderalbums_constants::PFA_NUM_FOLDERS_LEN,
            );

            println!("'{}' photo folders found", num_folders);

            idx += photofolderalbums_constants::PHOTOFOLDERALBUMS_LAST_HEADER_OFFSET;
        }

        idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE;
    }
}

pub fn parse_photo_type_file(
    itunesdb_file_as_bytes: Vec<u8>,
    mut csv_writer_obj: csv::Writer<std::fs::File>,
) {
    // Photo Database counters
    let mut num_image_lists = 0;
    let mut num_image_items = 0;
    let mut num_image_names = 0;
    let mut num_photo_albums = 0;
    let mut num_photo_data_objects = 0;

    let mut images_found: Vec<photo_database::Image> = Vec::new();

    let mut curr_img = photo_database::Image::default();

    let mut idx = 0;

    while idx < (itunesdb_file_as_bytes.len() - (itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE + 1))
    {
        let potential_photo_section_heading =
            &itunesdb_file_as_bytes[idx..idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

        if potential_photo_section_heading == photo_database_constants::IMAGE_LIST_KEY.as_bytes() {
            let image_list_num_images = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_LIST_NUM_IMAGES_OFFSET,
                photo_database_constants::IMAGE_LIST_NUM_IMAGES_LEN,
            );

            println!("{} images found", image_list_num_images);
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
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_ITEM_RATING_OFFSET,
                photo_database_constants::IMAGE_ITEM_RATING_LEN,
            );

            let image_item_orig_date_timestamp_raw = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_ITEM_ORIG_DATE_OFFSET,
                photo_database_constants::IMAGE_ITEM_ORIG_DATE_LEN,
            );

            let image_item_digitized_timestamp_raw = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_ITEM_DIGITIZED_DATE_OFFSET,
                photo_database_constants::IMAGE_ITEM_DIGITIZED_DATE_LEN,
            );

            let image_item_source_img_size = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_ITEM_SOURCE_IMG_SIZE_OFFSET,
                photo_database_constants::IMAGE_ITEM_SOURCE_IMG_SIZE_LEN,
            );

            println!(
                "ImageItem#{} : {} , ImgSize= {}, OrigDateTS= {} , DigitizedDateTS= {}",
                num_image_items,
                itunesdb_helpers::decode_itunes_stars(image_item_rating as u8),
                image_item_source_img_size,
                helpers::get_timestamp_as_mac(image_item_orig_date_timestamp_raw as u64),
                helpers::get_timestamp_as_mac(image_item_digitized_timestamp_raw as u64)
            );

            println!("==========");
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
            let ithmb_offset = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_NAME_ITHMB_OFFSET_OFFSET,
                photo_database_constants::IMAGE_NAME_ITHMB_OFFSET_LEN,
            );

            let image_name_img_size = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_NAME_IMG_SIZE_OFFSET,
                photo_database_constants::IMAGE_NAME_IMG_SIZE_LEN,
            );

            // TODO: Figure out why the Image Height and Image Width are both zero sometimes?
            let image_name_img_height = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_NAME_IMG_HEIGHT_OFFSET,
                photo_database_constants::IMAGE_NAME_IMG_HEIGHT_LEN,
            );

            let image_name_img_width = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
                photo_database_constants::IMAGE_NAME_IMG_WIDTH_OFFSET,
                photo_database_constants::IMAGE_NAME_IMG_WIDTH_LEN,
            );

            println!(
                "ImageName#{} : Size= {} bytes, Height={} , Width={} | thumbnail offset {}",
                num_image_names,
                image_name_img_size,
                image_name_img_height,
                image_name_img_width,
                ithmb_offset
            );
            println!("==========");

            num_image_names += 1;

            idx += photo_database_constants::IMAGE_NAME_LAST_OFFSET;

            //curr_img.file_size_bytes = image_name_img_size as u64;
            curr_img.set_filesize(image_name_img_size);

            curr_img.ithmb_offset = ithmb_offset;
        }
        // Parse Photo Album
        else if potential_photo_section_heading
            == photo_database_constants::PHOTO_ALBUM_KEY.as_bytes()
        {
            let _photo_album_item_count = helpers::get_slice_as_le_u32(
                idx,
                &itunesdb_file_as_bytes,
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
                &itunesdb_file_as_bytes,
                photo_database_constants::DATA_OBJECT_TYPE_OFFSET,
                photo_database_constants::DATA_OBJECT_TYPE_LEN,
            );

            if data_object_type == (photo_database::MhodType::AlbumName as u32)
                || data_object_type == (photo_database::MhodType::FileName as u32)
            {
                let data_object_subcontainer_str_len = helpers::get_slice_as_le_u32(
                    idx,
                    &itunesdb_file_as_bytes,
                    photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_OFFSET,
                    photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_LENGTH_LEN,
                );

                let data_object_subcontainer_encoding = helpers::get_slice_as_le_u32(
                    idx,
                    &itunesdb_file_as_bytes,
                    photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_OFFSET,
                    photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_ENCODING_LEN,
                );

                if data_object_subcontainer_encoding == 0 || data_object_subcontainer_encoding == 1
                {
                    // TODO: Figure out why I'm off by a width of 4 on the length.
                    // Same issue with UTF-16 encoding (below)

                    let data_object_string_bytes = helpers::get_slice_from_offset_with_len(
                        idx,
                        &itunesdb_file_as_bytes,
                        photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET,
                        data_object_subcontainer_str_len as usize,
                    );

                    let data_object_subcontainer_data =
                        std::str::from_utf8(&data_object_string_bytes)
                            .expect("Can't convert UTF-8 array to MHOD string");

                    //println!("MHOD substring = {}", data_object_subcontainer_data);

                    curr_img.set_filename(data_object_subcontainer_data.to_string());
                } else if data_object_subcontainer_encoding == 2 {
                    let data_object_pairwise_combined =
                        &helpers::return_utf16_from_utf8(&helpers::get_slice_from_offset_with_len(
                            idx,
                            &itunesdb_file_as_bytes,
                            photo_database_constants::DATA_OBJECT_STRING_SUBCONTAINER_DATA_OFFSET
                                + 4,
                            (data_object_subcontainer_str_len) as usize,
                        ));

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

            let _mhod_type = photo_database::decode_mhod_type(data_object_type as u16);

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

        idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE;
    } // end while

    println!("{} images found", images_found.len());

    // Setup columns of CSV file
    // TODO see if there's a way to get the struct field names as strings?
    csv_writer_obj
        .write_record(&[
            "Filename",
            "File size (bytes)",
            "File size",
            "Original Date (Mac epoch)",
            "Original Date",
            "Digitized Date (Mac epoch)",
            "Digitized Date",
            "iThmb Offset",
        ])
        .expect("Can't create CSV headers"); // TODO better log message

    for image in images_found.iter() {
        //println!("Image filename = {}, Image size (raw) = {}, Image size = {}", image.filename, image.file_size_bytes, image.file_size_human_readable);

        // Need quotes around filename in case there's spaces in it
        csv_writer_obj
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
                image.ithmb_offset.to_string(),
            ])
            .expect("Can't write row");
    }
}
