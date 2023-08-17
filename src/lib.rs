//! A library that provides simple types and traits for representing pitch, where the octave is divided into twelve equally tempered parts
use std::collections::HashSet;
use std::fmt::Display;
use std::iter::FromIterator;
use std::ops::Range;
use std::ops::{Add, Rem, Sub};

pub mod prelude {
    pub use super::*;
}

pub const A_440_FREQUENCY: f64 = 440.0;
pub const A_440_OCTAVE: u8 = 4;
pub const A_440_HALFSTEPS_FROM_0: u32 = 45;
pub const SEMITONE_FREQUENCY_RATIO: f64 = 1.059463094;
pub const BASS_VOICE_OCTAVE_RANGE: Range<u8> = 2..5;
pub const BASS_VOICE_PITCH_CLASS_LOWER_BOUND: u8 = 4;
pub const BASS_VOICE_PITCH_CLASS_UPPER_BOUND: u8 = 0;
pub const TENOR_VOICE_OCTAVE_RANGE: Range<u8> = 3..5;
pub const TENOR_VOICE_PITCH_CLASS_LOWER_BOUND: u8 = 3;
pub const TENOR_VOICE_PITCH_CLASS_UPPER_BOUND: u8 = 6;
pub const ALTO_VOICE_OCTAVE_RANGE: Range<u8> = 3..6;
pub const ALTO_VOICE_PITCH_CLASS_LOWER_BOUND: u8 = 7;
pub const ALTO_VOICE_PITCH_CLASS_UPPER_BOUND: u8 = 1;
pub const SOPRANO_VOICE_OCTAVE_RANGE: Range<u8> = 4..6;
pub const SOPRANO_VOICE_PITCH_CLASS_LOWER_BOUND: u8 = 2;
pub const SOPRANO_VOICE_PITCH_CLASS_UPPER_BOUND: u8 = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pitch {
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

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note = match self.pitch_class {
            0 => format!("{}{}", "C", self.octave),
            1 => format!("{}{}", "C#/Db", self.octave),
            2 => format!("{}{}", "D", self.octave),
            3 => format!("{}{}", "D#/Eb", self.octave),
            4 => format!("{}{}", "E", self.octave),
            5 => format!("{}{}", "F", self.octave),
            6 => format!("{}{}", "F#/Gb", self.octave),
            7 => format!("{}{}", "G", self.octave),
            8 => format!("{}{}", "G#/Ab", self.octave),
            9 => format!("{}{}", "A", self.octave),
            10 => format!("{}{}", "A#/Bb", self.octave),
            _ => format!("{}{}", "B", self.octave),
        };
        write!(
            f,
            "{} {:4}, pitch_class: {}",
            note, self.frequency, self.pitch_class
        )
    }
}
/// A trait for performing mod 12 arithmetic. Useful for comparing pitch classes when pitch classes are represented as integers modulo 12.
pub trait PitchClassArithmetic<T>
where
    T: Sized + Add<T> + Sub<T> + Rem<T>,
{
    /// Required method, takes self and returns the distance in half steps from `self` and `other`.
    fn dist(&self, other: &Self) -> Self;

    /// Required method, takes `other` and returns a boolean. True if the interval from `self` and `other` form a third.
    fn is_third(&self, other: &Self) -> bool;

    /// Required method, takes `other` and returns a boolean. True if the interval between `self` and `other` form a 5th.
    fn is_fifth(&self, other: &Self) -> bool;

    /// Required method, takes `other` and returns a boolean. True if the interval between `self` and `other` form a 7th.
    fn is_seventh(&self, other: &Self) -> bool;
}

// Implement for `u8`.
impl PitchClassArithmetic<u8> for u8 {
    /// Compute the distance between `self` and `other` modulo 12.
    fn dist(&self, other: &Self) -> Self {
        if self > other {
            (other + (12 - self)) % 12
        } else {
            other - self
        }
    }

    /// Checks whether the interval between `self` and `other` is a 3rd of some kind.
    /// Note it returns true in the case that the interval is a minor 3rd or major 3rd.
    fn is_third(&self, other: &Self) -> bool {
        self.dist(other) == 3 || self.dist(other) == 4
    }

