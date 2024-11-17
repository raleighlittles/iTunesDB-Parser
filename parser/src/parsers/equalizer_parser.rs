use crate::constants::equalizer_constants;
use crate::constants::itunesdb_constants;

use crate::helpers::helpers;

pub fn parse_equalizer_file(equalizer_file_as_bytes: Vec<u8>) {
    let mut idx: usize = 0;

    while idx < (equalizer_file_as_bytes.len() - itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE) {
        let equalizer_type_heading: &[u8] =
            &equalizer_file_as_bytes[idx..idx + itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE];

        if equalizer_type_heading
            == equalizer_constants::EQUALIZER_PRESET_CONTAINER_OBJECT_KEY.as_bytes()
        {
            let num_presets = helpers::get_slice_as_le_u32(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_NUMBER_OF_PRESETS_OFFSET,
                equalizer_constants::EQUALIZER_NUMBER_OF_PRESETS_LEN,
            );

            println!("Equalizer file has {} presets", num_presets);

            let preset_child_size = helpers::get_slice_as_le_u32(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_PRESET_CHILDSIZE_OFFSET,
                equalizer_constants::EQUALIZER_PRESET_CHILDSIZE_LEN,
            );

            if preset_child_size != equalizer_constants::EQUALIZER_PRESET_CHILDSIZE_VALUE as u32 {
                panic!("Invalid preset child size value of '{}'", preset_child_size);
            }

            println!("==========");
        } else if equalizer_type_heading
            == equalizer_constants::EQUALIZER_PRESET_PRESET_OBJECT_KEY.as_bytes()
        {
            let preset_name_length = helpers::build_le_u16_from_bytes(
                &equalizer_file_as_bytes[idx
                    + equalizer_constants::EQUALIZER_PRESET_PRESET_NAME_LENGTH_OFFSET
                    ..idx + equalizer_constants::EQUALIZER_PRESET_PRESET_NAME_LENGTH_OFFSET + 2],
            ) as usize;

            if preset_name_length > equalizer_constants::EQUALIZER_PRESET_FIELD_MAX_LENGTH {
                panic!(
                    "Invalid preset name length value of '{}'",
                    preset_name_length
                );
            }

            println!("Preset Name Length: {}", preset_name_length);

            // Factor of 2 to account for UTF-16 encoding (2 bytes per character)
            let preset_name_raw_bytes = helpers::get_slice_from_offset_with_len(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_PRESET_NAME_OFFSET,
                preset_name_length * 2,
            );

            println!(
                "Preset Name: {:?}",
                String::from_utf16(&helpers::return_utf16_from_utf8(&preset_name_raw_bytes))
                    .unwrap()
            );

            let preamp = helpers::get_slice_as_le_u32(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_PREAMP_OFFSET,
                equalizer_constants::EQUALIZER_PREAMP_LEN,
            );

            println!("Preamp value: {}", preamp);

            let num_itunes_bands = helpers::get_slice_as_le_u32(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_NUM_OF_ITUNES_BANDS_OFFSET,
                4,
            );

            if num_itunes_bands
                != equalizer_constants::EQUALIZER_NUM_OF_ITUNES_BANDS_EXPECTED_VALUE as u32
            {
                panic!(
                    "Invalid number of equalizer bands value of '{}'",
                    num_itunes_bands
                );
            }

            let equalizer_itunes_band_values_bytes = helpers::get_slice_from_offset_with_len(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_ITUNES_BAND_VALUES_OFFSET,
                equalizer_constants::EQUALIZER_ITUNES_BAND_VALUES_LEN,
            );

            for i in (0..equalizer_itunes_band_values_bytes.len()).step_by(4) {
                let band_value =
                    helpers::build_le_u32_from_bytes(&equalizer_itunes_band_values_bytes[i..i + 4])
                        as i32;

                if band_value > equalizer_constants::MAX_EQUALIZER_BAND_VALUE
                    || band_value < equalizer_constants::MIN_EQUALIZER_BAND_VALUE
                {
                    panic!("Invalid equalizer band value of '{}'", band_value);
                }

                println!("[iTunes] Band Value: {}", band_value);
            }

            let num_dsp_bands = helpers::get_slice_as_le_u32(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_NUM_OF_DSP_BANDS_OFFSET,
                4,
            );

            if num_dsp_bands
                != equalizer_constants::EQUALIZER_NUM_OF_DSP_BANDS_EXPECTED_VALUE as u32
            {
                panic!(
                    "Invalid number of equalizer bands value of '{}'",
                    num_dsp_bands
                );
            }

            let equalizer_dsp_band_values_bytes = helpers::get_slice_from_offset_with_len(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_DSP_BAND_VALUES_OFFSET,
                equalizer_constants::EQUALIZER_DSP_BAND_VALUES_LEN,
            );

            for i in (0..equalizer_dsp_band_values_bytes.len()).step_by(4) {
                let band_value =
                    helpers::build_le_u32_from_bytes(&equalizer_itunes_band_values_bytes[i..i + 4])
                        as i32;

                if band_value > equalizer_constants::MAX_EQUALIZER_BAND_VALUE
                    || band_value < equalizer_constants::MIN_EQUALIZER_BAND_VALUE
                {
                    panic!("Invalid equalizer band value of '{}'", band_value);
                }

                println!("[DSP] Band Value: {}", band_value);
            }

            println!("-----------");
        }

        idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE
    }
}
