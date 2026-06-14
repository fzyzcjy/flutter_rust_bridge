#![allow(warnings)]

use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fmt::Display;
use std::ops::Index;
use std::ops::Add;
use std::slice::SliceIndex;
use std::hash::Hash;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone, PartialEq)]
pub struct Grapheme {
    pub value: String,
}

impl Grapheme {
    pub fn new(value: &str) -> Grapheme {
        Grapheme {
            value: String::from(value),
        }
    }
}

impl std::fmt::Display for Grapheme {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.value)
    }
}

#[derive(Clone, PartialEq, Eq, Default, Hash)]
pub struct GraphemeMatch {
    pub start: usize,
    pub end: usize,
    pub text: EzStr,
}

pub type GraphemeSpan = GraphemeMatch;

impl GraphemeMatch {
    pub fn new<T>(start: usize, end: usize, text: T) -> Self
    where
        T: Into<EzStr>,
    {
        let it = GraphemeMatch { start, end, text:text.into()};
        it


    }




    pub fn as_str(&self) -> &str {
        &self.text.data
    }

    pub fn to_ezstr(&self) -> EzStr {
        self.text.clone()
    }

    pub fn is_valid(&self, source:&EzStr) -> bool {
        //let a = self.source.chars().collect::<Vec<_>>()[self.start..self.end].to_vec();
        // let a = &self.source.graphemes(true).collect::<Vec<_>>()[self.start..self.end].join("");

        let a = source.slice(self.start as i32,self.end as i32);
        let b = EzStr::new(&self.text.data);
        let ret = a == b;
        // if !ret {
        //     panic!("a:{a:?} b:{b:?}\n{self:?}")
        // }
        ret
    }

    pub fn ensure_is_valid<S: Into<EzStr> + Clone>(&self, source:S) -> () {
        let source = source.into();
        if !self.is_valid(&source) {

            // let span = &self.source[self.start..self.end];
            // let re = Regex::new(self.text.as_str()).unwrap();
            // let text_matches: Vec<_> = regex_find_graphemes_iter(&re, self.source).collect();
            let a = source.slice(self.start as i32,self.end as i32);
            let b = EzStr::new(&self.text.data);
            let re = Regex::new(b.data.as_str().replace("|",r"\|").as_str()).unwrap();
            let source:EzStr = source.into();
            let text_matches: Vec<_> = source.find_iter(&re).collect();

            let mut panic_str = String::new();
            if text_matches.len() > 0 {
                panic_str += format!("\n, so not found at [{},{}] but was found at        ",self.start,self.end).as_str();
                for re_match in text_matches{
                    panic_str += format!(
                        "[{},{}]: {:?}",
                        re_match.start,
                        re_match.end,
                        source.slice(re_match.start, re_match.end)
                    )
                    .as_str();
                }
                panic_str += "\n\n";
            }
            //panic!("source:{:?}\nre: {:?}\ntext_matches:{:?}",source,re,text_matches);
            panic_str += format!("substring: {b:?} not at source.slice({},{}): {a:?} \nInvalid {self:?}\n{:?}",
                                        self.start, self.end,
                                        &source).as_str();



            panic!("{}",panic_str);
        } else {
            println!("Successful match found at: {:?}", self);

        }
    }


}

impl<'a> Display for GraphemeMatch{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> fmt::Debug for GraphemeMatch{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GraphemeMatch {{ start: {:?}, end: {}, text: {:?} }}",
            self.start, self.end, self.text
        )
    }
}


pub struct EzStr {
    pub data: String,
    pub graphemes_data: OnceCell<Vec<Grapheme>>,
    grapheme_byte_index_data: OnceCell<Vec<(usize, usize)>>, // (byte_offset, grapheme_index)
}

impl Hash for EzStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl PartialEq for EzStr {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl PartialEq<&str> for EzStr {
    fn eq(&self, other: &&str) -> bool {
        self.data == *other
    }
}

impl PartialEq<str> for EzStr {
    fn eq(&self, other: &str) -> bool {
        self.data == other
    }
}

impl Eq for EzStr {}



impl EzStr {
    pub fn new<S: Into<String>>(data: S) -> Self {
        let data = data.into();
        EzStr {
            data,
            graphemes_data: OnceCell::new(),
            grapheme_byte_index_data: OnceCell::new(),
        }
    }