    /// Checks whether the interval between `self` and `other` is a 5th of some kind.
    /// Note it returns true in the case that the interval is a diminished 5th.
    fn is_fifth(&self, other: &Self) -> bool {
        self.dist(other) == 7 || self.dist(other) == 6
    }

    /// Checks whether the interval between `self` and `other` is a 7th of some kind.
    /// Note it returns true in the case that the interval is a diminished 7th.
    fn is_seventh(&self, other: &Self) -> bool {
        self.dist(other) == 11 || self.dist(other) == 10 || self.dist(other) == 9
    }
}

/// A trait that all harmonies, implement.
pub trait Harmony {
    /// Required method, each `Harmony` must implement a method to return duration of a sound wave.
    /// `duration` represents the time in seconds of the requested harmony, `sample_freq` represents the rate at which the
    /// sound wave is sampled.
    fn sound_wave(&self, duration: u32, sample_freq: u32) -> Vec<f64>;
}

/// A struct that represents a traditional harmony comprised of alto, soprano, tenor and bass voices. In short it represents a harmony
/// used in traditional four part voice leading.
pub struct SATB {
    /// Soprano voice
    pub soprano: Pitch,
    /// Alto voice,
    pub alto: Pitch,
    /// Tenor voice,
    pub tenor: Pitch,
    /// Bass voice,
    pub bass: Pitch,
    /// The root of the harmony
    root: u8,
    /// Collection of all possible pitch classes
    pitch_classes: HashSet<u8>,
}

impl SATB {
    /// Private helper method to validate the range of each voice in the given harmony comprised of `soprano`, `alto`, `tenor` and `bass`.
    /// Returns a boolean, true if all voices are within valid ranges and adjacent voices have a distance no greater than an octave between them, false otherwise.
    fn validate_voice_ranges(soprano: &Pitch, alto: &Pitch, tenor: &Pitch, bass: &Pitch) -> bool {
        // Check the bass
        if bass.octave < 2 || bass.octave > 4 {
            return false;
        } else {
            // Check basses end points
            if bass.octave == 2 && bass.pitch_class < 4 {
                return false;
            } else if bass.octave == 4 && bass.pitch_class > 0 {
                return false;
            } else if (bass.octave.abs_diff(tenor.octave) == 1
                && bass.octave.dist(&tenor.octave) > 7)
                || (bass.octave.abs_diff(tenor.octave) == 0 && bass.pitch_class > tenor.pitch_class)
            {
                return false;
            }
        }
        // Check the tenor
        if tenor.octave < 3 || tenor.octave > 4 {
            return false;
        } else {
            // Check the end points
            if tenor.octave == 3 && tenor.pitch_class < 3 {
                return false;
            } else if tenor.octave == 4 && tenor.pitch_class > 6 {
                return false;
            } else if compute_semi_tone_dist(
                (tenor.pitch_class, tenor.octave),
                (alto.pitch_class, alto.octave),
            ) > 12
                || (tenor.octave.abs_diff(alto.octave) == 0 && tenor.pitch_class > alto.pitch_class)
            {
                return false;
            }
        }
        // Check alto
        if alto.octave < 3 || alto.octave > 5 {
            return false;
        } else {
            // Check the end points of the alot voice
            if alto.octave == 3 && alto.pitch_class < 7 {
                return false;
            } else if alto.octave == 5 && alto.pitch_class > 1 {
                return false;
            } else if compute_semi_tone_dist(
                (alto.pitch_class, alto.octave),
                (soprano.pitch_class, soprano.octave),
            ) > 12
                || (alto.octave.abs_diff(soprano.octave) == 0
                    && alto.pitch_class > soprano.pitch_class)
            {
                return false;
            }
        }
        // Check soprano
        if soprano.octave < 4 || soprano.octave > 5 {
            return false;
        } else {
            // Check the end points of the valid range
            if soprano.octave == 4 && soprano.pitch_class < 2 {
                return false;
            } else if soprano.octave == 5 && soprano.pitch_class > 6 {
                return false;
            }
        }
        true
    }

