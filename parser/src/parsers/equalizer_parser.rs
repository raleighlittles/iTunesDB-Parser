use crate::constants::equalizer_constants;
use crate::constants::itunesdb_constants;

use crate::helpers::helpers;
use crate::equalizer;

pub fn parse_equalizer_file(equalizer_file_as_bytes: Vec<u8>, mut csv_writer_obj: csv::Writer<std::fs::File>) {
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
            let mut curr_equalizer_preset = equalizer::EqualizerPreset::default();

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

            curr_equalizer_preset.equalizer_preset_name_len = preset_name_length;

            println!("Preset Name Length: {}", preset_name_length);

            // Factor of 2 to account for UTF-16 encoding (2 bytes per character)
            let preset_name_raw_bytes = helpers::get_slice_from_offset_with_len(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_PRESET_NAME_OFFSET,
                preset_name_length * 2,
            );

            let preset_name_str = String::from_utf16(&helpers::return_utf16_from_utf8(&preset_name_raw_bytes))
            .unwrap();

            println!(
                "Preset Name: {:?}", preset_name_str
            );

            curr_equalizer_preset.equalizer_preset_name = preset_name_str;

            let preamp_value_raw = helpers::get_slice_as_le_u32(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_PREAMP_OFFSET,
                equalizer_constants::EQUALIZER_PREAMP_LEN,
            ) as i32;

            curr_equalizer_preset.equalizer_preamp_value_raw = preamp_value_raw;
            curr_equalizer_preset.equalizer_preamp_value_db = equalizer::convert_raw_gain_to_db(preamp_value_raw);

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

            curr_equalizer_preset.equalizer_num_of_bands = num_itunes_bands as u8;

            let equalizer_itunes_band_values_bytes = helpers::get_slice_from_offset_with_len(
                idx,
                &equalizer_file_as_bytes,
                equalizer_constants::EQUALIZER_ITUNES_BAND_VALUES_OFFSET,
                equalizer_constants::EQUALIZER_ITUNES_BAND_VALUES_LEN,
            );

            let mut curr_band_idx = 0;
            let mut curr_equalizer_frequency_settings = Vec::<equalizer::EqualizerFrequencySetting>::new();

            for i in (0..equalizer_itunes_band_values_bytes.len()).step_by(4) {

                let mut eq_freq_setting = equalizer::EqualizerFrequencySetting::default();

                let band_value_raw =
                    helpers::build_le_u32_from_bytes(&equalizer_itunes_band_values_bytes[i..i + 4])
                        as i32;

                if band_value_raw > equalizer_constants::MAX_EQUALIZER_BAND_VALUE
                    || band_value_raw < equalizer_constants::MIN_EQUALIZER_BAND_VALUE
                {
                    panic!("Invalid equalizer band value of '{}'", band_value_raw);
                }

                let band_value_db = equalizer::convert_raw_gain_to_db(band_value_raw);

                eq_freq_setting.gain_raw = band_value_raw;
                eq_freq_setting.gain_db = band_value_db;

                eq_freq_setting.real_frequency_hz = equalizer::EQUALIZER_ITUNES_BANDS_REAL_FREQUENCIES_HZ[curr_band_idx];

                eq_freq_setting.display_frequency = equalizer::EQUALIZER_ITUNES_BANDS_DISPLAY_FREQUENCIES[curr_band_idx].to_string();

                curr_equalizer_frequency_settings.push(eq_freq_setting);

                //println!("[iTunes] Band Value: {}", band_value);
                curr_band_idx += 1;
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

                //println!("[DSP] Band Value: {}", band_value);
            }

            csv_writer_obj.write_record(&["Preset Name", "Preset Name length", "Preamp Value (Raw)", "Preamp Value (dB)", "Band #1 Frequency (Hz)", "Band #1 Frequency display", "Band #1 Value (Raw)", "Band #1 Value (dB)", "Band #2 Frequency (Hz)", "Band #2 Frequency display", "Band #2 Value (Raw)", "Band #2 Value (dB)", "Band #3 Frequency (Hz)", "Band #3 Frequency display", "Band #3 Value (Raw)", "Band #3 Value (dB)", "Band #4 Frequency (Hz)", "Band #4 Frequency display", "Band #4 Value (Raw)", "Band #4 Value (dB)", "Band #5 Frequency (Hz)", "Band #5 Frequency display", "Band #5 Value (Raw)", "Band #5 Value (dB)", "Band #6 Frequency (Hz)", "Band #6 Frequency display", "Band #6 Value (Raw)", "Band #6 Value (dB)", "Band #7 Frequency (Hz)", "Band #7 Frequency display", "Band #7 Value (Raw)", "Band #7 Value (dB)", "Band #8 Frequency (Hz)", "Band #8 Frequency display", "Band #8 Value (Raw)", "Band #8 Value (dB)", "Band #9 Frequency (Hz)", "Band #9 Frequency display", "Band #9 Value (Raw)", "Band #9 Value (dB)", "Band #10 Frequency (Hz)", "Band #10 Frequency display", "Band #10 Value (Raw)", "Band #10 Value (dB)"]).unwrap();

            csv_writer_obj.write_record(&[format!("'{}'", curr_equalizer_preset.equalizer_preset_name), format!("{}", curr_equalizer_preset.equalizer_preset_name_len), format!("{}", curr_equalizer_preset.equalizer_preamp_value_raw), format!("{}", curr_equalizer_preset.equalizer_preamp_value_db), format!("{}", curr_equalizer_frequency_settings[0].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[0].display_frequency), format!("{}", curr_equalizer_frequency_settings[0].gain_raw), format!("{}", curr_equalizer_frequency_settings[0].gain_db), format!("{}", curr_equalizer_frequency_settings[1].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[1].display_frequency), format!("{}", curr_equalizer_frequency_settings[1].gain_raw), format!("{}", curr_equalizer_frequency_settings[1].gain_db), format!("{}", curr_equalizer_frequency_settings[2].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[2].display_frequency), format!("{}", curr_equalizer_frequency_settings[2].gain_raw), format!("{}", curr_equalizer_frequency_settings[2].gain_db), format!("{}", curr_equalizer_frequency_settings[3].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[3].display_frequency), format!("{}", curr_equalizer_frequency_settings[3].gain_raw), format!("{}", curr_equalizer_frequency_settings[3].gain_db), format!("{}", curr_equalizer_frequency_settings[4].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[4].display_frequency), format!("{}", curr_equalizer_frequency_settings[4].gain_raw), format!("{}", curr_equalizer_frequency_settings[4].gain_db), format!("{}", curr_equalizer_frequency_settings[5].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[5].display_frequency), format!("{}", curr_equalizer_frequency_settings[5].gain_raw), format!("{}", curr_equalizer_frequency_settings[5].gain_db), format!("{}", curr_equalizer_frequency_settings[6].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[6].display_frequency), format!("{}", curr_equalizer_frequency_settings[6].gain_raw), format!("{}", curr_equalizer_frequency_settings[6].gain_db), format!("{}", curr_equalizer_frequency_settings[7].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[7].display_frequency), format!("{}", curr_equalizer_frequency_settings[7].gain_raw), format!("{}", curr_equalizer_frequency_settings[7].gain_db), format!("{}", curr_equalizer_frequency_settings[8].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[8].display_frequency), format!("{}", curr_equalizer_frequency_settings[8].gain_raw), format!("{}", curr_equalizer_frequency_settings[8].gain_db), format!("{}", curr_equalizer_frequency_settings[9].real_frequency_hz), format!("{}", curr_equalizer_frequency_settings[9].display_frequency), format!("{}", curr_equalizer_frequency_settings[9].gain_raw), format!("{}", curr_equalizer_frequency_settings[9].gain_db)]).unwrap();

            println!("-----------");
        }

        idx += itunesdb_constants::DEFAULT_SUBSTRUCTURE_SIZE
    }
}
