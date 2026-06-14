use std::ops::Range;
use crate::frets::ToFrets;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::vec::IntoIter;
use itertools::Powerset;
pub use super::bitmap::*;

// struct PowerSetIterator {
//     cur: i32,
//     end: i32,
// }
//
// impl PowerSetIterator {
//     fn new() -> Self {
//         Self { cur: 0, end: 0 }
//     }
// }
// impl Iterator for PowerSetIterator {
//     type Item = i32;
//     fn next(&mut self) -> Option<Self::Item> {
//
//     }
// }

// impl<const N: usize> IntoIterator for PowerSet<N>{
//     type Item = i32;
//     type IntoIter = IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.to_indices().into_iter()
//     }
// }

/// Stores notes in Big Endian
#[derive(Debug, PartialEq, Clone, strum_macros::EnumIter, Eq, Hash, )]
pub enum PowerSet<const N: usize> {
    Change,
    ChangeCardinal(u32),
    Combinatoric,
    CombinatoricCardinal(u32),
    SmallEndian,
    BigEndian,
}

impl<const N: usize> Display for PowerSet<N>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}<{}>", self,N, )
        // write!(f, "BitmapPowerset<{}>::{:?}", N, self)
    }
}
//
// impl<const N: usize> Iterator for PowerSet<N>{
//     type Item = i32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }


impl<const N: usize> PowerSet<N> {
    pub fn to_index(&self, bitmap: &Bitmap<N>) -> i32 {
        match self {
            Self::ChangeCardinal(_) => {
                self.get_change_number(bitmap, true, false)
            }
            Self::CombinatoricCardinal(_) => {
                self.get_change_number(bitmap, true, true)
            }
            Self::Change => {
                self.get_change_number(bitmap, false, false)
            }
            Self::Combinatoric => {
                self.get_change_number(bitmap, false, true)
            }
            Self::SmallEndian => {
                let mut num = 0;
                for (i,v) in bitmap.into_iter().enumerate() {
                    if v == true {
                        num += 2_i32.pow(i as u32);
                    }
                }
                num
            }
            Self::BigEndian => {
                let mut num = 0;
                for (i,v) in bitmap.into_iter().rev().enumerate() {
                    if v == true {
                        num += 2_i32.pow(i as u32);
                    }
                }
                num
            }
        }
    }

    /// stores bitmap as big endian
    pub fn to_bitmap(&self, num: i32) -> Option<Bitmap<N>>{
        if !self.index_is_valid(num){
            //panic!("in {}, {} didn't work as index", self, num);
            return None;
        }
        let n = N as i32;
        let mut bitmap = Bitmap::<N>::new();
        let frets = match self {
            PowerSet::Change => {
                let mut frets: Vec<i32> = Vec::with_capacity(N);
                if num > 0 {
                    frets.push(0);
                }
                frets.extend(Self::pgen(num.abs() - 1, n - 1, true).unwrap());
                frets

            }
            PowerSet::ChangeCardinal(cardinality) => {
                let mut frets: Vec<i32> = Vec::with_capacity(N);


                if num > 0 {
                    frets.push(0);
                }
                frets.extend(Self::cgen(num.abs() - 1, n - 1, (*cardinality - 1) as i32, true).expect(
                    format!("Combination::cgen({}, {}, {}, {}) is invalid\n{}.to_bitmap({}) is not valid? min: {} max: {}", num - 1, n - 1, *cardinality - 1, true, self, num, self.min_index(), self.max_index()).as_str())
                );

                frets
            }
            PowerSet::Combinatoric => {
                Self::pgen(num, n, false).unwrap()
            }
            PowerSet::CombinatoricCardinal(note_num) => {
                Self::cgen(num, n, *note_num as i32, false).unwrap()
            }
            PowerSet::SmallEndian => {
                bitmap.into_iter().enumerate()
                    .filter(|(fret, _)| (num >> fret) & 1 == 1)
                    .map(|(fret, _)|n - 1 - fret as i32)
                    .collect()
                // for fret in 0..N {
                //     bitmap.set(fret as usize, ((num >> fret) & 1) == 1);
                // }

            }
            PowerSet::BigEndian => {
                bitmap.into_iter().enumerate()
                    .filter(|(fret, val)| (num >> fret) & 1 == 1)
                    .map(|(fret, val)| fret as i32)
                    .collect()
                // for i in 0..N {
                //     // bitmap[N - 1 - i] = ((num >> i) & 1) == 1;
                //     bitmap.set(i,  ((num >> i) & 1) == 1);
                // }
            },
        };
        for fret in &frets {
            debug_assert!(*fret < N as i32, "{} frets {:?}", self, frets);
            bitmap.set(*fret as usize, true);
        }
        Some(bitmap)
    }
    pub fn to_indices(&self) -> Vec<i32> {
        Vec::from(match self {
            Self::Change | Self::ChangeCardinal(_) => (self.min_index()..0).chain(1..self.max_index()).collect::<Vec<_>>(),
            _ => (self.min_index().. self.max_index()).collect(),
        })
    }

