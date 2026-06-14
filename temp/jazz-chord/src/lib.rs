/*!
A package used to manipulate notes and chords.
*/
#![allow(warnings)]
// #![allow(unused)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(non_snake_case)]
// #![allow(non_camel_case_types)]
// #![allow(unused_parens)]
// #![allow(clippy::ptr_arg)]
// #![allow(clippy::needless_pass_by_value)]
// #![allow(clippy::redundant_clone)]
// #![allow(clippy::needless_range_loop)]
// #![allow(clippy::trivially_copy_pass_by_ref)]
use crate::frets::ToPitchClassSet;
pub mod frets;
use frets::{Fret, FretExt, Frets, FretsExt, ToFret, ToFrets, ToChange};
mod power_set;
mod bitmap;
use bitmap::Bitmap;


use std::cmp::Reverse;


use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::cell::OnceCell;
use std::cmp;
use std::slice;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, Index, IndexMut, Range, RangeFrom, RangeTo};
use std::convert::{AsRef, };
use std::default::Default;
use std::string::ToString;
use std::sync::{Mutex, OnceLock};
use std::vec::IntoIter;



use once_cell::sync::Lazy;

use cached::proc_macro::cached;
use memoize::lazy_static::lazy_static;
use memoize::memoize;
use midi_file::core::{Channel, NoteNumber, Velocity};
use midi_file::file::{QuartersPerMinute, Track};

#[macro_use(concat_string)]
// for faster than format!() concatting
extern crate concat_string;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
// use crate::NoteEq::PitchClass;
use ezstr::{EzStr, GraphemeSpan};
use jazz_accidentals::Accidentals;

// Imports from Combinations


//Required to import when compiling as a lib
use std::convert::TryInto;
use std::time::Instant;
//use core::ops::RangeFrom;
use core;
use itertools;





use std::env;

use std::slice::SliceIndex;
use std::num::FpCategory::Nan;
use itertools::{Combinations, Itertools};
use std::io;
use crate::NoteEq::Equivalent;
use crate::power_set::PowerSet;
// Is this trait garbage?

trait NGram {
    fn a() -> String{
        "x".to_string()
    }
}

pub trait ToBitmap<const N: usize> {
    fn to_bitmap(&self) -> Bitmap<N>;
}




pub trait ToChordQuality {
    // fn to_chord_quality(&self, params: &ChordQualityParams) -> ChordQuality;
    fn to_chord_qualities_with_options(&self, params: &QualityParams) -> Vec<Quality>;

    fn to_chord_qualities(&self) -> Vec<Quality> {
        self.to_chord_qualities_with_options(&*QualityParams::DEFAULT)
    }

    fn to_chord_quality_with_options(&self, params: &QualityParams) -> Quality {
        self.to_chord_qualities_with_options(params).get(0).expect("no chord quality for this?").clone()
    }

    fn to_chord_quality(&self) -> Quality {
        self.to_chord_qualities().get(0).expect("no chord quality for this?").clone()
    }





    // fn to_chord_quality(&self, params: &ChordQualityParams) -> ChordQuality;
}



impl ToFret for Note {
    fn to_fret(&self) -> Fret {
        // <&Note as ToFret>::to_fret(&&*self)
        // self.to_fret()
        self.compute_fret()
    }
}

impl ToFret for &Note {
    fn to_fret(&self) -> Fret {
        self.compute_fret()
    }
}

//TODO move this downward towards ChordQuality?
#[derive(Clone)]
struct ChordQualityParseResult {
    input: EzStr,
    triad: Option<ChordTypeMatch>,
    extension: Option<ExtensionMatch>,
    modification: Option<QualityModTokenMatches>,

}

#[derive(Clone, Debug, Copy)]
#[derive(Eq, Hash, PartialEq, EnumIter)]
pub enum Degree {
    Ones,
    Thirds,
    Fifths,
    Sevenths,
    Ninths,
    Elevenths,
    Thirteenths,
}

impl Display for Degree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

static DEGREE_PREFERRED_NOTES: Lazy<HashMap<Degree, Change>> = Lazy::new(||{
    let mut m = HashMap::new();
    m.insert(Degree::Ones , Change::from_note_strings(&["1"]));
    m.insert(Degree::Thirds , Change::from_note_strings(&["3", "b3"]));
    m.insert(Degree::Fifths , Change::from_note_strings(&["5"]));
    m.insert(Degree::Sevenths , Change::from_note_strings(&["7", "b7", "bb7"])); // added bb7
    m.insert(Degree::Ninths , Change::from_note_strings(&["9"]));
    m.insert(Degree::Elevenths , Change::from_note_strings(&["11"]));
    m.insert(Degree::Thirteenths, Change::from_note_strings(&["13", "6"])); // added 6
    m
});
static DEGREE_ALLOWED_NOTES: Lazy<HashMap<Degree, Change>> = Lazy::new(||{
    let mut m = HashMap::new();
    m.insert(Degree::Ones , Change::from_note_strings(&["1", "b1", "#1"]));
    m.insert(Degree::Thirds , Change::from_note_strings(&["3", "b3", "4", "2", "#4", "b2"]));
    m.insert(Degree::Fifths , Change::from_note_strings(&["5", "b5", "#5"]));
    m.insert(Degree::Sevenths , Change::from_note_strings(&["7", "b7"])); // added b7
    m.insert(Degree::Ninths , Change::from_note_strings(&["9", "b9", "#9"]));
    m.insert(Degree::Elevenths , Change::from_note_strings(&["11", "#11"]));
    m.insert(Degree::Thirteenths, Change::from_note_strings(&["13", "b13", "#13"]));
    m
});

static DEGREE_TO_NATURAL_EXTENSION: Lazy<HashMap<Degree, Note>> = Lazy::new(||{
    let mut m = HashMap::new();
    m.insert(Degree::Ones , Note::new("1").unwrap());
    m.insert(Degree::Thirds ,  Note::new("3").unwrap());
    m.insert(Degree::Fifths ,  Note::new("5").unwrap());
    m.insert(Degree::Sevenths ,  Note::new("7").unwrap());
    m.insert(Degree::Ninths ,  Note::new("9").unwrap());
    m.insert(Degree::Elevenths ,  Note::new("11").unwrap());
    m.insert(Degree::Thirteenths,  Note::new("13").unwrap());
    m
});

impl Degree {
    pub const TRIADIC_ASCENDING_NO_ONE: [Degree; 6]  = [
        Degree::Thirds,
        Degree::Fifths,
        Degree::Sevenths,
        Degree::Ninths,
        Degree::Elevenths,
        Degree::Thirteenths
    ];

    pub const TRIADIC_ASCENDING: [Degree; 7]  = [
        Degree::Ones,
        Degree::Thirds,
        Degree::Fifths,
        Degree::Sevenths,
        Degree::Ninths,
        Degree::Elevenths,
        Degree::Thirteenths
    ];

    pub const TRIADIC_DESCENDING_NO_ONE: [Degree; 6]  = [
        Degree::Thirteenths,
        Degree::Elevenths,
        Degree::Ninths,
        Degree::Sevenths,
        Degree::Fifths,
        Degree::Thirds,
    ];

    pub const TRIADIC_DESCENDING: [Degree; 7]  = [
        Degree::Thirteenths,
        Degree::Elevenths,
        Degree::Ninths,
        Degree::Sevenths,
        Degree::Fifths,
        Degree::Thirds,
        Degree::Ones,
    ];

    pub fn into_string(&self) -> &'static str {
        match self {
            Self::Ones => "Ones",
            Self::Thirds => "Thirds",
            Self::Fifths => "Fifths",
            Self::Sevenths => "Sevenths",
            Self::Ninths => "Ninths",
            Self::Elevenths => "Elevenths",
            Self::Thirteenths => "Thirteenths",
        }
    }

    pub fn preferred_notes(&self) -> &'static Change {
        (*DEGREE_PREFERRED_NOTES).get(self).unwrap()
    }

    pub fn allowed_notes(&self) -> &'static Change {
        (*DEGREE_ALLOWED_NOTES).get(self).unwrap()
    }

    pub fn poss_exts_if_degree_at_end(&self) -> &'static [&'static Extension] {
        match self {
            Degree::Ones => {
                &[&Extension::Unison]
            }
            Degree::Thirds => { &[
                &Extension::MinorSecond,
                &Extension::MajorSecond,
                &Extension::MinorThird,
                &Extension::MajorThird,
                &Extension::Fourth,
                &Extension::AugmentedFourth,
            ]},
            Degree::Fifths => { &[&Extension::Five, &Extension::Fifth] }
            Degree::Sevenths => { &[&Extension::Seven, &Extension::MajorSeven]}
            Degree::Ninths => { &[&Extension::Nine, &Extension::MajorNine] }
            Degree::Elevenths => { &[&Extension::Eleven, &Extension::MajorEleven] }
            Degree::Thirteenths => { &[&Extension::Thirteen, &Extension::MajorThirteen, &Extension::Six] }
        }
    }

    pub fn to_natural_extension(&self) -> &'static Note {
        &*(DEGREE_TO_NATURAL_EXTENSION).get(self).unwrap()
    }

    /// Does not return the Ones degree like #1 for a b9 note
    pub fn options_for_note(note: &Note) -> Vec<Self> {
        // TODO make static somehow
        Degree::TRIADIC_ASCENDING_NO_ONE.iter().copied().filter(|deg|{
            deg.allowed_notes().contains_note(note, &NoteEq::PitchClass)
        }).collect()
    }

    pub fn from_note_degree(note: &Note) -> &Self {
        let binding = note.in_first_octave();
        let note = binding.degree.data.as_str();
        match note {
            "1" => &Self::Ones,
            "2" => &Self::Ninths,
            "3" => &Self::Thirds,
            "4" => &Self::Elevenths,
            "5" => &Self::Fifths,
            "6" => &Self::Thirteenths,
            "7" => &Self::Sevenths,
            _ => &Self::Ones// This should not happen
        }
    }

    pub fn within_change(&self, change: &Change, stop_on_preferred_notes:bool) -> Change {
        let mut notes = Vec::new();
        for p in self.allowed_notes() {
            if change.contains_pitch_class_of_note(&p) {
                    notes.push(p.clone());
                if stop_on_preferred_notes && !self.preferred_notes().is_empty() {
                // if stop_on_preferred_notes && !self.preferred_notes().is_empty() {
                    for stop_note in self.preferred_notes() {
                        if p.to_pitch_class() == stop_note.to_pitch_class() {
                            return Change::from_notes(notes);
                        }
                    }
                }
            }
        }
        Change::from_notes(notes)
    }
}


#[derive(Clone, Debug)]
pub struct DegreePossibilities {
    intervals: HashMap<Degree, Change>
}

impl Display for DegreePossibilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_debug()
        )
    }
}

// impl Debug for ChordalDegreePossibilityMatrix {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.write_fmt(format_args!("ChordalDegreePossibilityMatrix{{ {:?} }}", self.intervals.iter().map(|(degree, change)| format!("{} {}", degree, change))))
//     }
// }

impl DegreePossibilities {
    pub fn new() -> Self {
        let mut intervals: HashMap<Degree, Change> = HashMap::new();
        for degree in Degree::iter() {
            intervals.insert(degree, Change::new());
        }
        Self {
            intervals
        }
    }

    pub fn to_change(&self) -> Change {
        let mut notes: Change = Change::new();
        for interval_change in self.intervals.values() {
            notes.extend(interval_change);
        }
        notes
    }

    pub fn contains_note_in_degree(&self, note: &Note, degree: &Degree, eq: &NoteEq) -> bool{
        self.intervals.get(degree).unwrap().contains_note(note, eq)
    }

    pub fn contains_notes_in_degree(&self, degree: &Degree) -> bool {
        self.intervals.get(degree).unwrap().len() > 0
    }

    pub fn is_not_empty(&self) -> bool {
        for degree in self.intervals.keys() {
            if self.contains_notes_in_degree(degree) {
                return true;
            }
        }
        return false
    }

    pub fn contains_note(&self, note: &Note, eq:&NoteEq) -> bool {
        for change in self.intervals.values(){
            if change.contains_note(note, eq) {
                return true
            }
        }
        false
    }

    pub fn from_triad(triad: &Triad) -> DegreePossibilities {
        let mut matrix = DegreePossibilities::new();
        *matrix.intervals.get_mut(&Degree::Ones).unwrap() = Change::from_note(triad.notes()[0].clone());
        *matrix.intervals.get_mut(&Degree::Thirds).unwrap() = Change::from_note(triad.notes()[1].clone());
        *matrix.intervals.get_mut(&Degree::Fifths).unwrap() = Change::from_note(triad.notes()[2].clone());
        matrix
    }

    pub fn from_extension(extension: &Extension) -> DegreePossibilities {
        let mut matrix = DegreePossibilities::new();
        let notes = extension.notes();

        // if notes.len() == 2 {
        //     let second_note_degree = ChordalDegree::from_note(notes[1]);
        //     *matrix.intervals.get_mut(&ChordalDegree::Thirds).unwrap() = Change::from_note(triad.notes()[1].clone());
        // }
        for note in notes {
            let degree = Degree::from_note_degree(&note);
            // matrix = matrix.pick_note_for_degree();
            *matrix.intervals.get_mut(degree).unwrap() = Change::from_note(note.clone());
            //println!("happened with note {}", note)
        }

        //panic!("sextension: {}\nmatrix: {}", extension, matrix);
        // panic!("{}", Extension::ALL.iter().map(|e|
        //     {format!("{} {}",e.name(), e.notes())}
        // ).collect::<Vec<_>>().join("\n"));
        // debug_assert!("")
        matrix

    }

    /// It keeps the triad notes even if triad == None
    pub fn from_triad_extension(triad_extension: &TriadExtension) -> Self {
        let triad = triad_extension.triad.as_ref();
        let extension = triad_extension.extension.as_ref();
        let triad_matrix = if let Some(triad) = triad {
            DegreePossibilities::from_triad(triad)
        } else {
            DegreePossibilities::new()
        };
        let extension_possibilities = if let Some(extension) = extension {
            //println!("{}",&extension.notes());
            DegreePossibilities::from_extension(extension)
            // ChordalDegreePossibilityMatrix::all_from_change(&extension.notes())
        } else {
            DegreePossibilities::new()
        };
        let mut possibilities: Self = extension_possibilities.clone();
        //println!("triad: {:?}\nextension: {:?}\ntriad_matrix:\n{}\nextension_matrix:\n{}", triad, extension, triad_matrix.short_debug(), extension_possibilities.short_debug());
        // overwrite the thirds
        for degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter(){
            let triad_note = triad_matrix.intervals.get(degree).unwrap();
            // let extension_note: &Note = &extension_matrix.intervals.get(degree).unwrap()[0];
            if !triad_note.is_empty() {
                *possibilities.intervals.get_mut(degree).unwrap() = triad_note.clone();
            }
        }
        // let has_dim = triad.is_some() && [Triad::HalfDiminished, Triad::Diminished].contains(triad.unwrap());
        let is_dim = if let Some(triad) = triad && triad == &Triad::Diminished { true } else { false };
        let sevenths = possibilities.intervals.get(&Degree::Sevenths).unwrap();
        let has_flat_seven = !sevenths.is_empty() && sevenths.contains_note(&Note::new("b7").unwrap(), &NoteEq::Equivalent);
        if is_dim && has_flat_seven {
            let new_seventh = Change::from_note_strings(&["bb7"]);
            *possibilities.get_mut(&Degree::Sevenths) = new_seventh;
            //panic!("Fixing a dim seventh\n{}", possibilities.short_debug());


        }
       let one = Note::new("1").unwrap();
        if extension != Some(&Extension::NoChord)
        && !possibilities.contains_note(&one, &NoteEq::Equivalent){
            possibilities.intervals.insert(Degree::Ones, Change::from_notes(vec![one]));
        }
        possibilities
    }

    /// Picks ideal notes, and then returns matching triad extensions
    pub fn poss_triad_extensions(&self) -> Vec<TriadExtension>
    {
        let mut triad_extensions: Vec<TriadExtension> = Vec::new();
        let possibilities = self.pick_ideal_notes();
        let all_poss_triads = possibilities.possible_triads();       // Option<Vec<Triad>>
        let all_poss_exts = possibilities.possible_extensions();    // Option<Vec<Extension>>



        // panic!("{:#?} {:#?}", all_poss_triads, all_poss_exts);


        for triad in all_poss_triads.clone().into_iter() {
            for ext in &all_poss_exts {
                let key = TriadExtension::from(&Some(&triad.clone()), &Some(&ext.clone()));
                if key.is_legal() {
                    triad_extensions.push(key);
                }
                // let val = DegreePossibilities::from_triad_extension(&key);
                // triad_ext_to_symbol.insert(key, val);
            }
            triad_extensions.push(TriadExtension::from(&Some(&triad.clone()), &None))

        }
        //panic!("{:?} {:?}\n{}\n{:?}",all_poss_triads, all_poss_exts,possibilities.short_debug(), triad_extensions.as_slice().iter().map(|el|format!("{}",el)).collect::<Vec<_>>());
        triad_extensions
    }

    pub fn possible_extensions_option(&self) -> Option<Vec<Extension>> {
        let possible = self.possible_extensions();
        if possible.len() > 0 {
            Some(possible)
        } else {
            None
        }
    }

    pub fn possible_extensions(&self) -> Vec<Extension> {
        let mut possible_extensions: Vec<Extension> = Vec::new();
        // debug_assert!(thirds.len() == 1 && fifths.len() == 1, "thirds: {} fifths: {}",thirds,fifths);
        for degree in self.intervals.keys() {
            for note in self.intervals.get(&degree).unwrap(){
                if degree.preferred_notes().contains_enharmonic(note) {
                //if degree.preferred_notes().contains_enharmonic(&note) {
                    for extension in degree.poss_exts_if_degree_at_end(){
                        //println!("pushed {extension}");
                        if ! possible_extensions.contains(&extension){
                            possible_extensions.push(extension.clone().clone());
                        }
                    }
                }
            }
            // let extension_matrix = ChordalDegreePossibilityMatrix::from_change(&extension.notes());
        }
        return possible_extensions;
    }

    pub fn possible_extensions_attempt(&self, triad: &Option<Triad>) -> Vec<Extension> {
        let mut score_to_extensions: HashMap<i32, Vec<Extension>> = HashMap::new();
        for i in 0..7 {
            score_to_extensions.insert(i, vec![]);
        }
        // let mut possible_extensions:Vec<Extension> = Vec::new();
        let mut max_score = 0;
        let self_change = if let Some(triad) = triad {self.remove_triad(&triad).to_change()} else {self.to_change()};

        for extension in Extension::ALL.iter() {
            // let extension_change = if let Some(triad) = triad { extension.notes().remove_triad(&triad) } else {extension.notes()};
            let extension_change = extension.notes();
            let mut this_score = 0;
            for note in &self_change {
                if extension_change.contains(note) {
                    this_score += 1;
                }
            }
            for note in extension_change {
                if !self_change.contains(note) {
                    this_score -= 1;
                }
            }
            max_score = max_score.max(this_score);

            if this_score >= 0 {
                let score_to_extensions_copy = score_to_extensions.clone();
                score_to_extensions.get_mut(&this_score).expect(format!("self_change: {} extension: {:?} score: {}\nmatrix: {:?}", self_change, extension, this_score, score_to_extensions_copy).as_str()).push(extension.clone().clone());
            }
        }

        score_to_extensions.get(&max_score).unwrap().clone()
    }

