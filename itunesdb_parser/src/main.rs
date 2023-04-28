

fn main() {
    
    let itunesdb_filename : String = std::env::args().nth(1).expect("Missing parameter: iTunes DB filename");

    // TODO: Convert to enum
    let itunesdb_file_type : String = std::env::args().nth(2).expect("Missing parameter: iTunes DB file type. Values are: 'itunes', 'playcounts', 'otg', 'eq', 'art', or 'photo'.");

    let db_file_as_bytes : Vec<u8> = std::fs::read(itunesdb_filename).unwrap();

    const SUBSTRUCTURE_SIZE : usize = 4;

    // TODO: Move these to another module
    let image_list_key = "mhli";
    let image_list_key_ascii = image_list_key.as_bytes();

    let image_list_num_images_offset = 8; // 4 + 4
    let image_list_num_images_length = 4;

    if itunesdb_file_type == "photo" {

        // for seq in db_file_as_bytes.windows(SUBSTRUCTURE_SIZE) {
        //     let seq_as_ascii = std::str::from_utf8(&seq).expect("Can't convert sequence to string");
            
        //     // Search for ImageList sub-item first
        //     if seq_as_ascii == "mhli" {

        //     }
        // }

        let mut idx = 0;

        while idx < db_file_as_bytes.len() {
            
            // let subseq_as_str = std::str::from_utf8(&db_file_as_bytes[idx..(idx + SUBSTRUCTURE_SIZE)]).expect("Unable to convert ASCII sequence to string");

            // if subseq_as_str == image_list_key {
            //     println!("Image list found!");
            // }

            if (db_file_as_bytes[idx] == image_list_key_ascii[0]) && 
            (db_file_as_bytes[idx + 1] == image_list_key_ascii[1]) && 
            (db_file_as_bytes[idx + 2] == image_list_key_ascii[2]) && 
            (db_file_as_bytes[idx + 3] == image_list_key_ascii[3]) {

                //println!("Image list found, starting at idx {}", idx);

                let image_list_num_images : u32 = db_file_as_bytes[idx + image_list_num_images_offset .. idx + image_list_num_images_offset + image_list_num_images_length].iter().map(|i| (*i) as u32).sum();

                println!("{} images found", image_list_num_images);
            }

            idx = idx + SUBSTRUCTURE_SIZE;
        }
    }



}
