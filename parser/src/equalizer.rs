
pub struct EqualizerPreset {
    pub equalizer_preset_name: String,
    pub equalizer_preset_name_len: usize,
    pub equalizer_preamp_value_raw: i32,
    pub equalizer_preamp_value_db: f32,
    pub equalizer_num_of_bands: u8,
    pub equalizer_settings: Vec<EqualizerFrequencySetting>
}

impl Default for EqualizerPreset {

    fn default() -> EqualizerPreset {

        return EqualizerPreset {
            equalizer_preset_name: String::new(),
            equalizer_preset_name_len: 0,
            equalizer_preamp_value_raw: 0,
            equalizer_preamp_value_db: 0.0,
            equalizer_num_of_bands: 0,
            equalizer_settings: Vec::new()
        };
    }
 }

pub struct EqualizerFrequencySetting {
    pub real_frequency_hz: u32,
    pub display_frequency: String,
    pub gain_raw: i32,
    pub gain_db: f32
}

impl Default for EqualizerFrequencySetting {

    fn default() -> EqualizerFrequencySetting {

        return EqualizerFrequencySetting {
            real_frequency_hz: 0,
            display_frequency: String::new(),
            gain_raw: 0,
            gain_db: 0.0
        };
    }
}

pub fn convert_raw_gain_to_db(gain_raw: i32) -> f32 {
    return gain_raw as f32 / 100.0;
}

pub const EQUALIZER_ITUNES_BANDS_REAL_FREQUENCIES_HZ : [u32; 10] = [
    u32::pow(2, 5), // 32 Hz
    u32::pow(2,6), // 64 Hz
    u32::pow(2,7), // 128 Hz
    u32::pow(2,8), // 256 Hz
    u32::pow(2,9), // 512 Hz
    u32::pow(2, 10), // 1024 Hz
    u32::pow(2,11), // 2048 Hz
    u32::pow(2,12), // 4096 Hz
    u32::pow(2,13), // 8192 Hz
    u32::pow(2,14) // 16,384 Hz
];

pub const EQUALIZER_ITUNES_BANDS_DISPLAY_FREQUENCIES : [&str; 10] = [
    "32", "64", "125", "250", "500", "1K", "2K", "4K", "8K", "16K"
];