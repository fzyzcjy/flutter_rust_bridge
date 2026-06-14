use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::slice;
use std::vec::IntoIter;

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub enum Degree {
    Ones,
    Thirds,
    Fifths,
    Sevenths,
    Ninths,
    Elevenths,
    Thirteenths,
}

impl Degree {
    pub const TRIADIC_ASCENDING_NO_ONE: [Degree; 6] = [
        Degree::Thirds,
        Degree::Fifths,
        Degree::Sevenths,
        Degree::Ninths,
        Degree::Elevenths,
        Degree::Thirteenths,
    ];

    pub const TRIADIC_ASCENDING: [Degree; 7] = [
        Degree::Ones,
        Degree::Thirds,
        Degree::Fifths,
        Degree::Sevenths,
        Degree::Ninths,
        Degree::Elevenths,
        Degree::Thirteenths,
    ];

    pub const TRIADIC_DESCENDING_NO_ONE: [Degree; 6] = [
        Degree::Thirteenths,
        Degree::Elevenths,
        Degree::Ninths,
        Degree::Sevenths,
        Degree::Fifths,
        Degree::Thirds,
    ];

    pub const TRIADIC_DESCENDING: [Degree; 7] = [
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
        DEGREE_PREFERRED_NOTES.get(self).unwrap()
    }

    pub fn allowed_notes(&self) -> &'static Change {
        DEGREE_ALLOWED_NOTES.get(self).unwrap()
    }

    pub fn poss_exts_if_degree_at_end(&self) -> &'static [&'static Extension] {
        match self {
            Degree::Ones => &[&Extension::Unison],
            Degree::Thirds => &[
                &Extension::MinorSecond,
                &Extension::MajorSecond,
                &Extension::MinorThird,
                &Extension::MajorThird,
                &Extension::Fourth,
                &Extension::AugmentedFourth,
            ],
            Degree::Fifths => &[&Extension::Five, &Extension::Fifth],
            Degree::Sevenths => &[&Extension::Seven, &Extension::MajorSeven],
            Degree::Ninths => &[&Extension::Nine, &Extension::MajorNine],
            Degree::Elevenths => &[&Extension::Eleven, &Extension::MajorEleven],
            Degree::Thirteenths => &[
                &Extension::Thirteen,
                &Extension::MajorThirteen,
                &Extension::Six,
            ],
        }
    }

    pub fn to_natural_extension(&self) -> &'static Note {
        DEGREE_TO_NATURAL_EXTENSION.get(self).unwrap()
    }

    pub fn options_for_note(note: &Note) -> Vec<Self> {
        Degree::TRIADIC_ASCENDING_NO_ONE
            .iter()
            .copied()
            .filter(|degree| degree.allowed_notes().contains_note_text(&note.text))
            .collect()
    }

    pub fn from_note_degree(note: &Note) -> &'static Self {
        match note.degree_text() {
            "1" => &Self::Ones,
            "2" => &Self::Ninths,
            "3" => &Self::Thirds,
            "4" => &Self::Elevenths,
            "5" => &Self::Fifths,
            "6" => &Self::Thirteenths,
            "7" => &Self::Sevenths,
            _ => &Self::Ones,
        }
    }

    pub fn within_change(&self, change: &Change, stop_on_preferred_notes: bool) -> Change {
        let mut notes = Vec::new();
        for allowed_note in self.allowed_notes() {
            if change.contains_note_text(&allowed_note.text) {
                notes.push(allowed_note.clone());
                if stop_on_preferred_notes
                    && self
                        .preferred_notes()
                        .contains_note_text(&allowed_note.text)
                {
                    return Change::from_notes(notes);
                }
            }
        }
        Change::from_notes(notes)
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

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

    pub fn to_chord_string(&self, options: &QualityParams) -> String {
        match self {
            Self::Major => options.str_major_triad.clone(),
            Self::Minor => options.str_minor_triad.clone(),
            Self::Diminished => options.str_dim_triad.clone(),
            Self::HalfDiminished => options.str_half_diminished.clone(),
            Self::Augmented => options.str_aug_triad.clone(),
            Self::SusTwo => format!("{}2", options.str_sus_triad),
            Self::SusFlatTwo => {
                format!("{}{}2", options.str_sus_triad, options.accidental_text(-1))
            }
            Self::SusFour => format!("{}4", options.str_sus_triad),
            Self::SusSharpFour => {
                format!("{}{}4", options.str_sus_triad, options.accidental_text(1))
            }
        }
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

    pub fn possibilities(&self) -> &'static DegreePossibilities {
        EXTENSION_TO_POSSIBILITIES.get(self).unwrap()
    }

    pub fn extension_number(&self) -> Option<String> {
        self.extension_number_with_options(&QualityParams::default())
    }

    pub fn extension_number_with_options(&self, options: &QualityParams) -> Option<String> {
        match self {
            extension if [Self::Thirteen, Self::MajorThirteen].contains(extension) => {
                Some("13".to_owned())
            }
            extension if [Self::Eleven, Self::MajorEleven].contains(extension) => {
                Some("11".to_owned())
            }
            extension if [Self::Nine, Self::MajorNine].contains(extension) => Some("9".to_owned()),
            extension if [Self::Seven, Self::MajorSeven].contains(extension) => {
                Some("7".to_owned())
            }
            extension if [Self::Fifth, Self::Five].contains(extension) => Some("5".to_owned()),
            Self::Six => Some("6".to_owned()),
            Self::SixAddNine => Some(options.str_six_nines.clone()),
            _ => Some(format!("{}oopsiedaisy", self)),
        }
    }

    pub fn to_chord_string(&self, options: &QualityParams) -> String {
        let mut chord_string = String::new();
        if self.contains_major_token() {
            chord_string.push_str(&options.str_major_triad);
        } else if self.contains_minor_token() {
            chord_string.push_str(&options.str_minor_triad);
        }
        if let Some(extension_number) = self.extension_number() {
            chord_string.push_str(&extension_number);
        }
        chord_string
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

static DEGREE_PREFERRED_NOTES: Lazy<HashMap<Degree, Change>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Degree::Ones, Change::from_note_strings(&["1"]));
    map.insert(Degree::Thirds, Change::from_note_strings(&["3", "b3"]));
    map.insert(Degree::Fifths, Change::from_note_strings(&["5"]));
    map.insert(
        Degree::Sevenths,
        Change::from_note_strings(&["7", "b7", "bb7"]),
    );
    map.insert(Degree::Ninths, Change::from_note_strings(&["9"]));
    map.insert(Degree::Elevenths, Change::from_note_strings(&["11"]));
    map.insert(Degree::Thirteenths, Change::from_note_strings(&["13", "6"]));
    map
});

