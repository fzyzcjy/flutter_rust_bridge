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

    pub fn is_not_empty(&self) -> bool {
        for degree in self.intervals.keys() {
            if self.contains_notes_in_degree(degree) {
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
        let has_flat_seven = !sevenths.is_empty() && sevenths.contains_note_text("b7");
        if is_dim && has_flat_seven {
            *possibilities.get_mut(&Degree::Sevenths) = Change::from_note_strings(&["bb7"]);
        }

        if extension != Some(&Extension::NoChord)
            && !possibilities.to_change().contains_note_text("1")
        {
            possibilities
                .intervals
                .insert(Degree::Ones, Change::from_note(Note::new("1".to_owned())));
        }
        possibilities
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
                        .any(|degree_note| triad.notes().contains_note_text(&degree_note.text))
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
pub struct Note {
    pub text: String,
}

impl Note {
    pub fn new(text: String) -> Self {
        Self { text }
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