    pub fn total_values(&self) -> usize {
        match self{
            x if x.is_cardinal() => 2u32.pow(N as u32) as usize,
            Self::ChangeCardinal(cardinality) => {
                (Self::ChangeCardinal(*cardinality).max_index() - Self::ChangeCardinal(*cardinality).min_index() - 1) as usize
            }
            Self::CombinatoricCardinal(cardinality) => {
                (Self::ChangeCardinal(*cardinality).max_index() - Self::ChangeCardinal(*cardinality).min_index()) as usize
            }
            _ => unreachable!()
        }
    }

    fn get_change_number(
        &self,
        bitmap: &Bitmap<N>,
        relative_to_note_count: bool,
        combinatoric: bool,
    ) -> i32 {
        // index_offset: combinatoric uses 0-based expectation, non-combinatoric expects the root at index 1
        let mut index_offset: usize = if combinatoric { 0 } else { 1 };

        // Use N-1 for combinatoric counting of remaining choices (fixes binomial top)
        let combinator_size: i32 = (N - 1).try_into().unwrap();
        // let combinator_size: i32 = if combinatoric {N} else {(N - 1)}.try_into().unwrap();

        let mut chapter_beginning: i32 = 0;
        let mut chapter_index: i32 = 0;

        // copy frets (semitones) and, for non-combinatoric mode, ensure the root (0) is present
        let mut semitones = bitmap.to_frets();
        let C = Self::C;

        if !combinatoric && !semitones.contains((&0).into()) {
            semitones.insert(0, 0);
        }

        // compute chapter_beginning: sum of C(combinator_size, k) for k = 0..(semitones.len()-1)
        let start = if combinatoric { 0 } else { 1 };
        for chapter in start..semitones.len() {
            let k = (chapter - start) as i32;
            chapter_beginning += C(combinator_size, k);
        }

        // reduced_semitones: all but the last semitone; for non-combinatoric drop the inserted root
        let reduced_semitones: Vec<usize> = if semitones.len() <= 1 {
            Vec::new()
        } else if combinatoric {
            semitones[..semitones.len() - 1].iter().map(|s| *s as usize).collect()
            // semitones.iter().map(|s| *s as usize).collect()
        } else {
            // drop the inserted root (index 0) and the last element
            if semitones.len() <= 2 {
                Vec::new()
            } else {
                semitones[1..semitones.len() - 1].iter().map(|s| *s as usize).collect()
            }
        };

        // Walk reduced semitones and accumulate combinatoric contributions
        for (idx, &st) in reduced_semitones.iter().enumerate() {
            let expected = idx + index_offset;
            if st != expected {
                let distance_from_expected: i32 = (st - index_offset - idx).try_into().unwrap();
                let indexes_away_from_end: i32 = (reduced_semitones.len() - 1 - idx).try_into().unwrap();

                // set index_offset to the actual semitone position for subsequent expectations
                index_offset = st - idx;

                // add the binomial contributions for each step the semitone jumped forward
                for i in 0..distance_from_expected {
                    let top = combinator_size - (st as i32) + distance_from_expected - i;
                    let bottom = indexes_away_from_end + 1;
                    let add = C(top, bottom);
                    chapter_index += add;
                }
            }
        }

        // last bump: gap after the last selected semitone
        let last_bump: i32 = if !semitones.is_empty() {
            let last = *semitones.last().unwrap() as i32;
            let prev = semitones[semitones.len() - (2.min(semitones.len()))] as i32;
            (last - prev - 1) as i32
        } else {
            0
        };



        // I hate this!
        if combinatoric && semitones.len() >= 2 {
            if semitones.len() >= 4 {
                let start = if semitones.len() % 2 == 0 { 0 } else { 1 };
                let end: i32 = (semitones.len() - 4).try_into().unwrap();
                // println!("{:?}", (start..=end).step_by(2).collect::<Vec<_>>());
                for it in (start..=end).step_by(2) {
                    chapter_beginning += C(N.try_into().unwrap(), it.try_into().unwrap())
                }
            }
            chapter_beginning += C(N.try_into().unwrap(), (semitones.len() - 2).try_into().unwrap());
        }

        // Also hate this
        if combinatoric && semitones.len() == 1 {
        // if combinatoric && semitones.len() == 1 && semitones[0] != 0 {
            chapter_index = semitones[0].try_into().unwrap();
            // println!("\t\tadded 1 to chapter_index: {}", chapter_index);
        }


        // assemble pages
        let mut book_page = chapter_index + chapter_beginning + last_bump;
        let mut chapter_page = chapter_index + last_bump;
        //println!("chapter_page {} book_page {} chapter_index {} chapter_beginning {} last_bump {}", chapter_page, book_page, chapter_index, chapter_beginning, last_bump);

        // fallback when both are zero: use the first semitone as base
        // if book_page == 0 && chapter_page == 0 {
        //     if !semitones.is_empty() {
        //         // keep behavior consistent with non-combinatoric path
        //         book_page = semitones[0] as i32;
        //         chapter_page = book_page;
        //     }
        // }

        // non-combinatoric path uses 1-based book pages
        if !combinatoric {
            // println!("added 1 to book_page {} chapter_page {}", book_page, chapter_page);
            book_page += 1;
            chapter_page += 1;
            if semitones.len() == 0 || (semitones.len() == 1 && semitones[0] == 0) {
                book_page += 1;
                chapter_page += 1;
            }

        }

        // polarity: if the bitmap has no root (bit 0 unset) and we're in non-combinatoric mode,
        // reverse sign to indicate "negative" pages (preserve original behavior)
        // let has_no_one = !bitmap.get(0);
        let has_no_one = bitmap.get(0) == false;
        if combinatoric {
            if semitones.len() == 1 {
                book_page += 1;
                chapter_page += 1;
            }
        } else {
            if has_no_one {
                book_page *= -1;
                chapter_page *= -1;
                // println!("\nin {self}({bitmap}) flipped the bit to book_page {} chapter_page {}\n", book_page, chapter_page);
            }
            // else {
            //     println!("\nin {self}({bitmap}) didn't flip the bit to book_page {} chapter_page {} {} {}\n", book_page, chapter_page, bitmap, bitmap.get(0));
            // }
        }



        if relative_to_note_count {
            chapter_page as i32
        } else {
            book_page as i32
        }
    }