    pub fn possible_triads_option(&self) -> Option<Vec<Triad>> {
        let possible_triads = self.possible_triads();
        if possible_triads.len() > 0 {
            Some(possible_triads)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &Degree) -> &mut Change {
        self.intervals.get_mut(key).unwrap()
    }

    pub fn get(&self, key: &Degree) -> &Change{
        self.intervals.get(key).unwrap()
    }

    pub fn possible_triads(&self) -> Vec<Triad> {
        let mut possible_triads: Vec<Triad> = Vec::new();
        let mut triads: HashMap<i32, Vec<Triad>> = HashMap::new();
        for triad in Triad::ALL.iter() {
            let mut score = 0;
            for degree in &Degree::TRIADIC_ASCENDING_NO_ONE {
                if self.contains_notes_in_degree(degree) {
                    let degree_change = self.intervals.get(degree).unwrap();
                    // debug_assert!(degree_change.len() <= 1, "degree_change: {}", degree_change);

                    for degree_note in degree_change {
                        if triad.notes().contains_pitch_class(degree_note){
                            score += 1;
                        }
                        // println!("'{}'.contains_pitch_class('{}') == {}",
                        //          triad.notes(),degree_note,triad.notes().contains_pitch_class(degree_note))
                    }
                }
            }
            if !triads.contains_key(&score){
                triads.insert(score, vec![]);
            }
            triads.get_mut(&score).expect(
                format!("shit").as_str()
            ).push(triad.clone().clone());
            possible_triads.push(triad.clone().clone())
        }
        // let max_score = triads.keys().max().unwrap();
        // let possible_triads = triads.get(max_score).unwrap().clone();
        //panic!("self {}\nall_triads {:#?}\nmax_score {}\npossible_triads:{:#?}", self, triads, max_score, possible_triads);
        possible_triads

    }

    /// Ignores the 1. As in, it doesn't include it.
    fn all_from_change_no_one(change: &Change) -> Self {
        let mut result = DegreePossibilities::new();
        // let mut intervals = HashMap::<Degree, Change>::new();
        for scale_degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter() {
            let possibilities = scale_degree.within_change(change,false);
            *result.get_mut(scale_degree) = possibilities;
        }
        // result.eliminate_fixed_notes();
        result
    }

    /// Includes the 1. As in, it doesn't skip it. But it won't let you include altered 1.
    fn all_from_change(change: &Change) -> Self {
        let mut result = DegreePossibilities::new();
        let mut intervals = HashMap::<Degree, Change>::new();
        // skip the one
        for scale_degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter() {
            let possibilities = scale_degree.within_change(change,false);
            *result.get_mut(scale_degree) = possibilities;
        }
        // do the one
        let the_one = Note::new("1").unwrap();
        if change.contains_note(&the_one, &NoteEq::Equivalent){
            *result.get_mut(&Degree::Ones) = the_one.into();
        }
        // result.eliminate_fixed_notes();
        // debug_assert!(result.get(&Degree::Ones))
        // debug_assert!(result.get(&Degree::Ones))

        //Change::new().contains_notes(Change::from_notes_string(""))
        debug_assert!(!result.get(&Degree::Ones).contains_notes(
            &Change::from_notes_string("#1 b1"),
            &NoteMatch::Any(NoteEq::Equivalent)),
                      "{}", result.get(&Degree::Ones)
        );
        result
    }

    fn degrees_containing_notes(&self) -> Vec<Degree> {
        self.intervals.keys().filter(|deg| {
            self.intervals.get(deg).unwrap().len() > 0
        }).copied().collect::<Vec<Degree>>()
    }

    /// Removes pitch class
    fn remove_note_from_other_degrees(&mut self, note: &Note, degree: &Degree) {
        for other_degree in Degree::TRIADIC_ASCENDING.iter().filter(|d| *d != degree){
            let other_degree_change = self.intervals.get(&other_degree).unwrap().clone();
            for other_degree_note in &other_degree_change {
                // if other_degree_note.to_pitch_class() == note.to_pitch_class() {
                if other_degree_note.eq_note(note, &NoteEq::PitchClass) {
                    *self.intervals.get_mut(&other_degree).unwrap() = self.intervals.get(&other_degree).unwrap().remove_pitch_class_of_note(&other_degree_note);
                    // println!("'removed {} from {} resulting in {}", other_degree_note, other_degree, self.intervals.get(&other_degree).unwrap());
                    // println!("in {} 'removed {} from {} resulting in\n{}", degree, other_degree_note, other_degree, new.short_debug());
                }
            }
        }
    }

    /// Removes pitch class
    fn remove_note_from_all_degrees(&mut self, note: &Note) {
        //println!("in remove_note_from_all_degrees({note}) for\n{}", self);
        for degree in &Degree::TRIADIC_ASCENDING {
            //println!("removing {}", note);
            *self.get_mut(degree) = self.get(degree).without_note(note, &NoteEq::PitchClass);
        }
        //println!("after:\n{}", self);
        //panic!("thing: {}", Change::from_notes_string("9 b9").without_note(&Note::new("9").unwrap(), &NoteEq::PitchClass))
    }

    /// Remove other notes from this degree. Remove this note from other degrees. Ensure this ends up in the right degree
    fn pick_note_for_degree(&mut self, note: &Note, degree: &Degree) {
        // println!("pick_note_for_degree({note} {degree}) before:\n{}", self.short_debug());
        let this_degree_change: &Change = &self.get(&degree).clone();
        // Remove other notes from this degree
        for this_degree_note in this_degree_change {
            if this_degree_note.to_pitch_class() != note.to_pitch_class() {
                let this = self.get(&degree).clone().remove_pitch_class_of_note(&this_degree_note);
                *self.intervals.get_mut(&degree).unwrap() =this;
                //println!("!removed {} from {} resulting in {}", this_degree_note, degree, self.intervals.get(&degree).unwrap());
                // println!("note {} !removed {} from {} resulting in\n{}", note, this_degree_note, degree, matrix.short_debug());
            }
        }

        self.remove_note_from_other_degrees(note, degree);
        // This might be real bad
        // Ensure note in degree
        if !this_degree_change.contains_note(note, &NoteEq::PitchClass){
            let allowed_note = degree.allowed_notes().index_of_note(note, &NoteEq::PitchClass);
            if let Some(allowed_note) = allowed_note {
                self.intervals.get_mut(&degree).unwrap().push_note(&degree.allowed_notes()[allowed_note]);
            } else {
                panic!("{} - {} - {:?}", degree.allowed_notes(), note, allowed_note);
            }
        }
        // println!("pick_note_for_degree({note}) after:\n{}", self.short_debug());
    }

    fn without(&self, other: &Self) -> Self {
        let mut new: Self = self.clone();
        for (degree, other_change) in other.intervals.iter() {
            let mut new_change: Change = self.intervals.get(degree).unwrap().clone();
            for other_note in other_change {
                // if new_change.contains_enharmonic(&other_note) {
                // if new_change.contains_spelling_of_note(&other_note) {

                if new_change.contains_pitch_class_of_note(&other_note) {
                    let found_it = other_note.is_equivalent(&Note::new("#9").unwrap()) ;//&& *degree == ChordalDegree::Ninths;
                    if found_it {
                        println!{"new_change: {}\n other_change: {}", new_change, other_change}
                    }
                    new_change = new_change.remove_pitch_class_of_note(&other_note);

                    new.remove_note_from_other_degrees(&other_note, degree);
                    if found_it{
                        panic!{"new_change: {}\n other_change: {}", new_change, other_change}
                    }
                    // panic!("DOING IT to {} from {}", other_note, new_change);
                }
            }
            *new.intervals.get_mut(degree).unwrap() = new_change;
        }
        new
    }

    fn remove_triad(&self, triad: &Triad) -> Self {
        let mut matrix: Self = self.clone();
        for note in triad.notes(){
            for degree in [Degree::Thirds, Degree::Fifths]{
                // println!("Removed {} from {} degree: {}", note, degree, matrix.intervals.get(&degree).unwrap());
                *matrix.intervals.get_mut(&degree).unwrap() = matrix.intervals.get(&degree).unwrap().remove_pitch_class_of_note(&note);


            }
        }
        // println!("Resulting in\n{}", matrix.short_debug());
        matrix
    }

    fn clear_degree(&mut self, degree: &Degree) {
        *self.get_mut(&degree) = Change::new();
    }

    /// this will pick the best notes and then try to add the additions back in
    fn pick_ideal_notes(&self) -> Self{
        let mut ideal: Self = self.clone();
        for degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter() {
            let degree_possibilties = ideal.intervals.get(&degree).unwrap().clone();
            for possibility in &degree_possibilties {
                for preferred_note in degree.preferred_notes(){
                    // let is_same_pitch = &possibility.to_pitch_class() == &preferred_note.to_pitch_class();
                    let is_preferred_for_this_degree = degree.preferred_notes().contains_pitch_class(possibility);
                    // if (is_preferred_for_this_degree && !ideal.contains_notes_in_degree(degree))
                    if is_preferred_for_this_degree
                        || degree_possibilties.len() == 1 {
                        //println!("{possibility} ~is_preferred: {is_preferred_for_this_degree} is_same_pitch as {preferred_note}: {is_same_pitch} pick {} for {}", &degree_possibilties[0], degree);
                        ideal.pick_note_for_degree(&degree_possibilties[0], &degree);
                    }
                    else {
                        let mut preferred_other_degree_note: Option<&Note> = None;
                        if !is_preferred_for_this_degree {
                            for degree_to_find in Degree::TRIADIC_ASCENDING_NO_ONE.iter() {
                                if degree_to_find == degree { continue }
                                if degree_to_find.preferred_notes().contains_pitch_class(possibility)
                                && !ideal.contains_notes_in_degree(degree) {
                                    preferred_other_degree_note = Some(&degree_to_find.preferred_notes()[degree_to_find.preferred_notes()
                                        .index_of_note(possibility, &NoteEq::PitchClass).unwrap()]);
                                   ideal.pick_note_for_degree(possibility, &degree_to_find);
                                    //println!("{possibility} ^is_preferred: {is_preferred_for_this_degree} is_same_pitch as {preferred_note}: {is_same_pitch} pick {} for {}", &possibility, &degree_to_find);
                                    break;
                                }
                            }
                        }
                        // println!("{possibility} =is_preferred: {is_preferred_for_this_degree} is_same_pitch as {preferred_note}: {is_same_pitch} pick {} for {}", &degree_possibilties[0], degree);
                    }

                }
            }


            // if possibilities.len() == 1 {
            //     *self = self.pick_note_for_degree(&possibilities[0], &degree);
            // // Todo remove note as well?
            // panic!("{:#?}", self.intervals.get(&ChordalDegree::Thirds).unwrap());
            // }
        }

        // TODO this is a bit smelly because we could have just left the options in before
        // Now that we found the ideal notes we have to add any missing ones back in.
        // We might have nuked them from the step before
        for note in &self.to_change(){
            if !ideal.contains_note(note, &NoteEq::PitchClass){
                let all = DegreePossibilities::all_from_change_no_one(&note.to_change());
                let degree_options: Vec<_> = all.degrees_containing_notes().into_iter()
                    .filter(|degree|![Degree::Thirds, Degree::Fifths].contains(&degree)
                ).collect();
                match degree_options.len() {
                    1 => ideal.get_mut(&degree_options[0]).push_change(all.get(&degree_options[0])),
                    0 => panic!(),
                    _ => {
                        let mut empty_degree = None;
                        for degree_option in &degree_options {
                            if ideal.get(degree_option).is_empty(){
                                empty_degree = Some(degree_option);
                                break;
                            }
                        }
                        if let Some(empty_degree) = empty_degree {
                            ideal.get_mut(empty_degree).push_change(all.get(&degree_options[0]));
                        } else {
                            let flat_seven = Note::new("b7").unwrap();
                            if note.eq_note(&flat_seven, &NoteEq::Enharmonic){
                                ideal.get_mut(&Degree::Sevenths).push_note(&flat_seven)
                            } else {
                                panic!("self: {} note: {} degree_options: {:?}\nideal: {}\nself: {}",
                                       self.to_change().in_first_octave_sorted(), note, degree_options, ideal, self);
                            }
                        }
                    }
                }


                // debug_assert!(degree_options.len() == 1, "self: {} note: {} degree_options: {:?}\nideal: {}\nself: {}",
                //               self.to_change().in_first_octave_sorted(), note,degree_options,ideal, self);



                //panic!("missing note:\n{}\nself: {}\nideal:\n{}\ndegree_options: {:?}", all, self.to_change(), ideal, degree_options);
            }
        }

        // We took out the one before so lets add it back
        // nvm that didn't work
        *ideal.get_mut(&Degree::Ones) = self.get(&Degree::Ones).clone();

        debug_assert!(self.to_change().is_same_pitchclass_of_change(&ideal.to_change()),
                      "orig:\n{}\nideal:\n{}", self, ideal
        );
        ideal
    }


    fn multiline_debug(&self) -> String{
        let mut out = "".to_string();
        for scale_degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter() {
            let possibilities = &self.intervals.get(&scale_degree).unwrap();
            out.push_str(format!("\t{}\t{}\n", scale_degree.into_string(), possibilities).as_str());
        }
        out.pop();
        out
    }

    fn short_debug(&self) -> String {
        format!("{{{}}}",
        Degree::TRIADIC_ASCENDING.iter()
            .filter(|deg| self.contains_notes_in_degree(deg))
            .map(|deg|format!("{}: {}", deg, self.get(&deg)))
            .collect::<Vec<_>>().join(", ")
        )
    }
}


static TRIAD_TO_NOTES: Lazy<HashMap<Triad, Change>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for triad in Triad::ALL {
        let change = match triad {
            Triad::Major => { Change::from_notes_string("1 3 5") },
            Triad::Minor => { Change::from_notes_string("1 ♭3 5") },
            Triad::Diminished => { Change::from_notes_string("1 ♭3 ♭5") },
            Triad::HalfDiminished => { Change::from_notes_string("1 ♭3 ♭5") },
            Triad::Augmented => { Change::from_notes_string("1 3 ♯5") },
            Triad::SusTwo => { Change::from_notes_string("1 2 5") },
            Triad::SusFlatTwo => { Change::from_notes_string("1 ♭2 5") },
            Triad::SusFour => { Change::from_notes_string("1 4 5") },
            Triad::SusSharpFour => { Change::from_notes_string("1 ♯4 5") },
        };
        m.insert(triad.clone().clone(), change );
    }
    m
});

#[derive(Debug, Eq, PartialEq, Hash)]
#[derive(Clone)]
pub enum Triad {
    Major,
    Minor,
    Diminished,
    HalfDiminished,
    Augmented,
    SusTwo,
    SusFlatTwo,
    SusFour,
    SusSharpFour,
}

impl Triad {
    fn notes(&self) -> &'static Change {
        (*TRIAD_TO_NOTES).get(self).unwrap()
    }

    const fn triggers(&self) -> &'static [&'static str]{
        match self {
            Self::Major => { &["maj", "ma", "Ma", "MA"] },
            Self::Minor => { &["minor", "min", "mi", "m", "-"] },
            Self::Diminished => { &["dim", "di", "o", "0", "O", "°","º"] },
            Self::HalfDiminished => { &["ø", "⌀", "halfdim", "hdim"] },
            Self::Augmented => { &["aug", "+"] },
            Self::SusTwo => { &["sus2"] },
            Self::SusFlatTwo => { &["sus♭2", "susb2"] },
            Self::SusFour => { &["sus4", "sus"] },
            Self::SusSharpFour => { &["sus♯4", "sus#4"] },
        }
    }

    const fn name(&self) -> &'static str {
        match self {
            Self::Major => { &"Major" },
            Self::Minor => { &"Minor" },
            Self::Diminished => { &"Diminished" },
            Self::HalfDiminished => { &"Half Diminished" },
            Self::Augmented => { &"Augmented" },
            Self::SusTwo => { &"Suspended 2nd" },
            Self::SusFlatTwo => { &"Suspended ♭2nd" },
            Self::SusFour => { &"Suspended 4th" },
            Self::SusSharpFour => { &"Suspended Sharp 4th" },
        }
    }

    pub fn is_sus(&self) -> bool {
        Self::SUS_TRIADS.contains(&self)
    }

    pub fn to_chord_string(&self, options: &QualityParams) -> String {
        match self {
            Self::Major => { options.str_major_triad.to_string() },
            Self::Minor => { options.str_minor_triad.to_string() },
            Self::Diminished => { options.str_dim_triad.to_string() },
            Self::HalfDiminished => { options.str_half_diminished.to_string() },
            Self::Augmented => { options.str_aug_triad.to_string() },
            Self::SusTwo => { format!("{}2",options.str_sus_triad)},
            Self::SusFlatTwo => {
                format!("{}{}2",options.str_sus_triad, Accidentals::from_dist(-1, options.use_unicode, options.use_double_accidentals))
            },
            Self::SusFour => { format!("{}4",options.str_sus_triad) },
            Self::SusSharpFour => {
                format!("{}{}4",options.str_sus_triad, Accidentals::from_dist(1, options.use_unicode, options.use_double_accidentals))
            },
        }
    }

    // Use Lazy for collections since they contain interior mutable data
    pub const ALL: &'static [&'static Triad] = &[
        &Self::Major,
        &Self::Minor,
        &Self::Diminished,
        &Self::HalfDiminished,
        &Self::Augmented,
        &Self::SusTwo,
        &Self::SusFlatTwo,
        &Self::SusFour,
        &Self::SusSharpFour,
    ];

    pub const SUS_TRIADS: &'static [&'static Triad] = &[
        &Self::SusFour,
        &Self::SusTwo,
        &Self::SusFlatTwo,
        &Self::SusSharpFour,
    ];
}

pub(crate) enum BracketState {
    Unknown,
    Add,
    Remove,
}
impl PartialEq for &BracketState {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ChordAlterationToken {
    BracketOpen,
    BracketClose,
    Add,
    Remove,
    Note,
    Space,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ChordModificationMatch {
    pub span: GraphemeSpan,
    pub chord_mod: ChordAlterationToken
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QualityModTokenMatches {
    token_matches: Vec<ChordModificationMatch>,
}

impl QualityModTokenMatches {
    fn to_string(&self) -> String {
        self.token_matches.as_slice().iter().map(|tok|tok.span.text.as_str()).collect::<Vec<_>>().join("")
    }

    fn to_chord_modification_token(&self) -> Vec<&ChordAlterationToken> {
        self.token_matches.as_slice().iter().map(|token_match|&token_match.chord_mod ).collect()
    }

    /// will just ignore accidentals/unicode preferences so you have to handle that yourself.
    fn apply_to_triad_extension(&self, triad_extension: &TriadExtension) -> Change {
        //let mut is_doing_exclusions = false;
        let mut notes: Change = triad_extension.notes().clone();
        let mut state = BracketState::Unknown;
        for token in self.clone().into_iter() {
            match token.chord_mod {
                ChordAlterationToken::Note =>
                    match state {
                        BracketState::Unknown => {
                            if let Some(mod_note) = Note::new( & token.span) {
                                if let Some(target_n) = notes.first_note_with_degree( & mod_note.degree){
                                    //panic!("Fuck {} {}",self, target_n, );
                                    notes[target_n] = mod_note;
                                } else {
                                    notes.push_note(&mod_note);
                                }
                            } else {
                                panic ! ("Can't parse note: {}", token.span);
                            }
                        },
                        BracketState::Remove => { //doing exclusions
                            if let Some(mod_note) = Note::new( & token.span) {
                                if let Some(target_n) = notes.first_note_with_degree( & mod_note.degree){
                                    notes.remove(target_n);
                                }
                            }
                            //panic!("Hey {} {notes}", self.input);
                        },
                        BracketState::Add => {
                            if let Some(mod_note) = Note::new( & token.span) {
                                notes.push_note(&mod_note);
                            }
                        }
                    },
                ChordAlterationToken::Remove =>
                    state = BracketState::Remove,
                ChordAlterationToken::Add =>
                    state = BracketState::Add,
                _ => (),
            }
        }
        notes
    }

    pub(crate) fn is_equivalent(&self, other: &QualityModTokenMatches) -> bool {
        let triad_extension = TriadExtension::from(&Some(&Triad::Major),&None);
        let this = self.apply_to_triad_extension(&triad_extension);
        let other = self.apply_to_triad_extension(&triad_extension);
        this == other
    }


}

impl IntoIterator for QualityModTokenMatches {
    type Item = ChordModificationMatch;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.token_matches.into_iter()
    }
}


pub struct QualityModParseResult {
    tokens: QualityModTokenMatches,
}


#[derive(Debug,Clone,PartialEq, Eq, Hash)]
struct ChordTypeMatch {
    pub triad: Triad,
    pub span: GraphemeSpan,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct QualityMods {
    pub inclusions: Change,
    pub exclusions: Change,
    pub alterations: Change,
    pub additions: Change,
}

impl Display for QualityMods {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "inc: {} exc: {} alt: {} add: {}",
               self.inclusions,
               self.exclusions,
               self.alterations,
               self.additions,)
    }
}

impl QualityMods {
    fn to_change(&self) -> Change {
        Change::from_changes(&[&self.inclusions, &self.alterations, &self.additions])
    }

