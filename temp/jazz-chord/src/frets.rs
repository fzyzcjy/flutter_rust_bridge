use std::collections::HashMap;
//use itertools::Itertools;
use itertools::Itertools;
use std::fmt::{Formatter, Display};
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::iter::FromIterator;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use crate::{ToBitmap, ToChordQuality, Quality, Change, QualityParams, Note};
use crate::bitmap::Bitmap;
use crate::power_set::PowerSet;
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
//pub struct Fret(i16);

pub trait ToChange {
    fn to_change(&self) -> Change;
}

pub type Fret = i16;

pub trait FretExt {
    fn in_first_octave(self) -> Fret;

    fn octave(self) -> Fret;

}

impl FretExt for Fret {
    fn in_first_octave(self) -> Fret {
        self.rem_euclid(12)
    }
    fn octave(self) -> Fret {
        self / 12
    }

}
pub trait ToFret {
    fn to_fret(&self) -> Fret;
}

/// Implement ToFret on the member values to get ToFrets for free
pub trait ToFrets {
    fn to_frets(&self) -> Frets;
}

impl ToFret for i16 {
    fn to_fret(&self) -> Fret {
        *self
    }
}

impl ToFret for &i16 {
    fn to_fret(&self) -> Fret {
        **self
    }
}

impl<I, T> ToFrets for I
where
    I: IntoIterator<Item = T> + Clone,
    T: ToFret,
{
    fn to_frets(&self) -> Frets {
        self.clone()
            .into_iter()
            .map(|item| item.to_fret())
            .collect()
    }
}

impl<T> ToChange for T
where
    T: ToFrets
{
    fn to_change(&self) -> Change {
        Change::from_frets(self.to_frets())
    }
}


pub trait ToPitchClassSet {
    fn to_pitch_class_set(&self) -> Frets;

    fn change_number(&self) -> i32 {
        PowerSet::<12>::Change.to_index(&self.to_bitmap())
    }

    fn combinatoric_index(&self) -> i32 {
        PowerSet::<12>::Combinatoric.to_index(&self.to_bitmap())
    }

    fn small_endian(&self) -> i32 {
        PowerSet::<12>::SmallEndian.to_index(&self.to_bitmap())
    }

    fn big_endian(&self) -> i32 {
        PowerSet::<12>::SmallEndian.to_index(&self.to_bitmap())
    }



}


impl<I, T: ToFret> ToPitchClassSet for I
where
    I: IntoIterator<Item = T> + Clone,
    T: ToFret
{
    fn to_pitch_class_set(&self) -> Frets {
        Frets {
            data: self.clone()
                .into_iter()
                .map(|item| item.to_fret().in_first_octave())
                //.collect::<Vec<_>>()
                .sorted() //sort first, because,
                .dedup() // only removes contiguous dupes
                .collect::<Vec<_>>()
        }
    }


}



impl <T: ToPitchClassSet> ToChordQuality for T {
    fn to_chord_qualities_with_options(&self, params: &QualityParams) -> Vec<Quality> {
           Quality::from_pitch_class_set_with_options(&self.to_pitch_class_set(), params)
    }
}