    fn change_number_back(&self, bitmap: &Bitmap<N>, relative_to_note_count: bool, combinatoric: bool) -> i32 {
        //Because we are including the root note in the combinatorics
        let mut index_offset: usize = if combinatoric {0} else {1};
        // let mut index_offset: usize = 1;
        let combinator_size: i32 = if combinatoric {
            N.try_into().unwrap()
        } else {
            (N - 1).try_into().unwrap()
        };
        let mut chapter_beginning = 0;
        let mut chapter_index = 0;

        let mut semitones = bitmap.to_frets();
        // # Make negative pages positive at first
        let mut force_one = false;
        let C = Self::C;
        if !combinatoric && !semitones.contains((&0).into()) {
            semitones.insert(0, 0);
            force_one = true;
        }
        for chapter in if combinatoric {0} else {1}..semitones.len(){
            // don't know if this 0, 1 thing is right here
            chapter_beginning += C(combinator_size, (chapter - if combinatoric{0} else{1}).try_into().unwrap())
            // chapter_beginning += C(combinator_size, (chapter - 1).try_into().unwrap())
        }
        let reduced_semitones: Vec<_> = if combinatoric {
            semitones.iter().take(semitones.len() - 1).collect() }
        // semitones.iter().collect() }
        else {
            semitones.iter().take(semitones.len() - 1).skip(1).collect()
        };
        // println!("HOWDY {} semitones {} reduced_semitones: {:?}", self,semitones, reduced_semitones);
        for (idx, st) in reduced_semitones.iter().enumerate() {
            let st = **(st) as usize;
            if st != idx + index_offset {
                let distance_from_expected: i32 = (st - index_offset - idx).try_into().unwrap();
                let indexes_away_from_end: i32 = (reduced_semitones.len() - 1 - idx).try_into().unwrap();
                // Just mute this?
                //let _C = C(combinator_size - distance_from_expected,indexes_away_from_end) - 1;
                // println!( "HEY st {st} index_offset {index_offset} idx {idx}");
                index_offset = st + 0 - idx;
                for i in 0..distance_from_expected {
                    let _C = C(combinator_size - st as i32 + distance_from_expected - i,
                               indexes_away_from_end + 1);
                    if combinatoric {
                        chapter_index -= 1;
                    }
                    // println!("\t\t_C = {} chapter_index = {} + {} == {}", _C, chapter_index, _C, chapter_index + _C);
                    chapter_index += _C;

                }
            }
        }
        // println!();
        let last_bump = if !semitones.is_empty() {
            let last_bump =
                *semitones.last().unwrap()
                    - semitones[semitones.len()-(2.min(semitones.len())) ]
                    - 1;

            last_bump as i32
        } else { 0 };

        // println!("\n(_)for {}({}): chapter_index {} last_bump {} chapter_beginning: {}", self, bitmap, chapter_index, last_bump, chapter_beginning);

        let mut book_page = chapter_index + chapter_beginning + last_bump;
        let mut chapter_page = chapter_index + 1 + last_bump;
        if book_page == 0 && chapter_page == 0 {
            book_page = semitones[0] as i32;
            chapter_page = book_page
        }
        // add 1 to book page
        if !combinatoric{
            book_page += 1;
        }

        // first note is the one

        let has_no_one = !bitmap.get(0);
        // Reverse polarity if no one
        if has_no_one && !combinatoric {
            book_page *= -1;
            chapter_page = (chapter_page - 1) * -1;
        };
        // println!("\ncombinatoric {combinatoric} book_page: {book_page} last bump: {last_bump} chapter_index {chapter_index} chapter_beginning {chapter_beginning}");
        // println!("has_no_one: {has_no_one}");
        //Hardwire first subChange
        if semitones.len() == 1 && semitones[0] == 0 {
            book_page = 1;
            chapter_page = 1;
        }
        // Hardwire the None change
        if semitones.len() == 0 {
            book_page = -1;
        }

        (if relative_to_note_count {
            chapter_page
        } else {
            book_page
        } as i32)
    }

