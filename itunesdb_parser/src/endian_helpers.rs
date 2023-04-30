mod endian_helpers {

    // TODO: Build version that can handle endianness
    pub fn build_integer_from_bytes(bytes : Vec<u8>) -> u32 {

        let mut number: u32 = 0;
        const radix : u8 = 10;

        for (idx, item) in bytes.iter().enumerate() {
            number += ((*item) * radix.pow(idx as u32)) as u32;
        }

        return number;
    }

}