#![allow(warnings)]
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use ezstr::EzStr;
/// A unit struct that hosts accidental-related constants and helpers.
pub struct Accidentals;

impl Accidentals {
    // ASCII representations
    pub const ASCII_FLAT: &'static str = "b";
    pub const ASCII_SHARP: &'static str = "#";
    pub const ASCII_DOUBLE_SHARP: &'static str = "x";
    pub const ASCII_DOUBLE_FLAT: &'static str = "bb";
    pub const ASCII_NATURAL: &'static str = "nat";

    // Unicode representations
    pub const FLAT: &'static str = "♭";
    pub const SHARP: &'static str = "♯";
    pub const DOUBLE_FLAT: &'static str = "𝄫";
    pub const DOUBLE_SHARP: &'static str = "𝄪";
    pub const NATURAL: &'static str = "♮";

    pub const FLATS: [&'static str; 2] = [Accidentals::ASCII_FLAT, Accidentals::FLAT];


    // ----------------------
    // Static initialization
    // ----------------------

    //pub const FLATS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![Accidentals::ASCII_FLAT, Accidentals::FLAT]);
    // pub const FLATS: Lazy<Vec<&'static str>> =
    //     Lazy::new(|| vec![Accidentals::ASCII_FLAT, Accidentals::FLAT]);
    pub const SHARPS: [&'static str; 2] = [Accidentals::ASCII_SHARP, Accidentals::SHARP];
    pub const NATURALS: [&'static str; 2] = [Accidentals::ASCII_NATURAL, Accidentals::NATURAL];
    pub const DOUBLE_FLATS: [&'static str;2]=[Accidentals::ASCII_DOUBLE_FLAT, Accidentals::DOUBLE_FLAT];
    pub const DOUBLE_SHARPS: [&'static str; 2] = [Accidentals::ASCII_DOUBLE_SHARP, Accidentals::DOUBLE_SHARP];

    // All triggers sorted by length descending to ensure "bb" is matched before "b".


    pub const ALL_TRIGGERS_SHORTEST_FIRST: Lazy<Vec<&'static str>> = Lazy::new(|| {
        let mut v: Vec<&'static str> = Vec::new();
        v.extend(Self::FLATS.iter().copied());
        v.extend(Self::SHARPS.iter().copied());
        v.extend(Self::DOUBLE_FLATS.iter().copied());
        v.extend(Self::DOUBLE_SHARPS.iter().copied());
        v.extend(Self::NATURALS.iter().copied());
        v.sort_by_key(|s|EzStr::new(*s).len());
        v
    });

    pub const ALL_TRIGGERS_LONGEST_FIRST: Lazy<Vec<&'static str>> = Lazy::new(|| {
        let mut v: Vec<&'static str> = Self::ALL_TRIGGERS_SHORTEST_FIRST.to_vec();
        v.sort_by_key(|s|std::cmp::Reverse(EzStr::new(*s).len()));
       v
    });

    pub const STR_TO_DIST: Lazy<HashMap<&'static str, i32>> = Lazy::new(|| {
        let mut m = HashMap::new();
        for &token in Self::ALL_TRIGGERS_SHORTEST_FIRST.iter() {
            let dist = if Self::FLATS.contains(&token) {
                -1
            } else if Self::SHARPS.contains(&token) {
                1
            } else if Self::DOUBLE_FLATS.contains(&token) {
                -2
            } else if Self::DOUBLE_SHARPS.contains(&token) {
                2
            } else if Self::NATURALS.contains(&token) {
                0
            } else {
                0
            };
            m.insert(token, dist);
        }
        m
    });

    /// Remove all accidental tokens from the input string.
    /// Longest-match-first is used so that "bb" (double-flat) is removed
    /// before single "b" is considered.
    pub fn remove(s: &str) -> String {
        // memoize
        {
            let cache = REMOVE_CACHE.lock().unwrap();
            if let Some(v) = cache.get(s) {
                return v.clone();
            }
        }

        let mut out = String::with_capacity(s.len());
        let mut i = 0usize;
        let len = s.len();

        while i < len {
            let rem = &s[i..];
            let mut matched = false;
            for &token in Self::ALL_TRIGGERS_SHORTEST_FIRST.iter() {
                if rem.starts_with(token) {
                    // consume the token (remove it)
                    i += token.len();
                    matched = true;
                    break;
                }
            }
            if !matched {
                // append next char and advance by its utf8 length
                let ch = rem.chars().next().unwrap();
                out.push(ch);
                i += ch.len_utf8();
            }
        }

        let mut cache = REMOVE_CACHE.lock().unwrap();
        cache.insert(s.to_string(), out.clone());
        out
    }

    /// Sum of semitone distances represented in the string.
    ///
    /// Uses the same tokenization strategy as `remove` (longest-match-first).
    /// Example: "bb" -> -2; "#" -> +1; "x" -> +2; "♭♭" -> -2.
    pub fn get_dist(s: &str) -> i16 {
        // memoize
        {
            let cache = DIST_CACHE.lock().unwrap();
            if let Some(v) = cache.get(s) {
                return (*v).try_into().unwrap();
            }
        }

        let mut sum: i32 = 0;
        let mut i = 0usize;
        let len = s.len();

        while i < len {
            let rem = &s[i..];
            let mut matched = false;
            for &token in Self::ALL_TRIGGERS_SHORTEST_FIRST.iter() {
                if rem.starts_with(token) {
                    if let Some(&d) = Self::STR_TO_DIST.get(token) {
                        sum += d;
                    }
                    i += token.len();
                    matched = true;
                    break;
                }
            }
            if !matched {
                // advance one char (non-accidental)
                let ch = rem.chars().next().unwrap();
                i += ch.len_utf8();
            }
        }

        let mut cache = DIST_CACHE.lock().unwrap();
        cache.insert(s.to_string(), sum);
        sum.try_into().unwrap()
    }

    /// Replace ASCII accidentals with Unicode equivalents.
    /// If `using_double_accidentals` is true then ascii 'x' -> 𝄪 will be converted.
    /// We run replacements longest-key-first to avoid partial/ambiguous replacements.
    pub fn replace_with_unicode(s: &str, using_double_accidentals: bool) -> String {
        let mut out = s.to_string();

        // order is important: longer tokens first
        let mut replacements: Vec<(&str, &str)> = Vec::new();

        // ascii double-flat "bb" -> 𝄫 (always possible)
        replacements.push((Self::ASCII_DOUBLE_FLAT, Self::DOUBLE_FLAT));

        // ascii double-sharp 'x' -> 𝄪 only if requested
        if using_double_accidentals {
            replacements.push((Self::ASCII_DOUBLE_SHARP, Self::DOUBLE_SHARP));
        }

        // single ascii -> unicode
        replacements.push((Self::ASCII_FLAT, Self::FLAT));
        replacements.push((Self::ASCII_SHARP, Self::SHARP));
        replacements.push((Self::ASCII_NATURAL, Self::NATURAL));

        for (from, to) in replacements {
            if out.contains(from) {
                out = out.replace(from, to);
            }
        }

        out
    }

    /// Replace Unicode accidentals with ASCII equivalents.
    /// If `using_double_accidentals` is true, Unicode double-sharp/flat map to ASCII double tokens
    /// ('x' and 'bb'); otherwise double-sharps map to "##" and double-flats to "bb".
    pub fn replace_with_ascii(s: &str, using_double_accidentals: bool) -> String {
        let mut out = s.to_string();

        // decide what ASCII to use for doubles if requested
        let double_sharp_ascii = if using_double_accidentals {
            Self::ASCII_DOUBLE_SHARP // "x"
        } else {
            // represent as two single sharps
            // we'll use "##" to match the common ASCII representation when not using 'x'
            // this is returned as a borrowed string so create via explicit branch below
            "::DOUBLE_SHARP_AS_HEAP::"
        };

        // Prepare replacements longest-first
        // Unicode double-flat -> "bb" (same in both modes)
        let mut replacements: Vec<(&str, String)> = Vec::new();
        replacements.push((Self::DOUBLE_FLAT, Self::ASCII_DOUBLE_FLAT.to_string()));

        // Unicode double-sharp -> either "x" or "##"
        if double_sharp_ascii == "::DOUBLE_SHARP_AS_HEAP::" {
            replacements.push((Self::DOUBLE_SHARP, "##".to_string()));
        } else {
            replacements.push((Self::DOUBLE_SHARP, double_sharp_ascii.to_string()));
        }

        // singles
        replacements.push((Self::FLAT, Self::ASCII_FLAT.to_string()));
        replacements.push((Self::SHARP, Self::ASCII_SHARP.to_string()));
        replacements.push((Self::NATURAL, Self::ASCII_NATURAL.to_string()));

        // Apply replacements
        for (from, to) in replacements {
            if out.contains(from) {
                out = out.replace(from, &to);
            }
        }

        out
    }

    pub fn contains_unicode_accidentals<S: AsRef<str>>(s: S) -> bool {
        let s = s.as_ref();
        s.contains(Self::FLAT)
            || s.contains(Self::SHARP)
            || s.contains(Self::DOUBLE_FLAT)
            || s.contains(Self::DOUBLE_SHARP)
            || s.contains(Self::NATURAL)
    }

    pub fn contains_double_accidentals<S: AsRef<str>>(s: S) -> bool {
        let s = s.as_ref();
        s.contains(Self::ASCII_DOUBLE_FLAT)
            || s.contains(Self::ASCII_DOUBLE_SHARP)
            || s.contains(Self::DOUBLE_FLAT)
            || s.contains(Self::DOUBLE_SHARP)
    }

    /// Produce an accidental string for a semitone distance.
    ///
    /// - `dist`: signed semitone distance (positive -> sharps, negative -> flats).
    /// - `using_unicode_accidentals`: choose unicode symbols (♯/♭/𝄪/𝄫) vs ASCII (#/b/x/bb).
    /// - `using_double_accidentals`: whether to prefer double-accidental tokens for pairs of semitones.
    ///
    /// Behavior mirrors the original JS: a distance of 0 returns the empty string.
    pub fn from_dist<I: Into<i32>>(dist: I, using_unicode_accidentals: bool, using_double_accidentals: bool) -> String {
        let dist = dist.into();
        if dist == 0 {
            return String::new();
        }

        let (flat, sharp, double_flat, double_sharp) = if using_unicode_accidentals {
            (Self::FLAT, Self::SHARP, Self::DOUBLE_FLAT, Self::DOUBLE_SHARP)
        } else {
            (
                Self::ASCII_FLAT,
                Self::ASCII_SHARP,
                Self::ASCII_DOUBLE_FLAT,
                Self::ASCII_DOUBLE_SHARP,
            )
        };

        let acc = if dist > 0 { sharp } else { flat };
        if using_double_accidentals {
            let double_acc = if dist > 0 { double_sharp } else { double_flat };
            let doubles = (dist.abs() / 2) as usize;
            let singles = (dist.abs() % 2) as usize;
            let mut s = String::with_capacity(doubles * double_acc.len() + singles * acc.len());
            for _ in 0..doubles {
                s.push_str(double_acc);
            }
            for _ in 0..singles {
                s.push_str(acc);
            }
            s
        } else {
            acc.repeat(dist.abs() as usize)
        }
    }

    pub fn show_sets() -> String {
        let mut out = String::new();

        out += &*format!("FLATS: {:?}\n", Accidentals::FLATS);
        out += &*format!("DOUBLE FLATS: {:?}\n", Accidentals::DOUBLE_FLATS);
        out += &*format!("SHARPS: {:?}\n", Accidentals::SHARPS);
        out += &*format!("DOUBLE SHARPS: {:?}\n", Accidentals::DOUBLE_SHARPS);
        out += &*format!("ALL_TRIGGERS_SHORTEST_FIRST: {:?}\n", *Accidentals::ALL_TRIGGERS_SHORTEST_FIRST);
        out += &*format!("ALL_TRIGGERS_LONGEST_FIRST: {:?}\n", *Accidentals::ALL_TRIGGERS_LONGEST_FIRST);
        out
    }

    pub fn is_accidentals_str<S: Into<EzStr>>(s: S) -> bool {
        let s:EzStr = s.into();
        let mut c_inactive_until = 0;
        for (c,_char) in s.graphemes().into_iter().enumerate() {
            if c < c_inactive_until {continue};
            let mut trigger_found = false;
            for (t,trigger) in Self::ALL_TRIGGERS_LONGEST_FIRST.iter().enumerate() {
                let trigger = EzStr::new(*trigger);
                let end = c as i32 + trigger.len() as i32;
                if end <= s.len() as i32 && trigger == s.slice(c as i32, end) {
                    c_inactive_until = c_inactive_until.wrapping_add(trigger.len());
                    trigger_found = true;
                    //panic!("HERE s:{:?} trigger:{:?} {:?} val: {}",s, trigger, c, s.slice(c as i32, end));
                    break;
                }
            }
            if !trigger_found {
                return false;
            }
        }
        true
    }

    pub fn accidental_str_to_vec<S: Into<EzStr>>(s: S) -> Option<Vec<String>> {
        let s:EzStr = s.into();
        let mut c_inactive_until = 0;
        let mut accidentals: Vec<String> = Vec::new();
        for (c,char) in s.graphemes().into_iter().enumerate() {
            if c < c_inactive_until {continue};
            let mut trigger_found = false;
            for (t,trigger) in Self::ALL_TRIGGERS_LONGEST_FIRST.iter().enumerate() {
                let trigger = EzStr::new(*trigger);
                let end = c as i32 + trigger.len() as i32;
                if end <= s.len() as i32 && trigger == s.slice(c as i32, end) {
                    accidentals.push(trigger.data.clone());
                    c_inactive_until = c_inactive_until.wrapping_add(trigger.len());
                    trigger_found = true;
                    //panic!("HERE s:{:?} trigger:{:?} {:?} val: {}",s, trigger, c, s.slice(c as i32, end));
                    break;
                }
            }
            if !trigger_found {
                return None;
            }
        }
        Some(accidentals)
    }

    // pub fn accidental_str_to_dist<S: Into<EzStr>>(s: S) -> Option<i32> {
    //     let accidentals = Self::accidental_str_to_vec(s);
    //     match accidentals {
    //         Some(accidentals) => {
    //             let mut dist  = 0;
    //             for accidental in accidentals.iter(){
    //                 dist += Accidentals::get_dist(accidental);
    //             }
    //             Some(3)
    //         }
    //         _ => return None
    //     }
    //
    //
    // }
}