static DEGREE_ALLOWED_NOTES: Lazy<HashMap<Degree, Change>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Degree::Ones, Change::from_note_strings(&["1", "b1", "#1"]));
    map.insert(
        Degree::Thirds,
        Change::from_note_strings(&["3", "b3", "4", "2", "#4", "b2"]),
    );
    map.insert(
        Degree::Fifths,
        Change::from_note_strings(&["5", "b5", "#5"]),
    );
    map.insert(Degree::Sevenths, Change::from_note_strings(&["7", "b7"]));
    map.insert(
        Degree::Ninths,
        Change::from_note_strings(&["9", "b9", "#9"]),
    );
    map.insert(Degree::Elevenths, Change::from_note_strings(&["11", "#11"]));
    map.insert(
        Degree::Thirteenths,
        Change::from_note_strings(&["13", "b13", "#13"]),
    );
    map
});

static DEGREE_TO_NATURAL_EXTENSION: Lazy<HashMap<Degree, Note>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Degree::Ones, Note::new("1".to_owned()));
    map.insert(Degree::Thirds, Note::new("3".to_owned()));
    map.insert(Degree::Fifths, Note::new("5".to_owned()));
    map.insert(Degree::Sevenths, Note::new("7".to_owned()));
    map.insert(Degree::Ninths, Note::new("9".to_owned()));
    map.insert(Degree::Elevenths, Note::new("11".to_owned()));
    map.insert(Degree::Thirteenths, Note::new("13".to_owned()));
    map
});

