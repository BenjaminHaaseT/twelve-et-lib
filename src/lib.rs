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
    /// Private helper method to validate a given harmony
    fn validate_harmony(root: u8, soprano: u8, alto: u8, tenor: u8, bass: u8) -> bool {
        // Ensure atleast one voice is equal to the root
        assert!(root == soprano || root == alto || root == tenor || root == bass);

        // Count number of unique voices, if we have three unique voices,
        // we have a triad if we have four we have a potential seventh chord.
        let mut unique_voices = 1;
        if root ^ bass > 0 {
            unique_voices += 1;
        }
        if root ^ soprano > 0 {
            unique_voices += 1;
        }
        if root ^ alto > 0 {
            unique_voices += 1;
        }

        if unique_voices == 3 {
            // Check the inversion of the harmony
            if root.dist(&bass) == 0 {
                return (root.dist(&tenor) == 0
                    && ((root.is_fifth(&alto) && root.is_third(&soprano))
                        || (root.is_third(&alto) && root.is_fifth(&soprano))))
                    || (root.dist(&alto) == 0
                        && ((root.is_fifth(&tenor) && root.is_third(&soprano))
                            || (root.is_fifth(&soprano) && root.is_third(&tenor))))
                    || (root.dist(&soprano) == 0
                        && ((root.is_fifth(&tenor) && root.is_third(&alto))
                            || (root.is_fifth(&alto) && root.is_third(&tenor))));
            } else if root.is_third(&bass) {
            } else if root.is_fifth(&bass) {
            } else {
            }
        } else {
        }
    }

    /// Associated method for creating new `SATB` harmonies.
    ///
    /// `Panics`
    /// If the supplied pitches do not form a valid satb harmony, i.e. there is no third or
    /// there is a pitch that is not contained within a valid satb harmony with the supplied `root`.
    pub fn new(root: Pitch, soprano: Pitch, alto: Pitch, tenor: Pitch, bass: Pitch) -> Self {
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