    /// Associated helper  method to validate a given harmony, each voice is represented as a `Pitch`.
    fn validate_harmony(
        root: u8,
        soprano: &Pitch,
        alto: &Pitch,
        tenor: &Pitch,
        bass: &Pitch,
    ) -> bool {
        // Validate the range for each voice
        if !SATB::validate_voice_ranges(&soprano, &alto, &tenor, &bass) {
            return false;
        }
        // Ensure that atleast one voice is the root of the harmony
        if !(soprano.pitch_class == root
            || alto.pitch_class == root
            || tenor.pitch_class == root
            || bass.pitch_class == root)
        {
            return false;
        }
        // Count the number of distinct voices
        let mut distinct_voices = 1;
        if bass.pitch_class != root {
            distinct_voices += 1;
        }
        if tenor.pitch_class != root && tenor.pitch_class != bass.pitch_class {
            distinct_voices += 1;
        }
        if alto.pitch_class != root
            && alto.pitch_class != tenor.pitch_class
            && alto.pitch_class != bass.pitch_class
        {
            distinct_voices += 1;
        }
        if soprano.pitch_class != root
            && soprano.pitch_class != alto.pitch_class
            && soprano.pitch_class != tenor.pitch_class
            && soprano.pitch_class != bass.pitch_class
        {
            distinct_voices += 1;
        }

        // Ensure we have either 2, 3 or 4 distinct voices, all other cases are invalid harmonies.
        // The case where we have two distinc voices, all voices need to be either the root or the third only.
        if distinct_voices == 2 {
            if !((soprano.pitch_class == root || root.is_third(&soprano.pitch_class))
                && (alto.pitch_class == root || root.is_third(&alto.pitch_class))
                && (tenor.pitch_class == root || root.is_third(&tenor.pitch_class))
                && (bass.pitch_class == root || root.is_third(&bass.pitch_class)))
            {
                return false;
            } else {
                return true;
            }
        } else if distinct_voices == 3 {
            // We have a triad in this case, check that the voicing is valid for its inversion
            if bass.pitch_class == root {
                return (tenor.pitch_class == root
                    && ((root.is_third(&alto.pitch_class)
                        && root.is_fifth(&soprano.pitch_class))
                        || (root.is_third(&soprano.pitch_class)
                            && root.is_fifth(&alto.pitch_class))))
                    || (alto.pitch_class == root
                        && ((root.is_third(&tenor.pitch_class)
                            && root.is_fifth(&soprano.pitch_class))
                            || (root.is_third(&soprano.pitch_class)
                                && root.is_fifth(&tenor.pitch_class))))
                    || (soprano.pitch_class == root
                        && ((root.is_third(&tenor.pitch_class)
                            && root.is_fifth(&alto.pitch_class))
                            || (root.is_third(&alto.pitch_class)
                                && root.is_fifth(&tenor.pitch_class))));
            } else if root.is_third(&bass.pitch_class) {
                // Check if we have a diminished triad of some kind
                if (root.is_fifth(&soprano.pitch_class) && root.dist(&soprano.pitch_class) == 6)
                    || (root.is_fifth(&alto.pitch_class) && root.dist(&alto.pitch_class) == 6)
                    || (root.is_fifth(&tenor.pitch_class) && root.dist(&tenor.pitch_class) == 6)
                {
                    // Validate that atleast one voice is the third, i.e that the bass is doubled
                    return root.is_third(&soprano.pitch_class)
                        || root.is_third(&alto.pitch_class)
                        || root.is_third(&tenor.pitch_class);
                } else {
                    // Validate that the bass is not doubled in this case, that one voice is the root and other two are fifths
                    // or two voices are the root and one voice is the fifth
                    return (!root.is_third(&soprano.pitch_class)
                        && !root.is_third(&alto.pitch_class)
                        && !root.is_third(&tenor.pitch_class))
                        && ((root.is_fifth(&soprano.pitch_class)
                            || root.is_fifth(&alto.pitch_class)
                            || root.is_fifth(&tenor.pitch_class))
                            && (root == soprano.pitch_class
                                || root == alto.pitch_class
                                || root == tenor.pitch_class));
                }
            } else if root.is_fifth(&bass.pitch_class) {
                // Ensure that atleast one other voice is the bass
                return (root.is_fifth(&soprano.pitch_class)
                    || root.is_fifth(&alto.pitch_class)
                    || root.is_fifth(&tenor.pitch_class))
                    && ((root.is_third(&soprano.pitch_class)
                        || root.is_third(&alto.pitch_class)
                        || root.is_third(&tenor.pitch_class))
                        && (root == soprano.pitch_class
                            || root == alto.pitch_class
                            || root == tenor.pitch_class));
            } else {
                return false;
            }
        } else if distinct_voices == 4 {
            return (root == bass.pitch_class
                || root == tenor.pitch_class
                || root == alto.pitch_class
                || root == soprano.pitch_class)
                && (root.is_third(&bass.pitch_class)
                    || root.is_third(&tenor.pitch_class)
                    || root.is_third(&alto.pitch_class)
                    || root.is_third(&soprano.pitch_class))
                && (root.is_fifth(&bass.pitch_class)
                    || root.is_fifth(&tenor.pitch_class)
                    || root.is_fifth(&alto.pitch_class)
                    || root.is_fifth(&soprano.pitch_class))
                && (root.is_seventh(&bass.pitch_class)
                    || root.is_seventh(&tenor.pitch_class)
                    || root.is_seventh(&alto.pitch_class)
                    || root.is_seventh(&soprano.pitch_class));
        } else {
            return false;
        }
    }

