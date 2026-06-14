use std::fmt;
use itertools::Itertools;
use crate::frets::{Fret, Frets, ToFrets, ToPitchClassSet};
use crate::{Change, };
//
// // Todo refactor to accidentals methods
// const ACCIDENTAL_CHARS: [&str; 7] = ["#","♯","b","♭","♮","𝄫","𝄪"];
//
// fn nums_only(input: &str) -> String {
//     let mut result = String::new();
//     for c in input.chars() {
//         if c.is_ascii_digit() {
//             result.push(c);
//         }
//     }
//     result
// }
//
// fn accs_only(input: &str) -> String {
//     let mut result = String::new();
//     for c in input.chars() {
//         if is_accidental(&c.to_string()) {
//             result.push(c);
//         }
//     }
//     result
// }
//
//
// fn is_accidental(input: &str) -> bool {
//     if ACCIDENTAL_CHARS.contains(&input) {
//         return true;
//     }
//     false
// }
//
// fn is_all_accidentals(input: &str) -> bool {
//     input.chars().all(|c| is_accidental(c.to_string().as_str()))
// }
//
//
// fn accept_all_strings(value: impl Into<String>) {
//     let string_value = value.into();
//     println!("string_value: {string_value}");
// }
//
// fn acc_to_dist(acc: &str) -> i32 {
//     match acc {
//         "#" => 1,
//         "b" => -1,
//         "♯" => 1,
//         "♭" => -1,
//         "♮" => 0,
//         "𝄫" => -2,
//         "𝄪" => 2,
//         _ => panic!("Invalid input: {}", acc)
//     }
// }
//
// fn accs_to_dist(accs: &str) -> i32 {
//     assert!(is_all_accidentals(accs));
//     let mut dist = 0;
//     for c in accs.chars(){
//         dist += acc_to_dist(&c.to_string())
//     }
//     dist
// }
//
//


// #[derive(Debug)]
// struct Note {
//     note: String,
// }
//
// impl fmt::Display for Note {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.note)
//     }
// }
//
// impl AsRef<str> for Note {
//     fn as_ref(&self) -> &str {
//         &self.note
//     }
// }
//
// impl Note {
//
//     fn new(note: & str) -> Note {
//         if Note::is_valid(note){
//             return Note {
//                 note: String::from(note),
//             }
//         } else {
//             panic!("Invalid note: {}", note)
//         }
//     }
//
//
//     fn get_degree(& self) -> Note {
//         Note::new(nums_only(&self.note).as_ref())
//     }
//
//     fn get_accs(&self) -> &str {
//         let mut num_idx=0;
//         for c in self.note.chars(){
//
//             if c.is_numeric() {
//                 break;
//             }
//             num_idx += 1;
//         }
//         &self.note[..num_idx]
//     }
//
//     fn get_dist(&self) -> i32 {
//         Note::degree_to_dist(self.get_degree()) + accs_to_dist(self.get_accs())
//     }
//
//     pub fn degree_to_dist<T: ToString>(degree: T) -> i32 {
//         let d: i32; let ret: i32; let d_first_octave; let octave: i32;
//         if let Ok(n) = degree.to_string().parse::<i32>() {
//             // If the degree can be parsed as an integer, return it directly
//             d=n;
//         } else if degree.to_string().chars().all(|c| c.is_numeric() || c == '-') {
//             // If the degree is a string of numeric characters, parse it as an integer
//             d=degree.to_string().parse::<i32>().unwrap();
//         } else {
//             // Otherwise, return a default value of 0
//             return -1
//         }
//         if d == 0 {
//             panic!("0 is not a valid scale degree.");
//         }
//         octave = (d - 1) / 7;
//         d_first_octave = (d - 1) % 7 + 1;
//         //ret = (d_first_octave - 1) * 2 - (d_first_octave) / 4; // this yeilds semitones, only in first octave!
//
//         ret = match d_first_octave {1 => 0,2 => 2,3 => 4,4 => 5,5 => 7,6 => 9,7 => 11,
//             i32::MIN..=0_i32 | 8_i32..=i32::MAX => 0};
//         return ret + octave * 12;
//     }
//
//     fn from_dist(dist:&i32,only_flats: bool) -> Note {
//         let octave:i32 = dist/12;
//         let semitone:i32 = dist - octave * 12;
//         let mut deg:i32= 0;
//         deg = match semitone { 0=>1,2=>2,4=>3,5=>4,7=>5,9=>6,11=>7,_=>0};
//         let mut acc:&str = "";
//         if deg == 0 {
//             deg = match semitone {1=>2,3=>3,6=>5,8=>6,10=>7,_=>0} + only_flats as i32;
//             if !only_flats {
//                 acc = "b";
//             } else {
//                 acc = "#";
//                 deg -= 2;
//             }
//         }
//         assert!(deg != 0); // this means no degree was found
//         deg += octave * 7;
//         //println!("{:?}",deg.to_string().as_str().to_owned() + acc);
//         Note::new(&(acc.to_owned() + &(deg.to_string().as_str().to_owned())))
//     }
//
//
//
//     fn is_valid(note_str: &str) -> bool {
//         let number_part = nums_only(note_str);
//         let mut accidental_part = String::new();
//
//         if number_part.len() == 0 {
//             return false;
//         } else if number_part == "0" {
//             return false;
//         } else {
//             //println!("{:?} number_part:{}", note_str,number_part)
//         }
//
//         accidental_part = accs_only(note_str);
//         if note_str.find(&accidental_part) > Some(0) {
//             panic!("Accidentals need to start at the beginning of Note: {:?}", note_str)
//         }
//         //println!("{:?} accidental_part:{}", note_str,accidental_part);
//
//         if !is_all_accidentals(&accidental_part) {
//             return false;
//         }
//         true
//     }
// }
//
//


