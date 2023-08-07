//! A library that provides simple types and traits for representing pitch, where the octave is divided into twelve equally tempered parts
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::{Add, Rem, Sub};

pub const A_440_FREQUENCY: f64 = 440.0;
pub const A_440_OCTAVE: u8 = 4;
pub const A_440_HALFSTEPS_FROM_0: u32 = 45;
pub const SEMITONE_FREQUENCY_RATIO: f64 = 1.059463094;

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
    /// Required method, each `Harmony` must implement a method to return a full period representing its sound wave.
    fn one_period_frequency(&self) -> f64;
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
    root: Pitch,
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
            } else if bass.octave.abs_diff(tenor.octave) == 1 && bass.octave.dist(&tenor.octave) > 7
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
            } else if tenor.octave.abs_diff(alto.octave) == 1
                && tenor.pitch_class != alto.pitch_class
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
            } else if alto.octave.abs_diff(soprano.octave) == 1
                && alto.pitch_class != soprano.pitch_class
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
    /// Private helper method to validate a given harmony
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
            }
        } else if distinct_voices == 3 {
            // We have a triad in this case, check that the voicing is valid for its inversion
            if bass.pitch_class == root {
                return ((tenor.pitch_class == root
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
                                && root.is_fifth(&tenor.pitch_class)))));
            } else if root.is_third(&bass.pitch_class) {
            } else if root.is_fifth(&bass.pitch_class) {
            } else {
                return false;
            }
        } else if distinct_voices == 4 {
        } else {
            return false;
        }
    }

    /// Associated method for creating new `SATB` harmonies.
    ///
    /// `Panics`
    /// If the supplied pitches do not form a valid satb harmony, i.e. there is no third or
    /// there is a pitch that is not contained within a valid satb harmony with the supplied `root`.
    pub fn new(root: u8, soprano: Pitch, alto: Pitch, tenor: Pitch, bass: Pitch) -> Self {
        SATB::validate_harmony(
            root.pitch_class,
            soprano.pitch_class,
            alto.pitch_class,
            tenor.pitch_class,
            bass.pitch_class,
        );
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