    pub fn eq_mods(&self, other: &QualityMods, change_eq: &ChangeEq) -> bool {
        change_eq.eq(&self.inclusions, &other.inclusions) &&
        change_eq.eq(&self.exclusions, &other.exclusions) &&
        change_eq.eq(&self.additions, &other.additions) &&
        change_eq.eq(&self.alterations, &other.alterations)
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Extension {
    NoChord,
    MajorSeventh,
    Five,
    SixAddNine,
    Six,
    MajorSeven,
    Seven,
    MajorNine,
    Nine,
    MajorEleven,
    Eleven,
    MajorThirteen,
    Thirteen,
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    Fourth,
    AugmentedFourth,
    MinorFifth,
    Fifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}


static EXTENSION_TO_NOTES: Lazy<HashMap<Extension, Change>> = Lazy::new(||{
    let mut m = HashMap::new();
    for extension in Extension::ALL {
        let notes = match extension {
            Extension::NoChord => Change::new(),
            Extension::MajorSeventh => Change::from_note_strings(&["1", "7"]),
            Extension::Five => Change::from_note_strings(&["1", "5"]),
            Extension::SixAddNine => Change::from_note_strings(&["1", "3", "5", "6", "9"]),
            Extension::Six => Change::from_note_strings(&["1", "3", "5", "6"]),
            Extension::MajorSeven => Change::from_note_strings(&["1", "3", "5", "7"]),
            Extension::Seven => Change::from_note_strings(&["1", "3", "5", "b7"]),
            Extension::MajorNine => Change::from_note_strings(&["1", "3", "5", "7", "9"]),
            Extension::Nine => Change::from_note_strings(&["1", "3", "5", "b7", "9"]),
            Extension::MajorEleven => Change::from_note_strings(&["1", "3", "5", "7", "9", "11"]),
            Extension::Eleven => Change::from_note_strings(&["1", "3", "5", "b7", "9", "11"]),
            Extension::MajorThirteen => Change::from_note_strings(&["1", "3", "5", "7", "9", "11", "13"]),
            Extension::Thirteen => Change::from_note_strings(&["1", "3", "5", "b7", "9", "11", "13"]),
            Extension::Unison => Change::from_note_strings(&["1"]),
            Extension::MinorSecond => Change::from_note_strings(&["1", "b2"]),
            Extension::MajorSecond => Change::from_note_strings(&["1", "2"]),
            Extension::MinorThird => Change::from_note_strings(&["1", "b3"]),
            Extension::MajorThird => Change::from_note_strings(&["1", "3"]),
            Extension::Fourth => Change::from_note_strings(&["1", "4"]),
            Extension::AugmentedFourth => Change::from_note_strings(&["1", "#4"]),
            Extension::MinorFifth => Change::from_note_strings(&["1", "b5"]),
            Extension::Fifth => Change::from_note_strings(&["1", "5"]),
            Extension::MinorSixth => Change::from_note_strings(&["1", "b6"]),
            Extension::MajorSixth => Change::from_note_strings(&["1", "6"]),
            Extension::MinorSeventh => Change::from_note_strings(&["1", "b7"]),
        };
        m.insert(extension.clone().clone(), notes);
    }
    m
});

static EXTENSION_TO_POSSIBILITIES: Lazy<HashMap<Extension, DegreePossibilities>> = Lazy::new(||{
    let mut m = HashMap::new();
    for extension in Extension::ALL {
        m.insert(extension.clone().clone(), DegreePossibilities::all_from_change_no_one(extension.notes()));
    }
    m
});

impl Extension {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::NoChord => "No Chord",
            Self::MajorSeventh => "Major Seventh",
            Self::Five => "Five",
            Self::SixAddNine => "Six Add Nine",
            Self::Six => "Major Six",
            Self::MajorSeven => "Major Seven",
            Self::Seven => "Dominant Seven",
            Self::MajorNine => "Major Nine",
            Self::Nine => "Nine",
            Self::MajorEleven => "Major Eleven",
            Self::Eleven => "Eleven",
            Self::MajorThirteen => "Major Thirteen",
            Self::Thirteen => "Thirteen",
            Self::Unison => "Unison",
            Self::MinorSecond => "Minor Second",
            Self::MajorSecond => "Major Second",
            Self::MinorThird => "Minor Third",
            Self::MajorThird => "Major Third",
            Self::Fourth => "Fourth",
            Self::AugmentedFourth => "Augmented Fourth",
            Self::MinorFifth => "Minor Fifth",
            Self::Fifth => "Fifth",
            Self::MinorSixth => "Minor Sixth",
            Self::MajorSixth => "Major Sixth",
            Self::MinorSeventh => "Minor Seventh",
        }
    }

    pub const fn triggers(&self) -> &[&'static str] {
        //TODO add ma7 triangle
        match self {
            Self::NoChord => &["N.C"],
            Self::MajorSeventh => &["Ma7th", "ma7th", "Maj7th", "maj7th", "7th"],
            Self::Five => &["5th", "5"],
            Self::SixAddNine => &["69", "6/9"],
            Self::Six => &["6"],
            Self::MajorSeven => &["ma7", "Ma7", "MA7", "triangle7"],
            Self::Seven => &["7", "dom", "dominant"],
            Self::MajorNine => &["ma9", "Ma9", "MA9", "triangle9"],
            Self::Nine => &["9"],
            Self::MajorEleven => &["ma11", "Ma11", "MA11", "triangle11"],
            Self::Eleven => &["11"],
            Self::MajorThirteen => &["ma13", "Ma13", "MA13", "triangle13"],
            Self::Thirteen => &["13"],
            Self::Unison => &["1", "unison"],
            Self::MinorSecond => &["min2nd", "mi2nd", "m2nd"],
            Self::MajorSecond => &["Ma2nd", "ma2nd", "Maj2nd", "maj2nd", "2nd"],
            Self::MinorThird => &["min3rd", "mi3rd", "m3rd"],
            Self::MajorThird => &["Ma3rd", "ma3rd", "Maj3rd", "maj3rd", "3rd"],
            Self::Fourth => &["4th"],
            Self::AugmentedFourth => &["aug4th"],
            Self::MinorFifth => &["min5th", "mi5th", "m5th", "tritone"],
            Self::Fifth => &["5th", "5"],
            Self::MinorSixth => &["min6th", "mi6th", "m6th"],
            Self::MajorSixth => &["Ma6th", "ma6th", "Maj6th", "maj6th", "6th"],
            Self::MinorSeventh => &["min7th", "mi7th", "m7th"],
        }
    }

    pub fn hides_ma_triad_str(&self, hide_major_six_triad:bool) -> bool {
        match self {
            Self::MajorThirteen | Self::Thirteen | Self::Eleven | Self::MajorEleven |
            Self::Nine | Self::MajorNine | Self::Seven | Self::MajorSeven => true,
            Self::MajorSixth => match hide_major_six_triad {
                true => true,
                false => false,
            }
            _ => false
        }
        //
        // match hide_major_six_triad {
        //     true => {
        //         match self {
        //             Self::MajorSixth => true,
        //             x if x.is_dominant() => true,
        //             _ => false
        //         }
        //     }
        //     false => {
        //         self.is_dominant()
        //     }
        // }
    }

    pub fn is_dominant(&self) -> bool {
        match self {
            Self::Seven | Self::Nine | Self::Eleven | Self::Thirteen => true,
            _ => false,
        }
    }

    pub fn notes(&self) -> &'static Change {
        (*EXTENSION_TO_NOTES).get(self).unwrap()
    }

    pub fn possibilities(&self) -> &'static DegreePossibilities {
        (*EXTENSION_TO_POSSIBILITIES).get(self).unwrap()
    }

    pub fn extension_number(&self) -> Option<String> {
        self.extension_number_with_options(&*QualityParams::DEFAULT)
    }

    pub fn extension_number_with_options(&self, options: &QualityParams) -> Option<String> {
        match self {
            extension if [Self::Thirteen, Self::MajorThirteen].contains(extension) => Some("13".to_string()),
            extension if [Self::Eleven, Self::MajorEleven].contains(extension) => Some("11".to_string()),
            extension if [Self::Nine, Self::MajorNine].contains(extension) => Some("9".to_string()),
            extension if [Self::Seven, Self::MajorSeven].contains(extension) => Some("7".to_string()),
            extension if [Self::Fifth, Self::Five].contains(extension) => Some("5".to_string()),
            Self::Six => Some("6".to_string()),
            Self::SixAddNine => Some(options.str_six_nines.to_string()),
            _ => Some(format!("{}oopsiedaisy", self).to_string()),

        }
    }

    pub fn contains_major_token(&self) -> bool {
        match self {
            extension if Self::ADD_MAJ_TO_EXTENSION.contains(&self) => true,
            _ => { false }
        }
    }

    pub fn contains_minor_token(&self) -> bool {
        match self {
            extension if Self::CONTAINING_MINOR_TOKEN.contains(&self) => true,
            _ => { false }
        }
    }

    pub fn to_chord_string(&self, options: &QualityParams) -> String {
        let mut chord_string = String::new();
        if self.contains_major_token() {
            chord_string.push_str(options.str_major_triad);
        } else if self.contains_minor_token() {
            chord_string.push_str(options.str_minor_triad);
        }
        if let Some(extension_number) = self.extension_number() {
            chord_string.push_str(extension_number.as_str());
        }
        chord_string
    }

    const ALL: &'static [&'static Self] =
        &[
            &Self::NoChord,
            &Self::MajorSeventh,
            &Self::Five,
            &Self::SixAddNine,
            &Self::Six,
            &Self::MajorSeven,
            &Self::Seven,
            &Self::MajorNine,
            &Self::Nine,
            &Self::MajorEleven,
            &Self::Eleven,
            &Self::MajorThirteen,
            &Self::Thirteen,
            &Self::Unison,
            &Self::MinorSecond,
            &Self::MajorSecond,
            &Self::MinorThird,
            &Self::MajorThird,
            &Self::Fourth,
            &Self::AugmentedFourth,
            &Self::MinorFifth,
            &Self::Fifth,
            &Self::MinorSixth,
            &Self::MajorSixth,
            &Self::MinorSeventh,
        ];

    const ADD_MAJ_TO_EXTENSION: &'static [&'static Self] = &[
        &Self::MajorSeventh,
        //&Self::MajorSix,
        &Self::MajorSeven,
        &Self::MajorNine,
        &Self::MajorEleven,
        &Self::MajorThirteen,
        &Self::MajorSecond,
        &Self::MajorThird,
        &Self::MajorSixth,
    ];

    const CONTAINING_MINOR_TOKEN: &'static [&'static Self] = &[
        &Self::MinorSecond,
        &Self::MinorThird,
        &Self::MinorFifth,
        &Self::MinorSixth,
        &Self::MinorSeventh,
    ];

    const INTERVALS: &'static [&'static Self] = &[
        &Self::MinorSecond,
        &Self::MajorSecond,
        &Self::MinorThird,
        &Self::MajorThird,
        &Self::Fourth,
        &Self::AugmentedFourth,
        &Self::MinorFifth,
        &Self::Fifth, // Include Five?
        &Self::MinorSixth,
        &Self::MajorSixth,
        &Self::MinorSeventh,
        &Self::MajorSeventh,
    ];
}

static TRIAD_EXTENSIONS: Lazy<Vec<TriadExtension>> = Lazy::new(|| {
    let mut v: Vec<TriadExtension> = Vec::new();
    for triad in Triad::ALL {
        for extension in Extension::ALL {
            let triad_extension = TriadExtension::from(&Some(&triad.clone().clone()), &Some(&extension.clone().clone()));
            v.push(triad_extension);
        }
    }
    for triad in Triad::ALL {
        let triad_extension = TriadExtension::from(&Some(&triad.clone().clone()), &None);
        v.push(triad_extension);
    }
    for extension in Extension::ALL {
        let triad_extension = TriadExtension::from(&None, &Some(&extension.clone().clone()));
        v.push(triad_extension);
    }
    v
});

static TRIAD_EXTENSIONS_LEGAL: Lazy<Vec<TriadExtension>> = Lazy::new(|| {
    (*TRIAD_EXTENSIONS).clone().into_iter().filter(|ext|{
        ext.is_legal()
    }).collect()
});

static TRIAD_EXTENSIONS_POSSIBILITIES: Lazy<HashMap<TriadExtension, DegreePossibilities>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for triad_extension in &*TRIAD_EXTENSIONS {
        let possibilities = DegreePossibilities::from_triad_extension(&triad_extension);
        m.insert(triad_extension.clone(), possibilities);
    }
    m
});

static TRIAD_EXTENSIONS_NOTES: Lazy<HashMap<TriadExtension, Change>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for triad_extension in &*TRIAD_EXTENSIONS {
        // println!("{} {} {} {}", if triad_extension.is_legal() {"*"} else {"!"},triad_extension.clone(), TRIAD_EXTENSIONS_POSSIBILITIES.get(&triad_extension).unwrap().to_change().sorted_by_dist(), TRIAD_EXTENSIONS_POSSIBILITIES.get(&triad_extension).unwrap());
        m.insert(triad_extension.clone(), TRIAD_EXTENSIONS_POSSIBILITIES.get(&triad_extension).unwrap().to_change().sorted_by_dist());
    }
    m
});

impl Display for TriadExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let triad = if let Some(triad) = &self.triad{
            format!("{:?}",triad)
        } else {
            String::new()
        };
        let ext = if let Some(ext) = &self.extension{
            format!("{:?}",ext)
        } else {
            String::new()
        };
        write!(f, "{}_{}", triad, ext)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct TriadExtension {
    triad: Option<Triad>,
    extension: Option<Extension>,
}

impl TriadExtension {
    pub fn from(triad: &Option<&Triad>, extension: &Option<&Extension>) -> Self {
        Self { triad: triad.clone().cloned(), extension: extension.clone().cloned() }
    }

