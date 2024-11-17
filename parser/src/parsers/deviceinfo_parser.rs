
use crate::constants::deviceinfo_constants;
use crate::helpers::helpers;

pub fn parse_device_info_file(deviceinfo_file_as_bytes: Vec<u8>) {
    
    if deviceinfo_file_as_bytes.len() != deviceinfo_constants::DEVICEINFO_FILE_SIZE {
        panic!("Invalid DeviceInfo file size! Expected: {} | Got: {}", deviceinfo_constants::DEVICEINFO_FILE_SIZE, deviceinfo_file_as_bytes.len());
    }

    let ipod_name_length_raw = &deviceinfo_file_as_bytes[0..2];

    let ipod_name_length = helpers::build_le_u16_from_bytes(ipod_name_length_raw) as usize;
    
    // The strings are formatted using UTF-16 so this byte value must be a multiple of 2
    if ipod_name_length % 2 != 0 {
        panic!("Invalid iPod Name Length! Expected multiple of 2 | Got: {}", ipod_name_length);
    }

    println!("iPod Name Length: {}", ipod_name_length);

    // factor of 2 to account for UTF-16 encoding (2 bytes per character),
    // and the +2 to account for the length bytes
    let ipod_name_raw_bytes = &deviceinfo_file_as_bytes[2 .. (ipod_name_length * 2 + 2)];

    println!("iPod Name: {:?}", String::from_utf16(&helpers::return_utf16_from_utf8(ipod_name_raw_bytes)).unwrap());

    
}