    /// Associated method for creating a new `SATB` harmony.
    ///
    /// `Panics`
    /// If the supplied pitches do not form a valid satb harmony, i.e. there is no third or
    /// there is a pitch that is not contained within a valid satb harmony with the supplied `root`.
    pub fn new(root: u8, soprano: Pitch, alto: Pitch, tenor: Pitch, bass: Pitch) -> Self {
        if !SATB::validate_harmony(root, &soprano, &alto, &tenor, &bass) {
            panic!(
                "invalid harmony created with voices S: {}, A: {}, T: {}, B: {}",
                soprano, alto, tenor, bass
            );
        }
        SATB::new_unchecked(root, soprano, alto, tenor, bass)
    }

    /// Associated method for creating a new `SATB` harmony, without the checks for validity.
    pub fn new_unchecked(root: u8, soprano: Pitch, alto: Pitch, tenor: Pitch, bass: Pitch) -> Self {
        let mut pitch_classes = HashSet::new();
        pitch_classes.insert(soprano.pitch_class);
        pitch_classes.insert(alto.pitch_class);
        pitch_classes.insert(tenor.pitch_class);
        pitch_classes.insert(bass.pitch_class);

        SATB {
            soprano,
            alto,
            tenor,
            bass,
            root,
            pitch_classes,
        }
    }
}

use std::f64::consts::PI;

impl Harmony for SATB {
    fn sound_wave(&self, duration: u32, sample_freq: u32) -> Vec<f64> {
        let mut wave = Vec::new();
        for _ in 0..duration {
            for t in (0..sample_freq).map(|x| (x as f64) / (sample_freq as f64)) {
                wave.push(
                    f64::sin(self.soprano.frequency * 2.0 * PI * (t as f64))
                        + f64::sin(self.alto.frequency * 2.0 * PI * (t as f64))
                        + f64::sin(self.tenor.frequency * 2.0 * PI * (t as f64))
                        + f64::sin(self.bass.frequency * 2.0 * PI * (t as f64)),
                );
            }
        }
        wave
    }
}