    pub fn possibilities(&self) -> &'static DegreePossibilities {
        (*TRIAD_EXTENSIONS_POSSIBILITIES).get(self).unwrap()
    }

    pub fn notes(&self) -> &'static Change {
        (*TRIAD_EXTENSIONS_NOTES).get(self).unwrap()
    }

    pub fn all() -> &'static [Self] {
        &*TRIAD_EXTENSIONS
    }

    pub fn all_legal() -> &'static [Self] { &*TRIAD_EXTENSIONS_LEGAL }

    pub fn with_chord_mods(&self, chord_mods: &QualityMods) -> DegreePossibilities {
        let mut poss: DegreePossibilities = self.possibilities().clone();
        for exclusion in &chord_mods.exclusions {
            *(poss.get_mut(Degree::from_note_degree(exclusion))) = Change::new();

        }
        for addition in &chord_mods.additions {
            // Hopefully this is in the right degree
            poss.get_mut(Degree::from_note_degree(addition)).push_note(addition);
        }
        for alteration in &chord_mods.alterations {
            *(poss.get_mut(Degree::from_note_degree(alteration))) = Change::from_note(alteration.clone());
        }
        // inclusions is already included
        poss
    }

    pub fn is_legal(&self) -> bool {
        if let Some(ext) = &self.extension && ext.notes().len() <= 2 {
            if self.triad.is_none() {
                // disallowing intervals for now
                if *ext == Extension::Five {
                    return true;
                } else if *ext == Extension::NoChord {
                    return true;
                }
            }
            return false;
        }
        if let (Some(triad), Some(ext)) = (&self.triad, &self.extension) {
            if *triad == Triad::Diminished {
                if [Extension::Thirteen, Extension::SixAddNine, Extension::Six].contains(&ext){
                    return false;
                }
            } else if *triad == Triad::HalfDiminished {
                if !ext.notes().contains_note(&Note::new("b7").unwrap(), &NoteEq::Equivalent){
                    return false;
                }
            } else if *triad == Triad::SusTwo {
                if ext.notes().contains_note(&Note::new("2").unwrap(), &NoteEq::PitchClass) {
                    return false;
                }
            } else if *triad == Triad::SusFour {
                if ext.notes().contains_note(&Note::new("4").unwrap(), &NoteEq::PitchClass) {
                    return false;
                }
            }

        } else if let Some(extension) = &self.extension {
            // no triad
            return true;
            // return false;

        }
        //panic!("wewerwer {}", TriadExtension { triad: Some(Triad::Major), extension: Some(Extension::Five) }.is_legal());
        //println!("!!!{:?}",Self::all_legal()[0]);
        true
    }

    pub(crate) fn chord_mods_from_degree_possibilities(
        // &self, target: &DegreePossibilities, prefer_flat_7_over_sharp_thirteen: bool)
        &self, target: &DegreePossibilities, options: &QualityParams)
        -> QualityMods {
        let mut inclusions = Change::new();
        let mut exclusions = Change::new();
        let mut alterations = Change::new();
        let mut additions = Change::new();
        let mut remaining_poss: DegreePossibilities = target.clone();
        let mut included_or_altered_degrees: Vec<Degree> = vec![];
        //println!("remaining_poss:\n{}", remaining_poss);
        let show_debug = *self == TriadExtension::from(&Some(&Triad::SusTwo), &None);
        let show_debug = false;

        // inclusions only
        for degree in &self.possibilities().degrees_containing_notes(){
            let ext_degree_note: &Note = &self.possibilities().get(&degree)[0];
            debug_assert!(self.possibilities().get(&degree).len() == 1);
            // let it = remaining_poss.get(degree).contains_note(&ext_degree_note, &NoteEq::PitchClass);
            // if it {
            // if remaining_poss.get(degree).contains_note(&ext_degree_note, &NoteEq::PitchClass){
            if remaining_poss.contains_note(&ext_degree_note, &NoteEq::PitchClass){
                inclusions.push_note(&ext_degree_note);

                remaining_poss.remove_note_from_all_degrees(&ext_degree_note);
                if show_debug {
                    println!("added {ext_degree_note} to inclusions 1removed {ext_degree_note} at {degree} from remaining_poss: {} target: {}", &remaining_poss, target);
                }
                included_or_altered_degrees.push(degree.clone());
            } else {
                // if !target.contains_note(ext_degree_note, &NoteEq::PitchClass){
                if !target.contains_note_in_degree(ext_degree_note, degree, &NoteEq::PitchClass)
                    && !included_or_altered_degrees.as_slice().contains(&degree){
                    //debug_assert!(*ext_degree_note == Note::new("1").unwrap(), "triad_extension {} note{}",self,ext_degree_note);
                    if show_debug {
                        println!("because {} 1not removed {ext_degree_note} at {degree} from remaining_poss: {}",remaining_poss.contains_note(&ext_degree_note, &NoteEq::PitchClass), &remaining_poss, );
                    }
                    remaining_poss.remove_note_from_all_degrees(&ext_degree_note);
                }
            }
        }

        // alterations
        // for degree in &Degree::TRIADIC_DESCENDING_NO_ONE {
        for degree in &Degree::TRIADIC_ASCENDING_NO_ONE {
            match remaining_poss.get(degree).len() {
                0 => {
                    continue
                }
                1 => {
                    if self.possibilities().degrees_containing_notes().contains(degree) {
                        debug_assert!(remaining_poss.get(degree).len() <= 1);
                        let mut alteration_note:Note = remaining_poss.get(degree)[0].clone();


                        if !included_or_altered_degrees.as_slice().contains(degree){
                            alteration_note.change_degree_to(degree.to_natural_extension(), options.use_unicode, options.use_double_accidentals);
                            alterations.push_note(&alteration_note);

                            //panic!("{} {}", degree, alteration_note)
                        } else {
                            let non_triad_degrees = Degree::options_for_note(&alteration_note).into_iter().filter(|deg|{
                                ![Degree::Thirds, Degree::Fifths].contains(deg)
                            }).collect::<Vec<_>>();
                            let alteration_degree: &Degree = if non_triad_degrees.len() > 0 { &non_triad_degrees[0] } else { &Degree::options_for_note(&alteration_note)[0] };
                            alteration_note.change_degree_to(alteration_degree.to_natural_extension(), options.use_unicode, options.use_double_accidentals);
                            alteration_note.make_natural_explicit(options.use_unicode);
                            // panic!("{}",Degree::options_for_note(Note::new()))
                            //debug_assert!(non_triad_degrees.len() >= 1, "{} {} on {} -> {:?}", self, &alteration_note, degree, Degree::options_for_note(&alteration_note));

                            // opefully this works
                            //panic!("{} {:?}",alteration_note, possible_degrees);
                            additions.push_note(&alteration_note);
                        }

                        remaining_poss.remove_note_from_all_degrees(&alteration_note);
                        included_or_altered_degrees.push(degree.clone());
                        if show_debug {
                            println!("2removed {alteration_note} from remaining_poss {}", &remaining_poss);
                        }
                    }
                }
                _ => {
                    // They are not in triad_extension so just add them.
                    for note in &remaining_poss.get(degree).clone() {
                        additions.push_note(note);
                        remaining_poss.remove_note_from_all_degrees(note);
                    }

                    //panic!("{} {} inc: {}", self, remaining_poss.get(degree), inclusions)
                }
            }
            if remaining_poss.get(degree).len() == 1 {

            }
        }
        // exclusions
        for degree in &self.possibilities().degrees_containing_notes(){
            if !included_or_altered_degrees.as_slice().contains(&degree){
                let natural_extension = degree.to_natural_extension();
                // TODO Maybe this is smelly?
                if natural_extension.text() != "1" {
                    exclusions.push_note(natural_extension);
                }

            }
            // if poss_clone.get(degree).len() == 0 {
            //     exclusions.push_note(degree.to_natural_extension());
            // }
        }
        for degree in &Degree::TRIADIC_ASCENDING_NO_ONE {
            for note in remaining_poss.get(degree){
                if !additions.contains_note(note, &NoteEq::Equivalent) {
                    additions.push_note(note);
                    if show_debug {
                        println!("Pushed {} to additions: {}", note, additions);


                    }
                }
            }
        }
        // alterations pt ii
        for exclusion in exclusions.notes.clone().as_slice()
            .into_iter().filter(|note| ![Degree::Ones, Degree::Thirds].contains(Degree::from_note_degree(note))){
            for allowed_note in Degree::from_note_degree(&exclusion).allowed_notes(){
                if let Some(addition_matches) = additions.indices_of_note(allowed_note, &NoteEq::PitchClass){
                    debug_assert!(addition_matches.len() <= 1);
                    let addition: Note = additions[addition_matches[0]].clone();
                    let alteration = {
                        let mut alteration = addition.clone();
                        alteration.change_degree_to(exclusion, options.use_unicode, options.use_double_accidentals);
                        alteration
                    };
                    // println!("\tbefore inc: {} exc: {} alt: {} add: {}", inclusions, exclusions, alterations, additions);
                    alterations.push_note(&alteration);
                    additions.remove_note(&addition, &NoteMatch::First(NoteEq::Equivalent));
                    // println!("exc before removing {}: {}", exclusion,exclusions, );
                    exclusions.remove_note(&exclusion, &NoteMatch::First(NoteEq::Equivalent));
                    // println!("exc after: {}", exclusions);
                    included_or_altered_degrees.push(*Degree::from_note_degree(&exclusion));
                    remaining_poss.remove_note_from_all_degrees(&alteration);

                    if show_debug {
                        println!("for {}", self);
                        println!("\tincluded_or_altered_degrees {:?}", included_or_altered_degrees);
                        println!("after \tinc: {} exc: {} alt: {} add: {}", inclusions, exclusions, alterations, additions);
                        println!("\tremaining poss: {}", remaining_poss);
                        println!("found addition: {} for exclusion: {}", addition, exclusion);
                    }
                    break;
                }
            }
        }

        // #13 -> b7
        if options.prefer_flat_7_over_sharp_thirteen{
            let sharp_thirteen = Note::new("#13").unwrap();
            if additions.contains_note(&sharp_thirteen, &NoteEq::Equivalent) {
                let sharp_thirteen = additions.index_of_note(&sharp_thirteen, &NoteEq::Equivalent);
                if let Some(sharp_thirteen) = sharp_thirteen {
                    let flat_seven = Note::new("b7").unwrap();
                    if !additions.contains_note(&flat_seven, &NoteEq::Equivalent) {
                        additions[sharp_thirteen] = flat_seven;
                    } else {
                        additions.remove(sharp_thirteen);
                    }

                    // println!("in TriadExtensions::inclusions additions changed sharp thirteen to b7");
                }
                // extra_notes = extra_notes.pick_note_for_degree(&Note::from("b7"), &Degree::Sevenths);
            }
            // panic!("HEY {}\n{}",extra_notes.intervals.get(&ChordalDegree::Thirteenths).unwrap().contains_enharmonic("b7"), extra_notes.short_debug())
        }
        // ChordSymbolMods { inclusions, exclusions, alterations, additions }
        if exclusions.contains_strict_note(&Note::new("5").unwrap()){
            for addition in &additions {
                let fifth_poss = Degree::Fifths.allowed_notes().indices_of_note(addition, &NoteEq::Enharmonic);

                if let Some(fifth_poss) = fifth_poss {
                    for i in fifth_poss {
                        let note = &Degree::Fifths.allowed_notes()[i];
                        panic!("Fifth found {} yet 5 in additions: {}\n{}\ninc: {} exc: {} alt: {} add: {}", note, addition, self,
                               inclusions, exclusions, alterations, additions);
                    }

                }
            }
        }

        let ret = QualityMods { inclusions, exclusions, alterations, additions };
        // Validation
        let left = self.with_chord_mods(&ret).to_change().sorted_by_dist().without_note(&Note::new("1").unwrap(), &NoteEq::PitchClass);
        let left = ret.to_change().sorted_by_dist().without_note(&Note::new("1").unwrap(), &NoteEq::PitchClass);
        let right = target.to_change().sorted_by_dist().without_note(&Note::new("1").unwrap(), &NoteEq::PitchClass);
        debug_assert!(left.is_same_pitchclass_of_change(&right),
                      "in produced {} != target: {}\n{} {}\n produced(left):\n{}\ntarget(right):\n{}", left, right, self, ret, self.with_chord_mods(&ret), target
        );

        if false && *self == TriadExtension::from(&Some(&Triad::SusTwo), &Some(&Extension::Six)){
            panic!("triad_extension: {}\n{}\nremaining_poss:\n{}\ntarget:\n{}\nret {}",
                   self, self.possibilities(), remaining_poss, target, ret);
        }
        ret
    }

    pub(crate) fn inclusions_exclusions_alterations_additions_of_possibilities_old(
            &self, poss: &DegreePossibilities, options: &QualityParams)
            -> QualityMods {
        let mut inclusions = Change::new();
        let mut exclusions = Change::new();
        let mut alterations = Change::new();
        let mut additions = Change::new();
        //poss.assign_static_notes();

        // let self_notes = self.to_change();
        // let other_notes = other.to_change();
        //panic!("{}", poss);

        for degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter() {
            let extension_degree_note = self.possibilities().get(degree);
            let poss_degree_notes = poss.get(degree);

            if extension_degree_note.len() == 0 {
                for poss_degree_note in poss_degree_notes {
                    if !self.notes().contains_note(&poss_degree_note, &NoteEq::PitchClass) {
                        additions.push_change(poss_degree_notes);
                        println!("!{} in TriadExtensions::inclusions pushed {} to additions: {}", degree, poss_degree_notes, additions);
                    }
                    // else {
                    //     println!("selfnotes {} contains_note({})", self.notes(), poss_degree_note );
                    //     println!("in TriadExtensions::inclusions not not push {} to additions: {}", poss_degree_notes, additions);
                    // }
                }
            } else { //extension_degree_note.len() == 1
                if poss_degree_notes.len() != 0 {
                    let mut included_note: Option<&Note> = None;
                    // TODO try checking in triadextension instead of possibilities
                    for poss_degree_note in &poss_degree_notes.notes {
                        if poss_degree_note.eq_note(&extension_degree_note[0], &NoteEq::PitchClass) {
                            included_note = Some(&poss_degree_note);
                            inclusions.push_note(&extension_degree_note[0]);
                            println!("-{} in TriadExtensions::inclusions pushed {} to inclusions: {}", degree, &extension_degree_note[0], inclusions);
                        } else {
                            //exclusions.push_note(poss_degree_note);

                            println!("={} in TriadExtensions::inclusions pushed {} to exclusions: {}", degree,&extension_degree_note[0], exclusions);
                        }
                    }
                    if included_note.is_none() {
                        if poss_degree_notes.len() > 1 {
                            panic!();
                        } else {
                            alterations.push_note(&poss_degree_notes[0]);
                            println!("?{} in TriadExtensions::inclusions pushed {} to alterations: {} (inclusions: {})", degree,&poss_degree_notes[0], alterations, inclusions);
                        }
                    }
                } else { // poss_degree_notes.len() == 0
                    exclusions.push_note(&extension_degree_note[0]);
                    println!("$in TriadExtensions::inclusions pushed {} to exclusions: {}", &extension_degree_note[0], exclusions);
                }
            }
        }

        if options.prefer_flat_7_over_sharp_thirteen{
            let sharp_thirteen = Note::new("#13").unwrap();

            if additions.contains_note(&sharp_thirteen, &NoteEq::Equivalent) {
                let sharp_thirteen = additions.index_of_note(&sharp_thirteen, &NoteEq::Equivalent);
                if let Some(sharp_thirteen) = sharp_thirteen {
                    additions[sharp_thirteen] = Note::new("b7").unwrap();
                    println!("in TriadExtensions::inclusions additions changed sharp thirteen to b7");
                }
            }
            // panic!("HEY {}\n{}",extra_notes.intervals.get(&ChordalDegree::Thirteenths).unwrap().contains_enharmonic("b7"), extra_notes.short_debug())
        }

        let mut change: Change = inclusions.clone();
        change.push_change(&alterations);
        change.push_change(&additions);
        change = change.without_notes(&exclusions, &NoteEq::PitchClass);
        let debug_str=
            format!("self: {:?}\n{}\nother:\n{}\ninc: {}\nexc: {}\nalt:{}\nadd:{}",
                    self, self.possibilities().multiline_debug(), poss.multiline_debug(),
                    &inclusions, &exclusions, &alterations, &additions);
        // panic!("{}", TriadExtension::all_legal().into_iter().map(|ext| {
        //     format!("{} {}",ext, ext.notes().sorted_by_dist())
        // }).collect::<Vec<_>>().join("\n"));
        debug_assert!(!TriadExtension { triad: Some(Triad::Major), extension: Some(Extension::Five) }.is_legal());
        debug_assert!(change.is_same_pitchclass_of_change(&poss.to_change()),
                      "self: {} != other: {}\n{}",
                      change.in_first_octave_sorted(), poss.to_change().in_first_octave_sorted(), debug_str);
        QualityMods {inclusions, exclusions, alterations, additions}
    }
}
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct ExtensionMatch {
    pub extension: Extension,
    pub span: GraphemeSpan,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct KeyQuality {
    pub input: String,
    pub key: Key,
    pub quality: Option<Quality>
}

impl KeyQuality {
    pub fn from_string(input:String) -> Option<KeyQuality> {
        let key_parse_result = Key::parse(&input, true);
        if let Some(parsed_key) = key_parse_result {
            let key_str = parsed_key.letter.data.clone() + &*parsed_key.accidentals.data;
            let ending = &input[key_str.len()..];
            let quality = Quality::from_string(ending);

            return Some(KeyQuality {
                input,
                key: Key {
                    accidentals: parsed_key.accidentals,
                    letter: parsed_key.letter},
                quality,
            });
        } else if &input == "N.C" {
            // let key_quality = KeyQuality {
            //     input,
            //     key: Key::from("C########"),
            //     quality: Quality::from_string("N.C"),
            // };
            // panic!("{}", key_quality.keys());
            return Some(KeyQuality {
                input,
                key: Key::from("C########"),
                quality: Quality::from_string("N.C"),
            });
        }
    None
    }

    pub fn keys(&self) -> Keys {
        if let Some(quality) = &self.quality {
            quality.to_change().in_key(&self.key)
        } else {
            Change::from_notes_string("1 3 5").in_key(&self.key)
        }
    }
}

impl Display for KeyQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.quality {
            Some(quality) => {
                // write!(f, "{}{}", self.key, quality)
                write!(f, "{}", concat_string!(self.key.to_string(), quality.to_string()))
            },
            None => {
                write!(f, "{}", self.key)
            }
        }

    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct KeySlashQuality {
    pub input: String,
    pub left_chord: KeyQuality,
    pub right_chord: Option<KeyQuality>,
}

impl AsRef<KeySlashQuality> for KeySlashQuality {
    fn as_ref(&self) -> &KeySlashQuality {
        self
    }
}

impl KeySlashQuality {
    pub fn from_string(input: String) -> Option<Self> {
        let ignore_trigger = "6/9";
        let replace_trigger = "^6/9^";
        let input = input.replace(ignore_trigger, replace_trigger);
        let sides = input.split("/").collect::<Vec<&str>>();
        let left_side = sides[0].replace(replace_trigger,ignore_trigger);
        let mut right_chord = None;
        if sides.len() > 1 {
            right_chord = KeyQuality::from_string(sides[1].parse().unwrap());
        }

        if let Some(left_chord) = KeyQuality::from_string(left_side) {
            return Some(KeySlashQuality {
                input,
                left_chord,
                right_chord
            })
        } else if &input == "N.C" {
            return Some(
                KeySlashQuality {
                    input,
                    left_chord: KeyQuality::from_string("N.C".to_string()).unwrap(),
                    right_chord: None
                }
            )
        }
        None
    }

    pub fn keys(&self) -> Keys {
        let left_keys = self.left_chord.keys();
        let mut keys:Keys = Keys { keys: vec![] };
        if let Some(right_chord) = &self.right_chord {
            if let Some(right_quality) = &right_chord.quality {
                keys = right_chord.keys();
                keys.extend(left_keys)
            } else {
                // Cm/Eb or Cm/F
                let mut right_note_in_left = None;
                for (l,left_key) in left_keys.clone().into_iter().enumerate(){
                    if right_chord.key.frets_from_c() == left_key.frets_from_c() {
                        right_note_in_left = Some(l);
                        break;
                    }
                }
                if let Some(right_note_in_left) = right_note_in_left {
                    keys = Keys::from(&left_keys[right_note_in_left..]);
                    keys.extend(Keys::from(&left_keys[..right_note_in_left]));
                } else {
                    keys = Keys::from(&right_chord.key);
                    keys.extend(left_keys)
                }

            }
        } else {
            keys = left_keys;
        }
        keys
    }

    pub fn midi_notes(&self) -> Vec<u8> {
        self.keys().frets_from_c().iter().map(
            |fret|*fret as u8 + 62
        ).collect()

    }

    pub fn is_equivilent(&self, other: &KeySlashQuality) -> bool {
        if !self.left_chord.key.is_equivalent(&other.left_chord.key) {
            return false;
        } else {
            if let Some(left_quality) = &self.left_chord.quality {
                if let Some(other_left_quality) = &other.left_chord.quality {
                    if !left_quality.is_equivalent(&other_left_quality) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn add_to_midi_track(mut track: &mut Track, quarter_note_time:u32) -> () {
        let QUARTER: u32 = quarter_note_time;
        let DOTTED_QUARTER: u32 = (quarter_note_time as f64 * 1.5) as u32;
        let EIGTH: u32 = (quarter_note_time as f64 * 0.5) as u32;
        const C4: NoteNumber = NoteNumber::new(72);
        // some arbitrary velocity
        const V: Velocity = Velocity::new(64);

        // channel zero (displayed as channel 1 in any sequencer UI)
        const CH: Channel = Channel::new(0);

        track.push_note_on(0, CH, C4, V).unwrap();
        // the note-off event determines the duration of the note
        track
            .push_note_off(DOTTED_QUARTER, CH, C4, Velocity::default())
            .unwrap();
    }
}

impl Display for KeySlashQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let left_side = if let Some(quality) = &self.left_chord.quality
            && let Some(extension_match) = &quality.extension_match
            && extension_match.extension == Extension::NoChord {
            quality.to_string()
        } else {
            self.left_chord.to_string()
        };
        let right_side = match &self.right_chord.is_some() {
            true => "/".to_owned() + &*self.right_chord.as_ref().unwrap().to_string(),
            false => "".to_string(),
        };
        write!(f,"{}{}",left_side,right_side)
    }
}

#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct QualityParams {
    pub use_unicode: bool,
    pub use_double_accidentals: bool,
    pub prefer_flat_7_over_sharp_thirteen: bool,
    pub use_half_diminished: bool,
    pub str_major_triad: &'static str,
    pub str_minor_triad: &'static str,
    pub str_dim_triad: &'static str,
    pub str_dim_triad_ascii: &'static str,
    pub str_sus_triad: &'static str,
    pub use_augmented_chords: bool,
    pub str_aug_triad: &'static str,
    pub use_major_seven_triangle: bool,
    pub str_major_seven: &'static str,
    pub str_major_seven_ascii: &'static str,
    pub use_weird_sus_symbols: bool,
    pub use_half_diminished_chords: bool,
    pub str_half_diminished: &'static str,
    pub str_half_diminished_ascii: &'static str,
    pub use_six_nines: bool,
    pub str_six_nines: &'static str,
    pub move_root_if_no_one: bool,
    // useRomanFunction=None,
    // useUnicodeRoman=None,
    pub str_brackets_sep: &'static str,
    pub str_brackets_open: &'static str,
    pub str_brackets_close: &'static str,
    pub str_add_note: &'static str,
    pub str_remove_note: &'static str,
    pub str_change_note: &'static str,
    pub use_brackets: bool,
    pub name_diads_by_interval_name: bool,
    pub str_natural: &'static str,
    pub str_natural_ascii: &'static str,
    pub hide_major_six_triad: bool,
    // utiliseCaching=None,
    // returnAllCandidates=None,
    // makingTextMulticoloured=None,
    // outlineColourTag=None,
    // rootKey=None,
}



impl Default for QualityParams {
   fn default() -> Self {
        QualityParams {
            use_unicode: true,
            use_double_accidentals: false,
            prefer_flat_7_over_sharp_thirteen: true,
            use_half_diminished: true,
            str_major_triad: "ma",
            str_minor_triad: "m",
            str_dim_triad: "º",
            str_dim_triad_ascii: "dim",
            str_sus_triad: "sus",
            use_augmented_chords: false,
            str_aug_triad: "+",
            use_major_seven_triangle: false,
            str_major_seven: "△",
            str_major_seven_ascii: "^",
            use_weird_sus_symbols: true,
            use_half_diminished_chords: true,
            str_half_diminished: "ø",
            str_half_diminished_ascii: "hdim",
            use_six_nines: true,
            str_six_nines: "69",
            move_root_if_no_one: false,
            str_brackets_sep: " ",
            str_brackets_open: "(",
            str_brackets_close: ")",
            str_add_note: "add",
            str_remove_note: "no",
            str_change_note: "",
            use_brackets: true,
            name_diads_by_interval_name: true,
            str_natural: "♮",
            str_natural_ascii: "nat",
            hide_major_six_triad: true,
        }
    }
}

impl QualityParams {
    pub const DEFAULT:Lazy<Self> = Lazy::new(|| QualityParams::default());
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct Quality {
    pub input: EzStr,
    pub triad_match: Option<ChordTypeMatch>,
    pub extension_match: Option<ExtensionMatch>,
    pub modification: Option<QualityModTokenMatches>,
}

impl Default for Quality {
    fn default() -> Quality {
        Quality {
            input: "".into(),
            triad_match: None,
            extension_match: None,
            modification: None,
            // accidentals: Lazy::new(|| "".into()),
        }
    }
}

impl Display for Quality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut extension_str = String::new();
        let mut triad_str = String::new();
        let mut modification_str = String::new();

        if let Some(triad) = &self.triad_match {
            // triad_str = triad.chord_type.triggers[0].parse().unwrap();
            triad_str = triad.span.text.data.clone();
            triad_str = Accidentals::replace_with_unicode(&*triad_str, false)
        }
        if let Some(extension) = &self.extension_match {
            extension_str = extension.extension.triggers()[0].parse().unwrap();
            // These don't contain accidentals

        }
        if let Some(modification) = &self.modification {
            modification_str = modification.token_matches.iter().map(|m| m.span.to_string()).collect::<String>();
            modification_str = Accidentals::replace_with_unicode(&*modification_str, false)
            //panic!("self.modification: {:?}",modification);
        }
        if self.is_sus_chord() {
            write!(f, "{}{}", extension_str, triad_str, );
        } else {
            write!(f, "{}{}", triad_str, extension_str, );
        }
        write!(f, "{}", modification_str)
    }
}
static PITCH_CLASS_SET_TO_CHORD_QUALITIES: Lazy<Mutex<HashMap<QualityParams,Vec<Vec<Quality>>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});
static CHORD_QUALITY_TO_RATING: Lazy<Mutex<HashMap<QualityParams,Vec<Vec<i32>>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});
static CHORD_QUALITY_PARSE_STRING_CACHE: Lazy<Mutex<HashMap<String, Option<ChordQualityParseResult>>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(2048)));
static CHORD_QUALITY_TO_CHANGE_CACHE: Lazy<Mutex<HashMap<Quality, Change>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(2048)));
impl Quality {
    fn new<S: Into<EzStr> + Clone>(s:S) -> Option<Self> {
        let string = s.clone().into();
        let result = Self::parse_string(string.data);
        let mut triad = None;
        let mut extension = None;
        if result.is_some() {
            if result.clone().unwrap().triad.is_some(){
                triad = result.clone().unwrap().triad;
            }
            if result.clone().unwrap().extension.is_some(){
                extension = result.clone().unwrap().extension;
            }
        }
        if triad.is_none() && extension.is_none() {
            return None;
        }
        Some(Quality {
            input: s.into(),
            triad_match: triad,
            extension_match: extension,
            modification: result.unwrap().modification,
        })
    }

