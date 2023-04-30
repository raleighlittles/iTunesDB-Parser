

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
    const image_name_img_size_offset : u8 = 24; // 4 * 6
    const image_name_img_size_len : u8 = 4;

    const image_name_img_height_offset : u8 = 32; // 4 * 8
    const image_name_img_height_len : u8 = 2;

    const image_name_img_width_offset : u8 = image_name_img_height_offset + image_name_img_height_len;
    const image_name_img_width_len : u8 = image_name_img_height_len;


    // ----- PHOTO ALBUM -----
    const photo_album_key : &str = "mhba";
    const photo_album_key_ascii : &[u8] = photo_album_key.as_bytes();

    const photo_album_album_item_cnt_offset : u8 = 16; // 4 * 4
    const photo_album_album_item_cnt_len : u8 = 4;

    // ----- Data Object -----
    const data_object_key : &str = "mhod";
    const data_object_key_ascii  : &[u8] = data_object_key.as_bytes();

    const data_object_type_offset : u8 = 12; // 4 * 8
    const data_object_type_len : u8 = 2;

    // ----------------------- End key name definitions



    if itunesdb_file_type == "photo" {

        // for seq in db_file_as_bytes.windows(SUBSTRUCTURE_SIZE) {
        //     let seq_as_ascii = std::str::from_utf8(&seq).expect("Can't convert sequence to string");
            
        //     // Search for ImageList sub-item first
        //     if seq_as_ascii == "mhli" {

        //     }
        // }

        let mut idx = 0;

        while idx < db_file_as_bytes.len() { // TODO: Inside of each of the ifs, advance the index correctly
            
            // Parse Image List

            if (db_file_as_bytes[idx] == image_list_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == image_list_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == image_list_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == image_list_key_ascii[3]) {

                let image_list_num_images : u32 = db_file_as_bytes[idx + image_list_num_images_offset as usize .. idx + image_list_num_images_offset + image_list_num_images_len].iter().map(|i| (*i) as u32).sum();

                println!("ImageList info... NumImages={}", image_list_num_images);
            }

            // Parse Image Item

            if (db_file_as_bytes[idx] == image_item_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == image_item_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == image_item_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == image_item_key_ascii[3]) {

                let image_item_rating = std::str::from_utf8(&db_file_as_bytes[idx + image_item_rating_offset .. idx + image_item_rating_offset + image_item_rating_len]).expect("Can't convert raw image rating to string");

                // TODO: Add try-catch for commented out blocks

                //let image_item_orig_date = std::str::from_utf8(&db_file_as_bytes[idx + image_item_orig_date_offset .. idx + image_item_orig_date_offset + image_item_orig_date_len]).expect("Can't convert orig date to string");

                // let image_item_digitized_date = std::str::from_utf8(&db_file_as_bytes[idx + image_item_digitized_date_offset .. idx + image_item_digitized_date_offset + image_item_digitized_date_len]).expect("Can't convert digitized date to string");

                let image_item_source_img_size = std::str::from_utf8(&db_file_as_bytes[idx + image_item_source_img_size_offset .. idx + image_item_source_img_size_offset + image_item_source_img_size_len]).expect("Can't convert source image size to string");

                // println!("ImageItem info... Rating={} , OriginalDate={} , DigitizedDate={} , SourceImgSize={} ", image_item_rating, image_item_orig_date, image_item_digitized_date, image_item_source_img_size);

                // println!("ImageItem info... Rating={} , DigitizedDate={} , SourceImgSize={} ", image_item_rating, image_item_digitized_date, image_item_source_img_size);

                println!("ImageItem info... Rating={} , SourceImgSize={} ", image_item_rating, image_item_source_img_size);
            }

            // Parse Image Name
            



            idx = idx + SUBSTRUCTURE_SIZE;
        }
    }



}