    pub fn graphemes(&self) -> &Vec<Grapheme> {
        self.graphemes_data.get_or_init(|| {
            UnicodeSegmentation::graphemes(self.data.as_str(), true)
                .map(Grapheme::new)
                .collect()
        })
    }

    pub fn graphemes_byte_index(&self) -> &Vec<(usize, usize)> {
        self.grapheme_byte_index_data.get_or_init(|| {
            self.data
                .grapheme_indices(true)
                .enumerate()
                .map(|(gi, (bi, _))| (bi, gi))
                .collect()
        })
    }

    fn byte_range_to_grapheme_indices(&self, start: usize, end: usize) -> (usize, usize) {
        let idx = self.graphemes_byte_index();

        let g_start = match idx.binary_search_by_key(&start, |&(b, _)| b) {
            Ok(i) => idx[i].1,
            Err(i) => idx.get(i).map(|&(_, gi)| gi).unwrap_or(self.len()),
        };

        let g_end = match idx.binary_search_by_key(&end, |&(b, _)| b) {
            Ok(i) => idx[i].1,
            Err(i) => idx.get(i).map(|&(_, gi)| gi).unwrap_or(self.len()),
        };

        (g_start, g_end)
    }

    pub fn slice<S, E>(&self, start: S, end: E) -> EzStr
    where
        S: TryInto<i32>,
        S::Error: Debug,
        E: TryInto<i32>,
        E::Error: Debug,
    {
        let graphemes = self.graphemes();
        let mut ret = String::new();
        let mut start = start.try_into().unwrap();
        let mut end = end.try_into().unwrap();

        if start < 0 {
            start = graphemes.len() as i32 + start + 1;
        }
        if end < 0 {
            end = graphemes.len() as i32 + end + 1;
        }

        for i in start..end {
            ret += &graphemes[i as usize].value;
        }
        EzStr::new(&ret)
    }

    pub fn len(&self) -> usize {
        self.graphemes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_str(&self) -> &str {
        self.data.as_str()
    }

    pub fn contains<T: AsRef<str>>(&self, substring: T) -> bool {
        self.data.contains(substring.as_ref())
    }

    /// Returns the first match of the regex, in grapheme cluster indices.
    pub fn find<'a>(&'a self, regex: &Regex) -> Option<GraphemeMatch> {
        let data = &self.data;
        regex.find(data).map(|m| {
            let (g_start, g_end) = self.byte_range_to_grapheme_indices(m.start(), m.end());
            GraphemeMatch::new(
                g_start,
                g_end,
                self.slice(g_start as i32, g_end as i32),
            )
        })
    }


    pub fn findOLD(&self, regex: &Regex) -> Option<GraphemeMatch> {
        regex.find(&self.data).map( |m| {
            let (g_start, g_end) = self.byte_range_to_grapheme_indices(m.start(), m.end());
            GraphemeMatch::new(g_start, g_end, self.slice(g_start as i32, g_end as i32))
        })
    }



    /// Returns an iterator of matches of the regex, in grapheme cluster indices.
    pub fn find_iter(
        &self,
        regex: &Regex,
    ) -> impl Iterator<Item=GraphemeMatch> {
        let data = &self.data;
        regex.find_iter(data).map(move |m| {
            let (g_start, g_end) = self.byte_range_to_grapheme_indices(m.start(), m.end());
            GraphemeMatch::new(
                g_start,
                g_end,
                self.slice(g_start as i32, g_end as i32),
            )
        })
    }