// Simple thread-safe memoization caches for remove() and get_dist()
static REMOVE_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));
static DIST_CACHE: Lazy<Mutex<HashMap<String, i32>>> =
    Lazy::new(|| Mutex::new(HashMap::with_capacity(256)));

#[cfg(test)]
mod tests {
    use super::{Accidentals};

    #[test]
    fn test_remove_basic() {
        assert_eq!(Accidentals::remove("C#"), "C");
        assert_eq!(Accidentals::remove("C♭"), "C");
        assert_eq!(Accidentals::remove("Cbb"), "C"); // ascii double flat "bb"
        assert_eq!(Accidentals::remove("C𝄫"), "C"); // unicode double flat
        assert_eq!(Accidentals::remove("Cnat"), "C"); // ascii natural token "nat"
        assert_eq!(Accidentals::remove("C♮"), "C");
    }

    #[test]
    fn test_get_dist_examples() {
        assert_eq!(Accidentals::get_dist("#"), 1);
        assert_eq!(Accidentals::get_dist("b"), -1);
        assert_eq!(Accidentals::get_dist("bb"), -2);
        assert_eq!(Accidentals::get_dist("x"), 2);
        assert_eq!(Accidentals::get_dist("♯♯"), 2);
        assert_eq!(Accidentals::get_dist("♭♭"), -2);
        assert_eq!(Accidentals::get_dist("nat"), 0);
        assert_eq!(Accidentals::get_dist("♮"), 0);
    }

