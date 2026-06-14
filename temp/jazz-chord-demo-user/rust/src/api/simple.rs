use flutter_rust_bridge::frb;
use jazz_chord::{Change, KeySlashQuality, ToChordQuality};

#[frb(sync)]
pub fn parse_chord_string(input: String) -> String {
    KeySlashQuality::from_string(input)
        .map(|chord| chord.to_string())
        .unwrap_or_else(|| "?".to_owned())
}

#[frb(sync)]
pub fn notes_to_chord(input: String) -> String {
    Change::from_notes_string(input.as_str())
        .to_chord_quality()
        .to_string()
}
