use std::fmt;

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
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

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

impl Change {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
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
