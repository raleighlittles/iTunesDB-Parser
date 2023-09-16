/**
 * 
 * File: preferences.rs
 * 
 */

use std::num::Wrapping;

pub fn is_dst_enabled(raw_dst_setting : u8) -> bool {

    let dst_enabled_val : u8 = 0x3C; // 60d

    return raw_dst_setting == dst_enabled_val;
}

pub fn decode_timezone(raw_timezone_info : u8) -> u8 {

    let gmt_timezone_const = 0x19; // 25d, supposed to represent GMT (UTC+0)

    let timezone_hour = (Wrapping(raw_timezone_info)) - Wrapping(gmt_timezone_const);

    return timezone_hour.0 / 2;
}