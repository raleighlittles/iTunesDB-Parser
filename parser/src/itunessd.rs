/**
 * 
 * File: itunessd.rs
 * 
 * Note this is shared between both standard iTunesSD and iTunesSD 3rd Generation Shuffle
 * 
 */

 #[derive(Debug)]
 pub enum ITunesSdFileType {
     MP3 = 0x01,
     AAC = 0x02,
     WAV = 0x04,
 }
 
 impl std::fmt::Display for ITunesSdFileType {
     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         write!(f, "{:?}", self)
     }
 }
 
 impl TryFrom<u32> for ITunesSdFileType {
     type Error = String;

     fn try_from(value: u32) -> Result<Self, Self::Error> {
         match value {
             0x01 => Ok(ITunesSdFileType::MP3),
             0x02 => Ok(ITunesSdFileType::AAC),
             0x04 => Ok(ITunesSdFileType::WAV),
             _ => Err(format!("Unknown iTunesSD file type: {}", value)),
         }
     }
 }