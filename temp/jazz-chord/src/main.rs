#![allow(warnings)]
#[cfg_attr(feature = "hotpath", hotpath::main)]
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // use jazz_chord::frets::{Fret, Frets, FretExt, FretsExt};
    use jazz_chord::frets::*;
    use super::*; // Allows access to private items in the parent module
    use std::time::Instant;
    use jazz_chord::*;
    use std::collections::HashMap;
    use std::fmt::Display;
    use itertools::Itertools;
    use jazz_accidentals::Accidentals;

    #[test]
    fn convert_stuff(){
        // println!("match self {{");
        // for triad_type in &*TriadType::TRIADS {
        //     println!("\thashmap.insert(Self::{}, Change::from(\"{}\"));", triad_type.name, triad_type.notes());
        //     // println!("\tSelf::{} => {{ &{:?} }},", triad_type.name, triad_type.name);
        //     // println!("\tSelf::{} => {{ &{:?} }},", triad_type.name, triad_type.triggers);
        // }
        // println!("}}")

        // for extension in Extension::all() {
        //     let name: String =  extension.name().replace(" ", "");
        //     print!("Self");
        //     print!("::");
        //     print!("{}", name);
        //     // print!(" => Change::from_note_strings(&[{}])", extension.notes.notes.into_iter().map(|note| format!("{:?}",note.text.data.clone())).collect::<Vec<_>>().join(", "));
        //     println!(",");
        //     // println!("::{} => ", extension.name);
        // }
    }


    #[test]
    fn it_works() {
            let chord1 = Quality::from_string("ma6");
            let chord2 = Quality::from_string("m6");
    }

    #[test]
    fn test_note(){
        let note = Note::new("b5").unwrap();
        assert_eq!(note.accidentals.data, "b");
        assert_eq!(note.degree.data, "5" );
        assert!(Note::new("5b").is_none());
        assert!(Note::new("bb5").is_some());

        let bad_note = Note::new("b");
        assert_eq!(bad_note, None);
        let note = Note::new("bbb5").unwrap();
        println!("Note::new({:?}) accidentals:{:?} degree:{:?} frets_from_one:{:?}",note.text(),note.accidentals,note.degree, note.to_fret());

        let note = Note::new("bbbb5").unwrap();
        assert_eq!(note.to_single_accidentals(true), Note::new("b3").unwrap());
        assert_eq!(note.to_single_accidentals(false), Note::new("#2").unwrap());

        for i in 0..45 {

            //print!("{:?}: ",i);
            println!("{:?}: {:?}",i,Note::from_fret_single_flats(i as Fret));
        }

    }
    #[test]
    fn test_frets(){
        println!("Testing frets");
        // let mut frets = Frets::from(vec![1,5,6,11,13]);
        let mut frets =vec![1,5,6,11,13];
        frets[0] = 3;
        println!("{:?}", &frets[2..4] );
        for fret in frets {
            println!("{}",fret.to_fret());
        }
    }

    #[test]
    fn test_key(){
        let data = "Abbb";
        let bad_datas = vec!("bA","b2");
        let key = Key::new(data);
        if let Some(key) = key{
            println!("Key::new({:?}) == {:?} frets_from_c: {:?}", data,key.clone(),key.frets_from_c());
            assert_eq!(key.frets_from_c(),6);
            assert_eq!(Key::new("B###").unwrap().frets_from_c(),2);
            assert_eq!(Key::new("Cbbb").unwrap().frets_from_c(),9);
        } else {
            println!("Key::new({:?}) is invalid", data)
        }

        for bad_data in bad_datas {
            assert_eq!(Key::new(&*String::from(bad_data)), None, "bad_data: {:?} should not be valid", bad_data);
            println!("{:?} is not a valid Key", bad_data);
        }

        let to_key = "A#";
        let from_note = "bb7";
        let expected = "G";
        let left = Note::from(from_note).in_key(to_key);
        let right =Key::from(expected);
        assert!(left.is_equivalent(&right),"Note::from({from_note}).in_key({to_key}) == {left}, but expected: {right} (expected)");

        let left = Note::from("bb3").in_key("D");
        let right =Key::from("Fb");
        assert!(left.is_equivalent(&right),"{left} != {right} (expected)");
    }

    #[test]
    fn test_triad_extension(){
        let no_chord = TriadExtension::from(&None, &Some(&Extension::NoChord));
        assert_eq!(no_chord.notes(), &Change::new(), "Not empty. Instead is: {}", no_chord.notes());
        println!("Testing TriadExtension:\n{}\n\n",TriadExtension::all().into_iter().map(|sym|
        // println!("Testing TriadExtension:\n{}\n\n",TriadExtension::all_legal().into_iter().map(|sym|
            format!("{}{} {}", if sym.is_legal() {""} else {"!!"},
                    sym, sym.notes())).collect::<Vec<_>>().join("\n"));
    }

    fn test_chord_possibilities(){
        let mi_add_9 = Change::from_notes_string("1 2 b3 5");
        let changes = [mi_add_9, Change::from_notes_string("1 b2 b3 5 6")];
        let triad_extension = TriadExtension::from(&Some(&Triad::Minor), &None);


        // assert_eq!(triad_extension.)
        // assert_eq!(DegreePossibilities::from_triad_extension(TriadExtension::from(mi_add_9)),)
    }



    fn test_chord_mods(){
        let tests = {
            let mut m = HashMap::new();
            let annoying_chord = "º9(♭7 no 5)";
            m.insert(annoying_chord, QualityMods {
                inclusions: Change::from_notes_string("1 b3 9"),
                exclusions: Change::from_notes_string("5"),
                alterations: Change::from_notes_string("b7"),
                additions: Change::from_notes_string(""),
            });
            m
        };

        for (chord_str, mods) in tests{
            let chord = Quality::from_string(chord_str);
            let left = chord.unwrap().to_chord_mods(&QualityParams::default());
            let right = mods;
            assert!(left.eq_mods(&right, &ChangeEq::Unordered(NoteEq::Equivalent)), "left: {}\nright: {}", left, right);
        }
    }

    #[test]
    fn test_chord_quality(){
        #[cfg(feature = "hotpath")]
        let _hp = hotpath::init(
            "test_chord_quality".to_string(),
            &[50, 90, 95],
            hotpath::Format::Table,
        );
        // let input = "6";
        //
        // ChordQuality::extension_at_index(input, 0usize);
        // println!("ChordQuality::extension_at_start({:?}).unwrap() == {:?}", input,ChordQuality::extension_at_index(input, 0usize).unwrap());
        //
        let all_datas = vec![
            ("unison", Change::from_notes_string("1")), //1
            ("mi2nd", Change::from_notes_string("1 ♭2")), //2
            ("Ma2nd", Change::from_notes_string("1 2")), //3
            ("mi3rd", Change::from_notes_string("1 ♭3")), //4
            ("Ma3rd", Change::from_notes_string("1 3")), //5
            ("4th", Change::from_notes_string("1 4")), //6
            ("tritone", Change::from_notes_string("1 ♭5")), //7
            ("5th", Change::from_notes_string("1 5")), //8
            ("mi6th", Change::from_notes_string("1 ♭6")), //9
            ("Ma6th", Change::from_notes_string("1 6")), //10
            ("mi7th", Change::from_notes_string("1 ♭7")), //11
            ("Ma7th", Change::from_notes_string("1 7")), //12
            ("sus2(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2")), //13
            ("mi(no 5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3")), //14
            ("Ma(no 5 add ♭9)", Change::from_notes_string("1 ♭2 3")), //15
            ("sus4(no 5 add ♭9)", Change::from_notes_string("1 ♭2 4")), //16
            ("sus♭2(♭5)", Change::from_notes_string("1 ♭2 ♭5")), //17
            ("sus♭2", Change::from_notes_string("1 ♭2 5")), //18
            ("sus♭2(♯5)", Change::from_notes_string("1 ♭2 ♯5")), //19
            ("6sus♭2(no 5)", Change::from_notes_string("1 ♭2 6")), //20
            ("7sus♭2(no 5)", Change::from_notes_string("1 ♭2 ♭7")), //21
            ("Ma7sus♭2(no 5)", Change::from_notes_string("1 ♭2 7")), //22
            ("mi(no 5 add 9)", Change::from_notes_string("1 2 ♭3")), //23
            ("Ma(no 5 add 9)", Change::from_notes_string("1 2 3")), //24
            ("sus4(no 5 add 9)", Change::from_notes_string("1 2 4")), //25
            ("sus2(♭5)", Change::from_notes_string("1 2 ♭5")), //26
            ("sus2", Change::from_notes_string("1 2 5")), //27
            ("sus2(♯5)", Change::from_notes_string("1 2 ♯5")), //28
            ("6sus2(no 5)", Change::from_notes_string("1 2 6")), //29
            ("7sus2(no 5)", Change::from_notes_string("1 2 ♭7")), //30
            ("Ma7sus2(no 5)", Change::from_notes_string("1 2 7")), //31
            ("Ma(no 5 add ♯9)", Change::from_notes_string("1 ♯2 3")), //32
            ("mi(no 5 add 11)", Change::from_notes_string("1 ♭3 4")), //33
            ("dim", Change::from_notes_string("1 ♭3 ♭5")), //34
            ("mi", Change::from_notes_string("1 ♭3 5")), //35
            ("mi(♯5)", Change::from_notes_string("1 ♭3 ♯5")), //36
            ("mi6(no 5)", Change::from_notes_string("1 ♭3 6")), //37
            ("mi7(no 5)", Change::from_notes_string("1 ♭3 ♭7")), //38
            ("miMa7(no 5)", Change::from_notes_string("1 ♭3 7")), //39
            ("Ma(no 5 add 11)", Change::from_notes_string("1 3 4")), //40
            ("Ma(♭5)", Change::from_notes_string("1 3 ♭5")), //41
            ("Ma", Change::from_notes_string("1 3 5")), //42
            ("Ma(♯5)", Change::from_notes_string("1 3 ♯5")), //43
            ("6(no 5)", Change::from_notes_string("1 3 6")), //44
            ("7(no 5)", Change::from_notes_string("1 3 ♭7")), //45
            ("Ma7(no 5)", Change::from_notes_string("1 3 7")), //46
            ("sus4(♭5)", Change::from_notes_string("1 4 ♭5")), //47
            ("sus4", Change::from_notes_string("1 4 5")), //48
            ("sus4(♯5)", Change::from_notes_string("1 4 ♯5")), //49
            ("6sus4(no 5)", Change::from_notes_string("1 4 6")), //50
            ("7sus4(no 5)", Change::from_notes_string("1 4 ♭7")), //51
            ("Ma7sus4(no 5)", Change::from_notes_string("1 4 7")), //52
            ("sus♯4", Change::from_notes_string("1 ♯4 5")), //53
            ("sus♯4(♯5)", Change::from_notes_string("1 ♯4 ♯5")), //54
            ("6(♭5 no 3)", Change::from_notes_string("1 ♭5 6")), //55
            ("7(♭5 no 3)", Change::from_notes_string("1 ♭5 ♭7")), //56
            ("Ma7(♭5 no 3)", Change::from_notes_string("1 ♭5 7")), //57
            ("5(add ♭6)", Change::from_notes_string("1 5 ♭6")), //58
            ("5(add 6)", Change::from_notes_string("1 5 6")), //59
            ("7(no 3)", Change::from_notes_string("1 5 ♭7")), //60
            ("Ma7(no 3)", Change::from_notes_string("1 5 7")), //61
            ("6(♯5 no 3)", Change::from_notes_string("1 ♯5 6")), //62
            ("7(♯5 no 3)", Change::from_notes_string("1 ♯5 ♭7")), //63
            ("Ma7(♯5 no 3)", Change::from_notes_string("1 ♯5 7")), //64
            ("7(no 3 5 add 13)", Change::from_notes_string("1 6 ♭7")), //65
            ("Ma7(no 3 5 add 13)", Change::from_notes_string("1 6 7")), //66
            ("Ma7(no 3 5 add ♭7)", Change::from_notes_string("1 ♭7 7")), //67
            ("mi(no 5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3")), //68
            ("Ma(no 5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 3")), //69
            ("sus4(no 5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 4")), //70
            ("sus2(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭5")), //71
            ("sus2(add ♭9)", Change::from_notes_string("1 ♭2 2 5")), //72
            ("sus2(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♯5")), //73
            ("6sus2(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 6")), //74
            ("9sus♭2(no 5)", Change::from_notes_string("1 ♭2 2 ♭7")), //75
            ("Ma9sus♭2(no 5)", Change::from_notes_string("1 ♭2 2 7")), //76
            ("Ma(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3")), //77
            ("mi(no 5 add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4")), //78
            ("dim(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♭5")), //79
            ("mi(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 5")), //80
            ("mi(♯5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯5")), //81
            ("mi6(no 5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 6")), //82
            ("mi7(no 5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♭7")), //83
            ("miMa7(no 5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 7")), //84
            ("Ma(no 5 add ♭9 11)", Change::from_notes_string("1 ♭2 3 4")), //85
            ("Ma(♭5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♭5")), //86
            ("Ma(add ♭9)", Change::from_notes_string("1 ♭2 3 5")), //87
            ("Ma(♯5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♯5")), //88
            ("6(no 5 add ♭9)", Change::from_notes_string("1 ♭2 3 6")), //89
            ("7(no 5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♭7")), //90
            ("Ma7(no 5 add ♭9)", Change::from_notes_string("1 ♭2 3 7")), //91
            ("sus4(♭5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♭5")), //92
            ("sus4(add ♭9)", Change::from_notes_string("1 ♭2 4 5")), //93
            ("sus4(♯5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♯5")), //94
            ("6sus4(no 5 add ♭9)", Change::from_notes_string("1 ♭2 4 6")), //95
            ("7sus4(no 5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♭7")), //96
            ("Ma7sus4(no 5 add ♭9)", Change::from_notes_string("1 ♭2 4 7")), //97
            ("sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 ♯4 5")), //98
            ("sus♯4(♯5 add ♭9)", Change::from_notes_string("1 ♭2 ♯4 ♯5")), //99
            ("6sus♭2(♭5)", Change::from_notes_string("1 ♭2 ♭5 6")), //100
            ("7sus♭2(♭5)", Change::from_notes_string("1 ♭2 ♭5 ♭7")), //101
            ("Ma7sus♭2(♭5)", Change::from_notes_string("1 ♭2 ♭5 7")), //102
            ("sus♭2(add ♭6)", Change::from_notes_string("1 ♭2 5 ♭6")), //103
            ("6sus♭2", Change::from_notes_string("1 ♭2 5 6")), //104
            ("7sus♭2", Change::from_notes_string("1 ♭2 5 ♭7")), //105
            ("Ma7sus♭2", Change::from_notes_string("1 ♭2 5 7")), //106
            ("6sus♭2(♯5)", Change::from_notes_string("1 ♭2 ♯5 6")), //107
            ("7sus♭2(♯5)", Change::from_notes_string("1 ♭2 ♯5 ♭7")), //108
            ("Ma7sus♭2(♯5)", Change::from_notes_string("1 ♭2 ♯5 7")), //109
            ("7sus♭2(no 5 add 13)", Change::from_notes_string("1 ♭2 6 ♭7")), //110
            ("Ma7sus♭2(no 5 add 13)", Change::from_notes_string("1 ♭2 6 7")), //111
            ("Ma7sus♭2(no 5 add ♭7)", Change::from_notes_string("1 ♭2 ♭7 7")), //112
            ("Ma(no 5 add 9 ♯9)", Change::from_notes_string("1 2 ♯2 3")), //113
            ("mi11(no 5 7)", Change::from_notes_string("1 2 ♭3 4")), //114
            ("dim(add 9)", Change::from_notes_string("1 2 ♭3 ♭5")), //115
            ("mi(add 9)", Change::from_notes_string("1 2 ♭3 5")), //116
            ("mi(♯5 add 9)", Change::from_notes_string("1 2 ♭3 ♯5")), //117
            ("mi6/9(no 5)", Change::from_notes_string("1 2 ♭3 6")), //118
            ("mi9(no 5)", Change::from_notes_string("1 2 ♭3 ♭7")), //119
            ("miMa9(no 5)", Change::from_notes_string("1 2 ♭3 7")), //120
            ("11(no 5 7)", Change::from_notes_string("1 2 3 4")), //121
            ("Ma(♭5 add 9)", Change::from_notes_string("1 2 3 ♭5")), //122
            ("Ma(add 9)", Change::from_notes_string("1 2 3 5")), //123
            ("Ma(♯5 add 9)", Change::from_notes_string("1 2 3 ♯5")), //124
            ("6/9(no 5)", Change::from_notes_string("1 2 3 6")), //125
            ("9(no 5)", Change::from_notes_string("1 2 3 ♭7")), //126
            ("Ma9(no 5)", Change::from_notes_string("1 2 3 7")), //127
            ("sus4(♭5 add 9)", Change::from_notes_string("1 2 4 ♭5")), //128
            ("sus4(add 9)", Change::from_notes_string("1 2 4 5")), //129
            ("sus4(♯5 add 9)", Change::from_notes_string("1 2 4 ♯5")), //130
            ("6/9sus4(no 5)", Change::from_notes_string("1 2 4 6")), //131
            ("9sus4(no 5)", Change::from_notes_string("1 2 4 ♭7")), //132
            ("Ma9sus4(no 5)", Change::from_notes_string("1 2 4 7")), //133
            ("sus2(add ♯11)", Change::from_notes_string("1 2 ♯4 5")), //134
            ("sus2(♭5 add ♭6)", Change::from_notes_string("1 2 ♯4 ♯5")), //135
            ("6sus2(♭5)", Change::from_notes_string("1 2 ♭5 6")), //136
            ("7sus2(♭5)", Change::from_notes_string("1 2 ♭5 ♭7")), //137
            ("Ma7sus2(♭5)", Change::from_notes_string("1 2 ♭5 7")), //138
            ("sus2(add ♭6)", Change::from_notes_string("1 2 5 ♭6")), //139
            ("6sus2", Change::from_notes_string("1 2 5 6")), //140
            ("7sus2", Change::from_notes_string("1 2 5 ♭7")), //141
            ("Ma7sus2", Change::from_notes_string("1 2 5 7")), //142
            ("6sus2(♯5)", Change::from_notes_string("1 2 ♯5 6")), //143
            ("7sus2(♯5)", Change::from_notes_string("1 2 ♯5 ♭7")), //144
            ("Ma7sus2(♯5)", Change::from_notes_string("1 2 ♯5 7")), //145
            ("7sus2(no 5 add 13)", Change::from_notes_string("1 2 6 ♭7")), //146
            ("Ma7sus2(no 5 add 13)", Change::from_notes_string("1 2 6 7")), //147
            ("Ma7sus2(no 5 add ♭7)", Change::from_notes_string("1 2 ♯6 7")), //148
            ("Ma(no 5 add ♯9 11)", Change::from_notes_string("1 ♯2 3 4")), //149
            ("Ma(♭5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♭5")), //150
            ("Ma(add ♯9)", Change::from_notes_string("1 ♯2 3 5")), //151
            ("Ma(♯5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♯5")), //152
            ("6(no 5 add ♯9)", Change::from_notes_string("1 ♯2 3 6")), //153
            ("7(no 5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♭7")), //154
            ("Ma7(no 5 add ♯9)", Change::from_notes_string("1 ♯2 3 7")), //155
            ("dim(add 11)", Change::from_notes_string("1 ♭3 4 ♭5")), //156
            ("mi(add 11)", Change::from_notes_string("1 ♭3 4 5")), //157
            ("mi(♯5 add 11)", Change::from_notes_string("1 ♭3 4 ♯5")), //158
            ("mi6(no 5 add 11)", Change::from_notes_string("1 ♭3 4 6")), //159
            ("mi7(no 5 add 11)", Change::from_notes_string("1 ♭3 4 ♭7")), //160
            ("miMa7(no 5 add 11)", Change::from_notes_string("1 ♭3 4 7")), //161
            ("mi(add ♯11)", Change::from_notes_string("1 ♭3 ♯4 5")), //162
            ("dim(add ♭6)", Change::from_notes_string("1 ♭3 ♯4 ♯5")), //163
            ("dim7", Change::from_notes_string("1 ♭3 ♭5 6")), //164
            ("⌀7", Change::from_notes_string("1 ♭3 ♭5 ♭7")), //165
            ("dimMa7", Change::from_notes_string("1 ♭3 ♭5 7")), //166
            ("mi(add ♭6)", Change::from_notes_string("1 ♭3 5 ♭6")), //167
            ("mi6", Change::from_notes_string("1 ♭3 5 6")), //168
            ("mi7", Change::from_notes_string("1 ♭3 5 ♭7")), //169
            ("miMa7", Change::from_notes_string("1 ♭3 5 7")), //170
            ("mi6(♯5)", Change::from_notes_string("1 ♭3 ♯5 6")), //171
            ("mi7(♯5)", Change::from_notes_string("1 ♭3 ♯5 ♭7")), //172
            ("miMa7(♯5)", Change::from_notes_string("1 ♭3 ♯5 7")), //173
            ("mi7(no 5 add 13)", Change::from_notes_string("1 ♭3 6 ♭7")), //174
            ("miMa7(no 5 add 13)", Change::from_notes_string("1 ♭3 6 7")), //175
            ("miMa7(no 5 add ♭7)", Change::from_notes_string("1 ♭3 ♯6 7")), //176
            ("Ma(♭5 add 11)", Change::from_notes_string("1 3 4 ♭5")), //177
            ("Ma(add 11)", Change::from_notes_string("1 3 4 5")), //178
            ("Ma(♯5 add 11)", Change::from_notes_string("1 3 4 ♯5")), //179
            ("6(no 5 add 11)", Change::from_notes_string("1 3 4 6")), //180
            ("7(no 5 add 11)", Change::from_notes_string("1 3 4 ♭7")), //181
            ("Ma7(no 5 add 11)", Change::from_notes_string("1 3 4 7")), //182
            ("Ma(add ♯11)", Change::from_notes_string("1 3 ♯4 5")), //183
            ("Ma(♭5 add ♭6)", Change::from_notes_string("1 3 ♯4 ♯5")), //184
            ("6(♭5)", Change::from_notes_string("1 3 ♭5 6")), //185
            ("7(♭5)", Change::from_notes_string("1 3 ♭5 ♭7")), //186
            ("Ma7(♭5)", Change::from_notes_string("1 3 ♭5 7")), //187
            ("Ma(add ♭6)", Change::from_notes_string("1 3 5 ♭6")), //188
            ("6", Change::from_notes_string("1 3 5 6")), //189
            ("7", Change::from_notes_string("1 3 5 ♭7")), //190
            ("Ma7", Change::from_notes_string("1 3 5 7")), //191
            ("6(♯5)", Change::from_notes_string("1 3 ♯5 6")), //192
            ("7(♯5)", Change::from_notes_string("1 3 ♯5 ♭7")), //193
            ("Ma7(♯5)", Change::from_notes_string("1 3 ♯5 7")), //194
            ("7(no 5 add 13)", Change::from_notes_string("1 3 6 ♭7")), //195
            ("Ma7(no 5 add 13)", Change::from_notes_string("1 3 6 7")), //196
            ("Ma7(no 5 add ♭7)", Change::from_notes_string("1 3 ♯6 7")), //197
            ("sus4(add ♯11)", Change::from_notes_string("1 4 ♯4 5")), //198
            ("sus4(♭5 add ♭6)", Change::from_notes_string("1 4 ♭5 ♭6")), //199
            ("6sus4(♭5)", Change::from_notes_string("1 4 ♭5 6")), //200
            ("7sus4(♭5)", Change::from_notes_string("1 4 ♭5 ♭7")), //201
            ("Ma7sus4(♭5)", Change::from_notes_string("1 4 ♭5 7")), //202
            ("sus4(add ♭6)", Change::from_notes_string("1 4 5 ♭6")), //203
            ("6sus4", Change::from_notes_string("1 4 5 6")), //204
            ("7sus4", Change::from_notes_string("1 4 5 ♭7")), //205
            ("Ma7sus4", Change::from_notes_string("1 4 5 7")), //206
            ("6sus4(♯5)", Change::from_notes_string("1 4 ♯5 6")), //207
            ("7sus4(♯5)", Change::from_notes_string("1 4 ♯5 ♭7")), //208
            ("Ma7sus4(♯5)", Change::from_notes_string("1 4 ♯5 7")), //209
            ("7sus4(no 5 add 13)", Change::from_notes_string("1 4 6 ♭7")), //210
            ("Ma7sus4(no 5 add 13)", Change::from_notes_string("1 4 6 7")), //211
            ("Ma7sus4(no 5 add ♭7)", Change::from_notes_string("1 4 ♯6 7")), //212
            ("sus♯4(add ♭6)", Change::from_notes_string("1 ♯4 5 ♭6")), //213
            ("6sus♯4", Change::from_notes_string("1 ♯4 5 6")), //214
            ("7sus♯4", Change::from_notes_string("1 ♯4 5 ♭7")), //215
            ("Ma7sus♯4", Change::from_notes_string("1 ♯4 5 7")), //216
            ("sus♯4(♯5 add 6)", Change::from_notes_string("1 ♯4 ♯5 6")), //217
            ("7(♭5 no 3 add ♭13)", Change::from_notes_string("1 ♯4 ♯5 ♭7")), //218
            ("Ma7(♭5 no 3 add ♭13)", Change::from_notes_string("1 ♯4 ♯5 7")), //219
            ("7(♭5 no 3 add 13)", Change::from_notes_string("1 ♭5 6 ♭7")), //220
            ("Ma7(♭5 no 3 add 13)", Change::from_notes_string("1 ♭5 6 7")), //221
            ("Ma7(♭5 no 3 add ♭7)", Change::from_notes_string("1 ♭5 ♯6 7")), //222
            ("5(add ♭6 6)", Change::from_notes_string("1 5 ♭6 6")), //223
            ("7(no 3 add ♭13)", Change::from_notes_string("1 5 ♭6 ♭7")), //224
            ("Ma7(no 3 add ♭13)", Change::from_notes_string("1 5 ♭6 7")), //225
            ("7(no 3 add 13)", Change::from_notes_string("1 5 6 ♭7")), //226
            ("Ma7(no 3 add 13)", Change::from_notes_string("1 5 6 7")), //227
            ("5(add ♭7 7)", Change::from_notes_string("1 5 ♯6 7")), //228
            ("7(♯5 no 3 add 13)", Change::from_notes_string("1 ♯5 6 ♭7")), //229
            ("Ma7(♯5 no 3 add 13)", Change::from_notes_string("1 ♯5 6 7")), //230
            ("Ma7(♯5 no 3 add ♭7)", Change::from_notes_string("1 ♯5 ♯6 7")), //231
            ("Ma7(no 3 5 add ♭7 13)", Change::from_notes_string("1 6 ♭7 7")), //232
            ("Ma(no 5 add ♭9 9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3")), //233
            ("mi11(no 5 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4")), //234
            ("dim(add ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5")), //235
            ("mi(add ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 5")), //236
            ("mi(♯5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5")), //237
            ("mi6/9(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 6")), //238
            ("mi9(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♭7")), //239
            ("miMa9(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 7")), //240
            ("11(no 5 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4")), //241
            ("Ma(♭5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 3 ♭5")), //242
            ("Ma(add ♭9 9)", Change::from_notes_string("1 ♭2 2 3 5")), //243
            ("Ma(♯5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 3 ♯5")), //244
            ("6/9(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 6")), //245
            ("9(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♭7")), //246
            ("Ma9(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 7")), //247
            ("sus4(♭5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 4 ♭5")), //248
            ("sus4(add ♭9 9)", Change::from_notes_string("1 ♭2 2 4 5")), //249
            ("sus4(♯5 add ♭9 9)", Change::from_notes_string("1 ♭2 2 4 ♯5")), //250
            ("6/9sus4(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 4 6")), //251
            ("11sus♭2(no 5)", Change::from_notes_string("1 ♭2 2 4 ♭7")), //252
            ("Ma11sus♭2(no 5)", Change::from_notes_string("1 ♭2 2 4 7")), //253
            ("sus2(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♯4 5")), //254
            ("sus2(♭5 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5")), //255
            ("6sus2(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭5 6")), //256
            ("9sus♭2(♭5)", Change::from_notes_string("1 ♭2 2 ♭5 ♭7")), //257
            ("Ma9sus♭2(♭5)", Change::from_notes_string("1 ♭2 2 ♭5 7")), //258
            ("sus2(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 5 ♭6")), //259
            ("6sus2(add ♭9)", Change::from_notes_string("1 ♭2 2 5 6")), //260
            ("9sus♭2", Change::from_notes_string("1 ♭2 2 5 ♭7")), //261
            ("Ma9sus♭2", Change::from_notes_string("1 ♭2 2 5 7")), //262
            ("6sus2(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♯5 6")), //263
            ("9sus♭2(♯5)", Change::from_notes_string("1 ♭2 2 ♯5 ♭7")), //264
            ("Ma9sus♭2(♯5)", Change::from_notes_string("1 ♭2 2 ♯5 7")), //265
            ("9sus♭2(no 5 add 13)", Change::from_notes_string("1 ♭2 2 6 ♭7")), //266
            ("Ma9sus♭2(no 5 add 13)", Change::from_notes_string("1 ♭2 2 6 7")), //267
            ("Ma9sus♭2(no 5 add ♭7)", Change::from_notes_string("1 ♭2 2 ♯6 7")), //268
            ("Ma(no 5 add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4")), //269
            ("Ma(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5")), //270
            ("Ma(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5")), //271
            ("Ma(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5")), //272
            ("6(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 6")), //273
            ("7(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♭7")), //274
            ("Ma7(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 7")), //275
            ("dim(add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5")), //276
            ("mi(add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 5")), //277
            ("mi(♯5 add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5")), //278
            ("mi6(no 5 add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 6")), //279
            ("mi11(♭9 no 5)", Change::from_notes_string("1 ♭2 ♭3 4 ♭7")), //280
            ("miMa11(♭9 no 5)", Change::from_notes_string("1 ♭2 ♭3 4 7")), //281
            ("mi(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5")), //282
            ("dim(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5")), //283
            ("dim7(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♭5 6")), //284
            ("⌀7(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♭5 ♭7")), //285
            ("dimMa7(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♭5 7")), //286
            ("mi(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6")), //287
            ("mi6(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 5 6")), //288
            ("mi7(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 5 ♭7")), //289
            ("miMa7(add ♭9)", Change::from_notes_string("1 ♭2 ♭3 5 7")), //290
            ("mi6(♯5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯5 6")), //291
            ("mi7(♯5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯5 ♭7")), //292
            ("miMa7(♯5 add ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯5 7")), //293
            ("mi7(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 6 ♭7")), //294
            ("miMa7(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 6 7")), //295
            ("miMa7(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯6 7")), //296
            ("Ma(♭5 add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 ♭5")), //297
            ("Ma(add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 5")), //298
            ("Ma(♯5 add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 ♯5")), //299
            ("6(no 5 add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 6")), //300
            ("11(♭9 no 5)", Change::from_notes_string("1 ♭2 3 4 ♭7")), //301
            ("Ma11(♭9 no 5)", Change::from_notes_string("1 ♭2 3 4 7")), //302
            ("Ma(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5")), //303
            ("Ma(♭5 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5")), //304
            ("6(♭5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♭5 6")), //305
            ("7(♭5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♭5 ♭7")), //306
            ("Ma7(♭5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♭5 7")), //307
            ("Ma(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 3 5 ♭6")), //308
            ("6(add ♭9)", Change::from_notes_string("1 ♭2 3 5 6")), //309
            ("7(add ♭9)", Change::from_notes_string("1 ♭2 3 5 ♭7")), //310
            ("Ma7(add ♭9)", Change::from_notes_string("1 ♭2 3 5 7")), //311
            ("6(♯5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♯5 6")), //312
            ("7(♯5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♯5 ♭7")), //313
            ("Ma7(♯5 add ♭9)", Change::from_notes_string("1 ♭2 3 ♯5 7")), //314
            ("7(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 3 6 ♭7")), //315
            ("Ma7(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 3 6 7")), //316
            ("Ma7(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 3 ♯6 7")), //317
            ("sus4(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 4 ♯4 5")), //318
            ("sus4(♭5 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6")), //319
            ("6sus4(♭5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♭5 6")), //320
            ("7sus4(♭5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♭5 ♭7")), //321
            ("Ma7sus4(♭5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♭5 7")), //322
            ("sus4(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 4 5 ♭6")), //323
            ("6sus4(add ♭9)", Change::from_notes_string("1 ♭2 4 5 6")), //324
            ("7sus4(add ♭9)", Change::from_notes_string("1 ♭2 4 5 ♭7")), //325
            ("Ma7sus4(add ♭9)", Change::from_notes_string("1 ♭2 4 5 7")), //326
            ("6sus4(♯5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♯5 6")), //327
            ("7sus4(♯5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♯5 ♭7")), //328
            ("Ma7sus4(♯5 add ♭9)", Change::from_notes_string("1 ♭2 4 ♯5 7")), //329
            ("13(♭9 no 3 5)", Change::from_notes_string("1 ♭2 4 6 ♭7")), //330
            ("Ma13(♭9 no 3 5)", Change::from_notes_string("1 ♭2 4 6 7")), //331
            ("Ma7sus4(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 4 ♯6 7")), //332
            ("sus♯4(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6")), //333
            ("6sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 ♯4 5 6")), //334
            ("7sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 ♯4 5 ♭7")), //335
            ("Ma7sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 ♯4 5 7")), //336
            ("6sus♭2(♭5 add ♭6)", Change::from_notes_string("1 ♭2 ♯4 ♯5 6")), //337
            ("7sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 ♯4 ♯5 ♭7")), //338
            ("Ma7sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 ♯4 ♯5 7")), //339
            ("7sus♭2(♭5 add 13)", Change::from_notes_string("1 ♭2 ♭5 6 ♭7")), //340
            ("Ma7sus♭2(♭5 add 13)", Change::from_notes_string("1 ♭2 ♭5 6 7")), //341
            ("Ma7sus♭2(♭5 add ♭7)", Change::from_notes_string("1 ♭2 ♭5 ♯6 7")), //342
            ("6sus♭2(add ♭6)", Change::from_notes_string("1 ♭2 5 ♭6 6")), //343
            ("7sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 5 ♭6 ♭7")), //344
            ("Ma7sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 5 ♭6 7")), //345
            ("7sus♭2(add 13)", Change::from_notes_string("1 ♭2 5 6 ♭7")), //346
            ("Ma7sus♭2(add 13)", Change::from_notes_string("1 ♭2 5 6 7")), //347
            ("Ma7sus♭2(add ♭7)", Change::from_notes_string("1 ♭2 5 ♯6 7")), //348
            ("7sus♭2(♯5 add 13)", Change::from_notes_string("1 ♭2 ♯5 6 ♭7")), //349
            ("Ma7sus♭2(♯5 add 13)", Change::from_notes_string("1 ♭2 ♯5 6 7")), //350
            ("Ma7sus♭2(♯5 add ♭7)", Change::from_notes_string("1 ♭2 ♯5 ♯6 7")), //351
            ("Ma7sus♭2(no 5 add ♭7 13)", Change::from_notes_string("1 ♭2 6 ♭7 7")), //352
            ("11(no 5 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4")), //353
            ("Ma(♭5 add 9 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♭5")), //354
            ("Ma(add 9 ♯9)", Change::from_notes_string("1 2 ♯2 3 5")), //355
            ("Ma(♯5 add 9 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯5")), //356
            ("6/9(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 6")), //357
            ("9(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♭7")), //358
            ("Ma9(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 7")), //359
            ("dim11(no 7)", Change::from_notes_string("1 2 ♭3 4 ♭5")), //360
            ("mi11(no 7)", Change::from_notes_string("1 2 ♭3 4 5")), //361
            ("mi11(♯5 no 7)", Change::from_notes_string("1 2 ♭3 4 ♯5")), //362
            ("mi13(no 5 7)", Change::from_notes_string("1 2 ♭3 4 6")), //363
            ("mi11(no 5)", Change::from_notes_string("1 2 ♭3 4 ♭7")), //364
            ("miMa11(no 5)", Change::from_notes_string("1 2 ♭3 4 7")), //365
            ("mi(add 9 ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5")), //366
            ("dim(add ♭6 9)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5")), //367
            ("dim7(add 9)", Change::from_notes_string("1 2 ♭3 ♭5 6")), //368
            ("⌀9", Change::from_notes_string("1 2 ♭3 ♭5 ♭7")), //369
            ("dimMa9", Change::from_notes_string("1 2 ♭3 ♭5 7")), //370
            ("mi(add ♭6 9)", Change::from_notes_string("1 2 ♭3 5 ♭6")), //371
            ("mi6/9", Change::from_notes_string("1 2 ♭3 5 6")), //372
            ("mi9", Change::from_notes_string("1 2 ♭3 5 ♭7")), //373
            ("miMa9", Change::from_notes_string("1 2 ♭3 5 7")), //374
            ("mi6/9(♯5)", Change::from_notes_string("1 2 ♭3 ♯5 6")), //375
            ("mi9(♯5)", Change::from_notes_string("1 2 ♭3 ♯5 ♭7")), //376
            ("miMa9(♯5)", Change::from_notes_string("1 2 ♭3 ♯5 7")), //377
            ("mi9(no 5 add 13)", Change::from_notes_string("1 2 ♭3 6 ♭7")), //378
            ("miMa9(no 5 add 13)", Change::from_notes_string("1 2 ♭3 6 7")), //379
            ("miMa9(no 5 add ♭7)", Change::from_notes_string("1 2 ♭3 ♯6 7")), //380
            ("11(♭5 no 7)", Change::from_notes_string("1 2 3 4 ♭5")), //381
            ("11(no 7)", Change::from_notes_string("1 2 3 4 5")), //382
            ("11(♯5 no 7)", Change::from_notes_string("1 2 3 4 ♯5")), //383
            ("13(no 5 7)", Change::from_notes_string("1 2 3 4 6")), //384
            ("11(no 5)", Change::from_notes_string("1 2 3 4 ♭7")), //385
            ("Ma11(no 5)", Change::from_notes_string("1 2 3 4 7")), //386
            ("Ma(add 9 ♯11)", Change::from_notes_string("1 2 3 ♯4 5")), //387
            ("Ma(♭5 add ♭6 9)", Change::from_notes_string("1 2 3 ♯4 ♯5")), //388
            ("6/9(♭5)", Change::from_notes_string("1 2 3 ♭5 6")), //389
            ("9(♭5)", Change::from_notes_string("1 2 3 ♭5 ♭7")), //390
            ("Ma9(♭5)", Change::from_notes_string("1 2 3 ♭5 7")), //391
            ("Ma(add ♭6 9)", Change::from_notes_string("1 2 3 5 ♭6")), //392
            ("6/9", Change::from_notes_string("1 2 3 5 6")), //393
            ("9", Change::from_notes_string("1 2 3 5 ♭7")), //394
            ("Ma9", Change::from_notes_string("1 2 3 5 7")), //395
            ("6/9(♯5)", Change::from_notes_string("1 2 3 ♯5 6")), //396
            ("9(♯5)", Change::from_notes_string("1 2 3 ♯5 ♭7")), //397
            ("Ma9(♯5)", Change::from_notes_string("1 2 3 ♯5 7")), //398
            ("9(no 5 add 13)", Change::from_notes_string("1 2 3 6 ♭7")), //399
            ("Ma9(no 5 add 13)", Change::from_notes_string("1 2 3 6 7")), //400
            ("Ma9(no 5 add ♭7)", Change::from_notes_string("1 2 3 ♯6 7")), //401
            ("sus4(add 9 ♯11)", Change::from_notes_string("1 2 4 ♯4 5")), //402
            ("sus4(♭5 add ♭6 9)", Change::from_notes_string("1 2 4 ♭5 ♭6")), //403
            ("6/9sus4(♭5)", Change::from_notes_string("1 2 4 ♭5 6")), //404
            ("9sus4(♭5)", Change::from_notes_string("1 2 4 ♭5 ♭7")), //405
            ("Ma9sus4(♭5)", Change::from_notes_string("1 2 4 ♭5 7")), //406
            ("sus4(add ♭6 9)", Change::from_notes_string("1 2 4 5 ♭6")), //407
            ("6/9sus4", Change::from_notes_string("1 2 4 5 6")), //408
            ("9sus4", Change::from_notes_string("1 2 4 5 ♭7")), //409
            ("Ma9sus4", Change::from_notes_string("1 2 4 5 7")), //410
            ("6/9sus4(♯5)", Change::from_notes_string("1 2 4 ♯5 6")), //411
            ("9sus4(♯5)", Change::from_notes_string("1 2 4 ♯5 ♭7")), //412
            ("Ma9sus4(♯5)", Change::from_notes_string("1 2 4 ♯5 7")), //413
            ("13(no 3 5)", Change::from_notes_string("1 2 4 6 ♭7")), //414
            ("Ma13(no 3 5)", Change::from_notes_string("1 2 4 6 7")), //415
            ("Ma9sus4(no 5 add ♭7)", Change::from_notes_string("1 2 4 ♯6 7")), //416
            ("sus2(add ♭6 ♯11)", Change::from_notes_string("1 2 ♯4 5 ♭6")), //417
            ("6sus2(add ♯11)", Change::from_notes_string("1 2 ♯4 5 6")), //418
            ("9sus♯4", Change::from_notes_string("1 2 ♯4 5 ♭7")), //419
            ("Ma9sus♯4", Change::from_notes_string("1 2 ♯4 5 7")), //420
            ("6sus2(♭5 add ♭6)", Change::from_notes_string("1 2 ♯4 ♯5 6")), //421
            ("7sus2(♭5 add ♭13)", Change::from_notes_string("1 2 ♯4 ♯5 ♭7")), //422
            ("Ma7sus2(♭5 add ♭13)", Change::from_notes_string("1 2 ♯4 ♯5 7")), //423
            ("7sus2(♭5 add 13)", Change::from_notes_string("1 2 ♭5 6 ♭7")), //424
            ("Ma7sus2(♭5 add 13)", Change::from_notes_string("1 2 ♭5 6 7")), //425
            ("Ma7sus2(♭5 add ♭7)", Change::from_notes_string("1 2 ♭5 ♯6 7")), //426
            ("6sus2(add ♭6)", Change::from_notes_string("1 2 5 ♭6 6")), //427
            ("7sus2(add ♭13)", Change::from_notes_string("1 2 5 ♭6 ♭7")), //428
            ("Ma7sus2(add ♭13)", Change::from_notes_string("1 2 5 ♭6 7")), //429
            ("7sus2(add 13)", Change::from_notes_string("1 2 5 6 ♭7")), //430
            ("Ma7sus2(add 13)", Change::from_notes_string("1 2 5 6 7")), //431
            ("Ma7sus2(add ♭7)", Change::from_notes_string("1 2 5 ♯6 7")), //432
            ("7sus2(♯5 add 13)", Change::from_notes_string("1 2 ♯5 6 ♭7")), //433
            ("Ma7sus2(♯5 add 13)", Change::from_notes_string("1 2 ♯5 6 7")), //434
            ("Ma7sus2(♯5 add ♭7)", Change::from_notes_string("1 2 ♯5 ♯6 7")), //435
            ("Ma7sus2(no 5 add ♭7 13)", Change::from_notes_string("1 2 6 ♭7 7")), //436
            ("Ma(♭5 add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 ♭5")), //437
            ("Ma(add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 5")), //438
            ("Ma(♯5 add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 ♯5")), //439
            ("6(no 5 add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 6")), //440
            ("11(♯9 no 5)", Change::from_notes_string("1 ♯2 3 4 ♭7")), //441
            ("Ma11(♯9 no 5)", Change::from_notes_string("1 ♯2 3 4 7")), //442
            ("Ma(add ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5")), //443
            ("Ma(♭5 add ♭6 ♯9)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5")), //444
            ("6(♭5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♭5 6")), //445
            ("7(♭5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♭5 ♭7")), //446
            ("Ma7(♭5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♭5 7")), //447
            ("Ma(add ♭6 ♯9)", Change::from_notes_string("1 ♯2 3 5 ♭6")), //448
            ("6(add ♯9)", Change::from_notes_string("1 ♯2 3 5 6")), //449
            ("7(add ♯9)", Change::from_notes_string("1 ♯2 3 5 ♭7")), //450
            ("Ma7(add ♯9)", Change::from_notes_string("1 ♯2 3 5 7")), //451
            ("6(♯5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♯5 6")), //452
            ("7(♯5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♯5 ♭7")), //453
            ("Ma7(♯5 add ♯9)", Change::from_notes_string("1 ♯2 3 ♯5 7")), //454
            ("7(no 5 add ♯9 13)", Change::from_notes_string("1 ♯2 3 6 ♭7")), //455
            ("Ma7(no 5 add ♯9 13)", Change::from_notes_string("1 ♯2 3 6 7")), //456
            ("Ma7(no 5 add ♭7 ♯9)", Change::from_notes_string("1 ♯2 3 ♯6 7")), //457
            ("mi(add 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5")), //458
            ("dim(add ♭6 11)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6")), //459
            ("dim7(add 11)", Change::from_notes_string("1 ♭3 4 ♭5 6")), //460
            ("⌀7(add 11)", Change::from_notes_string("1 ♭3 4 ♭5 ♭7")), //461
            ("dimMa7(add 11)", Change::from_notes_string("1 ♭3 4 ♭5 7")), //462
            ("mi(add ♭6 11)", Change::from_notes_string("1 ♭3 4 5 ♭6")), //463
            ("mi6(add 11)", Change::from_notes_string("1 ♭3 4 5 6")), //464
            ("mi7(add 11)", Change::from_notes_string("1 ♭3 4 5 ♭7")), //465
            ("miMa7(add 11)", Change::from_notes_string("1 ♭3 4 5 7")), //466
            ("mi6(♯5 add 11)", Change::from_notes_string("1 ♭3 4 ♯5 6")), //467
            ("mi7(♯5 add 11)", Change::from_notes_string("1 ♭3 4 ♯5 ♭7")), //468
            ("miMa7(♯5 add 11)", Change::from_notes_string("1 ♭3 4 ♯5 7")), //469
            ("mi13(no 5 9)", Change::from_notes_string("1 ♭3 4 6 ♭7")), //470
            ("miMa13(no 5 9)", Change::from_notes_string("1 ♭3 4 6 7")), //471
            ("miMa7(no 5 add ♭7 11)", Change::from_notes_string("1 ♭3 4 ♯6 7")), //472
            ("mi(add ♭6 ♯11)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6")), //473
            ("mi6(add ♯11)", Change::from_notes_string("1 ♭3 ♯4 5 6")), //474
            ("mi7(add ♯11)", Change::from_notes_string("1 ♭3 ♯4 5 ♭7")), //475
            ("miMa7(add ♯11)", Change::from_notes_string("1 ♭3 ♯4 5 7")), //476
            ("dim7(add ♭6)", Change::from_notes_string("1 ♭3 ♯4 ♯5 6")), //477
            ("⌀7(add ♭13)", Change::from_notes_string("1 ♭3 ♯4 ♯5 ♭7")), //478
            ("dimMa7(add ♭13)", Change::from_notes_string("1 ♭3 ♯4 ♯5 7")), //479
            ("⌀7(add 13)", Change::from_notes_string("1 ♭3 ♭5 6 ♭7")), //480
            ("dimMa7(add 13)", Change::from_notes_string("1 ♭3 ♭5 6 7")), //481
            ("dimMa7(add ♭7)", Change::from_notes_string("1 ♭3 ♭5 ♯6 7")), //482
            ("mi6(add ♭6)", Change::from_notes_string("1 ♭3 5 ♭6 6")), //483
            ("mi7(add ♭13)", Change::from_notes_string("1 ♭3 5 ♭6 ♭7")), //484
            ("miMa7(add ♭13)", Change::from_notes_string("1 ♭3 5 ♭6 7")), //485
            ("mi7(add 13)", Change::from_notes_string("1 ♭3 5 6 ♭7")), //486
            ("miMa7(add 13)", Change::from_notes_string("1 ♭3 5 6 7")), //487
            ("miMa7(add ♭7)", Change::from_notes_string("1 ♭3 5 ♯6 7")), //488
            ("mi7(♯5 add 13)", Change::from_notes_string("1 ♭3 ♯5 6 ♭7")), //489
            ("miMa7(♯5 add 13)", Change::from_notes_string("1 ♭3 ♯5 6 7")), //490
            ("miMa7(♯5 add ♭7)", Change::from_notes_string("1 ♭3 ♯5 ♯6 7")), //491
            ("miMa7(no 5 add ♭7 13)", Change::from_notes_string("1 ♭3 6 ♭7 7")), //492
            ("Ma(add 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5")), //493
            ("Ma(♭5 add ♭6 11)", Change::from_notes_string("1 3 4 ♭5 ♭6")), //494
            ("6(♭5 add 11)", Change::from_notes_string("1 3 4 ♭5 6")), //495
            ("7(♭5 add 11)", Change::from_notes_string("1 3 4 ♭5 ♭7")), //496
            ("Ma7(♭5 add 11)", Change::from_notes_string("1 3 4 ♭5 7")), //497
            ("Ma(add ♭6 11)", Change::from_notes_string("1 3 4 5 ♭6")), //498
            ("6(add 11)", Change::from_notes_string("1 3 4 5 6")), //499
            ("7(add 11)", Change::from_notes_string("1 3 4 5 ♭7")), //500
            ("Ma7(add 11)", Change::from_notes_string("1 3 4 5 7")), //501
            ("6(♯5 add 11)", Change::from_notes_string("1 3 4 ♯5 6")), //502
            ("7(♯5 add 11)", Change::from_notes_string("1 3 4 ♯5 ♭7")), //503
            ("Ma7(♯5 add 11)", Change::from_notes_string("1 3 4 ♯5 7")), //504
            ("13(no 5 9)", Change::from_notes_string("1 3 4 6 ♭7")), //505
            ("Ma13(no 5 9)", Change::from_notes_string("1 3 4 6 7")), //506
            ("Ma7(no 5 add ♭7 11)", Change::from_notes_string("1 3 4 ♯6 7")), //507
            ("Ma(add ♭6 ♯11)", Change::from_notes_string("1 3 ♯4 5 ♭6")), //508
            ("6(add ♯11)", Change::from_notes_string("1 3 ♯4 5 6")), //509
            ("7(add ♯11)", Change::from_notes_string("1 3 ♯4 5 ♭7")), //510
            ("Ma7(add ♯11)", Change::from_notes_string("1 3 ♯4 5 7")), //511
            ("6(♭5 add ♭6)", Change::from_notes_string("1 3 ♯4 ♯5 6")), //512
            ("7(♭5 add ♭13)", Change::from_notes_string("1 3 ♯4 ♯5 ♭7")), //513
            ("Ma7(♭5 add ♭13)", Change::from_notes_string("1 3 ♯4 ♯5 7")), //514
            ("7(♭5 add 13)", Change::from_notes_string("1 3 ♭5 6 ♭7")), //515
            ("Ma7(♭5 add 13)", Change::from_notes_string("1 3 ♭5 6 7")), //516
            ("Ma7(♭5 add ♭7)", Change::from_notes_string("1 3 ♭5 ♯6 7")), //517
            ("6(add ♭6)", Change::from_notes_string("1 3 5 ♭6 6")), //518
            ("7(add ♭13)", Change::from_notes_string("1 3 5 ♭6 ♭7")), //519
            ("Ma7(add ♭13)", Change::from_notes_string("1 3 5 ♭6 7")), //520
            ("7(add 13)", Change::from_notes_string("1 3 5 6 ♭7")), //521
            ("Ma7(add 13)", Change::from_notes_string("1 3 5 6 7")), //522
            ("Ma7(add ♭7)", Change::from_notes_string("1 3 5 ♯6 7")), //523
            ("7(♯5 add 13)", Change::from_notes_string("1 3 ♯5 6 ♭7")), //524
            ("Ma7(♯5 add 13)", Change::from_notes_string("1 3 ♯5 6 7")), //525
            ("Ma7(♯5 add ♭7)", Change::from_notes_string("1 3 ♯5 ♯6 7")), //526
            ("Ma7(no 5 add ♭7 13)", Change::from_notes_string("1 3 6 ♭7 7")), //527
            ("sus4(add ♭6 ♯11)", Change::from_notes_string("1 4 ♯4 5 ♭6")), //528
            ("6sus4(add ♯11)", Change::from_notes_string("1 4 ♯4 5 6")), //529
            ("7sus4(add ♯11)", Change::from_notes_string("1 4 ♯4 5 ♭7")), //530
            ("Ma7sus4(add ♯11)", Change::from_notes_string("1 4 ♯4 5 7")), //531
            ("6sus4(♭5 add ♭6)", Change::from_notes_string("1 4 ♭5 ♭6 6")), //532
            ("7sus4(♭5 add ♭13)", Change::from_notes_string("1 4 ♭5 ♭6 ♭7")), //533
            ("Ma7sus4(♭5 add ♭13)", Change::from_notes_string("1 4 ♭5 ♭6 7")), //534
            ("7sus4(♭5 add 13)", Change::from_notes_string("1 4 ♭5 6 ♭7")), //535
            ("Ma7sus4(♭5 add 13)", Change::from_notes_string("1 4 ♭5 6 7")), //536
            ("Ma7sus4(♭5 add ♭7)", Change::from_notes_string("1 4 ♭5 ♯6 7")), //537
            ("6sus4(add ♭6)", Change::from_notes_string("1 4 5 ♭6 6")), //538
            ("7sus4(add ♭13)", Change::from_notes_string("1 4 5 ♭6 ♭7")), //539
            ("Ma7sus4(add ♭13)", Change::from_notes_string("1 4 5 ♭6 7")), //540
            ("7sus4(add 13)", Change::from_notes_string("1 4 5 6 ♭7")), //541
            ("Ma7sus4(add 13)", Change::from_notes_string("1 4 5 6 7")), //542
            ("Ma7sus4(add ♭7)", Change::from_notes_string("1 4 5 ♯6 7")), //543
            ("7sus4(♯5 add 13)", Change::from_notes_string("1 4 ♯5 6 ♭7")), //544
            ("Ma7sus4(♯5 add 13)", Change::from_notes_string("1 4 ♯5 6 7")), //545
            ("Ma7sus4(♯5 add ♭7)", Change::from_notes_string("1 4 ♯5 ♯6 7")), //546
            ("Ma7sus4(no 5 add ♭7 13)", Change::from_notes_string("1 4 6 ♭7 7")), //547
            ("6sus♯4(add ♭6)", Change::from_notes_string("1 ♯4 5 ♭6 6")), //548
            ("7sus♯4(add ♭13)", Change::from_notes_string("1 ♯4 5 ♭6 ♭7")), //549
            ("Ma7sus♯4(add ♭13)", Change::from_notes_string("1 ♯4 5 ♭6 7")), //550
            ("7sus♯4(add 13)", Change::from_notes_string("1 ♯4 5 6 ♭7")), //551
            ("Ma7sus♯4(add 13)", Change::from_notes_string("1 ♯4 5 6 7")), //552
            ("Ma7sus♯4(add ♭7)", Change::from_notes_string("1 ♯4 5 ♯6 7")), //553
            ("7(♭5 no 3 add ♭13 13)", Change::from_notes_string("1 ♯4 ♯5 6 ♭7")), //554
            ("Ma7(♭5 no 3 add ♭13 13)", Change::from_notes_string("1 ♯4 ♯5 6 7")), //555
            ("sus♯4(♯5 add ♭7 7)", Change::from_notes_string("1 ♯4 ♯5 ♯6 7")), //556
            ("Ma7(♭5 no 3 add ♭7 13)", Change::from_notes_string("1 ♭5 6 ♭7 7")), //557
            ("7(no 3 add ♭13 13)", Change::from_notes_string("1 5 ♭6 6 ♭7")), //558
            ("Ma7(no 3 add ♭13 13)", Change::from_notes_string("1 5 ♭6 6 7")), //559
            ("5(add ♭6 ♭7 7)", Change::from_notes_string("1 5 ♭6 ♭7 7")), //560
            ("5(add 6 ♭7 7)", Change::from_notes_string("1 5 6 ♭7 7")), //561
            ("Ma7(♯5 no 3 add ♭7 13)", Change::from_notes_string("1 ♯5 6 ♭7 7")), //562
            ("11(no 5 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4")), //563
            ("Ma(♭5 add ♭9 9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5")), //564
            ("Ma(add ♭9 9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 5")), //565
            ("Ma(♯5 add ♭9 9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5")), //566
            ("6/9(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 6")), //567
            ("9(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭7")), //568
            ("Ma9(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 7")), //569
            ("dim11(no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5")), //570
            ("mi11(no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5")), //571
            ("mi11(♯5 no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5")), //572
            ("mi13(no 5 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 6")), //573
            ("mi11(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭7")), //574
            ("miMa11(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 7")), //575
            ("mi(add ♭9 9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5")), //576
            ("dim(add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5")), //577
            ("dim7(add ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 6")), //578
            ("⌀9(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 ♭7")), //579
            ("dimMa9(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 7")), //580
            ("mi(add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6")), //581
            ("mi6/9(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 5 6")), //582
            ("mi9(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭7")), //583
            ("miMa9(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 5 7")), //584
            ("mi6/9(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 6")), //585
            ("mi9(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 ♭7")), //586
            ("miMa9(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 7")), //587
            ("mi9(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 6 ♭7")), //588
            ("miMa9(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 6 7")), //589
            ("miMa9(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯6 7")), //590
            ("11(♭5 no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5")), //591
            ("11(no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5")), //592
            ("11(♯5 no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5")), //593
            ("13(no 5 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 6")), //594
            ("11(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭7")), //595
            ("Ma11(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 7")), //596
            ("Ma(add ♭9 9 ♯11)", Change::from_notes_string("1 ♭2 2 3 ♯4 5")), //597
            ("Ma(♭5 add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5")), //598
            ("6/9(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♭5 6")), //599
            ("9(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♭5 ♭7")), //600
            ("Ma9(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♭5 7")), //601
            ("Ma(add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 3 5 ♭6")), //602
            ("6/9(add ♭9)", Change::from_notes_string("1 ♭2 2 3 5 6")), //603
            ("9(add ♭9)", Change::from_notes_string("1 ♭2 2 3 5 ♭7")), //604
            ("Ma9(add ♭9)", Change::from_notes_string("1 ♭2 2 3 5 7")), //605
            ("6/9(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯5 6")), //606
            ("9(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯5 ♭7")), //607
            ("Ma9(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯5 7")), //608
            ("9(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 6 ♭7")), //609
            ("Ma9(no 5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 6 7")), //610
            ("Ma9(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯6 7")), //611
            ("sus4(add ♭9 9 ♯11)", Change::from_notes_string("1 ♭2 2 4 ♯4 5")), //612
            ("sus4(♭5 add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6")), //613
            ("6/9sus4(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 4 ♭5 6")), //614
            ("11sus♭2(♭5)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭7")), //615
            ("Ma11sus♭2(♭5)", Change::from_notes_string("1 ♭2 2 4 ♭5 7")), //616
            ("sus4(add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 4 5 ♭6")), //617
            ("6/9sus4(add ♭9)", Change::from_notes_string("1 ♭2 2 4 5 6")), //618
            ("11sus♭2", Change::from_notes_string("1 ♭2 2 4 5 ♭7")), //619
            ("Ma11sus♭2", Change::from_notes_string("1 ♭2 2 4 5 7")), //620
            ("6/9sus4(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯5 6")), //621
            ("11sus♭2(♯5)", Change::from_notes_string("1 ♭2 2 4 ♯5 ♭7")), //622
            ("Ma11sus♭2(♯5)", Change::from_notes_string("1 ♭2 2 4 ♯5 7")), //623
            ("13sus♭2(no 5)", Change::from_notes_string("1 ♭2 2 4 6 ♭7")), //624
            ("Ma13sus♭2(no 5)", Change::from_notes_string("1 ♭2 2 4 6 7")), //625
            ("Ma11sus♭2(no 5 add ♭7)", Change::from_notes_string("1 ♭2 2 4 ♯6 7")), //626
            ("sus2(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6")), //627
            ("6sus2(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♯4 5 6")), //628
            ("9sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭7")), //629
            ("Ma9sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 2 ♯4 5 7")), //630
            ("6sus2(♭5 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 6")), //631
            ("9sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 ♭7")), //632
            ("Ma9sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 7")), //633
            ("9sus♭2(♭5 add 13)", Change::from_notes_string("1 ♭2 2 ♭5 6 ♭7")), //634
            ("Ma9sus♭2(♭5 add 13)", Change::from_notes_string("1 ♭2 2 ♭5 6 7")), //635
            ("Ma9sus♭2(♭5 add ♭7)", Change::from_notes_string("1 ♭2 2 ♭5 ♯6 7")), //636
            ("6sus2(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 5 ♭6 6")), //637
            ("9sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 2 5 ♭6 ♭7")), //638
            ("Ma9sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 2 5 ♭6 7")), //639
            ("9sus♭2(add 13)", Change::from_notes_string("1 ♭2 2 5 6 ♭7")), //640
            ("Ma9sus♭2(add 13)", Change::from_notes_string("1 ♭2 2 5 6 7")), //641
            ("Ma9sus♭2(add ♭7)", Change::from_notes_string("1 ♭2 2 5 ♯6 7")), //642
            ("9sus♭2(♯5 add 13)", Change::from_notes_string("1 ♭2 2 ♯5 6 ♭7")), //643
            ("Ma9sus♭2(♯5 add 13)", Change::from_notes_string("1 ♭2 2 ♯5 6 7")), //644
            ("Ma9sus♭2(♯5 add ♭7)", Change::from_notes_string("1 ♭2 2 ♯5 ♯6 7")), //645
            ("Ma9sus♭2(no 5 add ♭7 13)", Change::from_notes_string("1 ♭2 2 6 ♭7 7")), //646
            ("Ma(♭5 add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5")), //647
            ("Ma(add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 5")), //648
            ("Ma(♯5 add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5")), //649
            ("6(no 5 add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 6")), //650
            ("11(♭9 no 5 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭7")), //651
            ("Ma11(♭9 no 5 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 7")), //652
            ("Ma(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5")), //653
            ("Ma(♭5 add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5")), //654
            ("6(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 6")), //655
            ("7(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 ♭7")), //656
            ("Ma7(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 7")), //657
            ("Ma(add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6")), //658
            ("6(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5 6")), //659
            ("7(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭7")), //660
            ("Ma7(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5 7")), //661
            ("6(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 6")), //662
            ("7(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 ♭7")), //663
            ("Ma7(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 7")), //664
            ("7(no 5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 6 ♭7")), //665
            ("Ma7(no 5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 6 7")), //666
            ("Ma7(no 5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯6 7")), //667
            ("mi(add ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5")), //668
            ("dim(add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6")), //669
            ("dim7(add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 6")), //670
            ("⌀11(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭7")), //671
            ("dimMa11(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 7")), //672
            ("mi(add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6")), //673
            ("mi6(add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 5 6")), //674
            ("mi11(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭7")), //675
            ("miMa11(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 5 7")), //676
            ("mi6(♯5 add ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 6")), //677
            ("mi11(♯5 ♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 ♭7")), //678
            ("miMa11(♯5 ♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 7")), //679
            ("mi13(♭9 no 5)", Change::from_notes_string("1 ♭2 ♭3 4 6 ♭7")), //680
            ("miMa13(♭9 no 5)", Change::from_notes_string("1 ♭2 ♭3 4 6 7")), //681
            ("miMa11(♭9 no 5 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 ♯6 7")), //682
            ("mi(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6")), //683
            ("mi6(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 6")), //684
            ("mi7(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭7")), //685
            ("miMa7(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 7")), //686
            ("dim7(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 6")), //687
            ("⌀7(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 ♭7")), //688
            ("dimMa7(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 7")), //689
            ("⌀7(add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 ♭5 6 ♭7")), //690
            ("dimMa7(add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 ♭5 6 7")), //691
            ("dimMa7(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♭5 ♯6 7")), //692
            ("mi6(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 6")), //693
            ("mi7(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 ♭7")), //694
            ("miMa7(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 7")), //695
            ("mi7(add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 5 6 ♭7")), //696
            ("miMa7(add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 5 6 7")), //697
            ("miMa7(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 ♭3 5 ♯6 7")), //698
            ("mi7(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 ♯5 6 ♭7")), //699
            ("miMa7(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 ♯5 6 7")), //700
            ("miMa7(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 ♭3 ♯5 ♯6 7")), //701
            ("miMa7(no 5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 6 ♭7 7")), //702
            ("Ma(add ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5")), //703
            ("Ma(♭5 add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6")), //704
            ("6(♭5 add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 ♭5 6")), //705
            ("11(♭5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭7")), //706
            ("Ma11(♭5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♭5 7")), //707
            ("Ma(add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 3 4 5 ♭6")), //708
            ("6(add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 5 6")), //709
            ("11(♭9)", Change::from_notes_string("1 ♭2 3 4 5 ♭7")), //710
            ("Ma11(♭9)", Change::from_notes_string("1 ♭2 3 4 5 7")), //711
            ("6(♯5 add ♭9 11)", Change::from_notes_string("1 ♭2 3 4 ♯5 6")), //712
            ("11(♯5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♯5 ♭7")), //713
            ("Ma11(♯5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♯5 7")), //714
            ("13(♭9 no 5)", Change::from_notes_string("1 ♭2 3 4 6 ♭7")), //715
            ("Ma13(♭9 no 5)", Change::from_notes_string("1 ♭2 3 4 6 7")), //716
            ("Ma11(♭9 no 5 add ♭7)", Change::from_notes_string("1 ♭2 3 4 ♯6 7")), //717
            ("Ma(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6")), //718
            ("6(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 6")), //719
            ("7(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭7")), //720
            ("Ma7(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 7")), //721
            ("6(♭5 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 6")), //722
            ("7(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 ♭7")), //723
            ("Ma7(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 7")), //724
            ("7(♭5 add ♭9 13)", Change::from_notes_string("1 ♭2 3 ♭5 6 ♭7")), //725
            ("Ma7(♭5 add ♭9 13)", Change::from_notes_string("1 ♭2 3 ♭5 6 7")), //726
            ("Ma7(♭5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 3 ♭5 ♯6 7")), //727
            ("6(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 3 5 ♭6 6")), //728
            ("7(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 3 5 ♭6 ♭7")), //729
            ("Ma7(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 3 5 ♭6 7")), //730
            ("7(add ♭9 13)", Change::from_notes_string("1 ♭2 3 5 6 ♭7")), //731
            ("Ma7(add ♭9 13)", Change::from_notes_string("1 ♭2 3 5 6 7")), //732
            ("Ma7(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 3 5 ♯6 7")), //733
            ("7(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 3 ♯5 6 ♭7")), //734
            ("Ma7(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 3 ♯5 6 7")), //735
            ("Ma7(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 3 ♯5 ♯6 7")), //736
            ("Ma7(no 5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 3 6 ♭7 7")), //737
            ("sus4(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6")), //738
            ("6sus4(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 4 ♯4 5 6")), //739
            ("11sus♯4(♭9)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭7")), //740
            ("Ma11sus♯4(♭9)", Change::from_notes_string("1 ♭2 4 ♯4 5 7")), //741
            ("6sus4(♭5 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 6")), //742
            ("7sus4(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 ♭7")), //743
            ("Ma7sus4(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 7")), //744
            ("13(♭5 ♭9 no 3)", Change::from_notes_string("1 ♭2 4 ♭5 6 ♭7")), //745
            ("Ma13(♭5 ♭9 no 3)", Change::from_notes_string("1 ♭2 4 ♭5 6 7")), //746
            ("Ma7sus4(♭5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 4 ♭5 ♯6 7")), //747
            ("6sus4(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 4 5 ♭6 6")), //748
            ("7sus4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 4 5 ♭6 ♭7")), //749
            ("Ma7sus4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 4 5 ♭6 7")), //750
            ("13(♭9 no 3)", Change::from_notes_string("1 ♭2 4 5 6 ♭7")), //751
            ("Ma13(♭9 no 3)", Change::from_notes_string("1 ♭2 4 5 6 7")), //752
            ("Ma7sus4(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 4 5 ♯6 7")), //753
            ("13(♯5 ♭9 no 3)", Change::from_notes_string("1 ♭2 4 ♯5 6 ♭7")), //754
            ("Ma13(♯5 ♭9 no 3)", Change::from_notes_string("1 ♭2 4 ♯5 6 7")), //755
            ("Ma7sus4(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 4 ♯5 ♯6 7")), //756
            ("Ma13(♭9 no 3 5 add ♭7)", Change::from_notes_string("1 ♭2 4 6 ♭7 7")), //757
            ("6sus♯4(add ♭6 ♭9)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 6")), //758
            ("7sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 ♭7")), //759
            ("Ma7sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 7")), //760
            ("13(♭9 ♯11 no 3)", Change::from_notes_string("1 ♭2 ♯4 5 6 ♭7")), //761
            ("Ma13(♭9 ♯11 no 3)", Change::from_notes_string("1 ♭2 ♯4 5 6 7")), //762
            ("Ma7sus♯4(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 ♯4 5 ♯6 7")), //763
            ("13(♯5 ♭9 ♯11 no 3)", Change::from_notes_string("1 ♭2 ♯4 ♯5 6 ♭7")), //764
            ("Ma13(♯5 ♭9 ♯11 no 3)", Change::from_notes_string("1 ♭2 ♯4 ♯5 6 7")), //765
            ("Ma7sus♭2(♭5 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♯4 ♯5 ♯6 7")), //766
            ("Ma7sus♭2(♭5 add ♭7 13)", Change::from_notes_string("1 ♭2 ♭5 6 ♭7 7")), //767
            ("7sus♭2(add ♭13 13)", Change::from_notes_string("1 ♭2 5 ♭6 6 ♭7")), //768
            ("Ma7sus♭2(add ♭13 13)", Change::from_notes_string("1 ♭2 5 ♭6 6 7")), //769
            ("Ma7sus♭2(add ♭7 ♭13)", Change::from_notes_string("1 ♭2 5 ♭6 ♭7 7")), //770
            ("Ma7sus♭2(add ♭7 13)", Change::from_notes_string("1 ♭2 5 6 ♭7 7")), //771
            ("Ma7sus♭2(♯5 add ♭7 13)", Change::from_notes_string("1 ♭2 ♯5 6 ♭7 7")), //772
            ("11(♭5 no 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5")), //773
            ("11(no 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5")), //774
            ("11(♯5 no 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5")), //775
            ("13(no 5 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 6")), //776
            ("11(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭7")), //777
            ("Ma11(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 7")), //778
            ("Ma(add 9 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 ♯4 5")), //779
            ("Ma(♭5 add ♭6 9 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5")), //780
            ("6/9(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♭5 6")), //781
            ("9(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♭5 ♭7")), //782
            ("Ma9(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♭5 7")), //783
            ("Ma(add ♭6 9 ♯9)", Change::from_notes_string("1 2 ♯2 3 5 ♭6")), //784
            ("6/9(add ♯9)", Change::from_notes_string("1 2 ♯2 3 5 6")), //785
            ("9(add ♯9)", Change::from_notes_string("1 2 ♯2 3 5 ♭7")), //786
            ("Ma9(add ♯9)", Change::from_notes_string("1 2 ♯2 3 5 7")), //787
            ("6/9(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯5 6")), //788
            ("9(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯5 ♭7")), //789
            ("Ma9(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯5 7")), //790
            ("9(no 5 add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 6 ♭7")), //791
            ("Ma9(no 5 add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 6 7")), //792
            ("Ma9(no 5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯6 7")), //793
            ("mi11(no 7 add ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5")), //794
            ("dim11(no 7 add ♭6)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6")), //795
            ("dim7(add 9 11)", Change::from_notes_string("1 2 ♭3 4 ♭5 6")), //796
            ("⌀11", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭7")), //797
            ("dimMa11", Change::from_notes_string("1 2 ♭3 4 ♭5 7")), //798
            ("mi11(no 7 add ♭6)", Change::from_notes_string("1 2 ♭3 4 5 ♭6")), //799
            ("mi13(no 7)", Change::from_notes_string("1 2 ♭3 4 5 6")), //800
            ("mi11", Change::from_notes_string("1 2 ♭3 4 5 ♭7")), //801
            ("miMa11", Change::from_notes_string("1 2 ♭3 4 5 7")), //802
            ("mi13(♯5 no 7)", Change::from_notes_string("1 2 ♭3 4 ♯5 6")), //803
            ("mi11(♯5)", Change::from_notes_string("1 2 ♭3 4 ♯5 ♭7")), //804
            ("miMa11(♯5)", Change::from_notes_string("1 2 ♭3 4 ♯5 7")), //805
            ("mi13(no 5)", Change::from_notes_string("1 2 ♭3 4 6 ♭7")), //806
            ("miMa13(no 5)", Change::from_notes_string("1 2 ♭3 4 6 7")), //807
            ("miMa11(no 5 add ♭7)", Change::from_notes_string("1 2 ♭3 4 ♯6 7")), //808
            ("mi(add ♭6 9 ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6")), //809
            ("mi6/9(add ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 6")), //810
            ("mi9(add ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭7")), //811
            ("miMa9(add ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 7")), //812
            ("dim7(add ♭6 9)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 6")), //813
            ("⌀9(add ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 ♭7")), //814
            ("dimMa9(add ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 7")), //815
            ("⌀9(add 13)", Change::from_notes_string("1 2 ♭3 ♭5 6 ♭7")), //816
            ("dimMa9(add 13)", Change::from_notes_string("1 2 ♭3 ♭5 6 7")), //817
            ("dimMa9(add ♭7)", Change::from_notes_string("1 2 ♭3 ♭5 ♯6 7")), //818
            ("mi6/9(add ♭13)", Change::from_notes_string("1 2 ♭3 5 ♭6 6")), //819
            ("mi9(add ♭13)", Change::from_notes_string("1 2 ♭3 5 ♭6 ♭7")), //820
            ("miMa9(add ♭13)", Change::from_notes_string("1 2 ♭3 5 ♭6 7")), //821
            ("mi9(add 13)", Change::from_notes_string("1 2 ♭3 5 6 ♭7")), //822
            ("miMa9(add 13)", Change::from_notes_string("1 2 ♭3 5 6 7")), //823
            ("miMa9(add ♭7)", Change::from_notes_string("1 2 ♭3 5 ♯6 7")), //824
            ("mi9(♯5 add 13)", Change::from_notes_string("1 2 ♭3 ♯5 6 ♭7")), //825
            ("miMa9(♯5 add 13)", Change::from_notes_string("1 2 ♭3 ♯5 6 7")), //826
            ("miMa9(♯5 add ♭7)", Change::from_notes_string("1 2 ♭3 ♯5 ♯6 7")), //827
            ("miMa9(no 5 add ♭7 13)", Change::from_notes_string("1 2 ♭3 6 ♭7 7")), //828
            ("11(no 7 add ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5")), //829
            ("11(♭5 no 7 add ♭6)", Change::from_notes_string("1 2 3 4 ♭5 ♭6")), //830
            ("13(♭5 no 7)", Change::from_notes_string("1 2 3 4 ♭5 6")), //831
            ("11(♭5)", Change::from_notes_string("1 2 3 4 ♭5 ♭7")), //832
            ("Ma11(♭5)", Change::from_notes_string("1 2 3 4 ♭5 7")), //833
            ("11(no 7 add ♭6)", Change::from_notes_string("1 2 3 4 5 ♭6")), //834
            ("13(no 7)", Change::from_notes_string("1 2 3 4 5 6")), //835
            ("11", Change::from_notes_string("1 2 3 4 5 ♭7")), //836
            ("Ma11", Change::from_notes_string("1 2 3 4 5 7")), //837
            ("13(♯5 no 7)", Change::from_notes_string("1 2 3 4 ♯5 6")), //838
            ("11(♯5)", Change::from_notes_string("1 2 3 4 ♯5 ♭7")), //839
            ("Ma11(♯5)", Change::from_notes_string("1 2 3 4 ♯5 7")), //840
            ("13(no 5)", Change::from_notes_string("1 2 3 4 6 ♭7")), //841
            ("Ma13(no 5)", Change::from_notes_string("1 2 3 4 6 7")), //842
            ("Ma11(no 5 add ♭7)", Change::from_notes_string("1 2 3 4 ♯6 7")), //843
            ("Ma(add ♭6 9 ♯11)", Change::from_notes_string("1 2 3 ♯4 5 ♭6")), //844
            ("6/9(add ♯11)", Change::from_notes_string("1 2 3 ♯4 5 6")), //845
            ("9(add ♯11)", Change::from_notes_string("1 2 3 ♯4 5 ♭7")), //846
            ("Ma9(add ♯11)", Change::from_notes_string("1 2 3 ♯4 5 7")), //847
            ("6/9(♭5 add ♭13)", Change::from_notes_string("1 2 3 ♯4 ♯5 6")), //848
            ("9(♭5 add ♭13)", Change::from_notes_string("1 2 3 ♯4 ♯5 ♭7")), //849
            ("Ma9(♭5 add ♭13)", Change::from_notes_string("1 2 3 ♯4 ♯5 7")), //850
            ("9(♭5 add 13)", Change::from_notes_string("1 2 3 ♭5 6 ♭7")), //851
            ("Ma9(♭5 add 13)", Change::from_notes_string("1 2 3 ♭5 6 7")), //852
            ("Ma9(♭5 add ♭7)", Change::from_notes_string("1 2 3 ♭5 ♯6 7")), //853
            ("6/9(add ♭13)", Change::from_notes_string("1 2 3 5 ♭6 6")), //854
            ("9(add ♭13)", Change::from_notes_string("1 2 3 5 ♭6 ♭7")), //855
            ("Ma9(add ♭13)", Change::from_notes_string("1 2 3 5 ♭6 7")), //856
            ("9(add 13)", Change::from_notes_string("1 2 3 5 6 ♭7")), //857
            ("Ma9(add 13)", Change::from_notes_string("1 2 3 5 6 7")), //858
            ("Ma9(add ♭7)", Change::from_notes_string("1 2 3 5 ♯6 7")), //859
            ("9(♯5 add 13)", Change::from_notes_string("1 2 3 ♯5 6 ♭7")), //860
            ("Ma9(♯5 add 13)", Change::from_notes_string("1 2 3 ♯5 6 7")), //861
            ("Ma9(♯5 add ♭7)", Change::from_notes_string("1 2 3 ♯5 ♯6 7")), //862
            ("Ma9(no 5 add ♭7 13)", Change::from_notes_string("1 2 3 6 ♭7 7")), //863
            ("sus4(add ♭6 9 ♯11)", Change::from_notes_string("1 2 4 ♯4 5 ♭6")), //864
            ("6/9sus4(add ♯11)", Change::from_notes_string("1 2 4 ♯4 5 6")), //865
            ("11sus♯4", Change::from_notes_string("1 2 4 ♯4 5 ♭7")), //866
            ("Ma11sus♯4", Change::from_notes_string("1 2 4 ♯4 5 7")), //867
            ("6/9sus4(♭5 add ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 6")), //868
            ("9sus4(♭5 add ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 ♭7")), //869
            ("Ma9sus4(♭5 add ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 7")), //870
            ("13(♭5 no 3)", Change::from_notes_string("1 2 4 ♭5 6 ♭7")), //871
            ("Ma13(♭5 no 3)", Change::from_notes_string("1 2 4 ♭5 6 7")), //872
            ("Ma9sus4(♭5 add ♭7)", Change::from_notes_string("1 2 4 ♭5 ♯6 7")), //873
            ("6/9sus4(add ♭13)", Change::from_notes_string("1 2 4 5 ♭6 6")), //874
            ("9sus4(add ♭13)", Change::from_notes_string("1 2 4 5 ♭6 ♭7")), //875
            ("Ma9sus4(add ♭13)", Change::from_notes_string("1 2 4 5 ♭6 7")), //876
            ("13(no 3)", Change::from_notes_string("1 2 4 5 6 ♭7")), //877
            ("Ma13(no 3)", Change::from_notes_string("1 2 4 5 6 7")), //878
            ("Ma9sus4(add ♭7)", Change::from_notes_string("1 2 4 5 ♯6 7")), //879
            ("13(♯5 no 3)", Change::from_notes_string("1 2 4 ♯5 6 ♭7")), //880
            ("Ma13(♯5 no 3)", Change::from_notes_string("1 2 4 ♯5 6 7")), //881
            ("Ma9sus4(♯5 add ♭7)", Change::from_notes_string("1 2 4 ♯5 ♯6 7")), //882
            ("Ma13(no 3 5 add ♭7)", Change::from_notes_string("1 2 4 6 ♭7 7")), //883
            ("6sus2(add ♭6 ♯11)", Change::from_notes_string("1 2 ♯4 5 ♭6 6")), //884
            ("9sus♯4(add ♭13)", Change::from_notes_string("1 2 ♯4 5 ♭6 ♭7")), //885
            ("Ma9sus♯4(add ♭13)", Change::from_notes_string("1 2 ♯4 5 ♭6 7")), //886
            ("13(♯11 no 3)", Change::from_notes_string("1 2 ♯4 5 6 ♭7")), //887
            ("Ma13(♯11 no 3)", Change::from_notes_string("1 2 ♯4 5 6 7")), //888
            ("Ma9sus♯4(add ♭7)", Change::from_notes_string("1 2 ♯4 5 ♯6 7")), //889
            ("13(♯5 ♯11 no 3)", Change::from_notes_string("1 2 ♯4 ♯5 6 ♭7")), //890
            ("Ma13(♯5 ♯11 no 3)", Change::from_notes_string("1 2 ♯4 ♯5 6 7")), //891
            ("Ma7sus2(♭5 add ♭7 ♭13)", Change::from_notes_string("1 2 ♯4 ♯5 ♯6 7")), //892
            ("Ma7sus2(♭5 add ♭7 13)", Change::from_notes_string("1 2 ♭5 6 ♭7 7")), //893
            ("7sus2(add ♭13 13)", Change::from_notes_string("1 2 5 ♭6 6 ♭7")), //894
            ("Ma7sus2(add ♭13 13)", Change::from_notes_string("1 2 5 ♭6 6 7")), //895
            ("Ma7sus2(add ♭7 ♭13)", Change::from_notes_string("1 2 5 ♭6 ♭7 7")), //896
            ("Ma7sus2(add ♭7 13)", Change::from_notes_string("1 2 5 6 ♭7 7")), //897
            ("Ma7sus2(♯5 add ♭7 13)", Change::from_notes_string("1 2 ♯5 6 ♭7 7")), //898
            ("Ma(add ♯9 11 ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5")), //899
            ("Ma(♭5 add ♭6 ♯9 11)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6")), //900
            ("6(♭5 add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 ♭5 6")), //901
            ("11(♭5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭7")), //902
            ("Ma11(♭5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♭5 7")), //903
            ("Ma(add ♭6 ♯9 11)", Change::from_notes_string("1 ♯2 3 4 5 ♭6")), //904
            ("6(add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 5 6")), //905
            ("11(♯9)", Change::from_notes_string("1 ♯2 3 4 5 ♭7")), //906
            ("Ma11(♯9)", Change::from_notes_string("1 ♯2 3 4 5 7")), //907
            ("6(♯5 add ♯9 11)", Change::from_notes_string("1 ♯2 3 4 ♯5 6")), //908
            ("11(♯5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♯5 ♭7")), //909
            ("Ma11(♯5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♯5 7")), //910
            ("13(♯9 no 5)", Change::from_notes_string("1 ♯2 3 4 6 ♭7")), //911
            ("Ma13(♯9 no 5)", Change::from_notes_string("1 ♯2 3 4 6 7")), //912
            ("Ma11(♯9 no 5 add ♭7)", Change::from_notes_string("1 ♯2 3 4 ♯6 7")), //913
            ("Ma(add ♭6 ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6")), //914
            ("6(add ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 6")), //915
            ("7(add ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭7")), //916
            ("Ma7(add ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 7")), //917
            ("6(♭5 add ♭6 ♯9)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 6")), //918
            ("7(♭5 add ♯9 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 ♭7")), //919
            ("Ma7(♭5 add ♯9 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 7")), //920
            ("7(♭5 add ♯9 13)", Change::from_notes_string("1 ♯2 3 ♭5 6 ♭7")), //921
            ("Ma7(♭5 add ♯9 13)", Change::from_notes_string("1 ♯2 3 ♭5 6 7")), //922
            ("Ma7(♭5 add ♭7 ♯9)", Change::from_notes_string("1 ♯2 3 ♭5 ♯6 7")), //923
            ("6(add ♭6 ♯9)", Change::from_notes_string("1 ♯2 3 5 ♭6 6")), //924
            ("7(add ♯9 ♭13)", Change::from_notes_string("1 ♯2 3 5 ♭6 ♭7")), //925
            ("Ma7(add ♯9 ♭13)", Change::from_notes_string("1 ♯2 3 5 ♭6 7")), //926
            ("7(add ♯9 13)", Change::from_notes_string("1 ♯2 3 5 6 ♭7")), //927
            ("Ma7(add ♯9 13)", Change::from_notes_string("1 ♯2 3 5 6 7")), //928
            ("Ma7(add ♭7 ♯9)", Change::from_notes_string("1 ♯2 3 5 ♯6 7")), //929
            ("7(♯5 add ♯9 13)", Change::from_notes_string("1 ♯2 3 ♯5 6 ♭7")), //930
            ("Ma7(♯5 add ♯9 13)", Change::from_notes_string("1 ♯2 3 ♯5 6 7")), //931
            ("Ma7(♯5 add ♭7 ♯9)", Change::from_notes_string("1 ♯2 3 ♯5 ♯6 7")), //932
            ("Ma7(no 5 add ♭7 ♯9 13)", Change::from_notes_string("1 ♯2 3 6 ♭7 7")), //933
            ("mi(add ♭6 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6")), //934
            ("mi6(add 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 6")), //935
            ("mi7(add 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭7")), //936
            ("miMa7(add 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 7")), //937
            ("dim7(add ♭6 11)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 6")), //938
            ("⌀7(add 11 ♭13)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 ♭7")), //939
            ("dimMa7(add 11 ♭13)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 7")), //940
            ("⌀13(no 9)", Change::from_notes_string("1 ♭3 4 ♭5 6 ♭7")), //941
            ("dimMa7(add 11 13)", Change::from_notes_string("1 ♭3 4 ♭5 6 7")), //942
            ("dimMa7(add ♭7 11)", Change::from_notes_string("1 ♭3 4 ♭5 ♯6 7")), //943
            ("mi6(add ♭6 11)", Change::from_notes_string("1 ♭3 4 5 ♭6 6")), //944
            ("mi7(add 11 ♭13)", Change::from_notes_string("1 ♭3 4 5 ♭6 ♭7")), //945
            ("miMa7(add 11 ♭13)", Change::from_notes_string("1 ♭3 4 5 ♭6 7")), //946
            ("mi13(no 9)", Change::from_notes_string("1 ♭3 4 5 6 ♭7")), //947
            ("miMa13(no 9)", Change::from_notes_string("1 ♭3 4 5 6 7")), //948
            ("miMa7(add ♭7 11)", Change::from_notes_string("1 ♭3 4 5 ♯6 7")), //949
            ("mi13(♯5 no 9)", Change::from_notes_string("1 ♭3 4 ♯5 6 ♭7")), //950
            ("miMa13(♯5 no 9)", Change::from_notes_string("1 ♭3 4 ♯5 6 7")), //951
            ("miMa7(♯5 add ♭7 11)", Change::from_notes_string("1 ♭3 4 ♯5 ♯6 7")), //952
            ("miMa13(no 5 9 add ♭7)", Change::from_notes_string("1 ♭3 4 6 ♭7 7")), //953
            ("mi6(add ♭6 ♯11)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 6")), //954
            ("mi7(add ♯11 ♭13)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 ♭7")), //955
            ("miMa7(add ♯11 ♭13)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 7")), //956
            ("mi7(add ♯11 13)", Change::from_notes_string("1 ♭3 ♯4 5 6 ♭7")), //957
            ("miMa7(add ♯11 13)", Change::from_notes_string("1 ♭3 ♯4 5 6 7")), //958
            ("miMa7(add ♭7 ♯11)", Change::from_notes_string("1 ♭3 ♯4 5 ♯6 7")), //959
            ("⌀7(add ♭13 13)", Change::from_notes_string("1 ♭3 ♯4 ♯5 6 ♭7")), //960
            ("dimMa7(add ♭13 13)", Change::from_notes_string("1 ♭3 ♯4 ♯5 6 7")), //961
            ("dimMa7(add ♭7 ♭13)", Change::from_notes_string("1 ♭3 ♯4 ♯5 ♯6 7")), //962
            ("dimMa7(add ♭7 13)", Change::from_notes_string("1 ♭3 ♭5 6 ♭7 7")), //963
            ("mi7(add ♭13 13)", Change::from_notes_string("1 ♭3 5 ♭6 6 ♭7")), //964
            ("miMa7(add ♭13 13)", Change::from_notes_string("1 ♭3 5 ♭6 6 7")), //965
            ("miMa7(add ♭7 ♭13)", Change::from_notes_string("1 ♭3 5 ♭6 ♭7 7")), //966
            ("miMa7(add ♭7 13)", Change::from_notes_string("1 ♭3 5 6 ♭7 7")), //967
            ("miMa7(♯5 add ♭7 13)", Change::from_notes_string("1 ♭3 ♯5 6 ♭7 7")), //968
            ("Ma(add ♭6 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 ♭6")), //969
            ("6(add 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 6")), //970
            ("7(add 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 ♭7")), //971
            ("Ma7(add 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 7")), //972
            ("6(♭5 add ♭6 11)", Change::from_notes_string("1 3 4 ♭5 ♭6 6")), //973
            ("7(♭5 add 11 ♭13)", Change::from_notes_string("1 3 4 ♭5 ♭6 ♭7")), //974
            ("Ma7(♭5 add 11 ♭13)", Change::from_notes_string("1 3 4 ♭5 ♭6 7")), //975
            ("13(♭5 no 9)", Change::from_notes_string("1 3 4 ♭5 6 ♭7")), //976
            ("Ma13(♭5 no 9)", Change::from_notes_string("1 3 4 ♭5 6 7")), //977
            ("Ma7(♭5 add ♭7 11)", Change::from_notes_string("1 3 4 ♭5 ♯6 7")), //978
            ("6(add ♭6 11)", Change::from_notes_string("1 3 4 5 ♭6 6")), //979
            ("7(add 11 ♭13)", Change::from_notes_string("1 3 4 5 ♭6 ♭7")), //980
            ("Ma7(add 11 ♭13)", Change::from_notes_string("1 3 4 5 ♭6 7")), //981
            ("13(no 9)", Change::from_notes_string("1 3 4 5 6 ♭7")), //982
            ("Ma13(no 9)", Change::from_notes_string("1 3 4 5 6 7")), //983
            ("Ma7(add ♭7 11)", Change::from_notes_string("1 3 4 5 ♯6 7")), //984
            ("13(♯5 no 9)", Change::from_notes_string("1 3 4 ♯5 6 ♭7")), //985
            ("Ma13(♯5 no 9)", Change::from_notes_string("1 3 4 ♯5 6 7")), //986
            ("Ma7(♯5 add ♭7 11)", Change::from_notes_string("1 3 4 ♯5 ♯6 7")), //987
            ("Ma13(no 5 9 add ♭7)", Change::from_notes_string("1 3 4 6 ♭7 7")), //988
            ("6(add ♭6 ♯11)", Change::from_notes_string("1 3 ♯4 5 ♭6 6")), //989
            ("7(add ♯11 ♭13)", Change::from_notes_string("1 3 ♯4 5 ♭6 ♭7")), //990
            ("Ma7(add ♯11 ♭13)", Change::from_notes_string("1 3 ♯4 5 ♭6 7")), //991
            ("7(add ♯11 13)", Change::from_notes_string("1 3 ♯4 5 6 ♭7")), //992
            ("Ma7(add ♯11 13)", Change::from_notes_string("1 3 ♯4 5 6 7")), //993
            ("Ma7(add ♭7 ♯11)", Change::from_notes_string("1 3 ♯4 5 ♯6 7")), //994
            ("7(♭5 add ♭13 13)", Change::from_notes_string("1 3 ♯4 ♯5 6 ♭7")), //995
            ("Ma7(♭5 add ♭13 13)", Change::from_notes_string("1 3 ♯4 ♯5 6 7")), //996
            ("Ma7(♭5 add ♭7 ♭13)", Change::from_notes_string("1 3 ♯4 ♯5 ♯6 7")), //997
            ("Ma7(♭5 add ♭7 13)", Change::from_notes_string("1 3 ♭5 6 ♭7 7")), //998
            ("7(add ♭13 13)", Change::from_notes_string("1 3 5 ♭6 6 ♭7")), //999
            ("Ma7(add ♭13 13)", Change::from_notes_string("1 3 5 ♭6 6 7")), //1000
            ("Ma7(add ♭7 ♭13)", Change::from_notes_string("1 3 5 ♭6 ♭7 7")), //1001
            ("Ma7(add ♭7 13)", Change::from_notes_string("1 3 5 6 ♭7 7")), //1002
            ("Ma7(♯5 add ♭7 13)", Change::from_notes_string("1 3 ♯5 6 ♭7 7")), //1003
            ("6sus4(add ♭6 ♯11)", Change::from_notes_string("1 4 ♯4 5 ♭6 6")), //1004
            ("7sus4(add ♯11 ♭13)", Change::from_notes_string("1 4 ♯4 5 ♭6 ♭7")), //1005
            ("Ma7sus4(add ♯11 ♭13)", Change::from_notes_string("1 4 ♯4 5 ♭6 7")), //1006
            ("13sus♯4(no 9)", Change::from_notes_string("1 4 ♯4 5 6 ♭7")), //1007
            ("Ma13sus♯4(no 9)", Change::from_notes_string("1 4 ♯4 5 6 7")), //1008
            ("Ma7sus4(add ♭7 ♯11)", Change::from_notes_string("1 4 ♯4 5 ♯6 7")), //1009
            ("7sus4(♭5 add ♭13 13)", Change::from_notes_string("1 4 ♭5 ♭6 6 ♭7")), //1010
            ("Ma7sus4(♭5 add ♭13 13)", Change::from_notes_string("1 4 ♭5 ♭6 6 7")), //1011
            ("Ma7sus4(♭5 add ♭7 ♭13)", Change::from_notes_string("1 4 ♭5 ♭6 ♭7 7")), //1012
            ("Ma7sus4(♭5 add ♭7 13)", Change::from_notes_string("1 4 ♭5 6 ♭7 7")), //1013
            ("7sus4(add ♭13 13)", Change::from_notes_string("1 4 5 ♭6 6 ♭7")), //1014
            ("Ma7sus4(add ♭13 13)", Change::from_notes_string("1 4 5 ♭6 6 7")), //1015
            ("Ma7sus4(add ♭7 ♭13)", Change::from_notes_string("1 4 5 ♭6 ♭7 7")), //1016
            ("Ma7sus4(add ♭7 13)", Change::from_notes_string("1 4 5 6 ♭7 7")), //1017
            ("Ma7sus4(♯5 add ♭7 13)", Change::from_notes_string("1 4 ♯5 6 ♭7 7")), //1018
            ("7sus♯4(add ♭13 13)", Change::from_notes_string("1 ♯4 5 ♭6 6 ♭7")), //1019
            ("Ma7sus♯4(add ♭13 13)", Change::from_notes_string("1 ♯4 5 ♭6 6 7")), //1020
            ("Ma7sus♯4(add ♭7 ♭13)", Change::from_notes_string("1 ♯4 5 ♭6 ♭7 7")), //1021
            ("Ma7sus♯4(add ♭7 13)", Change::from_notes_string("1 ♯4 5 6 ♭7 7")), //1022
            ("sus♯4(♯5 add 6 ♭7 7)", Change::from_notes_string("1 ♯4 ♯5 6 ♭7 7")), //1023
            ("5(add ♭6 6 ♭7 7)", Change::from_notes_string("1 5 ♭6 6 ♭7 7")), //1024
            ("11(♭5 no 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5")), //1025
            ("11(no 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5")), //1026
            ("11(♯5 no 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5")), //1027
            ("13(no 5 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 6")), //1028
            ("11(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭7")), //1029
            ("Ma11(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 7")), //1030
            ("Ma(add ♭9 9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5")), //1031
            ("Ma(♭5 add ♭6 ♭9 9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5")), //1032
            ("6/9(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 6")), //1033
            ("9(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 ♭7")), //1034
            ("Ma9(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 7")), //1035
            ("Ma(add ♭6 ♭9 9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6")), //1036
            ("6/9(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 6")), //1037
            ("9(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭7")), //1038
            ("Ma9(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 7")), //1039
            ("6/9(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 6")), //1040
            ("9(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 ♭7")), //1041
            ("Ma9(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 7")), //1042
            ("9(no 5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 6 ♭7")), //1043
            ("Ma9(no 5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 6 7")), //1044
            ("Ma9(no 5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯6 7")), //1045
            ("mi11(no 7 add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5")), //1046
            ("dim11(no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6")), //1047
            ("dim7(add ♭9 9 11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 6")), //1048
            ("⌀11(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭7")), //1049
            ("dimMa11(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 7")), //1050
            ("mi11(no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6")), //1051
            ("mi13(no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 6")), //1052
            ("mi11(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭7")), //1053
            ("miMa11(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 7")), //1054
            ("mi13(♯5 no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 6")), //1055
            ("mi11(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 ♭7")), //1056
            ("miMa11(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 7")), //1057
            ("mi13(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 6 ♭7")), //1058
            ("miMa13(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 6 7")), //1059
            ("miMa11(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯6 7")), //1060
            ("mi(add ♭6 ♭9 9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6")), //1061
            ("mi6/9(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 6")), //1062
            ("mi9(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭7")), //1063
            ("miMa9(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 7")), //1064
            ("dim7(add ♭6 ♭9 9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 6")), //1065
            ("⌀9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 ♭7")), //1066
            ("dimMa9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 7")), //1067
            ("⌀9(add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 6 ♭7")), //1068
            ("dimMa9(add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 6 7")), //1069
            ("dimMa9(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 ♯6 7")), //1070
            ("mi6/9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 6")), //1071
            ("mi9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 ♭7")), //1072
            ("miMa9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 7")), //1073
            ("mi9(add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 5 6 ♭7")), //1074
            ("miMa9(add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 5 6 7")), //1075
            ("miMa9(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♯6 7")), //1076
            ("mi9(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 6 ♭7")), //1077
            ("miMa9(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 6 7")), //1078
            ("miMa9(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 ♯6 7")), //1079
            ("miMa9(no 5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 6 ♭7 7")), //1080
            ("11(no 7 add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5")), //1081
            ("11(♭5 no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6")), //1082
            ("13(♭5 no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 6")), //1083
            ("11(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭7")), //1084
            ("Ma11(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 7")), //1085
            ("11(no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6")), //1086
            ("13(no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 6")), //1087
            ("11(add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭7")), //1088
            ("Ma11(add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 7")), //1089
            ("13(♯5 no 7 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 6")), //1090
            ("11(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 ♭7")), //1091
            ("Ma11(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 7")), //1092
            ("13(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 6 ♭7")), //1093
            ("Ma13(no 5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 6 7")), //1094
            ("Ma11(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯6 7")), //1095
            ("Ma(add ♭6 ♭9 9 ♯11)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6")), //1096
            ("6/9(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 6")), //1097
            ("9(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭7")), //1098
            ("Ma9(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 7")), //1099
            ("6/9(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 6")), //1100
            ("9(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 ♭7")), //1101
            ("Ma9(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 7")), //1102
            ("9(♭5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 ♭5 6 ♭7")), //1103
            ("Ma9(♭5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 ♭5 6 7")), //1104
            ("Ma9(♭5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 ♭5 ♯6 7")), //1105
            ("6/9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 6")), //1106
            ("9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 ♭7")), //1107
            ("Ma9(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 7")), //1108
            ("9(add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 5 6 ♭7")), //1109
            ("Ma9(add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 5 6 7")), //1110
            ("Ma9(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 5 ♯6 7")), //1111
            ("9(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 ♯5 6 ♭7")), //1112
            ("Ma9(♯5 add ♭9 13)", Change::from_notes_string("1 ♭2 2 3 ♯5 6 7")), //1113
            ("Ma9(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯5 ♯6 7")), //1114
            ("Ma9(no 5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 3 6 ♭7 7")), //1115
            ("sus4(add ♭6 ♭9 9 ♯11)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6")), //1116
            ("6/9sus4(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 6")), //1117
            ("11sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭7")), //1118
            ("Ma11sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 7")), //1119
            ("6/9sus4(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 6")), //1120
            ("11sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 ♭7")), //1121
            ("Ma11sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 7")), //1122
            ("13sus♭2(♭5)", Change::from_notes_string("1 ♭2 2 4 ♭5 6 ♭7")), //1123
            ("Ma13sus♭2(♭5)", Change::from_notes_string("1 ♭2 2 4 ♭5 6 7")), //1124
            ("Ma11sus♭2(♭5 add ♭7)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♯6 7")), //1125
            ("6/9sus4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 6")), //1126
            ("11sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 ♭7")), //1127
            ("Ma11sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 7")), //1128
            ("13sus♭2", Change::from_notes_string("1 ♭2 2 4 5 6 ♭7")), //1129
            ("Ma13sus♭2", Change::from_notes_string("1 ♭2 2 4 5 6 7")), //1130
            ("Ma11sus♭2(add ♭7)", Change::from_notes_string("1 ♭2 2 4 5 ♯6 7")), //1131
            ("13sus♭2(♯5)", Change::from_notes_string("1 ♭2 2 4 ♯5 6 ♭7")), //1132
            ("Ma13sus♭2(♯5)", Change::from_notes_string("1 ♭2 2 4 ♯5 6 7")), //1133
            ("Ma11sus♭2(♯5 add ♭7)", Change::from_notes_string("1 ♭2 2 4 ♯5 ♯6 7")), //1134
            ("Ma13sus♭2(no 5 add ♭7)", Change::from_notes_string("1 ♭2 2 4 6 ♭7 7")), //1135
            ("6sus2(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 6")), //1136
            ("9sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 ♭7")), //1137
            ("Ma9sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 7")), //1138
            ("13sus♭2(♯11)", Change::from_notes_string("1 ♭2 2 ♯4 5 6 ♭7")), //1139
            ("Ma13sus♭2(♯11)", Change::from_notes_string("1 ♭2 2 ♯4 5 6 7")), //1140
            ("Ma9sus♯4(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♯6 7")), //1141
            ("13sus♭2(♯5 ♯11)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 6 ♭7")), //1142
            ("Ma13sus♭2(♯5 ♯11)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 6 7")), //1143
            ("Ma9sus♭2(♭5 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 ♯6 7")), //1144
            ("Ma9sus♭2(♭5 add ♭7 13)", Change::from_notes_string("1 ♭2 2 ♭5 6 ♭7 7")), //1145
            ("9sus♭2(add ♭13 13)", Change::from_notes_string("1 ♭2 2 5 ♭6 6 ♭7")), //1146
            ("Ma9sus♭2(add ♭13 13)", Change::from_notes_string("1 ♭2 2 5 ♭6 6 7")), //1147
            ("Ma9sus♭2(add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 5 ♭6 ♭7 7")), //1148
            ("Ma9sus♭2(add ♭7 13)", Change::from_notes_string("1 ♭2 2 5 6 ♭7 7")), //1149
            ("Ma9sus♭2(♯5 add ♭7 13)", Change::from_notes_string("1 ♭2 2 ♯5 6 ♭7 7")), //1150
            ("Ma(add ♭9 ♯9 11 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5")), //1151
            ("Ma(♭5 add ♭6 ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6")), //1152
            ("6(♭5 add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 6")), //1153
            ("11(♭5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭7")), //1154
            ("Ma11(♭5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 7")), //1155
            ("Ma(add ♭6 ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6")), //1156
            ("6(add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 6")), //1157
            ("11(♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭7")), //1158
            ("Ma11(♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 7")), //1159
            ("6(♯5 add ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 6")), //1160
            ("11(♯5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 ♭7")), //1161
            ("Ma11(♯5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 7")), //1162
            ("13(♭9 no 5 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 6 ♭7")), //1163
            ("Ma13(♭9 no 5 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 6 7")), //1164
            ("Ma11(♭9 no 5 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯6 7")), //1165
            ("Ma(add ♭6 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6")), //1166
            ("6(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 6")), //1167
            ("7(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭7")), //1168
            ("Ma7(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 7")), //1169
            ("6(♭5 add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 6")), //1170
            ("7(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 ♭7")), //1171
            ("Ma7(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 7")), //1172
            ("7(♭5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 6 ♭7")), //1173
            ("Ma7(♭5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 6 7")), //1174
            ("Ma7(♭5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 ♯6 7")), //1175
            ("6(add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 6")), //1176
            ("7(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 ♭7")), //1177
            ("Ma7(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 7")), //1178
            ("7(add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 5 6 ♭7")), //1179
            ("Ma7(add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 5 6 7")), //1180
            ("Ma7(add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♯6 7")), //1181
            ("7(♯5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 6 ♭7")), //1182
            ("Ma7(♯5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 6 7")), //1183
            ("Ma7(♯5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 ♯6 7")), //1184
            ("Ma7(no 5 add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 6 ♭7 7")), //1185
            ("mi(add ♭6 ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6")), //1186
            ("mi6(add ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 6")), //1187
            ("mi11(♭9 add ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭7")), //1188
            ("miMa11(♭9 add ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 7")), //1189
            ("dim7(add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 6")), //1190
            ("⌀11(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 ♭7")), //1191
            ("dimMa11(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 7")), //1192
            ("⌀13(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 6 ♭7")), //1193
            ("miMa13(♭5 ♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 6 7")), //1194
            ("dimMa11(♭9 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♯6 7")), //1195
            ("mi6(add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 6")), //1196
            ("mi11(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 ♭7")), //1197
            ("miMa11(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 7")), //1198
            ("mi13(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 5 6 ♭7")), //1199
            ("miMa13(♭9)", Change::from_notes_string("1 ♭2 ♭3 4 5 6 7")), //1200
            ("miMa11(♭9 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♯6 7")), //1201
            ("mi13(♯5 ♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 6 ♭7")), //1202
            ("miMa13(♯5 ♭9)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 6 7")), //1203
            ("miMa11(♯5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 ♯6 7")), //1204
            ("miMa13(♭9 no 5 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 6 ♭7 7")), //1205
            ("mi6(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 6")), //1206
            ("mi7(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 ♭7")), //1207
            ("miMa7(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 7")), //1208
            ("mi13(♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 6 ♭7")), //1209
            ("miMa13(♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 6 7")), //1210
            ("miMa7(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♯6 7")), //1211
            ("mi13(♯5 ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 6 ♭7")), //1212
            ("miMa13(♯5 ♭9 ♯11)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 6 7")), //1213
            ("dimMa7(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 ♯6 7")), //1214
            ("dimMa7(add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 ♭5 6 ♭7 7")), //1215
            ("mi7(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 6 ♭7")), //1216
            ("miMa7(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 6 7")), //1217
            ("miMa7(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 ♭7 7")), //1218
            ("miMa7(add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 5 6 ♭7 7")), //1219
            ("miMa7(♯5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 ♭3 ♯5 6 ♭7 7")), //1220
            ("Ma(add ♭6 ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6")), //1221
            ("6(add ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 6")), //1222
            ("11(♭9 add ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭7")), //1223
            ("Ma11(♭9 add ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 7")), //1224
            ("6(♭5 add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 6")), //1225
            ("11(♭5 ♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 ♭7")), //1226
            ("Ma11(♭5 ♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 7")), //1227
            ("13(♭5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♭5 6 ♭7")), //1228
            ("Ma13(♭5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♭5 6 7")), //1229
            ("Ma11(♭5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♯6 7")), //1230
            ("6(add ♭6 ♭9 11)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 6")), //1231
            ("11(♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 ♭7")), //1232
            ("Ma11(♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 7")), //1233
            ("13(♭9)", Change::from_notes_string("1 ♭2 3 4 5 6 ♭7")), //1234
            ("Ma13(♭9)", Change::from_notes_string("1 ♭2 3 4 5 6 7")), //1235
            ("Ma11(♭9 add ♭7)", Change::from_notes_string("1 ♭2 3 4 5 ♯6 7")), //1236
            ("13(♯5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♯5 6 ♭7")), //1237
            ("Ma13(♯5 ♭9)", Change::from_notes_string("1 ♭2 3 4 ♯5 6 7")), //1238
            ("Ma11(♯5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 3 4 ♯5 ♯6 7")), //1239
            ("Ma13(♭9 no 5 add ♭7)", Change::from_notes_string("1 ♭2 3 4 6 ♭7 7")), //1240
            ("6(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 6")), //1241
            ("7(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 ♭7")), //1242
            ("Ma7(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 7")), //1243
            ("13(♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 6 ♭7")), //1244
            ("Ma13(♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 6 7")), //1245
            ("Ma7(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♯6 7")), //1246
            ("13(♯5 ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 6 ♭7")), //1247
            ("Ma13(♯5 ♭9 ♯11)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 6 7")), //1248
            ("Ma7(♭5 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 ♯6 7")), //1249
            ("Ma7(♭5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 3 ♭5 6 ♭7 7")), //1250
            ("7(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 3 5 ♭6 6 ♭7")), //1251
            ("Ma7(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 3 5 ♭6 6 7")), //1252
            ("Ma7(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 3 5 ♭6 ♭7 7")), //1253
            ("Ma7(add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 3 5 6 ♭7 7")), //1254
            ("Ma7(♯5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 3 ♯5 6 ♭7 7")), //1255
            ("6sus4(add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 6")), //1256
            ("11sus♯4(♭9 add ♭13)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 ♭7")), //1257
            ("Ma11sus♯4(♭9 add ♭13)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 7")), //1258
            ("13sus♯4(♭9)", Change::from_notes_string("1 ♭2 4 ♯4 5 6 ♭7")), //1259
            ("Ma13sus♯4(♭9)", Change::from_notes_string("1 ♭2 4 ♯4 5 6 7")), //1260
            ("Ma11sus♯4(♭9 add ♭7)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♯6 7")), //1261
            ("13(♭5 ♭9 no 3 add ♭13)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 6 ♭7")), //1262
            ("Ma13(♭5 ♭9 no 3 add ♭13)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 6 7")), //1263
            ("Ma7sus4(♭5 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 ♭7 7")), //1264
            ("Ma13(♭5 ♭9 no 3 add ♭7)", Change::from_notes_string("1 ♭2 4 ♭5 6 ♭7 7")), //1265
            ("13(♭9 no 3 add ♭13)", Change::from_notes_string("1 ♭2 4 5 ♭6 6 ♭7")), //1266
            ("Ma13(♭9 no 3 add ♭13)", Change::from_notes_string("1 ♭2 4 5 ♭6 6 7")), //1267
            ("Ma7sus4(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 4 5 ♭6 ♭7 7")), //1268
            ("Ma13(♭9 no 3 add ♭7)", Change::from_notes_string("1 ♭2 4 5 6 ♭7 7")), //1269
            ("Ma13(♯5 ♭9 no 3 add ♭7)", Change::from_notes_string("1 ♭2 4 ♯5 6 ♭7 7")), //1270
            ("13(♭9 ♯11 no 3 add ♭13)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 6 ♭7")), //1271
            ("Ma13(♭9 ♯11 no 3 add ♭13)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 6 7")), //1272
            ("Ma7sus♯4(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 ♭7 7")), //1273
            ("Ma13(♭9 ♯11 no 3 add ♭7)", Change::from_notes_string("1 ♭2 ♯4 5 6 ♭7 7")), //1274
            ("Ma13(♯5 ♭9 ♯11 no 3 add ♭7)", Change::from_notes_string("1 ♭2 ♯4 ♯5 6 ♭7 7")), //1275
            ("Ma7sus♭2(add ♭7 ♭13 13)", Change::from_notes_string("1 ♭2 5 ♭6 6 ♭7 7")), //1276
            ("11(no 7 add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5")), //1277
            ("11(♭5 no 7 add ♭6 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6")), //1278
            ("13(♭5 no 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 6")), //1279
            ("11(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭7")), //1280
            ("Ma11(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 7")), //1281
            ("11(no 7 add ♭6 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6")), //1282
            ("13(no 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 6")), //1283
            ("11(add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭7")), //1284
            ("Ma11(add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 7")), //1285
            ("13(♯5 no 7 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 6")), //1286
            ("11(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 ♭7")), //1287
            ("Ma11(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 7")), //1288
            ("13(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 6 ♭7")), //1289
            ("Ma13(no 5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 6 7")), //1290
            ("Ma11(no 5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯6 7")), //1291
            ("Ma(add ♭6 9 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6")), //1292
            ("6/9(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 6")), //1293
            ("9(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭7")), //1294
            ("Ma9(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 7")), //1295
            ("6/9(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 6")), //1296
            ("9(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 ♭7")), //1297
            ("Ma9(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 7")), //1298
            ("9(♭5 add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 ♭5 6 ♭7")), //1299
            ("Ma9(♭5 add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 ♭5 6 7")), //1300
            ("Ma9(♭5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♭5 ♯6 7")), //1301
            ("6/9(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 6")), //1302
            ("9(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 ♭7")), //1303
            ("Ma9(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 7")), //1304
            ("9(add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 5 6 ♭7")), //1305
            ("Ma9(add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 5 6 7")), //1306
            ("Ma9(add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 5 ♯6 7")), //1307
            ("9(♯5 add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 ♯5 6 ♭7")), //1308
            ("Ma9(♯5 add ♯9 13)", Change::from_notes_string("1 2 ♯2 3 ♯5 6 7")), //1309
            ("Ma9(♯5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯5 ♯6 7")), //1310
            ("Ma9(no 5 add ♭7 ♯9 13)", Change::from_notes_string("1 2 ♯2 3 6 ♭7 7")), //1311
            ("mi11(no 7 add ♭6 ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6")), //1312
            ("mi13(no 7 add ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 6")), //1313
            ("mi11(add ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭7")), //1314
            ("miMa11(add ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 7")), //1315
            ("dim7(add ♭6 9 11)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 6")), //1316
            ("⌀11(add ♭13)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 ♭7")), //1317
            ("dimMa11(add ♭13)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 7")), //1318
            ("⌀13", Change::from_notes_string("1 2 ♭3 4 ♭5 6 ♭7")), //1319
            ("miMa13(♭5)", Change::from_notes_string("1 2 ♭3 4 ♭5 6 7")), //1320
            ("dimMa11(add ♭7)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♯6 7")), //1321
            ("mi13(no 7 add ♭6)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 6")), //1322
            ("mi11(add ♭13)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 ♭7")), //1323
            ("miMa11(add ♭13)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 7")), //1324
            ("mi13", Change::from_notes_string("1 2 ♭3 4 5 6 ♭7")), //1325
            ("miMa13", Change::from_notes_string("1 2 ♭3 4 5 6 7")), //1326
            ("miMa11(add ♭7)", Change::from_notes_string("1 2 ♭3 4 5 ♯6 7")), //1327
            ("mi13(♯5)", Change::from_notes_string("1 2 ♭3 4 ♯5 6 ♭7")), //1328
            ("miMa13(♯5)", Change::from_notes_string("1 2 ♭3 4 ♯5 6 7")), //1329
            ("miMa11(♯5 add ♭7)", Change::from_notes_string("1 2 ♭3 4 ♯5 ♯6 7")), //1330
            ("miMa13(no 5 add ♭7)", Change::from_notes_string("1 2 ♭3 4 6 ♭7 7")), //1331
            ("mi6/9(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 6")), //1332
            ("mi9(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 ♭7")), //1333
            ("miMa9(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 7")), //1334
            ("mi13(♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 6 ♭7")), //1335
            ("miMa13(♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 6 7")), //1336
            ("miMa9(add ♭7 ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♯6 7")), //1337
            ("mi13(♯5 ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 6 ♭7")), //1338
            ("miMa13(♯5 ♯11)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 6 7")), //1339
            ("dimMa9(add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 ♯6 7")), //1340
            ("dimMa9(add ♭7 13)", Change::from_notes_string("1 2 ♭3 ♭5 6 ♭7 7")), //1341
            ("mi9(add ♭13 13)", Change::from_notes_string("1 2 ♭3 5 ♭6 6 ♭7")), //1342
            ("miMa9(add ♭13 13)", Change::from_notes_string("1 2 ♭3 5 ♭6 6 7")), //1343
            ("miMa9(add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 5 ♭6 ♭7 7")), //1344
            ("miMa9(add ♭7 13)", Change::from_notes_string("1 2 ♭3 5 6 ♭7 7")), //1345
            ("miMa9(♯5 add ♭7 13)", Change::from_notes_string("1 2 ♭3 ♯5 6 ♭7 7")), //1346
            ("11(no 7 add ♭6 ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6")), //1347
            ("13(no 7 add ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 6")), //1348
            ("11(add ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭7")), //1349
            ("Ma11(add ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 7")), //1350
            ("13(♭5 no 7 add ♭6)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 6")), //1351
            ("11(♭5 add ♭13)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 ♭7")), //1352
            ("Ma11(♭5 add ♭13)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 7")), //1353
            ("13(♭5)", Change::from_notes_string("1 2 3 4 ♭5 6 ♭7")), //1354
            ("Ma13(♭5)", Change::from_notes_string("1 2 3 4 ♭5 6 7")), //1355
            ("Ma11(♭5 add ♭7)", Change::from_notes_string("1 2 3 4 ♭5 ♯6 7")), //1356
            ("13(no 7 add ♭6)", Change::from_notes_string("1 2 3 4 5 ♭6 6")), //1357
            ("11(add ♭13)", Change::from_notes_string("1 2 3 4 5 ♭6 ♭7")), //1358
            ("Ma11(add ♭13)", Change::from_notes_string("1 2 3 4 5 ♭6 7")), //1359
            ("13", Change::from_notes_string("1 2 3 4 5 6 ♭7")), //1360
            ("Ma13", Change::from_notes_string("1 2 3 4 5 6 7")), //1361
            ("Ma11(add ♭7)", Change::from_notes_string("1 2 3 4 5 ♯6 7")), //1362
            ("13(♯5)", Change::from_notes_string("1 2 3 4 ♯5 6 ♭7")), //1363
            ("Ma13(♯5)", Change::from_notes_string("1 2 3 4 ♯5 6 7")), //1364
            ("Ma11(♯5 add ♭7)", Change::from_notes_string("1 2 3 4 ♯5 ♯6 7")), //1365
            ("Ma13(no 5 add ♭7)", Change::from_notes_string("1 2 3 4 6 ♭7 7")), //1366
            ("6/9(add ♯11 ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 6")), //1367
            ("9(add ♯11 ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 ♭7")), //1368
            ("Ma9(add ♯11 ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 7")), //1369
            ("13(♯11)", Change::from_notes_string("1 2 3 ♯4 5 6 ♭7")), //1370
            ("Ma13(♯11)", Change::from_notes_string("1 2 3 ♯4 5 6 7")), //1371
            ("Ma9(add ♭7 ♯11)", Change::from_notes_string("1 2 3 ♯4 5 ♯6 7")), //1372
            ("13(♯5 ♯11)", Change::from_notes_string("1 2 3 ♯4 ♯5 6 ♭7")), //1373
            ("Ma13(♯5 ♯11)", Change::from_notes_string("1 2 3 ♯4 ♯5 6 7")), //1374
            ("Ma9(♭5 add ♭7 ♭13)", Change::from_notes_string("1 2 3 ♯4 ♯5 ♯6 7")), //1375
            ("Ma9(♭5 add ♭7 13)", Change::from_notes_string("1 2 3 ♭5 6 ♭7 7")), //1376
            ("9(add ♭13 13)", Change::from_notes_string("1 2 3 5 ♭6 6 ♭7")), //1377
            ("Ma9(add ♭13 13)", Change::from_notes_string("1 2 3 5 ♭6 6 7")), //1378
            ("Ma9(add ♭7 ♭13)", Change::from_notes_string("1 2 3 5 ♭6 ♭7 7")), //1379
            ("Ma9(add ♭7 13)", Change::from_notes_string("1 2 3 5 6 ♭7 7")), //1380
            ("Ma9(♯5 add ♭7 13)", Change::from_notes_string("1 2 3 ♯5 6 ♭7 7")), //1381
            ("6/9sus4(add ♯11 ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 6")), //1382
            ("11sus♯4(add ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 ♭7")), //1383
            ("Ma11sus♯4(add ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 7")), //1384
            ("13sus♯4", Change::from_notes_string("1 2 4 ♯4 5 6 ♭7")), //1385
            ("Ma13sus♯4", Change::from_notes_string("1 2 4 ♯4 5 6 7")), //1386
            ("Ma11sus♯4(add ♭7)", Change::from_notes_string("1 2 4 ♯4 5 ♯6 7")), //1387
            ("13(♭5 no 3 add ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 6 ♭7")), //1388
            ("Ma13(♭5 no 3 add ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 6 7")), //1389
            ("Ma9sus4(♭5 add ♭7 ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 ♭7 7")), //1390
            ("Ma13(♭5 no 3 add ♭7)", Change::from_notes_string("1 2 4 ♭5 6 ♭7 7")), //1391
            ("13(no 3 add ♭13)", Change::from_notes_string("1 2 4 5 ♭6 6 ♭7")), //1392
            ("Ma13(no 3 add ♭13)", Change::from_notes_string("1 2 4 5 ♭6 6 7")), //1393
            ("Ma9sus4(add ♭7 ♭13)", Change::from_notes_string("1 2 4 5 ♭6 ♭7 7")), //1394
            ("Ma13(no 3 add ♭7)", Change::from_notes_string("1 2 4 5 6 ♭7 7")), //1395
            ("Ma13(♯5 no 3 add ♭7)", Change::from_notes_string("1 2 4 ♯5 6 ♭7 7")), //1396
            ("13(♯11 no 3 add ♭13)", Change::from_notes_string("1 2 ♯4 5 ♭6 6 ♭7")), //1397
            ("Ma13(♯11 no 3 add ♭13)", Change::from_notes_string("1 2 ♯4 5 ♭6 6 7")), //1398
            ("Ma9sus♯4(add ♭7 ♭13)", Change::from_notes_string("1 2 ♯4 5 ♭6 ♭7 7")), //1399
            ("Ma13(♯11 no 3 add ♭7)", Change::from_notes_string("1 2 ♯4 5 6 ♭7 7")), //1400
            ("Ma13(♯5 ♯11 no 3 add ♭7)", Change::from_notes_string("1 2 ♯4 ♯5 6 ♭7 7")), //1401
            ("Ma7sus2(add ♭7 ♭13 13)", Change::from_notes_string("1 2 5 ♭6 6 ♭7 7")), //1402
            ("Ma(add ♭6 ♯9 11 ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6")), //1403
            ("6(add ♯9 11 ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 6")), //1404
            ("11(♯9 add ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭7")), //1405
            ("Ma11(♯9 add ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 7")), //1406
            ("6(♭5 add ♭6 ♯9 11)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 6")), //1407
            ("11(♭5 ♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 ♭7")), //1408
            ("Ma11(♭5 ♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 7")), //1409
            ("13(♭5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♭5 6 ♭7")), //1410
            ("Ma13(♭5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♭5 6 7")), //1411
            ("Ma11(♭5 ♯9 add ♭7)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♯6 7")), //1412
            ("6(add ♭6 ♯9 11)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 6")), //1413
            ("11(♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 ♭7")), //1414
            ("Ma11(♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 7")), //1415
            ("13(♯9)", Change::from_notes_string("1 ♯2 3 4 5 6 ♭7")), //1416
            ("Ma13(♯9)", Change::from_notes_string("1 ♯2 3 4 5 6 7")), //1417
            ("Ma11(♯9 add ♭7)", Change::from_notes_string("1 ♯2 3 4 5 ♯6 7")), //1418
            ("13(♯5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♯5 6 ♭7")), //1419
            ("Ma13(♯5 ♯9)", Change::from_notes_string("1 ♯2 3 4 ♯5 6 7")), //1420
            ("Ma11(♯5 ♯9 add ♭7)", Change::from_notes_string("1 ♯2 3 4 ♯5 ♯6 7")), //1421
            ("Ma13(♯9 no 5 add ♭7)", Change::from_notes_string("1 ♯2 3 4 6 ♭7 7")), //1422
            ("6(add ♭6 ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 6")), //1423
            ("7(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 ♭7")), //1424
            ("Ma7(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 7")), //1425
            ("13(♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 6 ♭7")), //1426
            ("Ma13(♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 6 7")), //1427
            ("Ma7(add ♭7 ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♯6 7")), //1428
            ("13(♯5 ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 6 ♭7")), //1429
            ("Ma13(♯5 ♯9 ♯11)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 6 7")), //1430
            ("Ma7(♭5 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 ♯6 7")), //1431
            ("Ma7(♭5 add ♭7 ♯9 13)", Change::from_notes_string("1 ♯2 3 ♭5 6 ♭7 7")), //1432
            ("7(add ♯9 ♭13 13)", Change::from_notes_string("1 ♯2 3 5 ♭6 6 ♭7")), //1433
            ("Ma7(add ♯9 ♭13 13)", Change::from_notes_string("1 ♯2 3 5 ♭6 6 7")), //1434
            ("Ma7(add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♯2 3 5 ♭6 ♭7 7")), //1435
            ("Ma7(add ♭7 ♯9 13)", Change::from_notes_string("1 ♯2 3 5 6 ♭7 7")), //1436
            ("Ma7(♯5 add ♭7 ♯9 13)", Change::from_notes_string("1 ♯2 3 ♯5 6 ♭7 7")), //1437
            ("mi6(add ♭6 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 6")), //1438
            ("mi7(add 11 ♯11 ♭13)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 ♭7")), //1439
            ("miMa7(add 11 ♯11 ♭13)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 7")), //1440
            ("mi13(no 9 add ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 6 ♭7")), //1441
            ("miMa13(no 9 add ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 6 7")), //1442
            ("miMa7(add ♭7 11 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♯6 7")), //1443
            ("⌀13(no 9 add ♭13)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 6 ♭7")), //1444
            ("dimMa7(add 11 ♭13 13)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 6 7")), //1445
            ("dimMa7(add ♭7 11 ♭13)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 ♭7 7")), //1446
            ("dimMa7(add ♭7 11 13)", Change::from_notes_string("1 ♭3 4 ♭5 6 ♭7 7")), //1447
            ("mi13(no 9 add ♭13)", Change::from_notes_string("1 ♭3 4 5 ♭6 6 ♭7")), //1448
            ("miMa13(no 9 add ♭13)", Change::from_notes_string("1 ♭3 4 5 ♭6 6 7")), //1449
            ("miMa7(add ♭7 11 ♭13)", Change::from_notes_string("1 ♭3 4 5 ♭6 ♭7 7")), //1450
            ("miMa13(no 9 add ♭7)", Change::from_notes_string("1 ♭3 4 5 6 ♭7 7")), //1451
            ("miMa13(♯5 no 9 add ♭7)", Change::from_notes_string("1 ♭3 4 ♯5 6 ♭7 7")), //1452
            ("mi7(add ♯11 ♭13 13)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 6 ♭7")), //1453
            ("miMa7(add ♯11 ♭13 13)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 6 7")), //1454
            ("miMa7(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 ♭7 7")), //1455
            ("miMa7(add ♭7 ♯11 13)", Change::from_notes_string("1 ♭3 ♯4 5 6 ♭7 7")), //1456
            ("dimMa7(add ♭7 ♭13 13)", Change::from_notes_string("1 ♭3 ♯4 ♯5 6 ♭7 7")), //1457
            ("miMa7(add ♭7 ♭13 13)", Change::from_notes_string("1 ♭3 5 ♭6 6 ♭7 7")), //1458
            ("6(add ♭6 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 6")), //1459
            ("7(add 11 ♯11 ♭13)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 ♭7")), //1460
            ("Ma7(add 11 ♯11 ♭13)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 7")), //1461
            ("13(no 9 add ♯11)", Change::from_notes_string("1 3 4 ♯4 5 6 ♭7")), //1462
            ("Ma13(no 9 add ♯11)", Change::from_notes_string("1 3 4 ♯4 5 6 7")), //1463
            ("Ma7(add ♭7 11 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 ♯6 7")), //1464
            ("13(♭5 no 9 add ♭13)", Change::from_notes_string("1 3 4 ♭5 ♭6 6 ♭7")), //1465
            ("Ma13(♭5 no 9 add ♭13)", Change::from_notes_string("1 3 4 ♭5 ♭6 6 7")), //1466
            ("Ma7(♭5 add ♭7 11 ♭13)", Change::from_notes_string("1 3 4 ♭5 ♭6 ♭7 7")), //1467
            ("Ma13(♭5 no 9 add ♭7)", Change::from_notes_string("1 3 4 ♭5 6 ♭7 7")), //1468
            ("13(no 9 add ♭13)", Change::from_notes_string("1 3 4 5 ♭6 6 ♭7")), //1469
            ("Ma13(no 9 add ♭13)", Change::from_notes_string("1 3 4 5 ♭6 6 7")), //1470
            ("Ma7(add ♭7 11 ♭13)", Change::from_notes_string("1 3 4 5 ♭6 ♭7 7")), //1471
            ("Ma13(no 9 add ♭7)", Change::from_notes_string("1 3 4 5 6 ♭7 7")), //1472
            ("Ma13(♯5 no 9 add ♭7)", Change::from_notes_string("1 3 4 ♯5 6 ♭7 7")), //1473
            ("7(add ♯11 ♭13 13)", Change::from_notes_string("1 3 ♯4 5 ♭6 6 ♭7")), //1474
            ("Ma7(add ♯11 ♭13 13)", Change::from_notes_string("1 3 ♯4 5 ♭6 6 7")), //1475
            ("Ma7(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 3 ♯4 5 ♭6 ♭7 7")), //1476
            ("Ma7(add ♭7 ♯11 13)", Change::from_notes_string("1 3 ♯4 5 6 ♭7 7")), //1477
            ("Ma7(♭5 add ♭7 ♭13 13)", Change::from_notes_string("1 3 ♯4 ♯5 6 ♭7 7")), //1478
            ("Ma7(add ♭7 ♭13 13)", Change::from_notes_string("1 3 5 ♭6 6 ♭7 7")), //1479
            ("13sus♯4(no 9 add ♭13)", Change::from_notes_string("1 4 ♯4 5 ♭6 6 ♭7")), //1480
            ("Ma13sus♯4(no 9 add ♭13)", Change::from_notes_string("1 4 ♯4 5 ♭6 6 7")), //1481
            ("Ma7sus4(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 4 ♯4 5 ♭6 ♭7 7")), //1482
            ("Ma13sus♯4(no 9 add ♭7)", Change::from_notes_string("1 4 ♯4 5 6 ♭7 7")), //1483
            ("Ma7sus4(♭5 add ♭7 ♭13 13)", Change::from_notes_string("1 4 ♭5 ♭6 6 ♭7 7")), //1484
            ("Ma7sus4(add ♭7 ♭13 13)", Change::from_notes_string("1 4 5 ♭6 6 ♭7 7")), //1485
            ("Ma7sus♯4(add ♭7 ♭13 13)", Change::from_notes_string("1 ♯4 5 ♭6 6 ♭7 7")), //1486
            ("11(no 7 add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5")), //1487
            ("11(♭5 no 7 add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6")), //1488
            ("13(♭5 no 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 6")), //1489
            ("11(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭7")), //1490
            ("Ma11(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 7")), //1491
            ("11(no 7 add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6")), //1492
            ("13(no 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 6")), //1493
            ("11(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭7")), //1494
            ("Ma11(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 7")), //1495
            ("13(♯5 no 7 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 6")), //1496
            ("11(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 ♭7")), //1497
            ("Ma11(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 7")), //1498
            ("13(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 6 ♭7")), //1499
            ("Ma13(no 5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 6 7")), //1500
            ("Ma11(no 5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯6 7")), //1501
            ("Ma(add ♭6 ♭9 9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6")), //1502
            ("6/9(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 6")), //1503
            ("9(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭7")), //1504
            ("Ma9(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 7")), //1505
            ("6/9(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 6")), //1506
            ("9(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 ♭7")), //1507
            ("Ma9(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 7")), //1508
            ("9(♭5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 6 ♭7")), //1509
            ("Ma9(♭5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 6 7")), //1510
            ("Ma9(♭5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 ♯6 7")), //1511
            ("6/9(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 6")), //1512
            ("9(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 ♭7")), //1513
            ("Ma9(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 7")), //1514
            ("9(add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 6 ♭7")), //1515
            ("Ma9(add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 6 7")), //1516
            ("Ma9(add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♯6 7")), //1517
            ("9(♯5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 6 ♭7")), //1518
            ("Ma9(♯5 add ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 6 7")), //1519
            ("Ma9(♯5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 ♯6 7")), //1520
            ("Ma9(no 5 add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 6 ♭7 7")), //1521
            ("mi11(no 7 add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6")), //1522
            ("mi13(no 7 add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 6")), //1523
            ("mi11(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭7")), //1524
            ("miMa11(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 7")), //1525
            ("dim7(add ♭6 ♭9 9 11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 6")), //1526
            ("⌀11(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 ♭7")), //1527
            ("dimMa11(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 7")), //1528
            ("⌀13(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 6 ♭7")), //1529
            ("miMa13(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 6 7")), //1530
            ("dimMa11(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♯6 7")), //1531
            ("mi13(no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 6")), //1532
            ("mi11(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 ♭7")), //1533
            ("miMa11(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 7")), //1534
            ("mi13(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 6 ♭7")), //1535
            ("miMa13(add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 6 7")), //1536
            ("miMa11(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♯6 7")), //1537
            ("mi13(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 6 ♭7")), //1538
            ("miMa13(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 6 7")), //1539
            ("miMa11(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 ♯6 7")), //1540
            ("miMa13(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 6 ♭7 7")), //1541
            ("mi6/9(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 6")), //1542
            ("mi9(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 ♭7")), //1543
            ("miMa9(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 7")), //1544
            ("mi13(♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 6 ♭7")), //1545
            ("miMa13(♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 6 7")), //1546
            ("miMa9(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♯6 7")), //1547
            ("mi13(♯5 ♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 6 ♭7")), //1548
            ("miMa13(♯5 ♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 6 7")), //1549
            ("dimMa9(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 ♯6 7")), //1550
            ("dimMa9(add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 ♭5 6 ♭7 7")), //1551
            ("mi9(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 6 ♭7")), //1552
            ("miMa9(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 6 7")), //1553
            ("miMa9(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 ♭7 7")), //1554
            ("miMa9(add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 5 6 ♭7 7")), //1555
            ("miMa9(♯5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯5 6 ♭7 7")), //1556
            ("11(no 7 add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6")), //1557
            ("13(no 7 add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 6")), //1558
            ("11(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭7")), //1559
            ("Ma11(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 7")), //1560
            ("13(♭5 no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 6")), //1561
            ("11(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 ♭7")), //1562
            ("Ma11(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 7")), //1563
            ("13(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 6 ♭7")), //1564
            ("Ma13(♭5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 6 7")), //1565
            ("Ma11(♭5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♯6 7")), //1566
            ("13(no 7 add ♭6 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 6")), //1567
            ("11(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 ♭7")), //1568
            ("Ma11(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 7")), //1569
            ("13(add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 6 ♭7")), //1570
            ("Ma13(add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 6 7")), //1571
            ("Ma11(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 ♯6 7")), //1572
            ("13(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 6 ♭7")), //1573
            ("Ma13(♯5 add ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 6 7")), //1574
            ("Ma11(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 ♯6 7")), //1575
            ("Ma13(no 5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 6 ♭7 7")), //1576
            ("6/9(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 6")), //1577
            ("9(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 ♭7")), //1578
            ("Ma9(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 7")), //1579
            ("13(♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 6 ♭7")), //1580
            ("Ma13(♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 6 7")), //1581
            ("Ma9(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♯6 7")), //1582
            ("13(♯5 ♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 6 ♭7")), //1583
            ("Ma13(♯5 ♯11 add ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 6 7")), //1584
            ("Ma9(♭5 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 ♯6 7")), //1585
            ("Ma9(♭5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 3 ♭5 6 ♭7 7")), //1586
            ("9(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 6 ♭7")), //1587
            ("Ma9(add ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 6 7")), //1588
            ("Ma9(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 ♭7 7")), //1589
            ("Ma9(add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 3 5 6 ♭7 7")), //1590
            ("Ma9(♯5 add ♭7 ♭9 13)", Change::from_notes_string("1 ♭2 2 3 ♯5 6 ♭7 7")), //1591
            ("6/9sus4(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 6")), //1592
            ("11sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 ♭7")), //1593
            ("Ma11sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 7")), //1594
            ("13sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 6 ♭7")), //1595
            ("Ma13sus♯4(add ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 6 7")), //1596
            ("Ma11sus♯4(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♯6 7")), //1597
            ("13sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 6 ♭7")), //1598
            ("Ma13sus♭2(♭5 add ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 6 7")), //1599
            ("Ma11sus♭2(♭5 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 ♭7 7")), //1600
            ("Ma13sus♭2(♭5 add ♭7)", Change::from_notes_string("1 ♭2 2 4 ♭5 6 ♭7 7")), //1601
            ("13sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 6 ♭7")), //1602
            ("Ma13sus♭2(add ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 6 7")), //1603
            ("Ma11sus♭2(add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 ♭7 7")), //1604
            ("Ma13sus♭2(add ♭7)", Change::from_notes_string("1 ♭2 2 4 5 6 ♭7 7")), //1605
            ("Ma13sus♭2(♯5 add ♭7)", Change::from_notes_string("1 ♭2 2 4 ♯5 6 ♭7 7")), //1606
            ("13sus♭2(♯11 add ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 6 ♭7")), //1607
            ("Ma13sus♭2(♯11 add ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 6 7")), //1608
            ("Ma9sus♯4(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 ♭7 7")), //1609
            ("Ma13sus♭2(♯11 add ♭7)", Change::from_notes_string("1 ♭2 2 ♯4 5 6 ♭7 7")), //1610
            ("Ma13sus♭2(♯5 ♯11 add ♭7)", Change::from_notes_string("1 ♭2 2 ♯4 ♯5 6 ♭7 7")), //1611
            ("Ma9sus♭2(add ♭7 ♭13 13)", Change::from_notes_string("1 ♭2 2 5 ♭6 6 ♭7 7")), //1612
            ("Ma(add ♭6 ♭9 ♯9 11 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6")), //1613
            ("6(add ♭9 ♯9 11 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 6")), //1614
            ("11(♭9 add ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭7")), //1615
            ("Ma11(♭9 add ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 7")), //1616
            ("6(♭5 add ♭6 ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 6")), //1617
            ("11(♭5 ♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 ♭7")), //1618
            ("Ma11(♭5 ♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 7")), //1619
            ("13(♭5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 6 ♭7")), //1620
            ("Ma13(♭5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 6 7")), //1621
            ("Ma11(♭5 ♭9 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♯6 7")), //1622
            ("6(add ♭6 ♭9 ♯9 11)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 6")), //1623
            ("11(♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 ♭7")), //1624
            ("Ma11(♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 7")), //1625
            ("13(♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 6 ♭7")), //1626
            ("Ma13(♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 6 7")), //1627
            ("Ma11(♭9 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♯6 7")), //1628
            ("13(♯5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 6 ♭7")), //1629
            ("Ma13(♯5 ♭9 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 6 7")), //1630
            ("Ma11(♯5 ♭9 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 ♯6 7")), //1631
            ("Ma13(♭9 no 5 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 6 ♭7 7")), //1632
            ("6(add ♭6 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 6")), //1633
            ("7(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 ♭7")), //1634
            ("Ma7(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 7")), //1635
            ("13(♭9 ♯11 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 6 ♭7")), //1636
            ("Ma13(♭9 ♯11 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 6 7")), //1637
            ("Ma7(add ♭7 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♯6 7")), //1638
            ("13(♯5 ♭9 ♯11 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 6 ♭7")), //1639
            ("Ma13(♯5 ♭9 ♯11 add ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 6 7")), //1640
            ("Ma7(♭5 add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 ♯6 7")), //1641
            ("Ma7(♭5 add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 ♭5 6 ♭7 7")), //1642
            ("7(add ♭9 ♯9 ♭13 13)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 6 ♭7")), //1643
            ("Ma7(add ♭9 ♯9 ♭13 13)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 6 7")), //1644
            ("Ma7(add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 ♭7 7")), //1645
            ("Ma7(add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 5 6 ♭7 7")), //1646
            ("Ma7(♯5 add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯5 6 ♭7 7")), //1647
            ("mi6(add ♭6 ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 6")), //1648
            ("mi11(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 ♭7")), //1649
            ("miMa11(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 7")), //1650
            ("mi13(♭9 add ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 6 ♭7")), //1651
            ("miMa13(♭9 add ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 6 7")), //1652
            ("miMa11(♭9 add ♭7 ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♯6 7")), //1653
            ("⌀13(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 6 ♭7")), //1654
            ("miMa13(♭5 ♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 6 7")), //1655
            ("dimMa11(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 ♭7 7")), //1656
            ("miMa13(♭5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 6 ♭7 7")), //1657
            ("mi13(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 6 ♭7")), //1658
            ("miMa13(♭9 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 6 7")), //1659
            ("miMa11(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 ♭7 7")), //1660
            ("miMa13(♭9 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 5 6 ♭7 7")), //1661
            ("miMa13(♯5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 4 ♯5 6 ♭7 7")), //1662
            ("mi13(♭9 ♯11 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 6 ♭7")), //1663
            ("miMa13(♭9 ♯11 add ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 6 7")), //1664
            ("miMa7(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 ♭7 7")), //1665
            ("miMa13(♭9 ♯11 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 6 ♭7 7")), //1666
            ("miMa13(♯5 ♭9 ♯11 add ♭7)", Change::from_notes_string("1 ♭2 ♭3 ♯4 ♯5 6 ♭7 7")), //1667
            ("miMa7(add ♭7 ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 ♭3 5 ♭6 6 ♭7 7")), //1668
            ("6(add ♭6 ♭9 11 ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 6")), //1669
            ("11(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 ♭7")), //1670
            ("Ma11(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 7")), //1671
            ("13(♭9 add ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 6 ♭7")), //1672
            ("Ma13(♭9 add ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 6 7")), //1673
            ("Ma11(♭9 add ♭7 ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♯6 7")), //1674
            ("13(♭5 ♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 6 ♭7")), //1675
            ("Ma13(♭5 ♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 6 7")), //1676
            ("Ma11(♭5 ♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 ♭7 7")), //1677
            ("Ma13(♭5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 3 4 ♭5 6 ♭7 7")), //1678
            ("13(♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 6 ♭7")), //1679
            ("Ma13(♭9 add ♭13)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 6 7")), //1680
            ("Ma11(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 ♭7 7")), //1681
            ("Ma13(♭9 add ♭7)", Change::from_notes_string("1 ♭2 3 4 5 6 ♭7 7")), //1682
            ("Ma13(♯5 ♭9 add ♭7)", Change::from_notes_string("1 ♭2 3 4 ♯5 6 ♭7 7")), //1683
            ("13(♭9 ♯11 add ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 6 ♭7")), //1684
            ("Ma13(♭9 ♯11 add ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 6 7")), //1685
            ("Ma7(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 ♭7 7")), //1686
            ("Ma13(♭9 ♯11 add ♭7)", Change::from_notes_string("1 ♭2 3 ♯4 5 6 ♭7 7")), //1687
            ("Ma13(♯5 ♭9 ♯11 add ♭7)", Change::from_notes_string("1 ♭2 3 ♯4 ♯5 6 ♭7 7")), //1688
            ("Ma7(add ♭7 ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 3 5 ♭6 6 ♭7 7")), //1689
            ("13sus♯4(♭9 add ♭13)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 6 ♭7")), //1690
            ("Ma13sus♯4(♭9 add ♭13)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 6 7")), //1691
            ("Ma11sus♯4(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 ♭7 7")), //1692
            ("Ma13sus♯4(♭9 add ♭7)", Change::from_notes_string("1 ♭2 4 ♯4 5 6 ♭7 7")), //1693
            ("Ma13(♭5 ♭9 no 3 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 4 ♭5 ♭6 6 ♭7 7")), //1694
            ("Ma13(♭9 no 3 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 4 5 ♭6 6 ♭7 7")), //1695
            ("Ma13(♭9 ♯11 no 3 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♯4 5 ♭6 6 ♭7 7")), //1696
            ("11(no 7 add ♭6 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6")), //1697
            ("13(no 7 add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 6")), //1698
            ("11(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭7")), //1699
            ("Ma11(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 7")), //1700
            ("13(♭5 no 7 add ♭6 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 6")), //1701
            ("11(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 ♭7")), //1702
            ("Ma11(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 7")), //1703
            ("13(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 6 ♭7")), //1704
            ("Ma13(♭5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 6 7")), //1705
            ("Ma11(♭5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♯6 7")), //1706
            ("13(no 7 add ♭6 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 6")), //1707
            ("11(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 ♭7")), //1708
            ("Ma11(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 7")), //1709
            ("13(add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 6 ♭7")), //1710
            ("Ma13(add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 6 7")), //1711
            ("Ma11(add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 ♯6 7")), //1712
            ("13(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 6 ♭7")), //1713
            ("Ma13(♯5 add ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 6 7")), //1714
            ("Ma11(♯5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 ♯6 7")), //1715
            ("Ma13(no 5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 6 ♭7 7")), //1716
            ("6/9(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 6")), //1717
            ("9(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 ♭7")), //1718
            ("Ma9(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 7")), //1719
            ("13(♯11 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 6 ♭7")), //1720
            ("Ma13(♯11 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 6 7")), //1721
            ("Ma9(add ♭7 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♯6 7")), //1722
            ("13(♯5 ♯11 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 6 ♭7")), //1723
            ("Ma13(♯5 ♯11 add ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 6 7")), //1724
            ("Ma9(♭5 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 ♯6 7")), //1725
            ("Ma9(♭5 add ♭7 ♯9 13)", Change::from_notes_string("1 2 ♯2 3 ♭5 6 ♭7 7")), //1726
            ("9(add ♯9 ♭13 13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 6 ♭7")), //1727
            ("Ma9(add ♯9 ♭13 13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 6 7")), //1728
            ("Ma9(add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 ♭7 7")), //1729
            ("Ma9(add ♭7 ♯9 13)", Change::from_notes_string("1 2 ♯2 3 5 6 ♭7 7")), //1730
            ("Ma9(♯5 add ♭7 ♯9 13)", Change::from_notes_string("1 2 ♯2 3 ♯5 6 ♭7 7")), //1731
            ("mi13(no 7 add ♭6 ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 6")), //1732
            ("mi11(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 ♭7")), //1733
            ("miMa11(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 7")), //1734
            ("mi13(add ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 6 ♭7")), //1735
            ("miMa13(add ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 6 7")), //1736
            ("miMa11(add ♭7 ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♯6 7")), //1737
            ("⌀13(add ♭13)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 6 ♭7")), //1738
            ("miMa13(♭5 add ♭13)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 6 7")), //1739
            ("dimMa11(add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 ♭7 7")), //1740
            ("miMa13(♭5 add ♭7)", Change::from_notes_string("1 2 ♭3 4 ♭5 6 ♭7 7")), //1741
            ("mi13(add ♭13)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 6 ♭7")), //1742
            ("miMa13(add ♭13)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 6 7")), //1743
            ("miMa11(add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 ♭7 7")), //1744
            ("miMa13(add ♭7)", Change::from_notes_string("1 2 ♭3 4 5 6 ♭7 7")), //1745
            ("miMa13(♯5 add ♭7)", Change::from_notes_string("1 2 ♭3 4 ♯5 6 ♭7 7")), //1746
            ("mi13(♯11 add ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 6 ♭7")), //1747
            ("miMa13(♯11 add ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 6 7")), //1748
            ("miMa9(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 ♭7 7")), //1749
            ("miMa13(♯11 add ♭7)", Change::from_notes_string("1 2 ♭3 ♯4 5 6 ♭7 7")), //1750
            ("miMa13(♯5 ♯11 add ♭7)", Change::from_notes_string("1 2 ♭3 ♯4 ♯5 6 ♭7 7")), //1751
            ("miMa9(add ♭7 ♭13 13)", Change::from_notes_string("1 2 ♭3 5 ♭6 6 ♭7 7")), //1752
            ("13(no 7 add ♭6 ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 6")), //1753
            ("11(add ♯11 ♭13)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 ♭7")), //1754
            ("Ma11(add ♯11 ♭13)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 7")), //1755
            ("13(add ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 6 ♭7")), //1756
            ("Ma13(add ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 6 7")), //1757
            ("Ma11(add ♭7 ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 ♯6 7")), //1758
            ("13(♭5 add ♭13)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 6 ♭7")), //1759
            ("Ma13(♭5 add ♭13)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 6 7")), //1760
            ("Ma11(♭5 add ♭7 ♭13)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 ♭7 7")), //1761
            ("Ma13(♭5 add ♭7)", Change::from_notes_string("1 2 3 4 ♭5 6 ♭7 7")), //1762
            ("13(add ♭13)", Change::from_notes_string("1 2 3 4 5 ♭6 6 ♭7")), //1763
            ("Ma13(add ♭13)", Change::from_notes_string("1 2 3 4 5 ♭6 6 7")), //1764
            ("Ma11(add ♭7 ♭13)", Change::from_notes_string("1 2 3 4 5 ♭6 ♭7 7")), //1765
            ("Ma13(add ♭7)", Change::from_notes_string("1 2 3 4 5 6 ♭7 7")), //1766
            ("Ma13(♯5 add ♭7)", Change::from_notes_string("1 2 3 4 ♯5 6 ♭7 7")), //1767
            ("13(♯11 add ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 6 ♭7")), //1768
            ("Ma13(♯11 add ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 6 7")), //1769
            ("Ma9(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 ♭7 7")), //1770
            ("Ma13(♯11 add ♭7)", Change::from_notes_string("1 2 3 ♯4 5 6 ♭7 7")), //1771
            ("Ma13(♯5 ♯11 add ♭7)", Change::from_notes_string("1 2 3 ♯4 ♯5 6 ♭7 7")), //1772
            ("Ma9(add ♭7 ♭13 13)", Change::from_notes_string("1 2 3 5 ♭6 6 ♭7 7")), //1773
            ("13sus♯4(add ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 6 ♭7")), //1774
            ("Ma13sus♯4(add ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 6 7")), //1775
            ("Ma11sus♯4(add ♭7 ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 ♭7 7")), //1776
            ("Ma13sus♯4(add ♭7)", Change::from_notes_string("1 2 4 ♯4 5 6 ♭7 7")), //1777
            ("Ma13(♭5 no 3 add ♭7 ♭13)", Change::from_notes_string("1 2 4 ♭5 ♭6 6 ♭7 7")), //1778
            ("Ma13(no 3 add ♭7 ♭13)", Change::from_notes_string("1 2 4 5 ♭6 6 ♭7 7")), //1779
            ("Ma13(♯11 no 3 add ♭7 ♭13)", Change::from_notes_string("1 2 ♯4 5 ♭6 6 ♭7 7")), //1780
            ("6(add ♭6 ♯9 11 ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 6")), //1781
            ("11(♯9 add ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 ♭7")), //1782
            ("Ma11(♯9 add ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 7")), //1783
            ("13(♯9 add ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 6 ♭7")), //1784
            ("Ma13(♯9 add ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 6 7")), //1785
            ("Ma11(♯9 add ♭7 ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♯6 7")), //1786
            ("13(♭5 ♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 6 ♭7")), //1787
            ("Ma13(♭5 ♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 6 7")), //1788
            ("Ma11(♭5 ♯9 add ♭7 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 ♭7 7")), //1789
            ("Ma13(♭5 ♯9 add ♭7)", Change::from_notes_string("1 ♯2 3 4 ♭5 6 ♭7 7")), //1790
            ("13(♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 6 ♭7")), //1791
            ("Ma13(♯9 add ♭13)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 6 7")), //1792
            ("Ma11(♯9 add ♭7 ♭13)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 ♭7 7")), //1793
            ("Ma13(♯9 add ♭7)", Change::from_notes_string("1 ♯2 3 4 5 6 ♭7 7")), //1794
            ("Ma13(♯5 ♯9 add ♭7)", Change::from_notes_string("1 ♯2 3 4 ♯5 6 ♭7 7")), //1795
            ("13(♯9 ♯11 add ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 6 ♭7")), //1796
            ("Ma13(♯9 ♯11 add ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 6 7")), //1797
            ("Ma7(add ♭7 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 ♭7 7")), //1798
            ("Ma13(♯9 ♯11 add ♭7)", Change::from_notes_string("1 ♯2 3 ♯4 5 6 ♭7 7")), //1799
            ("Ma13(♯5 ♯9 ♯11 add ♭7)", Change::from_notes_string("1 ♯2 3 ♯4 ♯5 6 ♭7 7")), //1800
            ("Ma7(add ♭7 ♯9 ♭13 13)", Change::from_notes_string("1 ♯2 3 5 ♭6 6 ♭7 7")), //1801
            ("mi13(no 9 add ♯11 ♭13)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 6 ♭7")), //1802
            ("miMa13(no 9 add ♯11 ♭13)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 6 7")), //1803
            ("miMa7(add ♭7 11 ♯11 ♭13)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 ♭7 7")), //1804
            ("miMa13(no 9 add ♭7 ♯11)", Change::from_notes_string("1 ♭3 4 ♯4 5 6 ♭7 7")), //1805
            ("dimMa7(add ♭7 11 ♭13 13)", Change::from_notes_string("1 ♭3 4 ♭5 ♭6 6 ♭7 7")), //1806
            ("miMa13(no 9 add ♭7 ♭13)", Change::from_notes_string("1 ♭3 4 5 ♭6 6 ♭7 7")), //1807
            ("miMa7(add ♭7 ♯11 ♭13 13)", Change::from_notes_string("1 ♭3 ♯4 5 ♭6 6 ♭7 7")), //1808
            ("13(no 9 add ♯11 ♭13)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 6 ♭7")), //1809
            ("Ma13(no 9 add ♯11 ♭13)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 6 7")), //1810
            ("Ma7(add ♭7 11 ♯11 ♭13)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 ♭7 7")), //1811
            ("Ma13(no 9 add ♭7 ♯11)", Change::from_notes_string("1 3 4 ♯4 5 6 ♭7 7")), //1812
            ("Ma13(♭5 no 9 add ♭7 ♭13)", Change::from_notes_string("1 3 4 ♭5 ♭6 6 ♭7 7")), //1813
            ("Ma13(no 9 add ♭7 ♭13)", Change::from_notes_string("1 3 4 5 ♭6 6 ♭7 7")), //1814
            ("Ma7(add ♭7 ♯11 ♭13 13)", Change::from_notes_string("1 3 ♯4 5 ♭6 6 ♭7 7")), //1815
            ("Ma13sus♯4(no 9 add ♭7 ♭13)", Change::from_notes_string("1 4 ♯4 5 ♭6 6 ♭7 7")), //1816
            ("11(no 7 add ♭6 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6")), //1817
            ("13(no 7 add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 6")), //1818
            ("11(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭7")), //1819
            ("Ma11(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 7")), //1820
            ("13(♭5 no 7 add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 6")), //1821
            ("11(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 ♭7")), //1822
            ("Ma11(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 7")), //1823
            ("13(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 6 ♭7")), //1824
            ("Ma13(♭5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 6 7")), //1825
            ("Ma11(♭5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♯6 7")), //1826
            ("13(no 7 add ♭6 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 6")), //1827
            ("11(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 ♭7")), //1828
            ("Ma11(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 7")), //1829
            ("13(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 6 ♭7")), //1830
            ("Ma13(add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 6 7")), //1831
            ("Ma11(add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♯6 7")), //1832
            ("13(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 6 ♭7")), //1833
            ("Ma13(♯5 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 6 7")), //1834
            ("Ma11(♯5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 ♯6 7")), //1835
            ("Ma13(no 5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 6 ♭7 7")), //1836
            ("6/9(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 6")), //1837
            ("9(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 ♭7")), //1838
            ("Ma9(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 7")), //1839
            ("13(♯11 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 6 ♭7")), //1840
            ("Ma13(♯11 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 6 7")), //1841
            ("Ma9(add ♭7 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♯6 7")), //1842
            ("13(♯5 ♯11 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 6 ♭7")), //1843
            ("Ma13(♯5 ♯11 add ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 6 7")), //1844
            ("Ma9(♭5 add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 ♯6 7")), //1845
            ("Ma9(♭5 add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♭5 6 ♭7 7")), //1846
            ("9(add ♭9 ♯9 ♭13 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 6 ♭7")), //1847
            ("Ma9(add ♭9 ♯9 ♭13 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 6 7")), //1848
            ("Ma9(add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 ♭7 7")), //1849
            ("Ma9(add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 6 ♭7 7")), //1850
            ("Ma9(♯5 add ♭7 ♭9 ♯9 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯5 6 ♭7 7")), //1851
            ("mi13(no 7 add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 6")), //1852
            ("mi11(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 ♭7")), //1853
            ("miMa11(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 7")), //1854
            ("mi13(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 6 ♭7")), //1855
            ("miMa13(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 6 7")), //1856
            ("miMa11(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♯6 7")), //1857
            ("⌀13(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 6 ♭7")), //1858
            ("miMa13(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 6 7")), //1859
            ("dimMa11(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 ♭7 7")), //1860
            ("miMa13(♭5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 6 ♭7 7")), //1861
            ("mi13(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 6 ♭7")), //1862
            ("miMa13(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 6 7")), //1863
            ("miMa11(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 ♭7 7")), //1864
            ("miMa13(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 6 ♭7 7")), //1865
            ("miMa13(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯5 6 ♭7 7")), //1866
            ("mi13(♯11 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 6 ♭7")), //1867
            ("miMa13(♯11 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 6 7")), //1868
            ("miMa9(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 ♭7 7")), //1869
            ("miMa13(♯11 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 6 ♭7 7")), //1870
            ("miMa13(♯5 ♯11 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 ♯5 6 ♭7 7")), //1871
            ("miMa9(add ♭7 ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 2 ♭3 5 ♭6 6 ♭7 7")), //1872
            ("13(no 7 add ♭6 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 6")), //1873
            ("11(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 ♭7")), //1874
            ("Ma11(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 7")), //1875
            ("13(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 6 ♭7")), //1876
            ("Ma13(add ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 6 7")), //1877
            ("Ma11(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♯6 7")), //1878
            ("13(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 6 ♭7")), //1879
            ("Ma13(♭5 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 6 7")), //1880
            ("Ma11(♭5 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 ♭7 7")), //1881
            ("Ma13(♭5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 6 ♭7 7")), //1882
            ("13(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 6 ♭7")), //1883
            ("Ma13(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 6 7")), //1884
            ("Ma11(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 ♭7 7")), //1885
            ("Ma13(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 5 6 ♭7 7")), //1886
            ("Ma13(♯5 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 4 ♯5 6 ♭7 7")), //1887
            ("13(♯11 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 6 ♭7")), //1888
            ("Ma13(♯11 add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 6 7")), //1889
            ("Ma9(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 ♭7 7")), //1890
            ("Ma13(♯11 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 6 ♭7 7")), //1891
            ("Ma13(♯5 ♯11 add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 3 ♯4 ♯5 6 ♭7 7")), //1892
            ("Ma9(add ♭7 ♭9 ♭13 13)", Change::from_notes_string("1 ♭2 2 3 5 ♭6 6 ♭7 7")), //1893
            ("13sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 6 ♭7")), //1894
            ("Ma13sus♯4(add ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 6 7")), //1895
            ("Ma11sus♯4(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 ♭7 7")), //1896
            ("Ma13sus♯4(add ♭7 ♭9)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 6 ♭7 7")), //1897
            ("Ma13sus♭2(♭5 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♭5 ♭6 6 ♭7 7")), //1898
            ("Ma13sus♭2(add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 4 5 ♭6 6 ♭7 7")), //1899
            ("Ma13sus♭2(♯11 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 2 ♯4 5 ♭6 6 ♭7 7")), //1900
            ("6(add ♭6 ♭9 ♯9 11 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 6")), //1901
            ("11(♭9 add ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 ♭7")), //1902
            ("Ma11(♭9 add ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 7")), //1903
            ("13(♭9 add ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 6 ♭7")), //1904
            ("Ma13(♭9 add ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 6 7")), //1905
            ("Ma11(♭9 add ♭7 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♯6 7")), //1906
            ("13(♭5 ♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 6 ♭7")), //1907
            ("Ma13(♭5 ♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 6 7")), //1908
            ("Ma11(♭5 ♭9 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 ♭7 7")), //1909
            ("Ma13(♭5 ♭9 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 6 ♭7 7")), //1910
            ("13(♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 6 ♭7")), //1911
            ("Ma13(♭9 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 6 7")), //1912
            ("Ma11(♭9 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 ♭7 7")), //1913
            ("Ma13(♭9 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 6 ♭7 7")), //1914
            ("Ma13(♯5 ♭9 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯5 6 ♭7 7")), //1915
            ("13(♭9 ♯11 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 6 ♭7")), //1916
            ("Ma13(♭9 ♯11 add ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 6 7")), //1917
            ("Ma7(add ♭7 ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 ♭7 7")), //1918
            ("Ma13(♭9 ♯11 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 6 ♭7 7")), //1919
            ("Ma13(♯5 ♭9 ♯11 add ♭7 ♯9)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 ♯5 6 ♭7 7")), //1920
            ("Ma7(add ♭7 ♭9 ♯9 ♭13 13)", Change::from_notes_string("1 ♭2 ♯2 3 5 ♭6 6 ♭7 7")), //1921
            ("mi13(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 6 ♭7")), //1922
            ("miMa13(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 6 7")), //1923
            ("miMa11(♭9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 ♭7 7")), //1924
            ("miMa13(♭9 add ♭7 ♯11)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 6 ♭7 7")), //1925
            ("miMa13(♭5 ♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♭5 ♭6 6 ♭7 7")), //1926
            ("miMa13(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 5 ♭6 6 ♭7 7")), //1927
            ("miMa13(♭9 ♯11 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 ♭3 ♯4 5 ♭6 6 ♭7 7")), //1928
            ("13(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 6 ♭7")), //1929
            ("Ma13(♭9 add ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 6 7")), //1930
            ("Ma11(♭9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 ♭7 7")), //1931
            ("Ma13(♭9 add ♭7 ♯11)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 6 ♭7 7")), //1932
            ("Ma13(♭5 ♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♭5 ♭6 6 ♭7 7")), //1933
            ("Ma13(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 3 4 5 ♭6 6 ♭7 7")), //1934
            ("Ma13(♭9 ♯11 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 3 ♯4 5 ♭6 6 ♭7 7")), //1935
            ("Ma13sus♯4(♭9 add ♭7 ♭13)", Change::from_notes_string("1 ♭2 4 ♯4 5 ♭6 6 ♭7 7")), //1936
            ("13(no 7 add ♭6 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 6")), //1937
            ("11(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 ♭7")), //1938
            ("Ma11(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 7")), //1939
            ("13(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 6 ♭7")), //1940
            ("Ma13(add ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 6 7")), //1941
            ("Ma11(add ♭7 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♯6 7")), //1942
            ("13(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 6 ♭7")), //1943
            ("Ma13(♭5 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 6 7")), //1944
            ("Ma11(♭5 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 ♭7 7")), //1945
            ("Ma13(♭5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 6 ♭7 7")), //1946
            ("13(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 6 ♭7")), //1947
            ("Ma13(add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 6 7")), //1948
            ("Ma11(add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 ♭7 7")), //1949
            ("Ma13(add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 5 6 ♭7 7")), //1950
            ("Ma13(♯5 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 4 ♯5 6 ♭7 7")), //1951
            ("13(♯11 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 6 ♭7")), //1952
            ("Ma13(♯11 add ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 6 7")), //1953
            ("Ma9(add ♭7 ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 ♭7 7")), //1954
            ("Ma13(♯11 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 6 ♭7 7")), //1955
            ("Ma13(♯5 ♯11 add ♭7 ♯9)", Change::from_notes_string("1 2 ♯2 3 ♯4 ♯5 6 ♭7 7")), //1956
            ("Ma9(add ♭7 ♯9 ♭13 13)", Change::from_notes_string("1 2 ♯2 3 5 ♭6 6 ♭7 7")), //1957
            ("mi13(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 6 ♭7")), //1958
            ("miMa13(add ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 6 7")), //1959
            ("miMa11(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 ♭7 7")), //1960
            ("miMa13(add ♭7 ♯11)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 6 ♭7 7")), //1961
            ("miMa13(♭5 add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♭5 ♭6 6 ♭7 7")), //1962
            ("miMa13(add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 4 5 ♭6 6 ♭7 7")), //1963
            ("miMa13(♯11 add ♭7 ♭13)", Change::from_notes_string("1 2 ♭3 ♯4 5 ♭6 6 ♭7 7")), //1964
            ("13(add ♯11 ♭13)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 6 ♭7")), //1965
            ("Ma13(add ♯11 ♭13)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 6 7")), //1966
            ("Ma11(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 ♭7 7")), //1967
            ("Ma13(add ♭7 ♯11)", Change::from_notes_string("1 2 3 4 ♯4 5 6 ♭7 7")), //1968
            ("Ma13(♭5 add ♭7 ♭13)", Change::from_notes_string("1 2 3 4 ♭5 ♭6 6 ♭7 7")), //1969
            ("Ma13(add ♭7 ♭13)", Change::from_notes_string("1 2 3 4 5 ♭6 6 ♭7 7")), //1970
            ("Ma13(♯11 add ♭7 ♭13)", Change::from_notes_string("1 2 3 ♯4 5 ♭6 6 ♭7 7")), //1971
            ("Ma13sus♯4(add ♭7 ♭13)", Change::from_notes_string("1 2 4 ♯4 5 ♭6 6 ♭7 7")), //1972
            ("13(♯9 add ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 6 ♭7")), //1973
            ("Ma13(♯9 add ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 6 7")), //1974
            ("Ma11(♯9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 ♭7 7")), //1975
            ("Ma13(♯9 add ♭7 ♯11)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 6 ♭7 7")), //1976
            ("Ma13(♭5 ♯9 add ♭7 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♭5 ♭6 6 ♭7 7")), //1977
            ("Ma13(♯9 add ♭7 ♭13)", Change::from_notes_string("1 ♯2 3 4 5 ♭6 6 ♭7 7")), //1978
            ("Ma13(♯9 ♯11 add ♭7 ♭13)", Change::from_notes_string("1 ♯2 3 ♯4 5 ♭6 6 ♭7 7")), //1979
            ("miMa13(no 9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♭3 4 ♯4 5 ♭6 6 ♭7 7")), //1980
            ("Ma13(no 9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 3 4 ♯4 5 ♭6 6 ♭7 7")), //1981
            ("13(no 7 add ♭6 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 6")), //1982
            ("11(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 ♭7")), //1983
            ("Ma11(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 7")), //1984
            ("13(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 6 ♭7")), //1985
            ("Ma13(add ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 6 7")), //1986
            ("Ma11(add ♭7 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♯6 7")), //1987
            ("13(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 6 ♭7")), //1988
            ("Ma13(♭5 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 6 7")), //1989
            ("Ma11(♭5 add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 ♭7 7")), //1990
            ("Ma13(♭5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 6 ♭7 7")), //1991
            ("13(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 6 ♭7")), //1992
            ("Ma13(add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 6 7")), //1993
            ("Ma11(add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 ♭7 7")), //1994
            ("Ma13(add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 6 ♭7 7")), //1995
            ("Ma13(♯5 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯5 6 ♭7 7")), //1996
            ("13(♯11 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 6 ♭7")), //1997
            ("Ma13(♯11 add ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 6 7")), //1998
            ("Ma9(add ♭7 ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 ♭7 7")), //1999
            ("Ma13(♯11 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 6 ♭7 7")), //2000
            ("Ma13(♯5 ♯11 add ♭7 ♭9 ♯9)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 ♯5 6 ♭7 7")), //2001
            ("Ma9(add ♭7 ♭9 ♯9 ♭13 13)", Change::from_notes_string("1 ♭2 2 ♭3 3 5 ♭6 6 ♭7 7")), //2002
            ("mi13(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 6 ♭7")), //2003
            ("miMa13(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 6 7")), //2004
            ("miMa11(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 ♭7 7")), //2005
            ("miMa13(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 6 ♭7 7")), //2006
            ("miMa13(♭5 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♭5 ♭6 6 ♭7 7")), //2007
            ("miMa13(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 5 ♭6 6 ♭7 7")), //2008
            ("miMa13(♯11 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 ♯4 5 ♭6 6 ♭7 7")), //2009
            ("13(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 6 ♭7")), //2010
            ("Ma13(add ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 6 7")), //2011
            ("Ma11(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 ♭7 7")), //2012
            ("Ma13(add ♭7 ♭9 ♯11)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 6 ♭7 7")), //2013
            ("Ma13(♭5 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♭5 ♭6 6 ♭7 7")), //2014
            ("Ma13(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 5 ♭6 6 ♭7 7")), //2015
            ("Ma13(♯11 add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 3 ♯4 5 ♭6 6 ♭7 7")), //2016
            ("Ma13sus♯4(add ♭7 ♭9 ♭13)", Change::from_notes_string("1 ♭2 2 4 ♯4 5 ♭6 6 ♭7 7")), //2017
            ("13(♭9 add ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 6 ♭7")), //2018
            ("Ma13(♭9 add ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 6 7")), //2019
            ("Ma11(♭9 add ♭7 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 ♭7 7")), //2020
            ("Ma13(♭9 add ♭7 ♯9 ♯11)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 6 ♭7 7")), //2021
            ("Ma13(♭5 ♭9 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♭5 ♭6 6 ♭7 7")), //2022
            ("Ma13(♭9 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 5 ♭6 6 ♭7 7")), //2023
            ("Ma13(♭9 ♯11 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 ♯4 5 ♭6 6 ♭7 7")), //2024
            ("miMa13(♭9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♭3 4 ♯4 5 ♭6 6 ♭7 7")), //2025
            ("Ma13(♭9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♭2 3 4 ♯4 5 ♭6 6 ♭7 7")), //2026
            ("13(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 6 ♭7")), //2027
            ("Ma13(add ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 6 7")), //2028
            ("Ma11(add ♭7 ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 ♭7 7")), //2029
            ("Ma13(add ♭7 ♯9 ♯11)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 6 ♭7 7")), //2030
            ("Ma13(♭5 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♭5 ♭6 6 ♭7 7")), //2031
            ("Ma13(add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 5 ♭6 6 ♭7 7")), //2032
            ("Ma13(♯11 add ♭7 ♯9 ♭13)", Change::from_notes_string("1 2 ♯2 3 ♯4 5 ♭6 6 ♭7 7")), //2033
            ("miMa13(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 2 ♭3 4 ♯4 5 ♭6 6 ♭7 7")), //2034
            ("Ma13(add ♭7 ♯11 ♭13)", Change::from_notes_string("1 2 3 4 ♯4 5 ♭6 6 ♭7 7")), //2035
            ("Ma13(♯9 add ♭7 ♯11 ♭13)", Change::from_notes_string("1 ♯2 3 4 ♯4 5 ♭6 6 ♭7 7")), //2036
            ("13(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 6 ♭7")), //2037
            ("Ma13(add ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 6 7")), //2038
            ("Ma11(add ♭7 ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 ♭7 7")), //2039
            ("Ma13(add ♭7 ♭9 ♯9 ♯11)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 6 ♭7 7")), //2040
            ("Ma13(♭5 add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♭5 ♭6 6 ♭7 7")), //2041
            ("Ma13(add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 5 ♭6 6 ♭7 7")), //2042
            ("Ma13(♯11 add ♭7 ♭9 ♯9 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 ♯4 5 ♭6 6 ♭7 7")), //2043
            ("miMa13(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 4 ♯4 5 ♭6 6 ♭7 7")), //2044
            ("Ma13(add ♭7 ♭9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 3 4 ♯4 5 ♭6 6 ♭7 7")), //2045
            ("Ma13(♭9 add ♭7 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 ♯2 3 4 ♯4 5 ♭6 6 ♭7 7")), //2046
            ("Ma13(add ♭7 ♯9 ♯11 ♭13)", Change::from_notes_string("1 2 ♯2 3 4 ♯4 5 ♭6 6 ♭7 7")), //2047
            ("Ma13(add ♭7 ♭9 ♯9 ♯11 ♭13)", Change::from_notes_string("1 ♭2 2 ♭3 3 4 ♯4 5 ♭6 6 ♭7 7")), //2048
        ];
        let datas = vec![
            ("sus4", Change::from_notes_string("1 4 5")),
            ("susb2", Change::from_notes_string("1 b2 5")),
            ("6", Change::from_notes_string("1 3 5 6")),
            ("mi6", Change::from_notes_string("1 b3 5 6")),
            ("7", Change::from_notes_string("1 3 5 b7")),
            ("ma", Change::from_notes_string("1 3 5")),
            ("m", Change::from_notes_string("1 b3 5")),
            ("7(b9)", Change::from_notes_string("1 3 5 b7 b9")),
            ("7b9", Change::from_notes_string("1 3 5 b7 b9")),
            ("13", Change::from_notes_string("1 3 5 b7 9 11 13")),
            ("13(#5)", Change::from_notes_string("1 3 #5 b7 9 11 13")),
            ("7sus4(b5)", Change::from_notes_string("1 4 b5 b7")),
            ("ma9sus4(b5)", Change::from_notes_string("1 4 b5 7 9")),
            ("9sus4(b5)", Change::from_notes_string("1 4 b5 b7 9")),
            ("mima7", Change::from_notes_string("1 b3 5 7")),
            ("mma9", Change::from_notes_string("1 b3 5 7 9")),
            ("dimma9", Change::from_notes_string("1 b3 b5 7 9")),
            ("dim9", Change::from_notes_string("1 b3 b5 bb7 9")),
            ("dim7", Change::from_notes_string("1 b3 b5 bb7")),
            ("halfdim7", Change::from_notes_string("1 b3 b5 b7")),
            ("1(5)", Change::from_notes_string("1 5")),
            ("5", Change::from_notes_string("1 5")),
            ("5add6", Change::from_notes_string("1 5 6")),
            ("N.C", Change::from_notes_string("")),
        ];




        let start_time = Instant::now();

        //let test_datas = &datas;
        let test_datas = &datas;
        let test_range = 1239..1300;
        let test_range = 2047..2048;
        let test_range = 1298..1300;
        let test_range = 1200..1300;
        // let num = 1280;
        // let test_range = num .. num + 1;
        // let test_range = 1..250;


        // jazz_chord::ChordQuality::from_string


        //let test_datas = [("m6(#11)", Change::from("1 b3 5 6 #11"))];

        let mut to_change_num_success = 0;
        let mut to_change_num_failure = 0;
        let mut to_chord_num_success = 0;
        let mut to_chord_num_failure = 0;
        for c in test_range.clone() {
            let change_number = c;
            let data = &all_datas[c - 1];

            let ref_str: &str = data.0;
            let ref_chord: Quality = Quality::from_string(ref_str).unwrap();
            let ref_change: &Change = &data.1;
            let from_change_is_legit = ref_change.to_chord_quality().to_change().to_pitch_class_set()
                == ref_change.to_pitch_class_set();

            if from_change_is_legit {
                to_change_num_success += 1;
            } else {
                panic!("No this did not work.");
                to_change_num_failure += 1;
            }
            let chords = ref_change.to_chord_qualities();



            if chords.is_empty(){
                to_chord_num_failure += 1;
            } else {
                to_chord_num_success += 1;
            }
            print!("\n#{}: {} == {} == {} ", change_number, ref_change, ref_str, ref_change.to_pitch_class_set(), );

            if chords[0].is_equivalent(&ref_chord){
                print!("{}", ref_chord);
            } else {
                print!("{} chords:\n\t", chords.len());

                for chord in &chords {
                    let chord_to_change = chord.to_change();
                    let to_change_is_legit = chord_to_change.to_pitch_class_set() == ref_change.to_pitch_class_set();
                    let extra_char = if to_change_is_legit { "" } else { "!!!" };

                    print!("{:<2}{:<25} {}", extra_char, chord.to_string(), chord.rating(&QualityParams::DEFAULT));
                    if !to_change_is_legit {
                        panic!("No. {} {} != expected: {} {}", chord, chord_to_change.to_pitch_class_set(), ref_str, ref_change.to_pitch_class_set());
                    }
                    if chord != chords.last().unwrap() {
                        print!("\n\t")
                    }
                }
            }
        }
        // for c in test_range.clone() {
        //     let data = &all_datas[c - 1];
        //
        //     let chord_input = data.0;
        //     let test_change = &data.1;
        //
        //     let chord_to_change = ChordQuality::from_string(&*chord_input).unwrap_or_default().to_change();
        //
        //     // panic!("{:?} {:?}", chord_to_change.in_first_octave_sorted(),test_change.in_first_octave_sorted() );
        //     let to_change_is_legit = chord_to_change.in_first_octave_sorted().to_frets() == test_change.in_first_octave_sorted().to_frets();
        //
        //     if !to_change_is_legit {
        //         print!{"!^"};
        //         to_change_num_failure += 1;
        //     } else {
        //         to_change_num_success += 1;
        //     }
        //
        //     print!("{} ChordQuality::from_string({:?}) => ", test_change.change_number(), chord_input);
        //     if let Some(chord) =  ChordQuality::from_string(&*chord_input) {
        //
        //         let to_chord_is_legit = chord_to_change.to_frets().to_pitch_class_set() == chord_to_change.to_chord_quality().to_frets().to_pitch_class_set();
        //         if !to_chord_is_legit {
        //             print!{"=-"};
        //             to_chord_num_failure += 1;
        //         } else {
        //             to_chord_num_success += 1;
        //         }
        //
        //         let left = chord.to_change().to_pitch_class_set();
        //         let right = chord.to_frets().to_pitch_class_set();
        //         // println!("{} == {}",chord, chord.short_debug());
        //
        //
        //         if to_change_is_legit {
        //             println!("{} == {}", chord, chord_to_change);
        //         } else {
        //             println!("{}  ?- notes: {} != expected: {test_change}",test_change.change_number(),chord_to_change.in_first_octave_sorted());
        //         }
        //         if !to_chord_is_legit {
        //             println!("{}  ?! chord: {} != expected: {} {} != {} alternatives:\n{}", test_change.change_number(),chord.to_change().to_chord_quality(), chord,chord_to_change.to_frets().in_first_octave(), chord_to_change.to_chord_quality().to_frets().in_first_octave(),
        //                      chord.to_change().to_chord_qualities().iter()
        //                          // .skip(1)
        //                          .map(|n| format!("{} {}", n.to_string(), n.rating(&ChordQualityParams::default())))
        //                          .collect::<Vec<String>>()
        //                          .join("\n\t")
        //                             );
        //         }
        //         //println!("change: {} chord: {} triad:{:?} mod: {:?}",chord.to_change(), &chord.clone(), &chord.triad.unwrap().span,chord.modification.unwrap());
        //
        //
        //
        //
        //         //println!("frets:{:?} change: {} {} display: {}",chord.to_frets(),chord.to_change(),chord.short_debug(), chord)
        //         assert_eq!(chord.to_change().in_first_octave_sorted().to_frets(),
        //                    test_change.in_first_octave_sorted().to_frets(), "ChordQuality::from_string({:?}) should == {}, but is {}", chord_input,*test_change, chord.to_change());
        //
        //         //fixme
        //
        //
        //         assert_eq!(left, right,
        //                    "Change({}).to_chord_quality\nshould == {:?}, but is {}",chord.to_change(), chord_input,chord.to_change().to_chord_quality(), );
        //
        //
        //     }
        //     else {
        //         println!("Invalid input ChordQuality! {}", chord_input);
        //     }
        //     println!();
        //
        //
        // }
        println!();
        println!("Tested parsing {:?} chord symbols into notes. {} failed. {} succeeded.", test_range.len(), to_change_num_failure, to_change_num_success);
        println!("Tested parsing {:?} notes back into chord symbols. {} failed. {} succeeded.", test_range.len(), to_chord_num_failure, to_chord_num_success);
        let elapsed_time = start_time.elapsed();

        // Print the duration
        println!("Elapsed time in test_chord_quality(): {:?}, which means it took {:?}/change",
                 elapsed_time, elapsed_time / test_range.len() as u32);

        println!();
    }


    #[test]
    fn test_speed(){
        let start_time = Instant::now();
        let num = 10000;
        let change = Change::from_notes_string("1 3 5");
        for i in 0..num{
            //let chord = ChordQuality::from_string("Ma13(#11no 5)");

             let chord = change.to_chord_quality();
        }
        // Calculate the elapsed time
        let elapsed_time = start_time.elapsed();

        // Print the duration
        println!("Elapsed time in test_speed(): {:?}, which means it took {:?}/item",
                 elapsed_time, elapsed_time / num);
        println!();
    }

    #[test]
    fn test_key_chord_symbol(){

        let start_time = Instant::now();
        let all_datas = &[
            ("Abma7(b5)", "Ab C Ebb G"),
        ];
        let datas = &all_datas;
        for data in datas.iter(){
            let chord = KeyQuality::from_string(data.0.into());
            if let Some(chord) = chord {
                println!("KeyChordQuality::from_string({}) == {}",data.0,chord);
            }
        }


        let elapsed_time = start_time.elapsed();
        println!("Elapsed time in test_key_chord_symbols(): {:?}, which means it took {:?}/item",
                 elapsed_time, elapsed_time / datas.len() as u32);
        println!();
    }

    #[test]
    fn test_key_polychord_symbol(){
        let mut num_success = 0;
        let mut num_failure = 0;

        let start_time = Instant::now();
        let all_datas = &[
            ("Abma7(b5)", "Ab C Ebb G"),
            ("A##ma7(b5)", "A♯♯ C♯♯♯ E♯ G♯♯♯"),
            ("Cma7(b5)", "C E Gb B"),
            ("C#ma7(b5)", "C# E# G B#"),
            ("Dma7(b5)", "D F# Ab C#"),
            ("G#ma7(b5)", "G# B# D Fx"),
            ("G#ma7(b5)/C#5", "C# G# G♯ B♯ D F♯♯"),
            ("A", "A C# E"),
            ("A/E", "E A C#"),
            ("A/Fb", "E A C#"),
            ("A/F", "F A C# E"),
            ("Eb/A", "A Eb G Bb"),
            ("N.C", ""),
            ("A1", "A"),
        ];
        let datas = &all_datas;
        for data in datas.iter(){

            let key_slash_quality = KeySlashQuality::from_string(data.0.into());
            if let Some(chord) = &key_slash_quality {

            } else {

            }
            let input = data.0;
            let expected_result = data.1;

            if let Some(key_slash_quality) = key_slash_quality {
                let keys = key_slash_quality.keys();
                let expected_keys = Keys::from(expected_result);
                let is_legit = key_slash_quality.keys().is_equivalent(&expected_keys);
                if !keys.is_empty() {
                    let root = &keys[0];
                    let change = keys.to_change(root);
                    let change_to_keys = change.in_key(root);
                    assert!(change_to_keys.is_equivalent(&keys));

                    if is_legit {
                        println!("KeySlashQuality::from_string( \"{input}\" ).keys(): {keys:>80}: in {root}: {change:>100} change_to_keys({change_to_keys}) self: {key_slash_quality}", keys = keys);
                    } else {
                        panic!("KeySlashQuality::from_string( \"{input}\" ).keys(): {keys:>80}\n ->{keys} !=\n ->{expected_keys} (expected result) self: {key_slash_quality}");
                    }
                } else {
                    if is_legit {
                        println!("KeySlashQuality::from_string( \"{input}\" ).keys(): {keys:>80}: in NOROOT: [] ) self: {key_slash_quality}", keys = keys);
                    } else {
                        panic!("KeySlashQuality::from_string( \"{input}\" ).keys(): {keys:>80}\n ->{keys} !=\n ->{expected_keys} (expected result) self: {key_slash_quality}");
                    }
                }
            } else {
                println!("!!KeySlashQuality::from_string( {} ).keys() == NUTHING",input);
            }
        }

        let elapsed_time = start_time.elapsed();
        println!("Elapsed time in test_key_chord_symbols(): {:?}, which means it took {:?}/item",
                 elapsed_time, elapsed_time / datas.len() as u32);
    }



}