    pub fn min_index(&self) -> i32 {
        match self {
            PowerSet::Change => -2i32.pow(N as u32 - 1),
            PowerSet::ChangeCardinal(cardinality) => {
                -Self::C(
                    N as i32 - 1,
                    (*cardinality).try_into().unwrap(),
                )
            }
            _ => {
                0
            }
        }
    }

    /// non inclusive
    pub fn max_index(&self) -> i32 {
        match self {
            PowerSet::Change => 2i32.pow(N as u32 - 1) + 1,
            PowerSet::ChangeCardinal(cardinality) => {
                match cardinality {
                    0 => -1,
                    _ => Self::C(
                        (N - 1).try_into().unwrap(),
                        (*cardinality - 1) as i32,
                    ),
                }
            }
            PowerSet::CombinatoricCardinal(cardinality) => {
                Self::C(
                    N as i32,
                    (*cardinality) as i32,
                )
            }
            _ => {
                2_i32.pow(N as u32)
            }
        }
    }



    pub fn index_is_valid(&self, num: i32) -> bool {
        match num >= self.min_index() && num < self.max_index(){
            true => {
                match self {
                    PowerSet::Change | PowerSet::ChangeCardinal(_) => num != 0,
                    _ => { true }
                }
            }
            false => false
        }
    }