static EXTENSION_TO_POSSIBILITIES: Lazy<HashMap<Extension, DegreePossibilities>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        for extension in Extension::ALL {
            map.insert(
                (*extension).clone(),
                DegreePossibilities::all_from_change_no_one(extension.notes()),
            );
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

#[derive(Clone, Debug)]
pub struct DegreePossibilities {
    pub intervals: HashMap<Degree, Change>,
}

impl DegreePossibilities {
    pub fn new() -> Self {
        let mut intervals = HashMap::new();
        for degree in Degree::TRIADIC_ASCENDING {
            intervals.insert(degree, Change::new());
        }
        Self { intervals }
    }

    pub fn to_change(&self) -> Change {
        let mut notes = Change::new();
        for interval_change in self.intervals.values() {
            notes.extend(interval_change);
        }
        notes
    }

    pub fn contains_notes_in_degree(&self, degree: &Degree) -> bool {
        self.intervals.get(degree).unwrap().len() > 0
    }

    pub fn contains_note_in_degree(&self, note: &Note, degree: &Degree, eq: &NoteEq) -> bool {
        self.intervals.get(degree).unwrap().contains_note(note, eq)
    }

    pub fn is_not_empty(&self) -> bool {
        for degree in self.intervals.keys() {
            if self.contains_notes_in_degree(degree) {
                return true;
            }
        }
        false
    }

    pub fn contains_note(&self, note: &Note, eq: &NoteEq) -> bool {
        for change in self.intervals.values() {
            if change.contains_note(note, eq) {
                return true;
            }
        }
        false
    }

    pub fn from_triad(triad: &Triad) -> Self {
        let mut matrix = DegreePossibilities::new();
        *matrix.intervals.get_mut(&Degree::Ones).unwrap() =
            Change::from_note(triad.notes()[0].clone());
        *matrix.intervals.get_mut(&Degree::Thirds).unwrap() =
            Change::from_note(triad.notes()[1].clone());
        *matrix.intervals.get_mut(&Degree::Fifths).unwrap() =
            Change::from_note(triad.notes()[2].clone());
        matrix
    }

    pub fn from_extension(extension: &Extension) -> Self {
        let mut matrix = DegreePossibilities::new();
        for note in extension.notes() {
            let degree = Degree::from_note_degree(note);
            *matrix.intervals.get_mut(degree).unwrap() = Change::from_note(note.clone());
        }
        matrix
    }

    fn all_from_change_no_one(change: &Change) -> Self {
        let mut result = DegreePossibilities::new();
        for scale_degree in Degree::TRIADIC_ASCENDING_NO_ONE {
            let possibilities = scale_degree.within_change(change, false);
            *result.get_mut(&scale_degree) = possibilities;
        }
        result
    }

    fn all_from_change(change: &Change) -> Self {
        let mut result = DegreePossibilities::new();
        for scale_degree in Degree::TRIADIC_ASCENDING_NO_ONE {
            let possibilities = scale_degree.within_change(change, false);
            *result.get_mut(&scale_degree) = possibilities;
        }

        let the_one = Note::new("1".to_owned());
        if change.contains_note(&the_one, &NoteEq::Equivalent) {
            *result.get_mut(&Degree::Ones) = Change::from_note(the_one);
        }

        result
    }

    fn degrees_containing_notes(&self) -> Vec<Degree> {
        self.intervals
            .keys()
            .filter(|degree| self.intervals.get(degree).unwrap().len() > 0)
            .copied()
            .collect()
    }

    pub fn from_triad_extension(triad_extension: &TriadExtension) -> Self {
        let triad = triad_extension.triad.as_ref();
        let extension = triad_extension.extension.as_ref();
        let triad_matrix = if let Some(triad) = triad {
            DegreePossibilities::from_triad(triad)
        } else {
            DegreePossibilities::new()
        };
        let extension_possibilities = if let Some(extension) = extension {
            DegreePossibilities::from_extension(extension)
        } else {
            DegreePossibilities::new()
        };
        let mut possibilities = extension_possibilities.clone();

        for degree in Degree::TRIADIC_ASCENDING_NO_ONE {
            let triad_note = triad_matrix.intervals.get(&degree).unwrap();
            if !triad_note.is_empty() {
                *possibilities.intervals.get_mut(&degree).unwrap() = triad_note.clone();
            }
        }

        let is_dim = triad == Some(&Triad::Diminished);
        let sevenths = possibilities.intervals.get(&Degree::Sevenths).unwrap();
        let has_flat_seven = !sevenths.is_empty()
            && sevenths.contains_note(&Note::new("b7".to_owned()), &NoteEq::Equivalent);
        if is_dim && has_flat_seven {
            *possibilities.get_mut(&Degree::Sevenths) = Change::from_note_strings(&["bb7"]);
        }

        let one = Note::new("1".to_owned());
        if extension != Some(&Extension::NoChord)
            && !possibilities.contains_note(&one, &NoteEq::Equivalent)
        {
            possibilities
                .intervals
                .insert(Degree::Ones, Change::from_note(one));
        }
        possibilities
    }

    pub fn poss_triad_extensions(&self) -> Vec<TriadExtension> {
        let mut triad_extensions = Vec::new();
        let possibilities = self.pick_ideal_notes();
        let possible_triads = possibilities.possible_triads();
        let possible_extensions = possibilities.possible_extensions();

        for triad in possible_triads {
            for extension in &possible_extensions {
                let triad_extension =
                    TriadExtension::from(&Some(&triad), &Some(&extension.clone()));
                if triad_extension.is_legal() {
                    triad_extensions.push(triad_extension);
                }
            }
            triad_extensions.push(TriadExtension::from(&Some(&triad), &None));
        }

        triad_extensions
    }

    pub fn possible_extensions_option(&self) -> Option<Vec<Extension>> {
        let possible = self.possible_extensions();
        (!possible.is_empty()).then_some(possible)
    }

    pub fn possible_extensions(&self) -> Vec<Extension> {
        let mut possible_extensions = Vec::new();
        for degree in self.intervals.keys() {
            for note in self.intervals.get(degree).unwrap() {
                if degree.preferred_notes().contains_note_text(&note.text) {
                    for extension in degree.poss_exts_if_degree_at_end() {
                        if !possible_extensions.contains(*extension) {
                            possible_extensions.push((**extension).clone());
                        }
                    }
                }
            }
        }
        possible_extensions
    }

    pub fn possible_triads_option(&self) -> Option<Vec<Triad>> {
        let possible_triads = self.possible_triads();
        (!possible_triads.is_empty()).then_some(possible_triads)
    }

    pub fn possible_triads(&self) -> Vec<Triad> {
        let mut possible_triads = Vec::new();
        for triad in Triad::ALL {
            let mut matches_at_least_one_degree = false;
            for degree in Degree::TRIADIC_ASCENDING_NO_ONE {
                if self.contains_notes_in_degree(&degree) {
                    let degree_change = self.intervals.get(&degree).unwrap();
                    if degree_change
                        .notes
                        .iter()
                        .any(|degree_note| triad.notes().contains_pitch_class_of_note(degree_note))
                    {
                        matches_at_least_one_degree = true;
                    }
                }
            }
            if matches_at_least_one_degree {
                possible_triads.push((*triad).clone());
            }
        }
        possible_triads
    }

    fn remove_note_from_other_degrees(&mut self, note: &Note, degree: &Degree) {
        for other_degree in Degree::TRIADIC_ASCENDING
            .iter()
            .filter(|other_degree| *other_degree != degree)
        {
            let other_degree_change = self.intervals.get(other_degree).unwrap().clone();
            for other_degree_note in &other_degree_change {
                if other_degree_note.eq_note(note, &NoteEq::PitchClass) {
                    *self.intervals.get_mut(other_degree).unwrap() = self
                        .intervals
                        .get(other_degree)
                        .unwrap()
                        .remove_pitch_class_of_note(other_degree_note);
                }
            }
        }
    }

    fn remove_note_from_all_degrees(&mut self, note: &Note) {
        for degree in Degree::TRIADIC_ASCENDING {
            *self.get_mut(&degree) = self.get(&degree).without_note(note, &NoteEq::PitchClass);
        }
    }

    fn pick_note_for_degree(&mut self, note: &Note, degree: &Degree) {
        let this_degree_change = self.get(degree).clone();
        for this_degree_note in &this_degree_change {
            if !this_degree_note.is_same_pitch_class(note) {
                *self.intervals.get_mut(degree).unwrap() = self
                    .get(degree)
                    .clone()
                    .remove_pitch_class_of_note(this_degree_note);
            }
        }

        self.remove_note_from_other_degrees(note, degree);

        if !this_degree_change.contains_note(note, &NoteEq::PitchClass) {
            let allowed_note = degree
                .allowed_notes()
                .index_of_note(note, &NoteEq::PitchClass);
            if let Some(allowed_note) = allowed_note {
                self.intervals
                    .get_mut(degree)
                    .unwrap()
                    .push_note(&degree.allowed_notes()[allowed_note]);
            }
        }
    }

    fn without(&self, other: &Self) -> Self {
        let mut new = self.clone();
        for (degree, other_change) in &other.intervals {
            let mut new_change = self.intervals.get(degree).unwrap().clone();
            for other_note in other_change {
                if new_change.contains_pitch_class_of_note(other_note) {
                    new_change = new_change.remove_pitch_class_of_note(other_note);
                    new.remove_note_from_other_degrees(other_note, degree);
                }
            }
            *new.intervals.get_mut(degree).unwrap() = new_change;
        }
        new
    }

    fn remove_triad(&self, triad: &Triad) -> Self {
        let mut matrix = self.clone();
        for note in triad.notes() {
            for degree in [Degree::Thirds, Degree::Fifths] {
                *matrix.intervals.get_mut(&degree).unwrap() = matrix
                    .intervals
                    .get(&degree)
                    .unwrap()
                    .remove_pitch_class_of_note(note);
            }
        }
        matrix
    }

    fn clear_degree(&mut self, degree: &Degree) {
        *self.get_mut(degree) = Change::new();
    }

    fn pick_ideal_notes(&self) -> Self {
        let mut ideal = self.clone();
        for degree in Degree::TRIADIC_ASCENDING_NO_ONE {
            let degree_possibilities = ideal.intervals.get(&degree).unwrap().clone();
            if degree_possibilities.is_empty() {
                continue;
            }

            let preferred_note = degree_possibilities
                .notes
                .iter()
                .find(|possibility| degree.preferred_notes().contains_pitch_class(*possibility))
                .or_else(|| degree_possibilities.notes.first());

            if let Some(preferred_note) = preferred_note {
                ideal.pick_note_for_degree(preferred_note, &degree);
            }
        }

        for note in &self.to_change() {
            if ideal.contains_note(note, &NoteEq::PitchClass) {
                continue;
            }

            let missing_note = Change::from_note(note.clone());
            let all = DegreePossibilities::all_from_change_no_one(&missing_note);
            for degree in all
                .degrees_containing_notes()
                .into_iter()
                .filter(|degree| [Degree::Thirds, Degree::Fifths].contains(degree))
            {
                if ideal.get(&degree).is_empty() {
                    ideal.get_mut(&degree).push_change(all.get(&degree));
                    break;
                }
            }
        }

        *ideal.get_mut(&Degree::Ones) = self.get(&Degree::Ones).clone();
        ideal
    }

    pub fn get_mut(&mut self, key: &Degree) -> &mut Change {
        self.intervals.get_mut(key).unwrap()
    }

    pub fn get(&self, key: &Degree) -> &Change {
        self.intervals.get(key).unwrap()
    }

    fn short_debug(&self) -> String {
        format!(
            "{{{}}}",
            Degree::TRIADIC_ASCENDING
                .iter()
                .filter(|degree| self.contains_notes_in_degree(degree))
                .map(|degree| format!("{}: {}", degree, self.get(degree)))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for DegreePossibilities {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DegreePossibilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_debug())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChangeEq {
    PitchClass,
    Deduplicated(NoteEq),
    Equal(NoteEq),
    Unordered(NoteEq),
}

impl ChangeEq {
    pub fn eq(&self, left: &Change, right: &Change) -> bool {
        match self {
            Self::PitchClass => left.is_same_pitchclass_of_change(right),
            Self::Deduplicated(note_eq) => left
                .dedup(note_eq)
                .is_same_pitchclass_of_change(&right.dedup(note_eq)),
            Self::Equal(note_eq) => {
                if left.len() != right.len() {
                    return false;
                }
                left.notes
                    .iter()
                    .zip(right.notes.iter())
                    .all(|(left_note, right_note)| left_note.eq_note(right_note, note_eq))
            }
            Self::Unordered(note_eq) => {
                left.len() == right.len()
                    && left
                        .notes
                        .iter()
                        .all(|note| right.contains_note(note, note_eq))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NoteEq {
    Enharmonic,
    PitchClass,
    Exact,
    Equivalent,
    Degree,
    DegreeEnharmonic,
}

impl NoteEq {
    pub fn eq(&self, note: &Note, other: &Note) -> bool {
        match self {
            Self::Exact => note.text == other.text,
            Self::Equivalent => note.is_equivalent(other),
            Self::Degree => note.degree_text() == other.degree_text(),
            Self::Enharmonic | Self::PitchClass | Self::DegreeEnharmonic => {
                note.pitch_class_text() == other.pitch_class_text()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NoteMatch {
    First(NoteEq),
    All(NoteEq),
    Any(NoteEq),
}

impl NoteMatch {
    pub fn contains_notes(&self, reference: &Change, target: &Change) -> bool {
        match self {
            Self::First(note_eq) | Self::Any(note_eq) => target
                .notes
                .iter()
                .any(|right_note| reference.contains_note(right_note, note_eq)),
            Self::All(note_eq) => target
                .notes
                .iter()
                .all(|right_note| reference.contains_note(right_note, note_eq)),
        }
    }

    pub fn note_eq(&self) -> &NoteEq {
        match self {
            Self::First(note_eq) | Self::All(note_eq) | Self::Any(note_eq) => note_eq,
        }
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

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_equivalent(&self, other: &Note) -> bool {
        self.text == other.text
            || (self.degree_text() == other.degree_text()
                && self.accidental_count() == other.accidental_count())
    }

    pub fn is_same_pitch_class(&self, other: &Note) -> bool {
        self.pitch_class_text() == other.pitch_class_text()
    }

    pub fn eq_note(&self, other: &Note, eq: &NoteEq) -> bool {
        eq.eq(self, other)
    }

    fn degree_text(&self) -> &str {
        let text = self
            .text
            .trim_start_matches(['b', '#', '♭', '♯'])
            .trim_start_matches("bb")
            .trim_start_matches("##");

        match text {
            "8" | "15" => "1",
            "9" => "2",
            "10" => "3",
            "11" => "4",
            "12" => "5",
            "13" => "6",
            "14" => "7",
            _ => text,
        }
    }

    fn pitch_class_text(&self) -> &str {
        match self.degree_text() {
            "8" | "15" => "1",
            "9" => "2",
            "10" => "3",
            "11" => "4",
            "12" => "5",
            "13" => "6",
            "14" => "7",
            text => text,
        }
    }

    fn accidental_count(&self) -> i32 {
        self.text
            .chars()
            .fold(0, |count, character| match character {
                'b' | '♭' => count - 1,
                '#' | '♯' => count + 1,
                _ => count,
            })
    }
}

impl From<&str> for Note {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl From<String> for Note {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&Note> for Note {
    fn from(value: &Note) -> Self {
        value.clone()
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

    pub fn from_note(note: Note) -> Self {
        Self { notes: vec![note] }
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

    pub fn eq_change(&self, other: &Change, change_eq: &ChangeEq) -> bool {
        change_eq.eq(self, other)
    }

    pub fn from_triad_extension(triad: Option<&Triad>, extension: Option<&Extension>) -> Self {
        let triad_extension = TriadExtension::from(&triad, &extension);
        DegreePossibilities::from_triad_extension(&triad_extension).to_change()
    }

    pub fn is_same_pitchclass_of_change(&self, other: &Change) -> bool {
        self.notes
            .iter()
            .all(|note| other.contains_pitch_class(note))
            && other
                .notes
                .iter()
                .all(|note| self.contains_pitch_class(note))
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

    pub fn index_of_note(&self, note: &Note, compare: &NoteEq) -> Option<usize> {
        for (index, self_note) in self.notes.iter().enumerate() {
            if compare.eq(note, self_note) {
                return Some(index);
            }
        }
        None
    }

    pub fn first_matching_note(&self, note: &Note, compare: &NoteEq) -> Option<Note> {
        for self_note in self {
            if compare.eq(self_note, note) {
                return Some(self_note.clone());
            }
        }
        None
    }

    pub fn without_notes(&self, notes: &Change, eq: &NoteEq) -> Self {
        Change::from_notes(
            self.notes
                .iter()
                .filter(|self_note| !notes.contains_note(self_note, eq))
                .cloned()
                .collect(),
        )
    }

    pub fn without_note(&self, note: &Note, eq: &NoteEq) -> Self {
        Change::from_notes(
            self.notes
                .iter()
                .filter(|self_note| !self_note.eq_note(note, eq))
                .cloned()
                .collect(),
        )
    }

    pub fn remove(&mut self, index: usize) {
        self.notes.remove(index);
    }

    pub fn contains<N: Into<Note>>(&self, note: N) -> bool {
        self.notes.contains(&note.into())
    }

    pub fn contains_note(&self, other: &Note, eq: &NoteEq) -> bool {
        self.notes.iter().any(|note| eq.eq(note, other))
    }

    pub fn contains_notes(&self, notes: &Change, note_match: &NoteMatch) -> bool {
        note_match.contains_notes(self, notes)
    }

    pub fn contains_pitch_class<N: Into<Note>>(&self, note: N) -> bool {
        self.contains_pitch_class_of_note(&note.into())
    }

    pub fn contains_pitch_class_of_note(&self, note: &Note) -> bool {
        self.notes
            .iter()
            .any(|self_note| self_note.is_same_pitch_class(note))
    }

    pub fn contains_strict_note(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }

    pub fn contains_enharmonic<N: Into<Note>>(&self, other: N) -> bool {
        let other = other.into();
        self.notes
            .iter()
            .any(|note| note.is_same_pitch_class(&other))
    }

    pub fn dedup(&self, note_eq: &NoteEq) -> Change {
        let mut notes = Change::new();
        for note in self {
            if !notes.contains_note(note, note_eq) {
                notes.push_note(note);
            }
        }
        notes
    }

    pub fn remove_exact_note<N: Into<Note>>(&self, note: N) -> Change {
        let note = note.into();
        Change::from_notes(
            self.notes
                .iter()
                .filter(|current_note| current_note != &&note)
                .cloned()
                .collect(),
        )
    }

    pub fn remove_equivalent_of_note(&self, note: &Note) -> Change {
        Change::from_notes(
            self.notes
                .iter()
                .filter(|current_note| !current_note.is_equivalent(note))
                .cloned()
                .collect(),
        )
    }

    pub fn remove_pitch_class_of_note(&self, note: &Note) -> Change {
        Change::from_notes(
            self.notes
                .iter()
                .filter(|current_note| !current_note.is_same_pitch_class(note))
                .cloned()
                .collect(),
        )
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualityParams {
    pub use_unicode: bool,
    pub use_double_accidentals: bool,
    pub str_major_triad: String,
    pub str_minor_triad: String,
    pub str_dim_triad: String,
    pub str_sus_triad: String,
    pub str_aug_triad: String,
    pub str_half_diminished: String,
    pub str_six_nines: String,
}

impl QualityParams {
    pub fn new() -> Self {
        Self::default()
    }

    fn accidental_text(&self, distance: i32) -> &'static str {
        match (self.use_unicode, self.use_double_accidentals, distance) {
            (true, _, -1) => "♭",
            (true, true, -2) => "𝄫",
            (true, _, -2) => "♭♭",
            (true, _, 1) => "♯",
            (true, true, 2) => "𝄪",
            (true, _, 2) => "♯♯",
            (_, _, -1) => "b",
            (_, _, -2) => "bb",
            (_, _, 1) => "#",
            (_, _, 2) => "##",
            _ => "",
        }
    }
}

impl Default for QualityParams {
    fn default() -> Self {
        Self {
            use_unicode: true,
            use_double_accidentals: false,
            str_major_triad: "ma".to_owned(),
            str_minor_triad: "m".to_owned(),
            str_dim_triad: "º".to_owned(),
            str_sus_triad: "sus".to_owned(),
            str_aug_triad: "+".to_owned(),
            str_half_diminished: "ø".to_owned(),
            str_six_nines: "69".to_owned(),
        }
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