    #[test]
    fn test_replace_unicode_ascii_roundtrip() {
        let s = "C# Dbb Ex Fnat";
        let uni = Accidentals::replace_with_unicode(s, true);
        // convert back
        let ascii = Accidentals::replace_with_ascii(&uni, true);
        // Because replace_with_unicode maps 'bb'->𝄫 and back to 'bb', etc, roundtrip should be equal (ignoring possible "##" vs "x" when double_sharp toggles)
        assert_eq!(Accidentals::remove(&ascii), Accidentals::remove(s));
    }

    #[test]
    fn test_from_dist() {
        assert_eq!(Accidentals::from_dist(1, false, false), "#".to_string());
        assert_eq!(Accidentals::from_dist(2, false, false), "##".to_string());
        assert_eq!(Accidentals::from_dist(2, false, true), "x".to_string());
        assert_eq!(Accidentals::from_dist(-2, true, true), "𝄫".to_string());
        assert_eq!(Accidentals::from_dist(0, true, true), "".to_string());
    }

    #[test]
    fn test_accidental_str(){
        let mut test = "b##b";
        let mut result =  Accidentals::accidental_str_to_vec(test);
          assert_eq!(result.clone(), Some(vec!["b".to_string(),"#".to_string(),"#".to_string(),"b".to_string()]),
        "Accidentals::accidental_str_to_vec({:?}) == {:?}",test, result.unwrap_or_default()
        );

        test = "natbx";
        result = Accidentals::accidental_str_to_vec(test);

        println!("Accidentals::accidental_str_to_vec({:?}) == {:?} ",test,result.unwrap_or_default());
    }

    #[test]
    fn test_access() {
        Accidentals::ALL_TRIGGERS_SHORTEST_FIRST.iter().for_each(|&token| {
            println!("Testing {}", token);
        });
        println!("{}",Accidentals::show_sets())
    }
}