    /// returns the i-th combination of k numbers chosen from 1,2,...,n or 0,1,...,n-1 if zero_indexed == true
    /// k should always be less than n, i should be less than C(n,k)
    /// i is zero indexed. k is one indexed
    fn cgen(i:i32,n:i32,k:i32,one_indexed:bool) -> Option<Vec<i32>>{
        let C = Self::C;
        debug_assert!(k<=n,"Shit");
        if !(i < C(n,k)) {
            return None
        }
        debug_assert!(i<C(n,k),"called Combination::cgen(i={},n={},k={}) i:{} should be less than the number of combinations. C(n={},k={}) = {}.",i,n,k,i,n,k,C(n,k));
        // if k < 0 {
        //     return None
        // }
        let mut c: Vec<i32> = Vec::with_capacity(k.try_into().expect(
            format!("k {}", k).as_str()
        ));
        let mut r = i + 1; // i+1 to make i zero indexed. Or i if you want 1 indexed
        let mut j = 0; // use 0 here to make the the values by 1 indexed
        let bump = if one_indexed {0} else {1};
        for s in 1..=k {
            let mut cs = j + 1;
            while r - C(n - cs, k - s) > 0 {
                r -= C(n - cs, k - s);
                cs += 1;
            }
            c.push(cs - bump); //-1 makes values returned 0-indexed, 0 makes them 1-indexed
            j = cs;
        }
        return Some(c)
    }

    ///computes nCk, the number of combinations n choose k
    ///In other words:
    /// how many combinations of cardinality k does a set with n elements have?
    fn C(n:i32,k:i32) -> i32{
        let mut result= 1;
        for i in 0..n {
            result *= i + 1;
        }
        for i in 0..k {
            result /= i + 1;
        }
        for i in 0..(n - k) {
            result /= i + 1;
        }
        result
    }

    //returns the i-th combination of numbers (0 or 1)..n using combinatorics
    fn pgen(i:i32,n:i32,one_indexed:bool) -> Option<Vec<i32>> {
        if i < (2_i32.pow(n.try_into().unwrap())) {
            let mut k = 0;
            let mut chapter_start = 0;
            for k_val in 0..=n {
                let chapter_len = Self::C(n,k_val);
                if (i - chapter_len) < chapter_start {
                    k = k_val;
                    break;
                }
                chapter_start += chapter_len;
            }
            Some(Self::cgen(i-chapter_start,n,k,one_indexed).unwrap())
        } else {
            None
        }
    }

    pub fn all_non_cardinal() -> [PowerSet<N>; 4] {
        [
            Self::Change,
            Self::Combinatoric,
            Self::BigEndian,
            Self::SmallEndian,
        ]
    }

