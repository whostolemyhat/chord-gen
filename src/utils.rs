use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::types::{Chord, GuitarString, Mode, DARK_COLOUR, LIGHT_COLOUR};

pub fn get_note_coords(
    note: &i32,
    string: GuitarString,
    string_space: &i32,
    min_fret: &i32,
) -> (i32, i32) {
    let offset_left = 50;
    let offset_top = 50;

    let mut offset_fret = *note;
    if min_fret > &1 {
        offset_fret = (note - min_fret) + 2; // 1=first playable pos
    }

    let x = offset_left + string as i32 * string_space;
    let y = offset_fret * string_space + offset_top - (string_space / 2); // fret
    (x, y)
}

#[derive(PartialEq, Debug)]
pub struct Palette<'a> {
    pub fg: &'a str,
    pub bg: &'a str,
}

pub fn get_palette<'a>(mode: Mode) -> Palette<'a> {
    match mode {
        Mode::Light => Palette {
            fg: DARK_COLOUR,
            bg: LIGHT_COLOUR,
        },
        Mode::Dark => Palette {
            fg: LIGHT_COLOUR,
            bg: DARK_COLOUR,
        },
    }
}

pub fn find_all(frets: &[i32], search: &i32) -> Vec<usize> {
    frets
        .iter()
        .enumerate()
        .filter(|(index, fret)| {
            // the E9 check!
            // does next fret exist and is played?
            if index + 1 < frets.len() && frets[index + 1] != -1 {
                // is next fret higher or eq?
                return *fret == search && frets[index + 1] >= **fret;
            }
            *fret == search
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
}

pub fn get_filename(chord: &Chord) -> u64 {
    let mut s = DefaultHasher::new();
    chord.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{Chord, GuitarString, Hand, Mode, DARK_COLOUR, LIGHT_COLOUR},
        utils::{find_all, get_filename, get_note_coords, get_palette, Palette},
    };

    #[test]
    fn should_get_note_coords() {
        assert_eq!(get_note_coords(&5, GuitarString::A, &40, &3), (90, 190));
        assert_eq!(get_note_coords(&5, GuitarString::E, &40, &3), (50, 190));
        assert_eq!(get_note_coords(&5, GuitarString::D, &40, &3), (130, 190));
        assert_eq!(get_note_coords(&5, GuitarString::G, &40, &3), (170, 190));
        assert_eq!(get_note_coords(&5, GuitarString::B, &40, &3), (210, 190));
        assert_eq!(
            get_note_coords(&5, GuitarString::HighE, &40, &3),
            (250, 190)
        );
    }

    #[test]
    fn shoud_get_palette() {
        assert_eq!(
            get_palette(Mode::Light),
            Palette {
                fg: DARK_COLOUR,
                bg: LIGHT_COLOUR
            }
        );
        assert_eq!(
            get_palette(Mode::Dark),
            Palette {
                fg: LIGHT_COLOUR,
                bg: DARK_COLOUR
            }
        );
    }

    #[test]
    fn check_find_all() {
        assert_eq!(find_all(&vec![2, 1, 3], &1), vec![1]);
        let empty_expected: Vec<usize> = vec![];
        assert_eq!(find_all(&vec![2, 2, 2], &1), empty_expected);
        assert_eq!(find_all(&vec![2, 2, 2], &2), vec![0, 1, 2]);
        assert_eq!(find_all(&vec![2, 2, 2, 3, 4, 2], &2), vec![0, 1, 2, 5]);

        // not if there's a lower neighbour later on - E9
        assert_eq!(find_all(&vec![2, 1, 2, 2, 4, 2], &2), vec![2, 3, 5]);
    }

    #[test]
    fn filenames_should_use_chord_hash() {
        let title = String::from("");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, -1, -1, -1, -1, -1],
            fingers: vec!["x", "x", "x", "x", "x", "x"],
            hand: Hand::Right,
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 3459505358187650210);

        let title = String::from("Hendrix♮");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 10698197080569506319);

        let title = String::from("Hendrix");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            hand: Hand::Left,
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 12438538594686784945);
    }
}
