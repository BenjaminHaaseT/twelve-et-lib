use hound;
use twelve_et::{Harmony, Pitch, SATB};

fn main() {
    let bass = Pitch::new(130.81, 0, 3);
    let tenor = Pitch::new(196.00, 7, 3);
    let alto = Pitch::new(329.63, 4, 4);
    let soprano = Pitch::new(523.25, 0, 5);
    let major_i = SATB::new(0, soprano, alto, tenor, bass);

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

    let tenor = Pitch::new(174.61, 5, 3);
    let alto = Pitch::new(293.66, 2, 4);
    let soprano = Pitch::new(440.0, 9, 4);
    let minor_ii_4_2 = SATB::new(2, soprano, alto, tenor, bass);
    for sample in minor_ii_4_2.sound_wave(5, 44100) {
        let _ = writer.write_sample(sample as f32);
    }

    let bass = Pitch::new(146.83, 2, 3);
    let alto = Pitch::new(392.00, 7, 4);
    let soprano = Pitch::new(493.88, 11, 4);
    let major_5_6 = SATB::new(7, soprano, alto, tenor, bass);
    for sample in major_5_6.sound_wave(5, 44100) {
        let _ = writer.write_sample(sample as f32);
    }
}