    // /// Returns an iterator of matches of the regex, in grapheme cluster indices.
    //     fn find_iterOLD<'a>(
    //         &'a self,
    //         regex: &'a Regex,
    //     ) -> impl Iterator<Item = GraphemeMatch> + 'a {
    //         regex.find_iter(&self.data).map(|m| {
    //             let (g_start, g_end) = self.byte_range_to_grapheme_indices(m.start(), m.end());
    //             GraphemeMatch::new(g_start, g_end, self.slice(g_start as i32, g_end as i32),self.data.as_str())
    //         })
    //     }
}


impl Clone for EzStr {
    fn clone(&self) -> EzStr {
        Self {data:self.data.clone(),
            graphemes_data: self.graphemes_data.clone(),
            grapheme_byte_index_data: self.grapheme_byte_index_data.clone(), }
    }
}


impl From<String> for EzStr {
    fn from(item: String) -> Self {
        EzStr::new(item)
    }
}

impl From<&String> for EzStr {
    fn from(item: &String) -> Self {
        EzStr::new(item)
    }
}

impl From<&EzStr> for EzStr {
    fn from(item: &EzStr) -> Self {
        item.clone()
    }
}

impl From<i16> for EzStr {
    fn from(item: i16) -> Self {
        EzStr::new(item.to_string())
    }
}

impl From<&str> for EzStr {
    fn from(item: &str) -> Self {
        EzStr::new(item)
    }
}

impl From<char> for EzStr {
    fn from(item: char) -> Self {
        EzStr::new(item.to_string())
    }
}

impl From<u8> for EzStr {
    fn from(item: u8) -> Self {
        EzStr::new(item.to_string())
    }
}

impl From<GraphemeMatch> for EzStr {
    fn from(grapheme_match: GraphemeMatch) -> Self {
        EzStr::new(grapheme_match.text)
    }
}

impl From<&GraphemeMatch> for EzStr {
    fn from(grapheme_match: &GraphemeMatch) -> Self {
        EzStr::new(&grapheme_match.text)
    }
}
impl Into<String> for EzStr {
    fn into(self) -> String {
        self.data
    }
}

impl Into<String> for &EzStr {
    fn into(self) -> String {
        self.data.clone()
    }
}

impl Into<i32> for &EzStr {
    fn into(self) -> i32 {
        self.data.parse::<i32>().unwrap()
    }
}

impl Into<i32> for EzStr {
    fn into(self) -> i32 {
        self.data.parse::<i32>().unwrap()
    }
}


impl Index<usize> for EzStr {
    type Output = Grapheme;
    fn index(&self, index: usize) -> &Self::Output {
        &self.graphemes()[index]
    }
}

impl IntoIterator for EzStr {
    type Item = Grapheme;
    type IntoIter = std::vec::IntoIter<Grapheme>;

    fn into_iter(self) -> Self::IntoIter {
        self.graphemes().clone().into_iter()
    }
}

impl AsRef<str> for EzStr {
    fn as_ref(&self) -> &str {
        self.data.as_str()
    }
}

impl<'a> IntoIterator for &'a EzStr {
    type Item = &'a Grapheme;
    type IntoIter = std::slice::Iter<'a, Grapheme>;

    fn into_iter(self) -> Self::IntoIter {
        self.graphemes().iter()
    }
}

impl Index<std::ops::Range<usize>> for EzStr {
    type Output = [Grapheme];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.graphemes()[index]
    }
}

// EzStr + &str
impl Add<&str> for EzStr {
    type Output = EzStr;
    fn add(self, other: &str) -> EzStr {
        EzStr::new(&(self.data + other))
    }
}

// EzStr + EzStr
impl Add<&EzStr> for &EzStr {
    type Output = EzStr;
    fn add(self, other: &EzStr) -> EzStr {
        EzStr::new(&(self.data.clone() + &other.data))
    }
}

// EzStr + &str
impl Add<&str> for &EzStr {
    type Output = EzStr;
    fn add(self, other: &str) -> EzStr {
        EzStr::new(&(self.data.clone() + other))
    }
}

// EzStr + String
impl Add<&String> for &EzStr {
    type Output = EzStr;
    fn add(self, other: &String) -> EzStr {
        EzStr::new(&(self.data.clone() + other))
    }
}

impl Default for EzStr {
    fn default() -> Self {
        EzStr::new("")
    }
}

impl fmt::Display for EzStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl fmt::Debug for EzStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.data))
    }
}
