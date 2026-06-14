use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::slice;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Major => "Major",
            Self::Minor => "Minor",
            Self::Diminished => "Diminished",
            Self::HalfDiminished => "Half Diminished",
            Self::Augmented => "Augmented",
            Self::SusTwo => "Suspended 2nd",
            Self::SusFlatTwo => "Suspended flat 2nd",
            Self::SusFour => "Suspended 4th",
            Self::SusSharpFour => "Suspended sharp 4th",
        }
    }

    pub fn is_sus(&self) -> bool {
        Self::SUS_TRIADS.contains(&self)
    }

    fn notes(&self) -> &'static Change {
        TRIAD_TO_NOTES.get(self).unwrap()
    }
}

impl fmt::Display for Triad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl Extension {
    pub const ALL: &'static [&'static Self] = &[
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

    pub const ADD_MAJ_TO_EXTENSION: &'static [&'static Self] = &[
        &Self::MajorSeventh,
        &Self::MajorSeven,
        &Self::MajorNine,
        &Self::MajorEleven,
        &Self::MajorThirteen,
        &Self::MajorSecond,
        &Self::MajorThird,
        &Self::MajorSixth,
    ];

    pub const CONTAINING_MINOR_TOKEN: &'static [&'static Self] = &[
        &Self::MinorSecond,
        &Self::MinorThird,
        &Self::MinorFifth,
        &Self::MinorSixth,
        &Self::MinorSeventh,
    ];

    pub const INTERVALS: &'static [&'static Self] = &[
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
        &Self::MajorSeventh,
    ];

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

    pub fn hides_ma_triad_str(&self, hide_major_six_triad: bool) -> bool {
        match self {
            Self::MajorThirteen
            | Self::Thirteen
            | Self::Eleven
            | Self::MajorEleven
            | Self::Nine
            | Self::MajorNine
            | Self::Seven
            | Self::MajorSeven => true,
            Self::MajorSixth => hide_major_six_triad,
            _ => false,
        }
    }

    pub fn is_dominant(&self) -> bool {
        matches!(
            self,
            Self::Seven | Self::Nine | Self::Eleven | Self::Thirteen
        )
    }

    pub fn contains_major_token(&self) -> bool {
        Self::ADD_MAJ_TO_EXTENSION.contains(&self)
    }

    pub fn contains_minor_token(&self) -> bool {
        Self::CONTAINING_MINOR_TOKEN.contains(&self)
    }

    pub fn notes(&self) -> &'static Change {
        EXTENSION_TO_NOTES.get(self).unwrap()
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

static TRIAD_TO_NOTES: Lazy<HashMap<Triad, Change>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for triad in Triad::ALL {
        let change = match triad {
            Triad::Major => Change::from_notes_string("1 3 5"),
            Triad::Minor => Change::from_notes_string("1 b3 5"),
            Triad::Diminished => Change::from_notes_string("1 b3 b5"),
            Triad::HalfDiminished => Change::from_notes_string("1 b3 b5"),
            Triad::Augmented => Change::from_notes_string("1 3 #5"),
            Triad::SusTwo => Change::from_notes_string("1 2 5"),
            Triad::SusFlatTwo => Change::from_notes_string("1 b2 5"),
            Triad::SusFour => Change::from_notes_string("1 4 5"),
            Triad::SusSharpFour => Change::from_notes_string("1 #4 5"),
        };
        map.insert((*triad).clone(), change);
    }
    map
});

static EXTENSION_TO_NOTES: Lazy<HashMap<Extension, Change>> = Lazy::new(|| {
    let mut map = HashMap::new();
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
            Extension::MajorThirteen => {
                Change::from_note_strings(&["1", "3", "5", "7", "9", "11", "13"])
            }
            Extension::Thirteen => {
                Change::from_note_strings(&["1", "3", "5", "b7", "9", "11", "13"])
            }
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
        map.insert((*extension).clone(), notes);
    }
    map
});

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct TriadExtension {
    triad: Option<Triad>,
    extension: Option<Extension>,
}

impl TriadExtension {
    pub fn from(triad: &Option<&Triad>, extension: &Option<&Extension>) -> Self {
        Self {
            triad: triad.clone().cloned(),
            extension: extension.clone().cloned(),
        }
    }

