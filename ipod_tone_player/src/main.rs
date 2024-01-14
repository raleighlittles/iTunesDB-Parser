use rodio::Source;

pub struct SoundTone {
    pub frequency_hz: u32,
    pub duration_ms: u32,
}

// Writes the given tone file out to a WAV file, and then returns the filesize of the created WAV
fn write_audio_to_wav(tone_arrangements: Vec<SoundTone>, output_wav_filename: &str) -> u64 {
    // WAV file parameters
    let sample_rate_khz: u32 = 192_000;

    let wav_spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate_khz,
        bits_per_sample: 16, // before increasing this value, make sure your sound card can support it
        sample_format: hound::SampleFormat::Int,
    };

    let mut wav_writer = hound::WavWriter::create(output_wav_filename, wav_spec).unwrap();

    for tone in tone_arrangements {
        let freq_hz = tone.frequency_hz;
        let duration_ms = tone.duration_ms;

        let freq_hz_int = freq_hz.to_string().parse::<u32>().unwrap();
        let duration_ms_int = duration_ms.to_string().parse::<u32>().unwrap();

        // Debug
        //println!("Frequency (Hz) = '{}' , Duration (ms) = '{}'", freq_hz, duration_ms);
        //println!("Frequency (Hz) = {}, Duration (ms) = {}", freq_hz_int, duration_ms_int);

        // A bunch of integer--float conversions because of how Rust handles division of integers
        let num_samples: u32 = (sample_rate_khz as f32 * (duration_ms_int as f32 / 1000.0)) as u32;

        if num_samples == 0 {
            panic!("Invalid value for samples. Check that the tone file is formatted correctly.");
        }

        for t in (0..num_samples as u32).map(|x| x as f32 / sample_rate_khz as f32) {
            let sample = (t * freq_hz_int as f32 * 2.0 * std::f32::consts::PI).sin();
            let amplitude = std::i16::MAX as f32;
            wav_writer
                .write_sample((sample * amplitude) as i16)
                .unwrap();
        }
    }

    wav_writer.finalize().unwrap();

    return std::fs::metadata(output_wav_filename).unwrap().len();
}

fn play_sound_aloud(tone_arrangements: Vec<SoundTone>) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    for tone in tone_arrangements {
        // Debug
        //println!("Playing sound at {} Hz for {} ms", tone.frequency_hz, tone.duration_ms);

        let sound_duration_ms: std::time::Duration =
            std::time::Duration::from_millis(tone.duration_ms as u64);

        let source = rodio::source::SineWave::new(tone.frequency_hz as f32)
            .take_duration(sound_duration_ms)
            .amplify(1.0);

        stream_handle.play_raw(source.convert_samples()).unwrap();
        std::thread::sleep(sound_duration_ms);
    }
}

fn parse_sounds_from_tone_file(tone_file_contents: String) -> Vec<SoundTone> {
    let mut sounds: Vec<SoundTone> = Vec::new();

    for text_line in tone_file_contents.lines() {
        if text_line == "Beep" {
            continue;
        } else {
            let tone_cmd: Vec<&str> = text_line.split_whitespace().collect();

            let [freq_hz, duration_ms] = [tone_cmd[0], tone_cmd[1]];

            let sound = SoundTone {
                frequency_hz: freq_hz.to_string().parse::<u32>().unwrap(),
                duration_ms: duration_ms.to_string().parse::<u32>().unwrap(),
            };

            sounds.push(sound);
        }
    }

    return sounds;
}

fn main() {
    let tone_filename: String = std::env::args()
        .nth(1)
        .expect("Missing first argument - tone file");
    let tone_filepath = std::path::Path::new(&tone_filename);

    if !tone_filepath.exists() {
        panic!("Tone file path '{}' doesn't exist", tone_filename);
    }

    let tone_file_text: String = std::fs::read_to_string(tone_filepath).unwrap();

    let tone_sounds = parse_sounds_from_tone_file(tone_file_text);

    let output_direction_arg : String = std::env::args().nth(2).expect("Missing second argument! Either '--file' to output to a wav file or '--sound' to play the sound directly.");

    if output_direction_arg == "--file" {
        let output_filename: String = tone_filename.clone() + ".wav";

        println!("Writing audio to '{}'", output_filename);

        let _ = write_audio_to_wav(tone_sounds, &output_filename);
    } else if output_direction_arg == "--sound" {
        play_sound_aloud(tone_sounds);
    }
}
