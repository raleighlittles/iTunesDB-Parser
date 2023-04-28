

fn main() {
    
    let itunesdb_filename : String = std::env::args().nth(1).expect("Missing parameter: iTunes DB filename");

    // TODO: Convert to enum
    let itunesdb_file_type : String = std::env::args().nth(2).expect("Missing parameter: iTunes DB file type. Values are: 'itunes', 'playcounts', 'otg', 'eq', 'art', or 'photo'.");

    let db_file_as_bytes : Vec<u8> = std::fs::read(itunesdb_filename).unwrap();

    const SUBSTRUCTURE_SIZE : usize = 4;

    // TODO: Move these to another module
    let image_list_key = "mhli";

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

            if (db_file_as_bytes[idx] == 0x6D) && (db_file_as_bytes[idx + 1] == 0x68) && (db_file_as_bytes[idx + 2] == 0x6C) && (db_file_as_bytes[idx + 3] == 0x69) {
                println!("Image list found, starting at idx {}", idx)
            }

            idx = idx + SUBSTRUCTURE_SIZE;
        }
    }



}