/// A function that will take two tuples of `u8` that represent different pitches i.e. pitch class and octave and compute the number of semitones between them.
/// Note that it computes the absolute difference in semitones.
pub fn compute_semi_tone_dist(pitch1: (u8, u8), pitch2: (u8, u8)) -> u32 {
    if pitch1.1 == pitch2.1 {
        let (high, low) = if pitch1.0 > pitch2.0 {
            (pitch1, pitch2)
        } else {
            (pitch2, pitch1)
        };
        return low.0.dist(&high.0) as u32;
    } else {
        let (high, low) = if pitch1.1 > pitch2.1 {
            (pitch1, pitch2)
        } else {
            (pitch2, pitch1)
        };
        // convert to semitones
        let high_semi_tones = 12 * (high.1 as u32) + (high.0 as u32);
        let low_semi_tones = 12 * (low.1 as u32) + (low.0 as u32);
        return high_semi_tones - low_semi_tones;
    }
}

/// A function for validating potential harmonies before being created, checks to ensure each voice is within a proper range.
/// Each voice is represented as a tuple of `u8`s i.e (pitch_class, octave).
/// Returns true if the given voices are all contained within their appropraite ranges, false otherwise.
pub fn validate_voice_ranges(
    soprano: (u8, u8),
    alto: (u8, u8),
    tenor: (u8, u8),
    bass: (u8, u8),
) -> bool {
    // Check the bass
    if bass.1 < 2 || bass.1 > 4 {
        return false;
    } else {
        // Check basses end points
        if bass.1 == 2 && bass.0 < 4 {
            return false;
        } else if bass.1 == 4 && bass.0 > 0 {
            return false;
        } else if (bass.1.abs_diff(tenor.1) == 1 && bass.0.dist(&tenor.0) > 7)
            || (bass.1.abs_diff(tenor.1) == 0 && bass.0 > tenor.0)
        {
            return false;
        }
    }
    // Check the tenor
    if tenor.1 < 3 || tenor.1 > 4 {
        return false;
    } else {
        // Check the end points
        if tenor.1 == 3 && tenor.0 < 3 {
            return false;
        } else if tenor.1 == 4 && tenor.0 > 6 {
            return false;
        } else if (tenor.1.abs_diff(alto.1) == 1 && tenor.0 != alto.0)
            || (tenor.1.abs_diff(alto.1) == 0 && tenor.0 > alto.0)
        {
            return false;
        }
    }
    // Check alto
    if alto.1 < 3 || alto.1 > 5 {
        return false;
    } else {
        // Check the end points of the alot voice
        if alto.1 == 3 && alto.0 < 7 {
            return false;
        } else if alto.1 == 5 && alto.0 > 1 {
            return false;
        } else if (alto.1.abs_diff(soprano.1) == 1 && alto.0 != soprano.0)
            || (alto.1.abs_diff(soprano.1) == 0 && alto.0 > soprano.0)
        {
            return false;
        }
    }
    // Check soprano
    if soprano.1 < 4 || soprano.1 > 5 {
        return false;
    } else {
        // Check the end points of the valid range
        if soprano.1 == 4 && soprano.0 < 2 {
            return false;
        } else if soprano.1 == 5 && soprano.0 > 6 {
            return false;
        }
    }

    true
}