struct BitmapIter<const N:usize> {
    data: Bitmap<N>,
    idx: usize,
}



/*impl Iterator for BitmapIter<N> {
    type Item = bool;
    fn next(&mut self) -> Option<bool>{

    }
}*/

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub (crate) struct Bitmap<const N: usize> {
    bools: [bool; N],
}



impl<const N: usize> fmt::Display for Bitmap<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",Bitmap::<N>::bools_to_bitstr(self.bools))
    }
}


/*impl<const N: usize> Index<usize> for Bitmap<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bools[index]
    }
}*/



impl<const N: usize> IntoIterator for Bitmap<N> {
    type Item = bool;
    type IntoIter = std::array::IntoIter<bool, N>;

    fn into_iter(self) -> Self::IntoIter {
        let bool_array: [bool; N] = self.bools.into();
        bool_array.into_iter()
    }
}

//These two actually work


impl<Idx, const N:usize> std::ops::Index<Idx> for Bitmap<N>
where
    Idx: std::slice::SliceIndex<[bool]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.bools[index]
    }
}

impl<Idx, const N: usize> std::ops::IndexMut<Idx> for Bitmap<N>
where
    Idx: std::slice::SliceIndex<[bool]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.bools[index]
    }
}



impl<const N: usize> std::iter::FromIterator<bool> for Bitmap<N> {
    fn from_iter<I: IntoIterator<Item=bool>>(iter: I) -> Self {
        let mut bools = [false; N];
        let mut i = 0;

        for bit in iter {
            if i >= N {
                break;
            }

            bools[i] = bit;
            i += 1;
        }

        Bitmap { bools }
    }
}

// impl<const N: usize> Iterator for Bitmap<N> {
//     type Item = bool;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx < self.bools.len() {
//             let item = self.bools[self.idx];
//             self.idx += 1;
//             return Some(item)
//         } else {
//             return None
//         }
//     }
// }
// impl<const N: usize, Idx:usize> Index<Idx> for Bitmap<N>{
//     type Output = usize;
//     fn index(&self,index: Idx) -> &Self::Output{
//         return self.bools[index];
//     }
// }
//
//
// impl<const N: usize> IntoIterator for Bitmap<N>{
//     type Item = bool;
//     type IntoIter = BitmapIntoIterator<N>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         BitmapIntoIterator {
//             bitmap: Bitmap::<N>::from( self.bools ),
//             index: 0,
//         }
//     }
// }

pub struct BitmapIntoIterator<const N: usize> {
    bitmap: Bitmap<N>,
    index: usize,
}



impl<const N: usize> Iterator for BitmapIntoIterator<N> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.index < self.bitmap.len() {
            Some(self.bitmap[self.index])
        } else {
            None
        }
    }
}

impl<const N: usize> ToFrets for Bitmap<N> {
    fn to_frets(&self) -> Frets {
        let mut frets = vec![];
        for i in 0..N {
            if self.get(i) {
                // frets.push((N - i) as Fret);
                frets.push(i as Fret);
            }
        }
        frets.into()
    }
}