    pub fn from_string<S: Into<EzStr>>(s: S) -> Option<Self> {
        Self::new(s.into())
    }

    pub fn triad_extension(&self) -> TriadExtension {
        let triad = if let Some(ext_match) = &self.triad_match {
            Some(&ext_match.triad)
        } else { None };
        let ext = if let Some(ext_match) = &self.extension_match {
            Some(&ext_match.extension)
        } else { None };
        // panic!("{} {:?} {:?}", self, triad, ext);
        TriadExtension::from(&triad, &ext)
    }

    pub fn to_chord_mods(&self, options: &QualityParams) -> QualityMods {
        let triad_extension = self.triad_extension();
        let poss = DegreePossibilities::all_from_change(&self.to_change()).pick_ideal_notes();
        let mods = triad_extension.chord_mods_from_degree_possibilities(&poss, options);
        mods
    }

    pub fn rating(&self, params: &QualityParams) -> i32 {
        let mods = self.to_chord_mods(&params);
        type Rating = i32;
        let mut score: Rating = 0;
        score += mods.inclusions.len() as Rating * 10;
        score += mods.additions.len() as Rating * 8;
        score += mods.exclusions.len() as Rating * -10;
        score += mods.alterations.len() as Rating * 7;

        if let Some(ext_match) = &self.extension_match {
            let ext = &ext_match.extension;
            let white_keys = TriadExtension::from(&Some(&Triad::Major), &Some(&Extension::MajorThirteen));
            if mods.additions.contains_notes(white_keys.notes(), &NoteMatch::Any(Equivalent)){
                score -= 10;
            }
        }
        if let Some(triad_match) = &self.triad_match {
            let triad = &triad_match.triad;
            if triad == &Triad::Augmented {
                if mods.alterations.contains_note(
                    &Note::new("5").unwrap(), &NoteEq::Degree
                ){
                    score -= 10;
                }
            } else if [Triad::SusSharpFour, Triad::SusFlatTwo].contains(triad){
                score -= 5;
            } else if triad == &Triad::SusTwo {
                score -= 1;
            }
        }
        score
    }

    fn parse_string(input: String) -> Option<ChordQualityParseResult> {
        {
            let cache = CHORD_QUALITY_PARSE_STRING_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&input) {
                return v.clone();
            }
        }
        let input = EzStr::from(input);
        let mut triad_at_start_result = Quality::triad_at_index(&input, 0usize);
        let mut extension_result = Quality::extension_at_index(&input, 0usize);
        let mut modifications_result = None;
        let mut start: usize = 0;
        let mut end: usize = input.len();
        if extension_result.is_some() {
            start = extension_result.as_ref().unwrap().span.text.len();
            if let Some(triad) = Quality::triad_at_index(&input, start){
                if triad.triad.is_sus(){
                    triad_at_start_result = Some(triad);
                }
            }
        }
        else if let Some(triad_at_start) = &triad_at_start_result {
            start = triad_at_start.span.text.len();
            //print!("start:{} end:{} input:{} triad_result:{}", start, end, input, triad_result.clone().unwrap().span.text);

            extension_result = Quality::extension_at_index(&input, start);
            if extension_result.is_some() {
                if input == EzStr::from("mima7"){
                    //panic!("{input} {:?}", extension_result.clone().unwrap());
                }
                //println!(" extension_result:{:?}",ChordQuality::extension_at_index(&input, start).unwrap());
            }

            // if input == EzStr::from("dim7") {
            //     panic!("input: {}, triad: {}, extension: {}", input,triad_at_start_result.clone().unwrap().span, &extension_result.unwrap().span);
            // }
        } else {
            //panic!("{:?}",ChordQuality::extension_at_start(input.slice(start as i32, end as i32)));
            return None;
        }
        // this is a weird way to handle this
        // if let Some(extension_result) = extension_result &&
        //     let Some(triad_at_start_result) = triad_at_start_result{
        //     if extension_result.span.start == triad_at_start_result.span.start {
        //         triad_at_start_result = None;
        //     }
        // }
        if let Some(extension) = &extension_result && triad_at_start_result.is_some() {
            if extension_result.as_ref().unwrap().span.start == triad_at_start_result.as_ref().unwrap().span.start {
                triad_at_start_result = None;
            }
        }
        let mut alterations_start = 0usize;
        if let Some(triad) = &triad_at_start_result {
            alterations_start = cmp::max(alterations_start, triad.span.end);
        }
        if let Some(extension) = &extension_result {
            alterations_start = cmp::max(alterations_start, extension.span.end);
        }
        // if input == EzStr::from("5(add ♭6)"){
        //     let mut panic_str = format!("SHIT input:{input} brackets_start:{alterations_start}");
        //         if triad_at_start_result.is_some() {
        //             panic_str += format!("triad: {}",triad_at_start_result.unwrap().span).as_str();
        //         }
        //         if extension_result.is_some() {
        //             panic_str += format!("extension: {}",extension_result.unwrap().span).as_str();
        //         }
        //
        //     panic!("{}", panic_str);
        // }
        modifications_result = Quality::alteration_tokens_at_index(&input, alterations_start);

        debug_assert!({
            if let Some(mut modifications_result) = modifications_result.clone() {
                if modifications_result.to_chord_modification_token().contains(&&ChordAlterationToken::BracketOpen)
                    && !modifications_result.to_chord_modification_token().contains(&&ChordAlterationToken::BracketClose){
                    return None;
                    panic!("ChordQuality::alteration_tokens_at_index({}, {}) = {:?}",input.clone(),alterations_start, modifications_result);
                } else {
                    true
                }
            } else { true }

        });

        let ret = Some(ChordQualityParseResult {
            input: input.clone().into(),
            triad: triad_at_start_result,
            extension: extension_result,
            modification: modifications_result,
        });
        let mut cache = CHORD_QUALITY_PARSE_STRING_CACHE.lock().unwrap();
        cache.insert(input.clone().into(), ret.clone());
        ret
        //if triad_at_start.is_none()
    }

    pub fn is_equivalent(&self, other: &Quality) -> bool {
        if self.extension_match.is_some() != other.extension_match.is_some()
            || self.triad_match.is_some() != other.triad_match.is_some()
            || self.modification.is_some() != other.modification.is_some()
        {
            return false;
        }
        if let (Some(this), Some(other)) = (&self.triad_match, &other.triad_match){
            if this.triad != other.triad {
                return false;
            }
        }
        if let (Some(this), Some(other)) = (&self.extension_match, &other.extension_match){
            if this.extension != other.extension {
                return false;
            }

        }
        if let (Some(this), Some(other)) = (&self.modification, &other.modification){
            if this.is_equivalent(other) {
                return false;
            }
        }
        true
    }

    fn triad_at_index(s:&EzStr, start:usize) -> Option<ChordTypeMatch> {
        // let mut matches = Vec::new();
        // let start = index.into();
        for triad in Triad::ALL.iter() {
            for trigger in triad.triggers().iter() {
                let trigger = EzStr::from(*trigger);
                let end = start + trigger.len();

                // if s == "mima7" && *trigger == "mi" {
                //     panic!("s: {}, trigger: {}, start: {}, end: {}, abool:{}, bbool:{}, cbool:{}, s[start..end]:{}, s:{}", s,trigger,start,end, s.contains(trigger),end <= s.len(),s[start..end] == s,&s[start..end],&s);
                // }
                if end <= s.len() && s.slice(start, end) == trigger {
                    //let found_index = s[start..].find(trigger).map(|idx|idx + start).unwrap();
                    let grapheme_match = GraphemeSpan{start:start,
                        end:end,
                        text: trigger.clone(),
                    };

                    //grapheme_match.ensure_is_valid(EzStr::from(EzStr::from(s.into())));
                    if s == "sus♯4" {

                    }
                    // Because "sus" is short for SusFour, we have to check the other suses
                    if grapheme_match.as_str() == "sus" {
                        for sus_triad in Triad::SUS_TRIADS {
                            for sus_trigger in sus_triad.triggers().iter() {
                                let sus_trigger = EzStr::from(*sus_trigger);
                                let new_end = start + sus_trigger.len();
                                if new_end <= s.len() && s.slice(start, new_end) == sus_trigger
                                && sus_trigger != "sus" {
                                    let ez = EzStr::from(&sus_trigger);

                                    let grapheme_match = GraphemeSpan::new(start, new_end, ez);

                                    //get rid of this
                                    //grapheme_match.ensure_is_valid(s);

                                    // panic!("sus_triad: {:?}\nsus_trigger: {}\nsus_trigger.len(): {}\ngrapheme_match: {:?}",
                                    //        sus_triad, sus_trigger, sus_trigger.len(),  grapheme_match);
                                    return Some(ChordTypeMatch{
                                        triad:sus_triad.clone().clone(), span:grapheme_match
                                    })
                                }
                                // else {
                                //     println!("sus_trigger: {}", sus_trigger)
                                // }
                            }
                            // println!("sus_triad: {:?}", sus_triad);
                        }
                        // panic!("triad: {:?} {grapheme_match} {:?}", triad, triad.triggers());
                    }
                    // matches.push(ChordTypeMatch{
                    //         triad:triad.clone().clone(), span:grapheme_match
                    // })
                    return Some(ChordTypeMatch{
                        triad:triad.clone().clone(), span:grapheme_match
                    })
                }
            }
        }
        None
    }

    pub fn extension_at_index(s:&EzStr, index:usize) -> Option<ExtensionMatch> {
        let start = index;
        // let s = s.clone().into();
        for extension in Extension::ALL {
            for trigger in extension.triggers().iter() {
                let trigger = EzStr::from(*trigger);
                let end = start + trigger.len();
                //if trigger.len() + index > s.len()
                //let found_index = s[start..].find(trigger).map(|idx|idx + index).unwrap();
                //panic!("{}",found_index);
                if end <= s.len() && s.slice(start,end) == trigger{
                    let grapheme_match = GraphemeSpan {
                        start,
                        end,
                        text: trigger.clone(),
                    };
                    //grapheme_match.ensure_is_valid(s);
                    return Some(ExtensionMatch {
                        extension: extension.clone().clone(),
                        span: grapheme_match
                    });
                }
            }
        }
        None
    }

    fn alteration_tokens_at_index(s:&EzStr, index:usize) -> Option<QualityModTokenMatches> {
        //println!("alteration_tokens_at_index({s}, {index}) -> " );
        let mut tokens:Vec<ChordModificationMatch> = Vec::new();
        let mut start = index;
        let triggers_to_mod_type = vec![
            (vec!["add", "+"], ChordAlterationToken::Add ),
            (vec!["no", "-"], ChordAlterationToken::Remove ),
            (vec!["("], ChordAlterationToken::BracketOpen ),
            (vec![" "], ChordAlterationToken::Space ),
            (vec![","], ChordAlterationToken::Comma ),
            (vec![")"], ChordAlterationToken::BracketClose ),
        ];
        while start < s.len() {
            let mut trigger_found = false;
            for (triggers, token_type) in &triggers_to_mod_type {
                if trigger_found { break; }
                for trigger in triggers {
                    if trigger_found { break; }
                    let trigger = EzStr::from(*trigger);
                    let end = start + trigger.len();
                    // if s == "mima7" && *trigger == "mi" {
                    //     panic!("s: {}, trigger: {}, start: {}, end: {}, abool:{}, bbool:{}, cbool:{}, s[start..end]:{}, s:{}", s,trigger,start,end, s.contains(trigger),end <= s.len(),s[start..end] == s,&s[start..end],&s);
                    // }
                    if end <= s.len() && s.slice(start, end) == trigger {
                        //let found_index = s[start..].find(trigger).map(|idx|idx + start).unwrap();
                        let grapheme_match = GraphemeSpan {
                            start: start,
                            end: end,
                            text: trigger.clone(),
                        };
                        // grapheme_match.ensure_is_valid(s);
                        tokens.push(ChordModificationMatch {
                            span: grapheme_match,
                            chord_mod: token_type.clone(),
                        });
                        start = end;
                        trigger_found = true;
                    }
                }
            }
            if !trigger_found {
                if let Some(note) = Note::parse_until_invalid(&s, start){
                    // TODO This allocates
                    let end = start + EzStr::from(note.text()).len();

                    tokens.push(ChordModificationMatch {
                        span:
                            GraphemeSpan { start, end, text: note.text().clone().into() },
                        chord_mod: ChordAlterationToken::Note,
                    });
                    start = end;
                    trigger_found = true;
                } else{
                    start += 1;
                    break;
                }
                //panic!("input: {s} trigger not found at start:{start}, which contains {}\ntokens{tokens:?}",s[start])
            }
        }
        if tokens.len() > 0 {
            // if s.contains("Ma7(no 5)") {
            //     panic!("input: {} found {:?}", s, tokens);
            // }
            return Some(QualityModTokenMatches { token_matches: tokens })
        }
        None
    }

    fn first_triad_in_str(s:&str) -> Option<Triad> {
        for triad in Triad::ALL {
            for trigger in triad.triggers().iter() {
                if s.contains(trigger) {
                    return Some(triad.clone().clone())
                }
            }
        }
        None
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    /// this is not doing anything to accidentals/unicode.
    /// and it's caching
    pub fn to_change(&self) -> Change {
        {
            let cache = CHORD_QUALITY_TO_CHANGE_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&self) {
                return v.clone();
            }
        }
        let notes = if let Some(mods) = &self.modification {
            // if self.input == "m(♯5)" {
            //     panic!("self: {} mods: {} triad_extension: {} mods_applied: {}",
            //            self, mods.to_string(), self.triad_extension(), mods.apply_to_triad_extension(&self.triad_extension()));
            // }
            mods.apply_to_triad_extension(&self.triad_extension())
        } else {
            self.triad_extension().notes().clone()
        };
        let mut cache = CHORD_QUALITY_TO_CHANGE_CACHE.lock().unwrap();
        cache.insert(self.clone(), notes.clone());

        notes
    }


    pub fn to_frets(&self) -> Frets {
        self.to_change().to_frets()
    }

    pub fn short_debug(&self) -> String {
        let mut ret = String::new();
        ret += "ChordQuality {triad: ";
        if let Some(triad) = self.triad_match.clone() {
            ret.push_str(format!("{:?}", &self.clone().triad_match.unwrap().span.text).as_str());
        } else {
            ret.push_str("None");
        }
        ret.push_str(", extension: ");
        if let Some(extension) = self.extension_match.clone() {
            ret.push_str(format!("{:?}", &self.clone().extension_match.unwrap().span.text).as_str());
        } else {
            ret.push_str("None");
        }
        ret.push_str(", mods: ");
        if let Some(modification) = self.modification.clone() {
            ret.push_str(format!("{:?}", &self.clone().modification.unwrap()).as_str());
        } else {
            ret.push_str("None");
        }
        ret.push_str("}");
        ret.push_str(format!(" == {}", self).as_str());
        ret
    }

    pub fn is_sus_chord(&self) -> bool {
        if let Some(triad) = &self.triad_match {
            if Triad::is_sus(&triad.triad) {
                return true;
            }
        }
        false
    }

    pub fn from_pitch_class_set_with_options(frets: &Frets, params: &QualityParams) -> Vec<Quality> {
        let cache_key = frets.big_endian() as usize;
        // Fast path: check cache while holding the lock, then drop the lock before heavy work.
        {
            let mut chord_qualities_cache = PITCH_CLASS_SET_TO_CHORD_QUALITIES.lock().unwrap();
            let mut chord_quality_ratings_cache = CHORD_QUALITY_TO_RATING.lock().unwrap();

            chord_qualities_cache.entry(params.clone()).or_insert_with(|| vec![Vec::new(); 4096]);
            chord_quality_ratings_cache.entry(params.clone()).or_insert_with(|| vec![Vec::new(); 4096]);


            if let Some(slot) = chord_qualities_cache.get(params).and_then(|v| v.get(cache_key)) {
                if !slot.is_empty() {
                    // println!("in from_pitch_class_set_with_options Returned cached value for cache_key: {}slot:\n{}", cache_key, slot.as_slice().into_iter().join("\n\t"));
                    return slot.clone();
                }
            }
        } // lock dropped here

        // Do the expensive calculation without holding the mutex.
        let chord_qualities = Self::calculate_from_pitch_class_set_with_options(frets, params);

        let ratings: Vec<i32> = chord_qualities.iter().map(|chord|{
            chord.rating(params)
        }).collect();

        // 1. Enumerate, map to (index, element), and collect into a vector
        let mut enumerated: Vec<(usize, &Quality)> = chord_qualities.iter().enumerate().collect();

        // 2. Sort by key (in this case, by the length of the string element)
        enumerated.sort_by_key(|&(_index, val)| Reverse(ratings[_index]));

        // Store result back into the cache.
        let mut chord_qualities_cache = PITCH_CLASS_SET_TO_CHORD_QUALITIES.lock().unwrap();
        let mut ratings_cache = CHORD_QUALITY_TO_RATING.lock().unwrap();
        if let Some(vecs) = chord_qualities_cache.get_mut(params) {
            // println!("in from_pitch_class_set_with_options calculated value for cache_key: {} val:\n{}", cache_key, val.as_slice().into_iter().join("\n\t"));
            vecs[cache_key] = enumerated.iter().map(|(_, chord)| chord.clone().clone()).collect::<Vec<_>>().clone();
        }
        if let Some(vecs) = ratings_cache.get_mut(params) {
            vecs[cache_key] = enumerated.iter().map(|(index, _)| ratings[*index]).collect::<Vec<_>>().clone();;
        }
        chord_qualities
    }

    pub fn calculate_from_pitch_class_set_with_options(frets: &Frets, params: &QualityParams) -> Vec<Quality> {
        // maybe skip this if you trust how it's called?
        let pitch_class_set = &frets.to_pitch_class_set();
        // let i = PowerSet::BigEndian::<12>.to_index(&pitch_class_set.to_bitmap()) as usize;

        // let mut all = PITCH_CLASS_SET_TO_CHORD_QUALITIES.lock().unwrap();
        // // maybe if params was implemented as only being different from default
        // // rather than every setting? The cost of hashing here would be less
        // all.entry(params.clone()).or_insert(vec![Vec::new(); 4096]);
        // let vals: &mut Vec<Vec<ChordQuality>> = all.get_mut(params).unwrap();
        // unsafe {
        //     if !vals.get_unchecked(i).is_empty() {
        //         return vals.get_unchecked(i).clone()
        //     }
        //
        // }


        let notes = &frets.to_change();
        // do we need pick ideal notes?
        // let poss =  DegreePossibilities::all_from_change_no_one(self);
        let poss = DegreePossibilities::all_from_change(notes).pick_ideal_notes();
        //panic!("self:{}\nall:{}\nideal:{}",self, DegreePossibilities::all_from_change_no_one(self),DegreePossibilities::all_from_change_no_one(self).pick_ideal_notes());

        //panic!("self: {self}\nextension_symbols: {:#?}", extension_symbols.keys());
        let mut chord_qualities:Vec<Quality> = Vec::new();
        let mut debug_str = String::new();

        debug_assert!(poss.poss_triad_extensions().len() > 0,
                      "possible triad_extensions {:?} not found for {}.\nposs_triads: {:?}\n poss_exts: {:?}",
                      poss.poss_triad_extensions(), notes, poss.possible_triads(), poss.possible_extensions() );


        // panic!("frets: {}\nideal:\n{}\nposs_triads:{:?}\nposs_extensions: {:?}\nposs_triad_extensions(): {}", frets, poss, poss.possible_triads(),poss.possible_extensions(),poss.poss_triad_extensions().as_slice().into_iter().map(|el|format!("{}", el)).collect::<Vec<_>>().as_slice().join("\n"));
        // if true || **pitch_class_set == [0,6,11]{
        //     panic!("Shit {}", poss.poss_triad_extensions().as_slice().into_iter().join("\n"));
        // } else {
        //     println!("blah {:?}",**pitch_class_set);
        // }


        for triad_extension in poss.poss_triad_extensions() {
            // for triad_extension in TriadExtension::all_legal().iter() {
            //debug

            // if triad_extension != TriadExtension::from(&Some(&Triad::SusTwo), &None){
            //     continue
            // }
            let triad = &triad_extension.triad;
            let extension = &triad_extension.extension;
            let symbol = triad_extension.possibilities();
            // let (inclusions, exclusions, alterations, additions)
            //     = symbol.inclusions_exclusions_alterations_additions_of_other(&possibilities, options);
            let chord_mods
                = triad_extension.chord_mods_from_degree_possibilities(&poss, params);
            let inclusions = &chord_mods.inclusions;
            let exclusions = &chord_mods.exclusions;
            let additions = &chord_mods.additions;
            let alterations = &chord_mods.alterations;
            // panic!("self: {}\nposs.poss_triad_extensions() {:?}\ntriad_extension: {}\nchord_mods: {}",
            //        self, poss.poss_triad_extensions(), triad_extension, chord_mods);

            // //wrong triad when we're specifying natural 3
            // if alterations.contains(Note::new("3").unwrap()){
            //     continue;
            // }
            // altering the 1 3 5 7th is probably bad
            if alterations.contains_notes(Triad::Major.notes(), &NoteMatch::Any(Equivalent)){
                continue;
            }
            // should probably be a dominant
            if alterations.contains_notes(&Change::from_notes_string("3 7"), &NoteMatch::Any(NoteEq::Degree)){
                continue
            }


            if let Some(triad) = triad {
                // If we're adding a 3 we should probably use major, although I'm not sure
                if additions.contains_note(&Note::new("3").unwrap(), &NoteEq::Equivalent){
                    continue
                }
                if additions.contains_note(&Note::new("5").unwrap(), &NoteEq::Equivalent){
                    continue
                }
                // dim or sus with no 3 should be minor, minor with no 3 should be major
                if ([Triad::HalfDiminished, Triad::Diminished, Triad::Minor].contains(triad)
                    || triad.is_sus())
                    && exclusions.contains_note(&Note::new("3").unwrap(), &NoteEq::Exact) {
                    // TODO smelly?
                    // println!("-=-=- skipping {} because reasons", triad_extension);
                    continue;
                }
                // Doing "add 7" or "add b7" doesn't make sense.. as we'd use a seventh chord
                // unless we have both sevenths
                if additions.contains_note(&Note::new("7").unwrap(), &NoteEq::Degree)
                    && !inclusions.contains_note(&Note::new("7").unwrap(), &NoteEq::Degree) {
                    // println!("-=-=- skipping {} because additions has 7", triad_extension);
                    continue;
                }
                // This triad has an altered 5th which we're excluding... so get rid of it
                // should be minor or major
                if [Triad::Augmented, Triad::Diminished, Triad::HalfDiminished].contains(triad)
                    && (exclusions.contains_note(&Note::new("5").unwrap(), &NoteEq::Exact)
                || alterations.contains_note(&Note::new("5").unwrap(), &NoteEq::Degree)) {
                    continue;
                }
                // Should be diminished
                if triad == &Triad::HalfDiminished && extension.is_none(){
                    continue;
                }
            }
            for note in inclusions {
                debug_assert!(notes.contains_note(&note, &NoteEq::Enharmonic), "shit self {notes} does not contain {note}, yet it is within inclusions")
            }
            // Construct chord symbol
            let mut chord_str = String::new();
            let mut triad_str = String::new();
            let mut extension_str: String = String::new();
            if let Some(extension) = extension {
                extension_str.push_str(extension.to_chord_string(params).as_str());
            }
            if let Some(triad) = triad {
                if let Some(extension) = extension
                    && triad == &Triad::Major
                    && extension.hides_ma_triad_str(params.hide_major_six_triad){
                    // println!("didn't push triad {:?} to str in {:?} {}.contains_major_token() {:?}", triad, triad_extension, extension, extension.contains_major_token() );
                } else {
                    triad_str.push_str(&triad.to_chord_string(&params).as_str());
                    // println!("pushed triad {:?} to str in {:?} {:?}", triad, triad_extension, extension );
                }
                // panic!("{:?} {:?} chord_str: {}", extension, triad, chord_str)
            }
            if let Some(triad) = triad && triad.is_sus() {
                chord_str.push_str(&extension_str.as_str());
                chord_str.push_str(&triad_str.as_str());
            } else {
                chord_str.push_str(&triad_str.as_str());
                chord_str.push_str(&extension_str.as_str());
            }
            // Do alterations

            let has_alterations = !alterations.is_empty();
            let has_additions = !additions.is_empty();
            let has_exclusions = !exclusions.is_empty();
            let has_brackets = has_additions || has_exclusions || has_alterations;

            if has_brackets {
                chord_str.push_str(params.str_brackets_open)
            }
            if has_alterations {
                chord_str.push_str(&alterations.notes.as_slice().iter().map(|note|{
                    let mut note_str:String = match note.is_natural() {
                        true => {
                            if params.use_unicode {
                                params.str_natural
                            } else {
                                params.str_natural_ascii
                            }
                        }, false => {
                            ""
                        }
                    }.to_string();
                    note_str.push_str(&note.text());
                    note_str
                }).collect::<Vec<_>>().as_slice().join(params.str_brackets_sep));

                if has_additions || has_exclusions {
                    chord_str.push_str(params.str_brackets_sep);
                }
            }
            if has_exclusions {
                chord_str.push_str(params.str_remove_note);
                chord_str.push_str(params.str_brackets_sep);

                chord_str.push_str(&exclusions.notes.iter().map(|c|c.degree_note().text()).collect::<Vec<_>>().join(params.str_brackets_sep).as_str());
                if has_additions {
                    chord_str.push_str(params.str_brackets_sep);
                }
            }
            if has_additions {
                chord_str.push_str(params.str_add_note);
                if !params.str_add_note.is_empty() {
                    chord_str.push_str(params.str_brackets_sep);
                }
                chord_str.push_str(&additions.sorted_by_dist().join(params.str_brackets_sep).as_str());
                // last elem gets no sep
                // chord_str.push_str(options.str_brackets_sep);
            }


            // validation
            if cfg!(debug_assertions) {
                for addition in additions {
                    // unnecessary addition because triad contains already
                    if let Some(triad) = triad && triad.notes().contains_enharmonic(addition){
                        let panic_str = format!("self: {}\nextension: {extension:?} triad {triad:?} {}\n_+ inclusions {} additions {} exclusions {} alterations {}\n",notes,Change::from_triad_extension(Some(&triad), extension.as_ref()),inclusions, additions, exclusions, alterations);
                        panic!("Error:\n{panic_str} triad note: {} should not be in additions: {}. triad_notes: {}\nsymbol:\n{}\npossibilities:\n{}", addition, additions, triad.notes(), symbol, poss)
                    }
                    // unnecessary addition because extension contains already
                    if let Some(extension) = extension && triad_extension.notes().contains_note(&addition, &NoteEq::Enharmonic){
                        // if let Some(extension) = extension && extension.notes().contains_note(&addition, &NoteEq::Enharmonic){

                        panic!("for {} with {} Error: extensions note: {} should not be in additions.\n{}\nextension_notes: {}\nsymbol:\n{}", notes.in_first_octave_sorted(),triad_extension,addition, chord_mods, extension.notes(), symbol)
                    }
                }
            }
            //println!("{} is it {} for {} with {}", triad_extension.with_chord_mods(&chord_mods).to_change().sorted_by_dist(), self.sorted_by_dist(), triad_extension, chord_mods);
            //debug_assert!()
            if has_brackets {
                chord_str.push_str(params.str_brackets_close);
            }
            // done layout of quality
            debug_assert!(
                notes.eq_change(&Quality::from_string(&chord_str).unwrap().to_change(), &ChangeEq::Unordered(NoteEq::PitchClass)),
                "{}\n{} != {}\n\t{}\n!=\n\t{}",
                &chord_str,
                notes, &Quality::from_string(&chord_str).unwrap().to_change(),
                chord_mods, &Quality::from_string(&chord_str).unwrap().to_chord_mods(params),
            );
            if cfg!(debug_assertions) {
                let chord_quality_mods = &Quality::from_string(&chord_str).unwrap().to_chord_mods(params);
                debug_assert!(
                    chord_mods.eq_mods(
                        chord_quality_mods,
                        &ChangeEq::Unordered(NoteEq::Equivalent)) == true,
                    "{} is {}\nfrom Change method for {}\n\tchord_mods: {chord_mods}\n!=\nfrom ChordQuality method for {}\n\tchord_mods: {}\ntriad_extension.possibilities():\n\t{}\nchord quality possibilities:\n\t{poss}:",
                    &chord_str, notes,
                    triad_extension,
                    Quality::from_string(&chord_str).unwrap().triad_extension(),
                    chord_quality_mods,
                    DegreePossibilities::all_from_change(&Quality::from_string(&chord_str).unwrap().to_change()).pick_ideal_notes(),
                );
            }
            // triad_extension.possibilities()
            // if let Some(triad) = triad && triad == &Triad::Diminished && let Some(extension) = extension && extension == &Extension::MajorThirteen {
            //     // println!("self:{self} {:?} {:?}\n{}", triad, extension, TriadExtension::from(&Some(triad), &Some(extension)).possibilities());
            // }
            chord_qualities.push(Quality::from_string(&chord_str).expect(
                format!("Invalid chord quality generated: {chord_str}\n{} ==? {}\ntriad: {:?} extension: {:?}\n{chord_mods}\ntriad_extension({triad:?} {extension:?}) == symbol:\n{}", notes, chord_str, triad, extension, symbol).as_str())
            );
            // if true || chord_str == "ma7(no 5)" {
            //     debug_str.push_str(format!("triad_extension {:?}\ntriad_str {:?} extension_str {:?}",triad_extension, triad_str, extension_str ).as_str());
            // }
            // debug_str.push_str(format!("_+ inclusions {} additions {} exclusions {} alterations {}\n",inclusions, additions, exclusions, alterations).as_str());
        }
        // panic!("{debug_str}\nself {}\n{}\nposs_triad_extensions:\n{:#?}", self,
        //        chord_qualities.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join(",\n"),
        //        possibilities.poss_triad_extensions()
        // );

        //chord_qualities.sort_by_key(|a| Reverse(a.rating(options)));
        // unsafe {
        //     *(vals.get_unchecked_mut(i)) = chord_qualities.clone();
        // }
        chord_qualities
    }
}

