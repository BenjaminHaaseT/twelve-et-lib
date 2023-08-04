//! A library that provides simple types and traits for representing pitch, where the octave is divided into twelve equally tempered parts

const A_440_FREQUENCY: f64 = 440.0;
const A_440_OCTAVE: u8 = 4;
const A_440_HALFSTEPS_FROM_0: u32 = 45;
const SEMITONE_FREQUENCY_RATIO: f64 = 1.059463094;

#[derive(Debug, PartialEq)]
struct Pitch {
    /// The frequency of the pitch
    frequency: f64,
    /// The pitch class, represented as a `u8` modulo 12
    pitch_class: u8,
    /// Denotes the octave the pitch resides in
    octave: u8,
    /// Denotes the number of halfsteps away from 0
    half_steps_from_0: u32,
}

impl Pitch {
    /// Associated method to create a new `Ptich`.
    pub fn new(frequency: f64, pitch_class: u8, octave: u8) -> Self {
        let half_steps_from_0 = Pitch::compute_half_steps_from_zero(pitch_class, octave);
        Pitch {
            frequency,
            pitch_class,
            octave,
            half_steps_from_0,
        }
    }

    /// Associated method for computing the number of half steps away from zero given a `pitch_class` and an `octave`
    pub fn compute_half_steps_from_zero(pitch_class: u8, octave: u8) -> u32 {
        if octave > 0 {
            (octave as u32 - 1) * 12 + (pitch_class as u32)
        } else {
            pitch_class as u32
        }
    }

    /// Associated method to compute the frequency of a new pitch given an octave and a pitch class
    pub fn compute_frequency(pitch_class: u8, octave: u8) -> f64 {
        // Compute number of half steps away from 0
        let num_semitones = Pitch::compute_half_steps_from_zero(pitch_class, octave);
        // Compute and return frequency
        A_440_FREQUENCY
            * f64::powi(
                SEMITONE_FREQUENCY_RATIO,
                (num_semitones as i32) - (A_440_HALFSTEPS_FROM_0 as i32),
            )
    }
}

// struct Harmony {
//     pitches: Vec<Pitch>,
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_new_pitch() {
        let a_440 = Pitch::new(A_440_FREQUENCY, 9, 4);
        println!("{:?}", a_440);
        assert_eq!(a_440, Pitch::new(A_440_FREQUENCY, 9, 4));
    }

    #[test]
    fn test_compute_half_steps_from_zero() {
        let middle_c = Pitch::compute_half_steps_from_zero(0, 4);
        println!("{}", middle_c);
        assert_eq!(middle_c, 36);

        let a_440 = Pitch::compute_half_steps_from_zero(9, 4);
        println!("{}", a_440);
        assert_eq!(a_440, A_440_HALFSTEPS_FROM_0);
    }

    #[test]
    fn test_compute_frequency() {
        let a_440 = Pitch::compute_frequency(9, 4);
        println!("{}", a_440);
        assert_eq!(a_440, A_440_FREQUENCY);

        let middle_c = Pitch::compute_frequency(0, 4);
        println!("{}", middle_c);
        println!("{}", f64::abs(middle_c - 261.625580_f64));
        assert!(f64::abs(middle_c - 261.625580_f64) < 0.0001_f64);
    }
}
