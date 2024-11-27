/**
 * 
 * File: itunessd.rs
 * 
 * 
 */

 #[derive(Debug)] 
 pub enum iTunesSDFileType
{
    MP3 = 0x01,
    AAC = 0x02,
    WAV = 0x04
}

impl std::fmt::Display for iTunesSDFileType{
    
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

pub fn decode_itunessd_file_type(file_type: u32) -> iTunesSDFileType
{
    match file_type
    {
        0x01 => iTunesSDFileType::MP3,
        0x02 => iTunesSDFileType::AAC,
        0x04 => iTunesSDFileType::WAV,
        _ => panic!("Unknown iTunesSD file type: {}", file_type)
    }
}