#[derive(Debug)]
pub enum ChangeEq {
    /// Only the first occurrence of note in each change matches
    ///
    /// Then the notes are compared in order
    Deduplicated(NoteEq),
    /// Each contains every note of the other, regardless of order
    ///
    /// O(n2)
    Unordered(NoteEq),
    /// Notes are in same order and equal each other
    Equal(NoteEq),
    /// Deduplicated, octave normalized and accepting all enharmonic values.
    PitchClass,
}
impl ChangeEq {
    pub fn eq(&self, left: &Change, right: &Change) -> bool {
        match self {
            Self::PitchClass => {
                left.is_same_pitchclass_of_change(right)
            }
            ChangeEq::Deduplicated(note_eq) => {
                let right_dedup = right.dedup(note_eq);
                for (n,left_note) in left.dedup(note_eq).into_iter().enumerate() {
                    if !left_note.eq_note(&right_dedup[n], note_eq){
                        return false
                    }
                }
                true
            }
            ChangeEq::Equal(note_eq) => {
                for n in 0..left.len() {
                    if !left[n].eq_note(&right[n], note_eq){
                        return false
                    }
                }
                true
            }
            ChangeEq::Unordered(note_eq) => {
                if left.len() == right.len() {
                    for note in left {
                        if !right.contains_note(note, note_eq) {
                            return false
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum NoteEq {
    /// If notes are equivalent and are within the same octave
    ///
    /// 13 != 6, ######5 != bbbbbb5
    ///
    /// x5 == ##5, b5 == ♭5, b3 == #2
    Enharmonic,
    /// Uses octave equivalence.
    ///
    /// This will return that bb3 == 2 == 9
    PitchClass,
    /// This will match only if the data is exactly the same.
    Exact,
    /// This will compare the degree and number of accidentals
    /// returns true if those are equal. This way:
    ///
    /// x5 == ##5, b5 == ♭5,
    ///
    /// but 13 != 6, b3 != #2
    Equivalent,
    /// This will only compare the degree
    ///
    /// b6 == #6, but 13 != 6
    Degree,
    /// This will compare the degree enharmonically
    ///
    /// b6 == #6, and 13 == 6
    DegreeEnharmonic,
}

impl NoteEq {
    pub fn eq(&self, note: &Note, other: &Note) -> bool {
        match self {
            NoteEq::Enharmonic => { note.is_same_pitch_class(other) }
            NoteEq::PitchClass => { note.is_same_pitch_class(other) }
            NoteEq::Exact => { note.text() == other.text() }
            NoteEq::Equivalent => { note.is_equivalent(other)}
            NoteEq::Degree => { note.degree == other.degree }
            NoteEq::DegreeEnharmonic => { note.degree_note().in_first_octave() == other.degree_note().in_first_octave()  }
        }
    }
}

#[derive(Debug, PartialEq)]
pub (crate) enum NoteMatch {
    First(NoteEq),
    All(NoteEq),
    Any(NoteEq),
}

impl NoteMatch {
    fn contains_notes(&self, reference: &Change, target: &Change) -> bool {
        match self {
            NoteMatch::First(note_eq) | NoteMatch::Any(note_eq) => {
                for right_note in target {
                    if reference.contains_note(right_note, note_eq) {
                        return true
                    }
                }
                false
            }
            NoteMatch::All(note_eq) => {
                for right_note in target {
                    if !reference.contains_note(right_note, note_eq) {
                        return false
                    }
                }
                true
            }
        }
    }

    fn contained_notes(&self, reference: &Change, target: &Change) -> Change {
        let indices = self.indices_of_notes(reference, target);
        // panic!("{self:?} ref: {reference} target: {target}: {:?}", indices);
        if let Some(indices) = indices {
            indices.into_iter().map(|i| &reference[i]).into_iter().collect()
        } else {
            Change::new()
        }
    }

    fn indices_of_notes(&self, reference: &Change, target: &Change) -> Option<Vec<usize>> {
        let mut notes = vec![];
        'target: for (_, target_note) in target.notes.iter().enumerate() {
            for (l, left_note) in reference.notes.iter().enumerate() {
                if left_note.eq_note(target_note, self.note_eq()) {
                    notes.push(l);
                    if *self == Self::First(self.note_eq().clone()){
                        continue 'target;
                    }
                }
            }
        }
        if !notes.is_empty(){
            return Some(notes);
        }
        None
    }

    fn remove_note(&self, reference: &mut Change, note: &Note) {
        let indices = self.indices_of_note(reference, note);
        if let Some(indices) = indices {
            if !indices.is_empty() {
                reference.notes = (0..reference.len())
                    .filter(|i| !indices.as_slice().contains(i))
                    .map(|i| reference[i].clone())
                    .collect();
            } else {
                panic!("Failed to remove note: {} from: with {:?}", note, reference);
            }
        } else {
            panic!("Failed to remove note: {} from: with {:?}", note, reference);
        }
    }

    fn remove_notes(&self, reference: &mut Change, target: &Change) {
        target.notes.iter().map(|note| reference.remove_note(note, self));
    }

    fn indices_of_note(&self, reference: &Change, target: &Note) -> Option<Vec<usize>> {
        let mut notes = vec!();
        match self {
            NoteMatch::First(note_eq) => {
                for (n,note) in reference.notes.iter().enumerate() {
                    if note_eq.eq(note, target) {
                        return Some(vec![n]);
                    }
                }
                None
            }
            NoteMatch::Any(note_eq) | NoteMatch::All(note_eq) => {
                for (n,ref_note) in reference.notes.iter().enumerate() {
                    if note_eq.eq(ref_note, target) {
                        notes.push(n);
                    }
                }
                if !notes.is_empty(){
                    Some(notes)
                } else {
                    None
                }
            }
        }
    }


    fn note_eq(&self) -> &NoteEq {
        match self {
            Self::First(eq) => eq,
            Self::All(eq) => eq,
            Self::Any(eq) => eq,
        }
    }
}

// enum NoteOrd {
//     Distance,
//
// }



#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct Note {
    // pub text: EzStr,
    pub accidentals: EzStr,
    pub degree: EzStr,
}

static NOTE_FRETS_FROM_ONE_CACHE: Lazy<Mutex<HashMap<String, Fret>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));
static NOTE_PITCH_CLASS_CACHE: Lazy<Mutex<HashMap<String, Fret>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));
static NOTE_IN_FIRST_OCTAVE_CACHE: Lazy<Mutex<HashMap<String, Note>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));
static NOTE_PARSE_CACHE: Lazy<Mutex<HashMap<String, Option<(String, String)>>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));
static NOTE_PARSE_UNTIL_INVALID_CACHE: Lazy<Mutex<HashMap<(String, usize), Option<Note>>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));

impl From<&str> for Note {
    fn from(s:&str) -> Note {
        Note::new(s).expect(format!("REASON: s: {} is not a note", s).as_str())
    }
}

impl From<String> for Note {
    fn from(s:String) -> Note {
        Note::new(s).expect("REASON")
    }
}

impl From<&EzStr> for Note {
    fn from(s:&EzStr) -> Note {
        Note::new(s.data.clone()).expect("REASON")
    }
}

impl Into<EzStr> for Note {
    fn into(self) -> EzStr {
        self.text().into()
    }
}

impl Into<Note> for &Note {
    fn into(self) -> Note {
        self.clone()
    }
}

impl<'a> Into<Change> for &Note {
    fn into(self) -> Change {
        Change::from_note(self.clone())
    }
}

impl<'a> Into<Change> for Note {
    fn into(self) -> Change {
        Change::from_note(self)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.unicode_accidentals(false))
    }
}

impl Note {
    pub const ALLOWED_JAZZ: Lazy<Change> = Lazy::new(|| Change::from_note_strings(
        &["1","b2","2","b3","3","4","#4","b5","5","#5","b6","6","b7","7","#2","#6",]));

    pub const fn scale_degree_to_fret_first_octave(degree: Fret) -> Option<Fret> {
        match degree {
            1 => Some(0),
            2 => Some(2),
            3 => Some(4),
            4 => Some(5),
            5 => Some(7),
            6 => Some(9),
            7 => Some(11),
            _ => None,
        }
    }
    pub const fn fret_to_scale_degree_single_sharps(fret: Fret) -> Option<Fret> {
        match fret {
            0 | 1 => Some(1),
            2 | 3 => Some(2),
            4 => Some(3),
            5 | 6 => Some(4),
            7 | 8 => Some(5),
            9 | 10 => Some(6),
            11 => Some(7),
            _ => None,
        }
    }

    pub const fn fret_to_scale_degree_single_flats(fret: Fret) -> Option<Fret> {
        match fret {
            0 => Some(1),
            1 | 2 => Some(2),
            3 | 4 => Some(3),
            5 => Some(4),
            6 | 7 => Some(5),
            8 | 9 => Some(6),
            10 | 11 => Some(7),
            _ => None,
        }
    }

    pub fn text(&self) -> String {
        concat_string!(self.accidentals, self.degree)
    }

