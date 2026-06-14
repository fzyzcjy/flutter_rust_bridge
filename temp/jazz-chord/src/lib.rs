use std::fmt;

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
