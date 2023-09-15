/*
 * File: playcounts.rs
 * 
*/

pub struct PlayCountEntry {

    pub num_plays : usize,
    pub num_skips : usize,
    pub rating : &str,
    pub last_played_ts : chrono::DateTime<chrono::Utc>,
    pub last_played_epoch : u64
}