    pub fn change_degree_to(&mut self, note: &Note, use_unicode: bool, use_double_accidentals: bool) {
        let note_degree = note.degree_note();
        let degrees_fret_dist = (self.degree_note().to_fret() - note_degree.to_fret()).rem_euclid(12);
        let acc_dist = degrees_fret_dist + self.accidentals_dist();
        let acc_dist = ((acc_dist + 6) % 12) - 6; // fix too many accidentals
        let note_string = format!("{}{}", Accidentals::from_dist(acc_dist, use_unicode, use_double_accidentals),
                                  note_degree,
        );
        *self = Note::new(&note_string).expect(&note_string);
        //println!("Changing degree of {} to {} == {}",self,note, ret);
        // Note::new(format!("{}{}",note_degree.as_str(),
        //                   Accidentals::from_dist(degrees_fret_dist + self.accidentals_dist(), use_unicode, use_double_accidentals))).unwrap();
    }

    pub fn compute_fret(&self) -> Fret {
        // memoize
        {
            let cache = NOTE_FRETS_FROM_ONE_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&self.text()) {
                return *v;
            }
        }
        let accidentals_frets:Fret = Accidentals::get_dist(&self.accidentals.data);
        let degree_frets:Fret = Note::degree_to_fret((&self.degree.data).parse::<Fret>().unwrap());

        let ret = accidentals_frets + degree_frets;
        // memoize
        let mut cache = NOTE_FRETS_FROM_ONE_CACHE.lock().unwrap();
        cache.insert(self.text(), ret);
        ret
    }

    /// 5 -> nat5, (use_unicode == false)
    /// or 5 -> ♮5,  (use_unicode == true)
    /// Non-naturals are unaffected
    pub fn make_natural_explicit(&mut self, use_unicode: bool) {
        match self.accidentals_dist(){
            0 => {
                match use_unicode {
                    true => {
                        self.accidentals = Accidentals::NATURAL.into();
                    }
                    false => {
                        self.accidentals = Accidentals::ASCII_NATURAL.into();
                    }
                }
                // self.text.data.insert_str(0,self.accidentals.as_str());
            }
            _ => {

            }
        }
    }

    /// ```
    /// use jazz_chord::Change;
    /// assert!(Change::from_notes_string("1 b2 bb3").to_single_accidentals(true) == Change::from_notes_string("1 b2 2"), "in Note::to_single_accidentals(use_flats: {}) {}", true, Change::from_notes_string("1 b2 bb3").to_single_accidentals(true));
    /// assert!(Change::from_notes_string("1 b2 bb3").to_single_accidentals(false) == Change::from_notes_string("1 #1 2"), "in Note::to_single_accidentals(use_flats: {}) {}", false, Change::from_notes_string("1 b2 bb3").to_single_accidentals(false)); //Change::from_notes_string("1 b2 bb3").to_single_accidentals(false)
    /// ```
    pub fn to_single_accidentals(&self, use_flats: bool) -> Note {
        let fret = self.to_fret();
        let octave = fret / 12;
        let first_octave_fret = fret % 12;
        let fret_to_scale_degree = if use_flats { Note::fret_to_scale_degree_single_flats } else { Note::fret_to_scale_degree_single_sharps };

        let degree = Note::from(fret_to_scale_degree((first_octave_fret + 7 * octave)).unwrap().to_string());
        let acc_dist = fret - degree.to_fret();
        // let acc_dist = degree.to_fret() - fret;
        let mut accidentals = String::new();
        if acc_dist != 0 {
            let using_unicode = Accidentals::contains_unicode_accidentals(&self.accidentals);
            let using_doubles = Accidentals::contains_double_accidentals(&self.accidentals);
            accidentals = Accidentals::from_dist(acc_dist, using_unicode, using_doubles);
        }

        // let note =         Note { text: (accidentals.clone() + &degree.text.data.clone()).into(),
        //     accidentals: accidentals.into(),
        //     degree: degree.to_string().into() };
        // panic!("WOAH self: {} note: {} use_flats: {}", self, note, use_flats);

        Note {
            accidentals: accidentals.into(),
            degree: degree.to_string().into() }

    }
    
    pub fn new<T: Into<EzStr>>(text: T) -> Option<Self> {
        let text = text.into();
        let result = Self::parse(&text);
        if result.is_none() {
            None
        } else {
            Some(
                Self {
                    accidentals: result.clone().unwrap().0.into(),
                    degree: result.unwrap().1.into() }
            )
        }
    }

    pub fn to_change(&self) -> Change {
        Change::from_note(self.clone())
    }

    pub fn is_natural(&self) -> bool {
        Accidentals::get_dist(self.accidentals.data.as_str()) == 0
    }

    pub fn degree_note(&self) -> Note {
        Note::from(&self.degree)
    }

    fn parse(text: &EzStr) -> Option<(String, String)> {
        // memoize
        {
            let cache = NOTE_PARSE_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&text.data) {
                return v.clone();
            }
        }
        if text.len() == 0 {
            panic!("REASON this note string has nothing in it");
        }
        let mut accidentals = String::new();
        let mut degree = String::new();
        let mut is_parsing_accidentals = true;
        let mut c_inactive_until = 0usize;
        let mut ret:Option<(String, String)>;
        for (c, char) in text.graphemes().iter().enumerate() {
            if c < c_inactive_until {
                continue;
            }
            if is_parsing_accidentals {
                let mut trigger_found = false;
                for trigger in Accidentals::ALL_TRIGGERS_LONGEST_FIRST.iter() {
                    let trigger = EzStr::from(*trigger);
                    let end = c + trigger.len();
                    if end < text.len() && trigger == text.slice(c, end) {
                        accidentals.push_str(&trigger.data);
                        c_inactive_until = c_inactive_until.wrapping_add(trigger.len());
                        trigger_found = true;
                        break;
                    }
                }
                if !trigger_found {
                    is_parsing_accidentals = false;
                }
            }
            if !is_parsing_accidentals {
                if !text[c].value.chars().next().unwrap().is_numeric() {
                    ret = None;
                    let mut cache = NOTE_PARSE_CACHE.lock().unwrap();
                    cache.insert(text.data.clone(), ret);
                    return None;


                } else {
                    degree.push_str(&text[c].value);
                }
            }
        }

        ret = Some((accidentals, degree));
        let mut cache = NOTE_PARSE_CACHE.lock().unwrap();
        cache.insert(text.data.clone(), ret.clone());
        ret
    }


    fn eq_note(&self, note: &Note, eq:&NoteEq) -> bool {
        eq.eq(self, note)
    }

    /// Same Note degree and same number of accidentals
    /// Using double accidentals or unicode/ascii doesn't affect this.
    fn is_equivalent(&self,note: &Note) -> bool {
        self.degree == note.degree &&
            Accidentals::get_dist((&self.accidentals.data).as_ref()) == Accidentals::get_dist(&note.accidentals.data)
    }

    fn is_same_pitch_class(&self, other: &Note) -> bool {
        self.to_pitch_class() == other.to_pitch_class()
    }

    fn in_first_octave(&self) -> Note {
        // memoize
        {
            let cache = NOTE_IN_FIRST_OCTAVE_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&self.text()) {
                return v.clone();
            }
        }
        let mut degree = self.degree.data.parse::<u8>().expect(format!("degree not a u8 {:?}",self).as_str());
        degree = (degree - 1 ) % 7 + 1;
        let text = self.accidentals.as_ref().to_owned() + &*degree.to_string();
        let note = Note::new(text).unwrap();
        // memoize
        let mut cache = NOTE_IN_FIRST_OCTAVE_CACHE.lock().unwrap();
        cache.insert(self.text(), note.clone());
        note
    }

    pub fn accidentals_dist(&self) -> Fret {
        Accidentals::get_dist(&*self.accidentals.data)
    }
    pub fn degree_to_fret<N: Into<Fret>>(degree:N) -> Fret {
        let degree = degree.into();
        let octave = (degree - 1) / 7; // floor
        let degree_first_octave = (degree - 1) % 7 + 1;
        (octave * 12 + Self::scale_degree_to_fret_first_octave(degree_first_octave).unwrap()) as Fret
    }

    pub fn to_pitch_class(&self) -> Fret {
        // memoize
        {
            let cache = NOTE_PITCH_CLASS_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&self.text()) {
                return *v;
            }
        }
        let ret = self.to_fret().in_first_octave();
        // memoize
        let mut cache = NOTE_PITCH_CLASS_CACHE.lock().unwrap();
        cache.insert(self.text(), ret);
        ret
    }

    pub fn from_fret_single_flats<F: Into<Fret>>(fret: F) -> Note {
        let fret = fret.into();
        let octave = fret / 12;
        let first_octave_fret = fret % 12;
        let first_octave_degree  = Note::fret_to_scale_degree_single_flats(first_octave_fret).unwrap();
        let accidental_dist = first_octave_fret - Self::scale_degree_to_fret_first_octave(first_octave_degree).unwrap() ;
        Note {
            accidentals: Accidentals::from_dist(accidental_dist,false,false).into(),
            degree: EzStr::from((first_octave_degree + 7 * octave).to_string())
        }
    }

    pub fn unicode_accidentals(&self, using_double_accidentals: bool) -> String {
        concat_string!( Accidentals::replace_with_unicode(self.accidentals.data.as_str(), using_double_accidentals),
            self.degree
        )
    }

    pub fn parse_until_invalid(text: &EzStr,start:usize) -> Option<Note>{
        {
            let cache = NOTE_PARSE_UNTIL_INVALID_CACHE.lock().unwrap();
            if let Some(v) = cache.get(&(text.clone().data, start)) {
                return v.clone();
            }
        }
        let mut accidentals = String::new();
        let mut degree = String::new();
        let mut is_parsing_accidentals = true;
        let mut c_inactive_until = start;
        let mut ret:Option<Note>;
        for (c, char) in text.graphemes().iter().enumerate() {
            if c < c_inactive_until {
                continue;
            }
            if is_parsing_accidentals {
                let mut trigger_found = false;
                for trigger in Accidentals::ALL_TRIGGERS_LONGEST_FIRST.iter() {
                    let trigger = EzStr::from(*trigger);
                    let end = c + trigger.len();
                    if end < text.len() && trigger == text.slice(c, end) {
                        accidentals.push_str(&trigger.data);
                        c_inactive_until = c_inactive_until.wrapping_add(trigger.len());
                        trigger_found = true;
                        break;
                    }
                }
                if !trigger_found {
                    is_parsing_accidentals = false;
                    if c == 0 {
                        let mut cache = NOTE_PARSE_UNTIL_INVALID_CACHE.lock().unwrap();
                        cache.insert((text.data.clone(), start), None);
                        return None;
                    }
                }
            }
            if !is_parsing_accidentals {
                if !text[c].value.chars().next().unwrap().is_numeric() {
                    if degree.len() == 0 {
                        let mut cache = NOTE_PARSE_UNTIL_INVALID_CACHE.lock().unwrap();
                        cache.insert((text.data.clone(), start), None);
                        return None;
                    } else {
                        break;
                    }
                } else {
                    degree.push_str(&text[c].value);
                }
            }
        }
        ret = Some(Note{
            accidentals:accidentals.into(), degree: EzStr::from(degree)
        });
        let mut cache = NOTE_PARSE_UNTIL_INVALID_CACHE.lock().unwrap();
        cache.insert((text.data.clone(), start), ret.clone());
        ret
    }
    pub fn in_key<K: Into<Key>>(&self, key: K) -> Key {
        let key = key.into();
        let note_degree:Fret = self.degree.data.parse::<Fret>().unwrap();
        let new_key_letter_char = Key::degree_to_letter(
            (Key::letter_to_degree(key.letter.data.parse().unwrap()).unwrap()
                + note_degree - 1) % 7
        ).unwrap();
        let new_key_letter = Key::from(new_key_letter_char);
        let target_fret = (key.letter_note().frets_from_c() + self.degree_note().to_fret()).rem_euclid( 12);
        let mut acc_dist = target_fret - new_key_letter.frets_from_c() + Accidentals::get_dist(key.accidentals.as_ref()) + Accidentals::get_dist(self.accidentals.as_ref());

        let accidentals = Accidentals::from_dist(acc_dist, true, false);
        // let ret_str = (&*new_key_letter.letter).to_owned() + &*accidentals;
        let ret = Key{ accidentals: accidentals.into(), letter:new_key_letter.letter};
        // let panic_str = format!("key:{key} note:{self}\nkey.frets_from_c(): {}\nself.fret(): {}\nnew_key_letter: {}\nnew_key_letter.frets_from_c():{}\nret_str:{}\ntarget_fret:{target_fret} acc_dist: {acc_dist}",
        //                         key.frets_from_c(),
        //                         self.to_fret(),
        //                         new_key_letter,
        //                         new_key_letter.frets_from_c(),
        //                         ret,);
        //
        // if ret.is_equivalent(&Key::from("G♯♯♯")) {
        //     //panic!("{}", panic_str);
        // } else {
        //     //println!("{}", panic_str );
        // }
       ret
    }
}

#[derive(Debug)]
struct MidiNote {
    note: u8,
    velocity: u8,
    beats: f32,
}

pub(crate) struct Utility;

impl Utility {
    pub fn clean_commas_and_spaces(text:&str) -> String {
        let mut text = String::from(text.clone());

        while text.contains("  ") {
            text = (&*text.clone().replace("  ", " ")).parse().unwrap();

            text = (&*text.replace(",", "")).parse().unwrap();
        }
        text.trim();

        String::from(text)
    }

    pub fn split_outside_brackets(input: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut bracket_depth = 0;

        for c in input.chars(){
            match c {
                '(' => {
                    bracket_depth += 1;
                    current.push(c);
                },
                ')' => {
                    bracket_depth -= 1;
                    current.push(c);
                },
                ' ' if bracket_depth == 0 => {
                    if !current.is_empty() {
                        result.push(current.clone());
                        current.clear();
                    }
                },
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            result.push(current);
        }
        result
    }

    ///This will turn a string like this:
    /// "1, 2, 3,6, 8,   9,10"
    ///Into this:
    /// ["1","2","3","6","8","9","10"]
    ///
    ///```rust, ignore
    /// # pub use jazz_chord::Utility;
    ///
    /// assert_eq!(Utility.split_on_sep("1, 2, 3,6, 8,   9,10"), vec!["1","2","3","6","8","9","10"])
    /// ```
    #[inline]
    pub fn split_on_sep(s:&str) -> Vec<String>{
        let s = s.replace(",", " ");
        s.split_whitespace().map(|s| s.to_string()).collect() // Split on spaces and collect into a Vec<String>
    }
}

/// A wrapper for multiple notes.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Change {
    pub notes: Vec<Note>,
}

impl std::ops::Index<usize> for Change {
    type Output = Note;
    fn index(&self, index: usize) -> &Self::Output {
        &self.notes[index]
    }
}

impl std::ops::IndexMut<usize> for Change {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.notes[index]
    }
}

impl AsRef<[Note]> for Change {
    fn as_ref(&self) -> &[Note] {
        self.notes.as_slice()
    }
}

impl IntoIterator for Change {
    type Item = Note;
    type IntoIter = IntoIter<Note>;
    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}

impl<'a> IntoIterator for &'a Change {
    type Item = &'a Note;
    type IntoIter = slice::Iter<'a, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.iter()
    }
}
impl<'a> IntoIterator for &'a mut Change {
    type Item = &'a mut Note;
    type IntoIter = slice::IterMut<'a, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.iter_mut()
    }
}

impl<'a> FromIterator<&'a Note> for Change {
    fn from_iter<T: IntoIterator<Item=&'a Note>>(iter: T) -> Self {
        let mut c = Change::new();
        for i in iter {
            c.push_note(i);
        }
        c
    }
}

// I added this?
impl FromIterator<Note> for Change {
    fn from_iter<T: IntoIterator<Item=Note>>(iter: T) -> Self {
        let mut c = Change::new();
        for i in iter {
            c.push_note(&i);
        }
        c
    }
}

impl<'a> Extend<&'a Note> for Change {
    fn extend<I: IntoIterator<Item = &'a Note>>(&mut self, iter: I) {
        for elem in iter {
            self.push_note(elem); // push_note takes &Note
        }
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",Accidentals::replace_with_unicode(&*self.join(" "), false))
    }
}

impl NGram for Change {

}

static CHANGE_TO_ALL_CHORD_QUALITIES: Lazy<Mutex<HashMap<Bitmap<12>, Quality>>> = Lazy::new(|| Mutex::new(HashMap::with_capacity(4096)));

impl Change {
    pub const fn new() -> Self {
        Change {notes: Vec::new()}
    }

    pub fn from_notes(notes: Vec<Note>) -> Self {
        Change { notes }
    }

    pub fn from_changes(changes: &[&Change]) -> Self {
        let mut change = changes[0].clone();
        for other in changes.iter().skip(1){
            change.push_change(other);
        }
        change
    }

    pub fn from_note_strings(note_strings: &[&str]) -> Self {
        Change::from_notes(
            note_strings.into_iter().map(|note| Note::from(*note)).collect()
        )
    }

    pub fn from_notes_string(notes_string: &str) -> Self {
        Change::from_note_strings((&Utility::clean_commas_and_spaces(notes_string).as_str().split_whitespace().collect::<Vec<_>>()).as_ref())

    }

    pub fn eq_change(&self, other: &Change, change_eq: &ChangeEq) -> bool {
        change_eq.eq(self, other)
    }

    /// Only cares if the pitch classes are the same. So, "1 3 5 7 9" and "1 2 3 5 7" are the same
    /// Will deduplicate.
    pub fn is_same_pitchclass_of_change(&self, other: &Change) -> bool {
        let mut a = self.to_frets().in_first_octave_sorted();
        let mut b = other.to_frets().in_first_octave_sorted();
        a.dedup();
        b.dedup();
        a == b
    }

    pub fn from_note(note: Note) -> Self {
        Change {notes: vec![note]}
    }
    pub fn first_note_with_degree<I: Into<i32>>(&self,degree:I) -> Option<usize>{
        let degree = degree.into();
        for (n,note) in self.into_iter().enumerate(){
            if note.degree == EzStr::new(degree.to_string()) {
                return Some(n)
            }
        }
        None
    }
    pub fn from_frets<F: IntoIterator<Item = Fret>>(frets:F) -> Change {
        let mut notes = Vec::new();
        for fret in frets {
            notes.push(Note::from_fret_single_flats(fret));
        }
        Change{notes}
    }

    pub fn to_frets(&self) -> Frets {
        self.notes.iter().map(|i|i.to_fret()).collect()
    }