/*








impl<const N: usize> Iterator for Bitmap<N> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index() {
            Some(i) if i < N => {
                self.set_index(Some(i + 1));
                Some(self.bools[i])
            },
            _ => {
                self.index = None;
                None
            }
        }
    }
}

impl<const N: usize> std::iter::DoubleEndedIterator for Bitmap<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.bools.iter().rev().next().copied()
    }
}
*/
impl<const N: usize> ToPitchClassSet for Bitmap<N> {
    fn to_pitch_class_set(&self) -> Frets {
        let mut pitch_classes = vec![];

        for (n, val) in self.bools.iter().rev().enumerate(){ //rev.enumerate starts counting from the end
            if *val {
                pitch_classes.push(n as Fret);
            }
        }
        pitch_classes.to_frets()
    }
}


impl<const N: usize> Bitmap<N> {
    pub fn len(&self) -> usize{
        self.bools.len()
    }

    fn from_small_endian(bools: [bool; N]) -> Bitmap<N> {
        Bitmap { bools }
    }

    fn from_big_endian(bools: [bool; N]) -> Bitmap<N> {
        Bitmap { bools: bools.into_iter().rev().collect_array::<N>().unwrap() }
    }

    /// Initialized to all false values
    pub fn new() -> Bitmap<N> {
        Bitmap::from_big_endian([false; N])
    }

    // fn from_dists(dists:&[i32]) -> Bitmap<N>
    // {
    //     let mut bools:[bool;N] = [false;N];
    //     for dist in dists.iter() {
    //         bools[*dist as usize] = true;
    //     }
    //     Bitmap::<N>::new(bools)
    // }

    /// This lets you use indexing that is in the order that you would think of the notes.
    ///
    /// The root is index 0... keep in mind if you index into the member variable it may be backward
    ///
    /// So better to use these set and get methods
    pub fn get(&self, idx: usize) -> bool {
        self.bools[self.bools.len() - idx - 1] == true
    }

    /// This lets you use indexing that is in the order that you would think of the notes.
    ///
    /// The root is index 0... keep in mind if you index into the member variable it may be backward
    ///
    /// So better to use these set and get methods
    pub fn set(&mut self, idx: usize, val: bool) {
        let i = self.bools.len().checked_sub(  idx + 1).expect(
            format!("Hey bools {self} idx {idx}").as_str()
        );
        // *self.bools.get_mut(i).expect(
        //     format!("Hey bools idx {idx} thing {thing:?}").as_str()
        // ) = val;
        self.bools[i] = val;
    }

    pub fn note_count(&self) -> usize {
        self.into_iter().filter(|bit| *bit == true).collect::<Vec<_>>().len()
    }

    pub fn from_bitstr_small_endian(bitstr: &str) -> Bitmap<N> {
        Bitmap::<N>::from_small_endian(Bitmap::<N>::bitstr_to_bools(bitstr))
    }

    pub fn from_bitstr_big_endian(bitstr: &str) -> Bitmap<N> {
        Bitmap::<N>::from_big_endian(Bitmap::<N>::bitstr_to_bools(bitstr))
    }
    
    pub fn from_notes_big_endian<T>(notes: T) -> Bitmap<N>
    where
        T: ToPitchClassSet,
    // T: AsRef<str>,
    {
        let mut bools = [false;N];
        //should I error if the notes are higher than bit length?

        for dist in notes.to_pitch_class_set(){
            if dist < N.try_into().unwrap() {
                bools[dist as usize] = true;
            }
        }
        Bitmap::<N>::from_big_endian(bools)
    }

    ///Makes Bitmap from notesstr
    ///
    /// # Arguments
    ///
    /// * `notesstr` - A string of notes formatted like the following
    ///
    ///expects a string like "1, b2, #6, 7"
    ///
    ///but can also accept "1 2 3 4 5"
    ///
    /// or ............... "1,2,3,4,##5"
    ///
    /// # Examples
    ///
    ///```rust,ignore
    /// use jazz_chord::Bitmap;
    /// let note = Bitmap::<12>::from_notes_string("1, 2, 4, b5");
    /// ```
    pub fn from_notes_string(notes_string: &str) -> Bitmap<N>{
        Bitmap::<N>::from_notes_big_endian(Change::from_notes_string(notes_string))
    }

    /*fn from_comb_num(num:u32) -> Bitmap<N>{

    }*/


    fn from_bin_num(num:u32) -> Bitmap<N>{
        // moved to PowerSet
        let mut result = [false; N];
        if num >= 2u32.pow(N as u32) {
            panic!("Invalid ring_num={} supplied to from_ring_num.\nValid numbers for ring_num are between 0 and {}", num, <u32 as TryInto<u32>>::try_into(2u32.pow(N as u32)).unwrap() - 1)
        }
        for i in 0..N {
            result[i] = ((num >> i) & 1) == 1;
        }
        Bitmap::<N>::from_big_endian(result)
    }