    pub fn is_cardinal(&self) -> bool {
        Self::all_non_cardinal().contains(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Instant;
    use itertools::Itertools;
    use super::*; // Brings items into scope
    #[test]
    fn test_power_set() {
        let start_time_of_bin = Instant::now();
        let mut failed: HashMap<PowerSet<12>, Vec<Bitmap<12>>> = HashMap::new();
        let mut succeeded: HashMap<PowerSet<12>, Vec<Bitmap<12>>> = HashMap::new();

        for power_set in PowerSet::<12>::all_non_cardinal(){
            assert_eq!(power_set.clone().to_indices().len(), power_set.total_values(),
                       "Wrong number for total: {} {}", power_set, power_set.total_values()
            );
        }

        // for i in PowerSet::<12>::SubCombinatoric(7).into_iter(){
        //     println!("{}", i);
        // }
        let test_nums = (1..2).collect::<Vec<i32>>();
        let test_nums = (-100..1000).collect::<Vec<i32>>();
        let power_sets = [
            PowerSet::<12>::Change,
            PowerSet::<12>::Combinatoric,
            PowerSet::<12>::SmallEndian,
            PowerSet::<12>::BigEndian,
            PowerSet::<12>::ChangeCardinal(7),
            // PowerSet::<12>::SubChange(6),
            // PowerSet::<12>::SubChange(5),
            // PowerSet::<12>::SubChange(4),
            // PowerSet::<12>::SubChange(3),
            // PowerSet::<12>::SubChange(2),
            // PowerSet::<12>::SubChange(1),
            // PowerSet::<12>::SubChange(0),
            PowerSet::<12>::CombinatoricCardinal(7),
            // PowerSet::<12>::SubCombinatoric(6),
            // PowerSet::<12>::SubCombinatoric(5),
            // PowerSet::<12>::SubCombinatoric(4),
            // PowerSet::<12>::SubCombinatoric(3),
            // PowerSet::<12>::SubCombinatoric(2),
            // PowerSet::<12>::SubCombinatoric(1),
            // PowerSet::<12>::SubCombinatoric(0),
        ];
        for power_set in &power_sets {
            failed.insert(power_set.clone(), vec![]);
            succeeded.insert(power_set.clone(), vec![]);
            println!("for {}, min_index {} max_index {}", power_set, power_set.min_index(), power_set.max_index());
        }

        let power_set = PowerSet::<3>::CombinatoricCardinal(2);
        for trigram in &power_set.to_indices() {

            println!("trigram {}: {}", trigram, power_set.to_bitmap(*trigram).unwrap());
        }

        // test change number

        for c in &test_nums {
            // make sure Change still works
            // if let Some(change) = &PowerSet::<12>::Change.to_bitmap(c) {
            //     debug_assert!(c == PowerSet::<12>::Change.from_bitmap(&PowerSet::<12>::Change.to_bitmap(c).unwrap()),
            //     "input {} output{}", c, PowerSet::<12>::Change.from_bitmap(&PowerSet::<12>::Change.to_bitmap(c).unwrap())
            //     );
            // }
            print!("for {} ", c);
            for powerset in power_sets.iter() {
                let bitmap = powerset.to_bitmap(*c);
                
                println!();
                if let Some(bitmap) = &bitmap {
                    let assertion = *c == powerset.to_index(bitmap);
                    if !assertion {
                        print!("!!")
                    }
                    print!("\twith {}.to_bitmap({}):\t", powerset, c);
                    if assertion {
                        print!("{} to index: {}",bitmap, powerset.to_index(bitmap));
                    } else {
                        failed.get_mut(powerset).unwrap().push(*bitmap);
                        print!("{} !{} became {} which is {} off", bitmap, c, powerset.to_index(bitmap), powerset.to_index(bitmap) - c)
                    }
                    print!(" semitones: {}", bitmap.to_frets())
                } else {
                    print!("\twith {}.to_bitmap({}):\tNone", powerset, c);

                }
            }
            println!();
        }
        assert!(failed.values().filter(|v| v.len() != 0).collect::<Vec<_>>().len() == 0, "because {:?} Failed to do {} of {}:\n{}\n{}",
                failed.values().filter(|v| v.len() != 0).collect::<Vec<_>>().len() == 0,
                failed.values().map(|v|v.len()).sum::<usize>(),
                power_sets.len() * test_nums.len(),
                failed.iter().map(|(k, v)| format!("{} failed for {}",v.len(), k, )).join("\n"),
                failed.iter().map(|(k,v)| format!("\n{}: {} failed.\n\t{}", k,v.len(),v.iter().join("\n\t"))).join("\n"));

        // let i=900;
        // let n= 12;
        // let k= 7;
        // println!("Starting C(i:{}, n:{}, k:{}).", i, n, k,);
        // for i in 0..4096{
        //     println!("Combination::cgen(i:{}, n:{}, k:{}) = {:?}.", i, n, k, Combination::cgen(i, n, k,false).unwrap_or_default());
        // }
        // for i in 0..4096 {
        //     println!("Combination::pgen(i:{}, n:{}) = {:?}",i,n,Combination::pgen(i,n,false).unwrap());
        // }
        // println!("HELLO?");
        println!("It took {:?} to run this file.", start_time_of_bin.elapsed());

    }
}