/// A function for determining whether or not that the given tuples of (pitch_class, octave) form a valid SATB harmony in classical voice leading.
/// Returns true if `soprano`, `alto`, `tenor` and `bass` form a valid harmony determined by the rulest of 4 part harmony in classical voice leading,
/// false otherwise.
fn validate_harmony(
    root: u8,
    soprano: (u8, u8),
    alto: (u8, u8),
    tenor: (u8, u8),
    bass: (u8, u8),
) -> bool {
    // Validate the range for each voice
    if !validate_voice_ranges(soprano, alto, tenor, bass) {
        return false;
    }
    // Ensure that atleast one voice is the root of the harmony
    if !(soprano.0 == root || alto.0 == root || tenor.0 == root || bass.0 == root) {
        return false;
    }
    // Count the number of distinct voices
    let mut distinct_voices = 1;
    if bass.0 != root {
        distinct_voices += 1;
    }
    if tenor.0 != root && tenor.0 != bass.0 {
        distinct_voices += 1;
    }
    if alto.0 != root && alto.0 != tenor.0 && alto.0 != bass.0 {
        distinct_voices += 1;
    }
    if soprano.0 != root && soprano.0 != alto.0 && soprano.0 != tenor.0 && soprano.0 != bass.0 {
        distinct_voices += 1;
    }

    // Ensure we have either 2, 3 or 4 distinct voices, all other cases are invalid harmonies.
    // The case where we have two distinc voices, all voices need to be either the root or the third only.
    if distinct_voices == 2 {
        if !((soprano.0 == root || root.is_third(&soprano.0))
            && (alto.0 == root || root.is_third(&alto.0))
            && (tenor.0 == root || root.is_third(&tenor.0))
            && (bass.0 == root || root.is_third(&bass.0)))
        {
            return false;
        } else {
            return true;
        }
    } else if distinct_voices == 3 {
        // We have a triad in this case, check that the voicing is valid for its inversion
        if bass.0 == root {
            return (tenor.0 == root
                && ((root.is_third(&alto.0) && root.is_fifth(&soprano.0))
                    || (root.is_third(&soprano.0) && root.is_fifth(&alto.0))))
                || (alto.0 == root
                    && ((root.is_third(&tenor.0) && root.is_fifth(&soprano.0))
                        || (root.is_third(&soprano.0) && root.is_fifth(&tenor.0))))
                || (soprano.0 == root
                    && ((root.is_third(&tenor.0) && root.is_fifth(&alto.0))
                        || (root.is_third(&alto.0) && root.is_fifth(&tenor.0))));
        } else if root.is_third(&bass.0) {
            // Check if we have a diminished triad of some kind
            if (root.is_fifth(&soprano.0) && root.dist(&soprano.0) == 6)
                || (root.is_fifth(&alto.0) && root.dist(&alto.0) == 6)
                || (root.is_fifth(&tenor.0) && root.dist(&tenor.0) == 6)
            {
                // Validate that atleast one voice is the third, i.e that the bass is doubled
                return root.is_third(&soprano.0)
                    || root.is_third(&alto.0)
                    || root.is_third(&tenor.0);
            } else {
                // Validate that the bass is not doubled in this case, that one voice is the root and other two are fifths
                // or two voices are the root and one voice is the fifth
                return (!root.is_third(&soprano.0)
                    && !root.is_third(&alto.0)
                    && !root.is_third(&tenor.0))
                    && ((root.is_fifth(&soprano.0)
                        || root.is_fifth(&alto.0)
                        || root.is_fifth(&tenor.0))
                        && (root == soprano.0 || root == alto.0 || root == tenor.0));
            }
        } else if root.is_fifth(&bass.0) {
            // Ensure that atleast one other voice is the bass
            return (root.is_fifth(&soprano.0)
                || root.is_fifth(&alto.0)
                || root.is_fifth(&tenor.0))
                && ((root.is_third(&soprano.0)
                    || root.is_third(&alto.0)
                    || root.is_third(&tenor.0))
                    && (root == soprano.0 || root == alto.0 || root == tenor.0));
        } else {
            return false;
        }
    } else if distinct_voices == 4 {
        return (root == bass.0 || root == tenor.0 || root == alto.0 || root == soprano.0)
            && (root.is_third(&bass.0)
                || root.is_third(&tenor.0)
                || root.is_third(&alto.0)
                || root.is_third(&soprano.0))
            && (root.is_fifth(&bass.0)
                || root.is_fifth(&tenor.0)
                || root.is_fifth(&alto.0)
                || root.is_fifth(&soprano.0))
            && (root.is_seventh(&bass.0)
                || root.is_seventh(&tenor.0)
                || root.is_seventh(&alto.0)
                || root.is_seventh(&soprano.0));
    } else {
        return false;
    }
}

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

    #[test]
    fn test_compute_semi_tone_dist() {
        let dist = compute_semi_tone_dist((4, 3), (7, 4));
        println!("{:?}", dist);
        assert_eq!(dist, 15);

        let dist = compute_semi_tone_dist((4, 4), (7, 4));
        println!("{:?}", dist);
        assert_eq!(dist, 3);

        let dist = compute_semi_tone_dist((4, 4), (0, 5));
        println!("{:?}", dist);
        assert_eq!(dist, 8);
    }
}
