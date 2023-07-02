/**
 * File: helpers.rs
 *
 */
// pub mod helpers {

    // TODO: Once Rust adds support for default arguments, add the following arguments:
    //       * endianness
    //       * radix
    pub fn build_le_u32_from_bytes(bytes: &[u8]) -> u32 {
        let mut number: u32 = 0;
        const RADIX: u32 = 256;

        for (idx, item) in bytes.iter().enumerate() {
            let summand: u32 = RADIX
                .checked_pow(idx as u32)
                .unwrap_or_else(|| panic!("Can't raise {} to power {}", RADIX, idx))
                as u32;

            number += (summand as u32) * (*item as u32);
        }

        return number;
    }
//}
