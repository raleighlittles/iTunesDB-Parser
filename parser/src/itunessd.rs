/**
 * 
 * File: itunessd.rs
 * 
 * Note this is shared between both standard iTunesSD and iTunesSD 3rd Generation Shuffle
 * 
 */

 #[derive(Debug)]
 pub enum iTunesSDFileType {
     MP3 = 0x01,
     AAC = 0x02,
     WAV = 0x04,
 }
 
 impl std::fmt::Display for iTunesSDFileType {
     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         write!(f, "{:?}", self)
     }
 }
 
 impl TryFrom<u32> for iTunesSDFileType {
     type Error = String;
 
     fn try_from(value: u32) -> Result<Self, Self::Error> {
         match value {
             0x01 => Ok(iTunesSDFileType::MP3),
             0x02 => Ok(iTunesSDFileType::AAC),
             0x04 => Ok(iTunesSDFileType::WAV),
             _ => Err(format!("Unknown iTunesSD file type: {}", value)),
         }
     }
 }
 