    fn to_bin_num(&self) -> u32 {
        let mut num = 0;
        for (i, &value) in self.bools.iter().enumerate() {
            if value {
                num |= 1 << i;
            }
        }
        num
    }

    fn from_ring_num(num:u32) -> Bitmap<N>{
        let mut result = [false; N];
        if num >= 2u32.pow(N as u32) {
            panic!("Invalid ring_num={} supplied to from_ring_num.\nValid numbers for ring_num are between 0 and {}", num, <u32 as TryInto<u32>>::try_into(2u32.pow(N as u32)).unwrap() - 1)
        }
        for i in 0..N {
            result[N - 1 - i] = ((num >> i) & 1) == 1;
        }
        Bitmap::<N>::from_big_endian(result)
    }

    fn to_ring_num(&self) -> u32 {
        let mut num = 0;
        for (i, &value) in self.bools.iter().enumerate() {
            if value {
                num |= 1 << (N - 1 - i);
            }
        }
        num
    }





    /*fn from_comb_num(num:u32) -> Bitmap<N>{
    fn to_comb_num

    }*/

    fn bitstr_to_bools(bitstr: &str) -> [bool; N] {
        let mut bool_arr = [false; N];
        let bitstr_padded = format!("{:0>width$}", bitstr, width = N);
        for i in 0..N {
            bool_arr[i] = bitstr_padded.chars().nth(i).unwrap() == '1';
        }
        bool_arr
    }

    pub fn bools_to_bitstr(bools: [bool;N]) -> String {
        let mut s = String::new();
        for c in bools.iter() {
            if *c {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s
    }
    fn reversed(&self) -> Bitmap<N>{
        let bools: [bool;N] = {
            let mut bools = self.bools;
            bools.reverse();
            bools
        };
        Bitmap::<N>::from_big_endian( bools )
    }
}
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub (crate) struct Bitmap<const N: usize> {
//     bools: [bool; N],
// }

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Instant;
    use crate::{bitmap, Change};
    use crate::frets::Fret;
    use super::*; // Brings private items into scope
    #[test]
    fn test_bitmap() {
        let start_time_of_bin = Instant::now();
        // env::set_var("RUST_BACKTRACE", "full");
        // env::set_var("RUSTFLAGS", "-C debuginfo=full");
        let dim_seven_bools_small_endian = [true, false, false, true, false, false, true, false, false, true, false, false, ];

        let bitstr = "100";
        let notes = ["1", "b3", "b7"];
        let notesstr = "1 b3 #5,   7";
        let mut bitmap = Bitmap::<12>::from_small_endian(dim_seven_bools_small_endian);
        //bitmap[0] = false;
        println!("Bitmap::<12>::new({:?}):\n\t-> {}", dim_seven_bools_small_endian, bitmap);
        let bitmap2 = Bitmap::<3>::from_bitstr_small_endian(bitstr);
        println!("Bitmap::<3>::from_bitstr_small_endian({:?}):\n\t-> {}", bitstr, bitmap2);
        let bitmap3 = Bitmap::<12>::from_notes_big_endian(&Change::from_note_strings(&notes));
        println!("Bitmap::<12>::new_from_notes({:?}):\n\t-> {}", notes, bitmap3);
        let bitmap4 = Bitmap::<12>::from_notes_string(notesstr);
        println!("Bitmap::<12>::new_from_notesstr({:?}):\n\t-> {}", notesstr, bitmap4);

        println!("bitmap:{} bitmap2:{} bitmap3:{} bitmap4:{}", bitmap, bitmap2, bitmap3, bitmap4);

        for bit in bitmap {
            print!("{:?}", bit);
        }
        println!();
        bitmap[3] = false;


        println!("{:?}", &bitmap[0..5]);

        // bitmap[3..6] = *[true, true, false].as_ref();
        let dists: [Fret; 7] = [0, 2, 4, 5, 7, 9, 11];
        let mut bitmap = Bitmap::<12>::from_notes_big_endian(dists);
        println!("Yo {} rev {}", bitmap, bitmap.reversed());
        let k = 12;

        /*for i in 0..12 {
            for n in 1..=12 {
                println!("C(i:{}, n:{}, k:{}) = {:?}.", i, n, k, Combination::cgen(i, n, k));
            }
        }*/
    }
}