// impl<const N: usize> ToBitmap<N> for dyn ToPitchClass {
//     fn to_bitmap(&self) -> Bitmap<N> {
//         Bitmap::from_notes(self.to_pitch_class())
//     }
// }
impl<T: ToPitchClassSet + ?Sized, const N: usize> ToBitmap<N> for T {
    fn to_bitmap(&self) -> Bitmap<N> {
        Bitmap::from_notes_big_endian(self.to_pitch_class_set())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frets {
    data: Vec<Fret>,
}

impl Frets {
    const NEGATIVE_HARMONY_FRET_CONVERSION: [Fret; 12] = [
        7,
        6,
        5,
        4,
        3,
        2,
        1,
        0,
        11,
        10,
        9,
        8,
    ];


    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { data: Vec::with_capacity(cap) }
    }


}



// Iteration
impl IntoIterator for Frets {
    type Item = Fret;
    type IntoIter = std::vec::IntoIter<Fret>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a Frets {
    type Item = &'a Fret;
    type IntoIter = std::slice::Iter<'a, Fret>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a> IntoIterator for &'a mut Frets {
    type Item = &'a mut Fret;
    type IntoIter = std::slice::IterMut<'a, Fret>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

// Conversions
impl From<Vec<Fret>> for Frets {
    fn from(v: Vec<Fret>) -> Self {
        Frets { data: v }
    }
}

impl From<Frets> for Vec<Fret> {
    fn from(f: Frets) -> Self {
        f.data
    }
}

impl From<&[Fret]> for Frets {
    fn from(f: &[Fret]) -> Self {
        Frets {data: f.to_vec()}
    }
}

impl AsRef<[Fret]> for Frets {
    fn as_ref(&self) -> &[Fret] {
        self.data.as_slice()
    }
}

impl AsMut<[Fret]> for Frets {
    fn as_mut(&mut self) -> &mut [Fret] {
        self.data.as_mut_slice()
    }
}

// Extend
impl Extend<Fret> for Frets {
    fn extend<I: IntoIterator<Item = Fret>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

// FromIterator
impl FromIterator<Fret> for Frets {
    fn from_iter<I: IntoIterator<Item = Fret>>(iter: I) -> Self {
        Frets { data: iter.into_iter().collect() }
    }
}

// Default
impl Default for Frets {
    fn default() -> Self {
        Frets::new()
    }
}

// Deref and DerefMut gives you collection methods
// If you also have Index/IndexMut
// Deref / DerefMut – forward Vec<i16> methods
impl Deref for Frets {
    type Target = Vec<Fret>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Frets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
// TryFrom<&[i16]> – trivial here, but leaves room for future validation
// impl TryFrom<&[Fret]> for Frets {
//     type Error = &'static str;
//
//     fn try_from(slice: &[Fret]) -> Result<Self, Self::Error> {
//         // place for validation if you ever want it
//         Ok(Frets { data: slice.to_vec() })
//     }
// }

impl Display for Frets {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.data)
    }
}


pub trait FretsExt {
    fn in_first_octave(&self) -> Frets;
    fn in_first_octave_sorted(&self) -> Frets;

    fn negative_harmony(&self, fret: Fret) -> Frets;
}

impl FretsExt for Frets {
    fn in_first_octave(&self) -> Frets {
        self.data.clone().into_iter().map(|fret| fret.in_first_octave()).collect()
    }
    fn in_first_octave_sorted(&self) -> Frets {
        let mut data = self.in_first_octave();
        data.sort();
        data.into()
    }

    fn negative_harmony(&self, root: Fret) -> Frets {
        let conversion = &Self::NEGATIVE_HARMONY_FRET_CONVERSION;
        self.iter().map(|fret|{
            (conversion[fret.in_first_octave() as usize] + fret.octave() * 12 - root).in_first_octave()
        }).collect()

        //     0   1    2   3    4     5     6      7     8     9    10  11
        //
        //     7   6   5   4     3     2     1      0     11   10   9     8


        //     1    b2  2   b3    3     4     b5    5    b6    6    b7    7
        //
        //     5   b5   4   3     b3    2     b2    1     7    b7    6    b6
        //
        //
        //
        // 1    2m    3m   4      5      6m   7dim
        //
        // 1m   b7    b6    5m   4m   b3   2dim
    }
}

#[test]
pub fn test_fret() {
    let fret = 6;
    let note = Note::new("b5").unwrap();
    let note_to_fret =  note.to_fret();
    let note_from_fret = Note::from_fret_single_flats(fret);
    debug_assert!(fret == note_to_fret, "These should be even. fret: {:?} note: {} note_to_fret: {}", fret,note, note_to_fret);



    debug_assert!(note == note_from_fret, "These should be even. fret: {:?} note: {} note_to_fret: {:?}", fret,note, note_from_fret);


    let frets = [0, 4,  7];
    // let frets = [1, 3, 5, 7, 9];
    let change = Change::from_frets(frets);
    let change_to_frets = change.to_frets();
    println!("frets: {:?} change: {} change_to_frets: {:?}", frets,change, change_to_frets);


    for i in 0..12 {
        println!("frets: {:?} negative_harmony({}) == {}", frets, i, Frets{data: frets.into() }.negative_harmony(i))

    }
}