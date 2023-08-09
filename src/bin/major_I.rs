use hound;
use twelve_et::{Harmony, Pitch, SATB};

fn main() {
    let bass = Pitch::new(130.81, 0, 3);
    let tenor = Pitch::new(196.00, 7, 3);
    let alto = Pitch::new(261.63, 0, 4);
    let soprano = Pitch::new(329.63, 4, 4);
    let major_i = SATB::new_unchecked(0, soprano, alto, tenor, bass);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create("sin.wav", spec).unwrap();
    let sample = major_i.sound_wave(5, 44100);

    for samp in sample {
        let _ = writer.write_sample(samp as f32);
    }

    let tenor = Pitch::new(220.00, 9, 3);
    let alto = Pitch::new(293.66, 2, 4);
    let soprano = Pitch::new(349.23, 5, 4);
    let minor_ii_4_2 = SATB::new_unchecked(2, soprano, alto, tenor, bass);
    for sample in minor_ii_4_2.sound_wave(5, 44100) {
        let _ = writer.write_sample(sample as f32);
    }
}