    pub fn is_legal(&self) -> bool {
        if let Some(extension) = &self.extension {
            if extension.notes().len() <= 2 {
                if self.triad.is_none()
                    && (*extension == Extension::Five || *extension == Extension::NoChord)
                {
                    return true;
                }
                return false;
            }
        }

        if let (Some(triad), Some(extension)) = (&self.triad, &self.extension) {
            if *triad == Triad::Diminished {
                if [Extension::Thirteen, Extension::SixAddNine, Extension::Six].contains(extension)
                {
                    return false;
                }
            } else if *triad == Triad::HalfDiminished {
                if !extension.notes().contains_note_text("b7") {
                    return false;
                }
            } else if *triad == Triad::SusTwo {
                if extension.notes().contains_note_text("2") {
                    return false;
                }
            } else if *triad == Triad::SusFour {
                if extension.notes().contains_note_text("4") {
                    return false;
                }
            }
        }

        true
    }

    pub fn all() -> &'static [Self] {
        &TRIAD_EXTENSIONS
    }

    pub fn all_legal() -> &'static [Self] {
        &TRIAD_EXTENSIONS_LEGAL
    }
}

impl fmt::Display for TriadExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let triad = if let Some(triad) = &self.triad {
            format!("{:?}", triad)
        } else {
            String::new()
        };
        let extension = if let Some(extension) = &self.extension {
            format!("{:?}", extension)
        } else {
            String::new()
        };
        write!(f, "{}_{}", triad, extension)
    }
}

static TRIAD_EXTENSIONS: Lazy<Vec<TriadExtension>> = Lazy::new(|| {
    let mut values = Vec::new();
    for triad in Triad::ALL {
        for extension in Extension::ALL {
            values.push(TriadExtension::from(
                &Some(&(*triad).clone()),
                &Some(&(*extension).clone()),
            ));
        }
    }
    for triad in Triad::ALL {
        values.push(TriadExtension::from(&Some(&(*triad).clone()), &None));
    }
    for extension in Extension::ALL {
        values.push(TriadExtension::from(&None, &Some(&(*extension).clone())));
    }
    values
});

static TRIAD_EXTENSIONS_LEGAL: Lazy<Vec<TriadExtension>> = Lazy::new(|| {
    TRIAD_EXTENSIONS
        .clone()
        .into_iter()
        .filter(TriadExtension::is_legal)
        .collect()
});

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    pub text: String,
}

impl Note {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    fn from_iter<T: IntoIterator<Item = &'a Note>>(iter: T) -> Self {
        let mut change = Change::new();
        for note in iter {
            change.push_note(note);
        }
        change
    }
}

impl FromIterator<Note> for Change {
    fn from_iter<T: IntoIterator<Item = Note>>(iter: T) -> Self {
        let mut change = Change::new();
        for note in iter {
            change.push_note(&note);
        }
        change
    }
}

impl<'a> Extend<&'a Note> for Change {
    fn extend<I: IntoIterator<Item = &'a Note>>(&mut self, iter: I) {
        for note in iter {
            self.push_note(note);
        }
    }
}

impl Change {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn from_notes(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    pub fn from_changes(changes: &[&Change]) -> Self {
        let mut change = changes[0].clone();
        for other in changes.iter().skip(1) {
            change.push_change(other);
        }
        change
    }

    pub fn from_note_strings(note_strings: &[&str]) -> Self {
        Self::from_notes(
            note_strings
                .iter()
                .map(|note| Note::new((*note).to_owned()))
                .collect(),
        )
    }

    pub fn from_notes_string(notes_string: &str) -> Self {
        let notes = notes_string
            .replace(',', " ")
            .split_whitespace()
            .map(|text| Note::new(text.to_owned()))
            .collect();
        Self { notes }
    }

    pub fn join(&self, separator: &str) -> String {
        self.notes
            .iter()
            .map(|note| note.text.as_str())
            .collect::<Vec<_>>()
            .join(separator)
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    fn contains_note_text(&self, text: &str) -> bool {
        self.notes.iter().any(|note| note.text == text)
    }

    fn push_note(&mut self, note: &Note) {
        self.notes.push(note.clone());
    }

    fn push_change(&mut self, change: &Change) {
        self.notes.extend(change.notes.clone());
    }
}

impl Default for Change {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.join(" "))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quality {
    pub input: String,
}

impl Quality {
    pub fn from_string(input: String) -> Option<Self> {
        (!input.trim().is_empty()).then_some(Self { input })
    }
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.input)
    }
}

pub trait ToChordQuality {
    fn to_chord_quality(&self) -> Quality;
}

impl ToChordQuality for Change {
    fn to_chord_quality(&self) -> Quality {
        Quality {
            input: self.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeySlashQuality {
    pub input: String,
}

impl KeySlashQuality {
    pub fn from_string(input: String) -> Option<Self> {
        (!input.trim().is_empty()).then_some(Self { input })
    }
}

impl fmt::Display for KeySlashQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.input)
    }
}