    pub fn join(&self,sep:&str) -> String {
        if self.len() == 0 {
            return String::new();
        }
        let mut ret = String::new();
        let notes = self.clone().notes;
        for note in notes[0..self.notes.len() - 1].iter(){
            ret += &*note.text();
            ret += sep.as_ref();
        }
        ret += &notes[self.len() - 1].text();
        ret
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn indices_of_note(&self, note:&Note, compare:&NoteEq) -> Option<Vec<usize>> {
        let mut indices = vec![];
        for (n,self_note) in self.notes.iter().enumerate(){
            if compare.eq(&note, self_note){
                indices.push(n);
            }
        }
        if !indices.is_empty(){
            return Some(indices);
        }
        None
    }

    pub fn index_of_note(&self, note:&Note, compare:&NoteEq) -> Option<usize> {
        for (n,self_note) in self.notes.iter().enumerate(){
            if compare.eq(&note, self_note){
                return Some(n);
            }
        }
        None
    }

    pub fn first_matching_note(&self, note:&Note, compare:&NoteEq) -> Option<&Note> {
        for self_note in self {
            if compare.eq(&self_note, note){
                return Some(self_note)
            }
        }
        None
    }

    pub fn without_notes(&self, notes: &Change, eq: &NoteEq) -> Self {
        let mut new = self.clone();
        for self_note in self {
            if !notes.contains_note(&self_note, eq){
                new.push_note(&self_note);
            }
        }
        new
    }

    /// Returns new notes without the note
    /// ```
    /// use jazz_chord::{Change, Note, NoteEq};
    /// assert!(Change::from_notes_string("9 b9").without_note(&Note::new("9").unwrap(), &NoteEq::Exact) == Change::from_notes_string("b9"));
    /// // panic!("Change.without_note: {}", Change::from_notes_string("9 b9").without_note(&Note::new("9").unwrap(), &NoteEq::PitchClass))
    /// ```
    pub fn without_note(&self, note: &Note, eq: &NoteEq) -> Self {
        Change::from_notes(
        self.notes.clone().into_iter()
            .filter(|self_note|
                !self_note.eq_note(note, eq)
            ).collect()
        )
    }

    pub fn remove(&mut self, n:usize) {
        self.notes.remove(n);
    }

    pub fn contains<N: Into<Note>>(&self,note:N) -> bool {
        return self.notes.contains(&note.into());
    }

    pub fn contains_note(&self, other: &Note, eq: &NoteEq) -> bool {
        for note in self {
            if eq.eq(&note, other) {
                return true
            }
        }
        false
    }

    pub fn contains_notes(&self, notes: &Change, note_match: &NoteMatch) -> bool {
        note_match.contains_notes(self, notes)
    }

    pub fn contained_notes(&self, notes: &Change, note_eq: &NoteEq){
        NoteMatch::All(note_eq.clone()).contained_notes(self, notes);
    }

    pub fn dedup(&self, note_eq: &NoteEq) -> Change {
        let mut notes = Change::new();
        for (n,note) in self.into_iter().enumerate() {
            let indices = self.indices_of_note(note, note_eq);
            if let Some(indices) = indices {
                match indices.len() {
                    0 => panic!("Not found!"),
                    1 => notes.push_note(&self[indices[0]]),
                    _ => {
                        if n == indices[0] {
                            notes.push_note(&self[indices[0]]);
                        }
                    }
                }
            }
        }
        notes
    }

    /// This will compare the degree and number of accidentals
    /// returns true if those are equal. This way:
    /// x5 == ##5, b5 == ♭5,
    /// but 13 != 6, b3 != #2
    pub fn contains_spelling_of_note(&self, note: &Note) -> bool {
        for self_note in &self.notes{
            if self_note.is_equivalent(&note) {
                return true
            }
        }
        false
    }

    /// Uses octave equivalence. This will return that bb3 == 2 == 9
    /// Coerces strings into Notes
    pub fn contains_pitch_class<N: Into<Note> + Clone>(&self,note:N) -> bool {
        self.contains_pitch_class_of_note(&note.into())
    }

    /// Uses octave equivalence. This will return that bb3 == 2 == 9
    pub fn contains_pitch_class_of_note(&self,note:&Note) -> bool {
        let other = note.to_pitch_class();
        for self_note in self.into_iter(){
            if self_note.to_pitch_class() == other {
                return true
            }
        }
        false
    }

    /// This will match only if the data is exactly the same. Use the other contains methods
    pub fn contains_strict_note(&self,note: &Note) -> bool {
        self.notes.contains(&note)
    }

    pub fn position<N: Into<Note> + Clone>(&self,note:N) -> Option<usize> {
        return self.notes.iter().position(|n| *n == note.clone().into());
    }


    pub fn in_first_octave(&self) -> Change {
        Change {
            notes: match self.notes.len() {
                0 => Vec::new(),
                _ => self.clone().notes.iter().map(|note|
                    note.in_first_octave()).collect(),
            }
        }
    }

    pub fn sorted_by_dist(&self) -> Change {
        let mut notes = self.clone().notes;
        notes.sort_by(|a,b| a.to_fret().cmp(&b.to_fret()));
        Change {
            notes
        }
    }

    pub fn in_first_octave_sorted(&self) -> Change {
        Change {
            notes: match self.notes.len(){
                0 => Vec::new(),
                _ => self.in_first_octave().sorted_by_dist().notes.clone(),
            },
        }
    }

    pub fn in_key(&self,key:&Key) -> Keys {
        Keys {
            keys: self.notes.iter().map( | note|note.in_key(key)).collect()
        }
    }

    pub fn contains_enharmonic(&self, other: impl Into<Note>) -> bool{
        // let st = note.into().to_fret();
        let other = other.into();
        for note in &self.notes{
            // if note.to_fret() == st {
            if note.is_same_pitch_class(&other){
                return true
            }
        }
        false
    }

    pub fn to_single_accidentals(&self, use_flats: bool) -> Change {
        let notes = self.notes.clone().into_iter().map(|note| {
            note.to_single_accidentals(use_flats)
        }).collect::<Vec<Note>>();
        Change::from_notes(notes)
    }

    pub fn straighten_enharmonics(&self, allowed_notes: &Change) -> Change {
        // let allowed_notes:Change = allowed_notes.into();
        let mut straightened_notes = self.clone();
        let mut note_matrix:Vec<Vec<Note>> = Vec::with_capacity(self.len());
        let mut note_freedoms:Vec<usize> = Vec::with_capacity(self.len());
        for (n,note) in self.notes.iter().enumerate() {
                note_matrix.push(Vec::new());
            for allowed_note in allowed_notes {
                if note.to_fret().in_first_octave() == allowed_note.to_fret() {
                    note_matrix.last_mut().unwrap().push(allowed_note.clone());
                }
            }
            if note_matrix.last().unwrap().len() > 1 {
                note_freedoms.push(n);
            }

        }

        // Get rid of invalid notes. E.g replace bb3 with 2 when in Western
        for (n,note) in self.notes.iter().enumerate() {
            if !allowed_notes.contains_spelling_of_note(&note.in_first_octave()){
                let mut potential_replacements:Vec<Note> = Vec::new();
                for replacement in &note_matrix[n] {
                    if replacement.to_fret() == note.in_first_octave().to_fret() {
                        potential_replacements.push(replacement.clone());
                    }
                }
                // TODO Just Take the first one????
                // TODO This will pick the wrong octave if it's not in the first octave
               straightened_notes[n] = potential_replacements[0].clone();
            }
        }
        // panic!("note_matrix:\n{:#?}\nnote_freedoms:\n{:?}\nallowed_notes:\n{}\nself: {}\nstraightened: {}",
        //        note_matrix, note_freedoms, allowed_notes, self, straightened_notes);

        straightened_notes

        // Normalize: convert every note to an allowed enharmonic spelling that minimizes accidentals (subject to allowedNotes).
        // Sort: order notes by semitone.
        // Compute generic intervals for the sorted notes.
        // While generic intervals are not all distinct (or until no change):
        // For each duplicate or clump, attempt the smallest local change that resolves the conflict:
        // If a note can be moved up one generic degree without leaving allowedNotes and without creating a new conflict, do it.
        // Else if a note can be moved down one generic degree similarly, do it.
        // Prefer moves that reduce total accidentals or move notes into the preferred superset (jazz/carnatic) according to flags.
        // After any change, recompute generic intervals and repeat.
        // Post-process: attempt to lower accidentals for contiguous runs if doing so reduces total accidentals and keeps generic intervals distinct.
        // Apply final preference rules (prefer sharps/flats for certain degrees) and return.
    }

    /// It keeps the triad notes even if triad == None
    pub fn from_triad_extension(triad: Option<&Triad>, extension: Option<&Extension>) -> Self {
        let triad_possibilities = if let Some(triad) = triad {
            DegreePossibilities::from_triad(triad)
        } else {
            DegreePossibilities::new()
        };
        let extension_possibilities = if let Some(extension) = extension {
            // println!("{}",&extension.notes());
            DegreePossibilities::from_extension(extension)

            // ChordalDegreePossibilityMatrix::all_from_change(&extension.notes())
        } else {
            DegreePossibilities::new()
        };
        let mut possibilities = extension_possibilities.clone();
        // overwrite the thirds
        for degree in Degree::TRIADIC_ASCENDING_NO_ONE.iter(){
            let triad_note = triad_possibilities.intervals.get(degree).unwrap();
            // let extension_note: &Note = &extension_matrix.intervals.get(degree).unwrap()[0];
            if !triad_note.is_empty() {
                *possibilities.intervals.get_mut(degree).unwrap() = triad_note.clone();
            }
        }
        //println!("triad: {:?}\nextension: {:?}\ntriad_matrix:\n{}\nextension_matrix:\n{}", triad, extension, triad_possibilities.short_debug(), extension_possibilities.short_debug());
        possibilities.to_change()
    }

    pub fn remove_triad(&self, triad: &Triad, note_group: &NoteMatch) -> Change {
        let mut change: Change = self.clone();
        for note in triad.notes() {
            if change.contains_note(&note, note_group.note_eq()){
                change.remove_note(&note, note_group);
            } else {
                panic!("self {} does not contain triad note {:?}", self, triad );
            }
        }
        change
    }

    pub fn remove_note(&mut self, note: &Note, note_matches: &NoteMatch){
        note_matches.remove_note(self, note);
    }

    /// Removes exact occurrences of a note. must be spelled the same
    pub fn remove_exact_note(&self, note: impl Into<Note>) -> Change {
        let note = note.into();
        let mut notes = Vec::new();

        for current_note in &self.notes {
            if current_note != &note {
                notes.push(current_note.clone());
            }
        }
        Change { notes }
    }

    pub fn remove_equivalent_of_note(&self, note: &Note) -> Change {
        let mut notes = Vec::new();
        for current_note in &self.notes {
            if current_note.is_equivalent(note) {
                notes.push(current_note.clone());
            }
        }
        Change { notes }
    }

    /// Removes all occurrences of a note
    pub fn remove_pitch_class_of_note(&self, note: &Note) -> Change {
        let mut notes = Vec::new();

        for current_note in &self.notes {
            if current_note.to_pitch_class() != note.to_pitch_class() {
                notes.push(current_note.clone());
            }
        }
        Change { notes }
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    fn push_note(&mut self, note: &Note) {
        self.notes.push(note.clone());
    }

    fn push_change(&mut self, change: &Change) {
        self.notes.extend(change.notes.clone())
    }
}
#[derive(Debug)]
struct KeyParseResult {
    accidentals: EzStr,
    letter: EzStr,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    pub accidentals: EzStr,
    pub letter: EzStr,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter,
               Accidentals::replace_with_unicode(self.accidentals.as_ref(), true))
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Key::new(s).unwrap()
    }
}

impl From <&String> for Key {
    fn from(s: &String) -> Self {
        Key::new(s).unwrap()
    }
}

impl From<char> for Key {
    fn from(c: char) -> Self {
        Key::new(&*c.to_string()).unwrap()
    }
}

impl From<&Key> for Key {
    fn from(k: &Key) -> Self {
        k.clone()
    }
}

impl From <&EzStr> for Key {
    fn from(e: &EzStr) -> Self {
        Key::new(e.as_ref()).unwrap()
    }
}


// struct FormatSettings {
//     using_unicode_accidentals: bool,
//     using_double_accidentals: bool,
// }

impl Key {
    pub fn text(&self) -> String {
        concat_string!(self.letter, self.accidentals)
    }

    pub const fn letter_to_frets_from_c(letter: char) -> Option<Fret> {
        match letter {
            'C' => Some(0),
            'D' => Some(2),
            'E' => Some(4),
            'F' => Some(5),
            'G' => Some(7),
            'A' => Some(9),
            'B' => Some(11),
            _ => None,
        }
    }

    pub const fn letter_to_degree(letter: char) -> Option<Fret> {
        match letter {
            'C' => Some(0),
            'D' => Some(1),
            'E' => Some(2),
            'F' => Some(3),
            'G' => Some(4),
            'A' => Some(5),
            'B' => Some(6),
            _ => None,
        }
    }

    pub fn letter_note(&self) -> Key {
        Key::from(self.letter.as_ref())
    }

    pub fn is_equivalent(&self, other: &Key) -> bool {
        self.letter == other.letter &&
            Accidentals::get_dist(&self.accidentals.data) == Accidentals::get_dist(&other.accidentals.data)
    }

    pub const fn degree_to_letter(degree: Fret) -> Option<char> {
        match degree {
            0 => Some('C'),
            1 => Some('D'),
            2 => Some('E'),
            3 => Some('F'),
            4 => Some('G'),
            5 => Some('A'),
            6 => Some('B'),
            _ => None,
        }
    }

    pub fn new(data: &str) -> Option<Self> {
        let parse_result = Key::parse(data, false);
        //panic!("data: {:?}, parse_result:{:?}",data,parse_result);
        match parse_result {
            Some(x) => {
                Some(Key {
                    accidentals: x.accidentals,
                    letter: x.letter,
                })
            }
            _ => None,
        }
    }

    fn parse(data: &str, allow_malformed_end:bool) -> Option<KeyParseResult> {
        if data.is_empty(){
            return None;
        }
        let letter:char;
        let mut accidentals = String::new();
        let letter_start = 0usize;
        let accidentals_start = 1usize;
        if "ABCDEFG".contains(&data.chars().nth(0).unwrap().to_string()) {
            letter = data.chars().nth(0).unwrap()
        } else {
            return None;
        }
        let ending = &data[accidentals_start..];
        match allow_malformed_end {
            false => {
                if Accidentals::is_accidentals_str(ending) {
                    accidentals = ending.parse().unwrap();
                } else {
                    return None
                }
            },
            true => {
                let data = EzStr::from(data);
                let mut c = accidentals_start;
                while c < data.len() {
                    let mut trigger_found = false;
                    for trigger in Accidentals::ALL_TRIGGERS_LONGEST_FIRST.iter() {
                        let trigger  = EzStr::from(*trigger);
                        let trigger_len = trigger.len();
                        if trigger_len + c <= data.len() && data.slice(c, trigger_len + c) == trigger{
                            accidentals.push_str(trigger.as_ref());
                            c += trigger_len;
                            trigger_found = true;
                            break;
                        }
                    }
                    if !trigger_found {
                        break;
                    }
                }
            }
        }
        Some(KeyParseResult {
            accidentals: accidentals.into(),
            letter: letter.into(), })
    }

    /// Returns the number of frets (semitones) up from C. Always returns a value from 0-11
    pub fn frets_from_c(&self) -> Fret{
        let letter_dist_result = Key::letter_to_frets_from_c(self.letter.data.chars().nth(0).unwrap());
        if letter_dist_result.is_none() {
            panic!("{:?} {:?}",self,
                   self.letter.data.chars().nth(0).unwrap().to_string());
        }
        let letter_dist = letter_dist_result.unwrap();
        let accidentals_dist = Accidentals::get_dist(&self.accidentals.data);
        //panic!("NOW {} {}",letter_dist,accidentals_dist);
        (letter_dist + accidentals_dist).rem_euclid( 12)
    }

    pub fn note_of_key(&self, other: &Key) -> Note {
        let self_degree = Self::letter_to_degree(self.letter.data.chars().nth(0).unwrap()).unwrap();
        let other_degree = Self::letter_to_degree(other.letter.data.chars().nth(0).unwrap()).unwrap();
        let degree_dist = (other_degree - self_degree).rem_euclid(7) as Fret;
        let note_degree = Note::new(degree_dist + 1).unwrap();
        let keys_dist = (other.frets_from_c() - self.frets_from_c()).rem_euclid(12);
        // let acc_dist = other.frets_from_c() - key_degree.frets_from_c() - Accidentals::get_dist(&other.accidentals.data) ;
        let acc_dist = keys_dist - note_degree.to_pitch_class();
        // let acc_dist = other.frets_from_c() - degree_key.frets_from_c() - Accidentals::get_dist(&self.accidentals.data) ;
        let note =         Note {
            accidentals: Accidentals::from_dist(acc_dist, true, true).into() ,
            degree: (degree_dist + 1).into()
        };
        // if other.is_equivalent(&Key::new("E𝄫").unwrap()) {
        //     panic!("yo self: {} other: {}\nself_degree: {} other_degree: {}\n degree_dist: {}\nkey_degree: {} acc_dist: {}\nnote: {} keys_dist: {}", self, other, self_degree, other_degree, degree_dist, key_degree, acc_dist, note, keys_dist);
        //     //degree_key.accidentals = Accidentals::from_dist(acc_dist, true, true).into();
        // }
        note
    }

    pub fn key_of_note(&self, note: &Note) -> Key {
        note.in_key(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keys {
    keys: Vec<Key>,
}

impl Keys {
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
    pub fn is_equivalent(&self, other: &Keys) -> bool {
        if self.keys.len() == 0 {
            if other.keys.len() == 0{
                return true;
            }
            return false;
        }
        else if other.keys.len() == 0 {
            return false;
        }
        for k in 0..self.keys.len() {
            if !self.keys[k].is_equivalent(&other.keys[k]) {
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn frets_from_c(&self) -> Vec<Fret>{
        self.keys.iter().map(|k| k.frets_from_c()).collect()
    }

    pub fn to_change(&self, root: &Key) -> Change {
        let change: Change = self.into_iter().map(|key| root.note_of_key(key)).collect();
        change
    }

    // pub fn to_key_slash_chords() -> KeyQuality {
    //
    // }
}


impl Display for Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               &self.keys.iter().map(|key|
                key.to_string()
               ).collect::<Vec<_>>().join(" "))
    }
}
impl Extend<Key> for Keys {
    fn extend<T: IntoIterator<Item=Key>>(&mut self, iter: T) {
        self.keys.extend(iter);
    }
}

impl Into<Vec<Key>> for Keys {
    fn into(self) -> Vec<Key> {
        self.keys
    }
}

impl IntoIterator for Keys {
    type Item = Key;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}

impl<'a> IntoIterator for &'a Keys {
    type Item = &'a Key;
    type IntoIter = slice::Iter<'a, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter()
    }
}

impl<'a> IntoIterator for &'a mut Keys {
    type Item = &'a mut Key;
    type IntoIter = slice::IterMut<'a, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter_mut()
    }
}

impl Index<usize> for Keys {
    type Output = Key;
    fn index(&self, index: usize) -> &Self::Output {
        &self.keys[index]
    }
}

impl Index<Range<usize>> for Keys {
    type Output = [Key];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.keys.as_slice()[index]
    }
}

impl Index<RangeFrom<usize>> for Keys {
    type Output = [Key];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {

        &self.keys.as_slice()[index]
    }
}

impl Index<RangeTo<usize>> for Keys {
    type Output = [Key];
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {

        &self.keys.as_slice()[index]
    }
}

impl IndexMut<usize> for Keys {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.keys[index]
    }
}

impl Borrow<Vec<Key>> for Keys {
    fn borrow(&self) -> &Vec<Key> {
        self.keys.borrow()
    }
}

impl Borrow<Vec<Key>> for &Keys {
    fn borrow(&self) -> &Vec<Key> {
        self.keys.borrow()
    }
}

impl From<String> for Keys {
    fn from(text: String) -> Self {
        let mut data = Utility::clean_commas_and_spaces(&text);
        let note_strs = text.split(' ').collect::<Vec<&str>>();
        Self {keys: note_strs.into_iter().map(|arg: &str| Key::new(arg).unwrap()).collect()}
    }
}

impl From<&str> for Keys {
    fn from(text: &str) -> Self {
        let mut data = Utility::clean_commas_and_spaces(&text);
        if data.len() == 0 {
            return Self {keys: Vec::new()}
        }
        let note_strs = data.split(' ').collect::<Vec<&str>>();
        Self {keys: note_strs.into_iter().map(|arg: &str|
            Key::new(arg).expect(format!("Hey, {} is not a valid Key",arg).as_str())).collect()}
    }
}

impl From<&[Key]> for Keys {
    fn from(data: &[Key]) -> Self {
        Keys {
            keys: data.to_vec()
        }
    }
}

impl From<&Key> for Keys {
    fn from(value: &Key) -> Self {
        Keys {
            keys: vec![value.clone()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Brings private items into scope
    #[test]
    fn test_change_matches() {
        let change = Change::from_notes_string("1 2 #2 b3 #4 b5");
        let flat_three = Change::from_notes_string("b3");
        let target = &flat_three;
        let note_match = NoteMatch::All(NoteEq::Equivalent);
        let result = note_match.contained_notes(&change, &target);
        assert!(
            result
            ==
                flat_three,
            "{note_match:?} {change} contains? {target} == {result}"
        );

        assert!(
            NoteMatch::All(NoteEq::PitchClass).contained_notes(
                &change, &flat_three)
                .eq_change(
                    &Change::from_notes_string("b3 #2"),
                    &ChangeEq::Unordered(NoteEq::Equivalent)
                ),
            "{}", NoteMatch::All(NoteEq::PitchClass).contained_notes(
                &change, &flat_three)
        );

        assert!(change.contains_notes(
            &Change::from_notes_string("b3 #2"),
            &NoteMatch::All(NoteEq::PitchClass)))
    }
}