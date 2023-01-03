use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;

use std::io::Write;
use tera::{Context as TeraContext, Tera};

#[derive(Hash)]
pub struct Chord<'a> {
    pub frets: Vec<i32>,       // -1 = skip
    pub fingers: Vec<&'a str>, // 'x' = skip
    pub title: &'a str,
}

#[derive(Debug)]
enum GuitarString {
    E = 0,
    A = 1,
    D = 2,
    G = 3,
    B = 4,
    HighE = 5,
}

impl TryFrom<i32> for GuitarString {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(GuitarString::A),
            2 => Ok(GuitarString::D),
            3 => Ok(GuitarString::G),
            4 => Ok(GuitarString::B),
            5 => Ok(GuitarString::HighE),
            _ => Ok(GuitarString::E),
        }
    }
}

fn svg_draw_finger(finger: &str, i: usize, string_space: &i32) -> String {
    let x = 50 + (i as i32 * string_space);
    let y = if finger == "0" || finger == "x" {
        35
    } else {
        265
    };

    format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"16\" fill=\"#223\" font-weight=\"400\">{}</text>",
        x, y, finger
    )
}

fn svg_draw_min_fret(min_fret: &i32, string_space: &i32) -> String {
    let offset_top = 50;

    let x = 32;
    let y = string_space * 2 + offset_top - (string_space / 2);
    format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"end\" font-size=\"16\" fill=\"#223\" font-weight=\"400\">{}</text>",
        x, y, min_fret
    )
}

fn svg_draw_note(note: &i32, string: GuitarString, string_space: &i32, min_fret: &i32) -> String {
    if note <= &0 {
        return "".to_string();
    }

    let offset_left = 50;
    let offset_top = 50;

    let mut offset_fret = *note;
    if min_fret > &0 {
        offset_fret = (note - min_fret) + 2;
    }

    let x = offset_left + string as i32 * string_space;
    let y = offset_fret * string_space + offset_top - (string_space / 2); // fret
    format!("<circle cx=\"{}\" cy=\"{}\" r=\"15\" />", x, y)
}

pub fn render_svg(chord_settings: Chord, output_dir: &str) -> Result<(), std::io::Error> {
    let string_space = 40;
    let margin = 30;

    let mut fingers = "".to_string();
    for (i, finger) in chord_settings.fingers.iter().enumerate() {
        fingers += &svg_draw_finger(finger, i, &string_space);
    }

    let lowest_fret: &i32 = chord_settings
        .frets
        .iter()
        .filter(|fret| **fret > 1)
        .min()
        .unwrap_or(&0);

    let show_nut = (chord_settings.frets.contains(&0) && lowest_fret < &5)
        || chord_settings.frets.contains(&1);
    let nut_width = if show_nut { 10 } else { 2 };

    let mut notes = "".to_string();
    for (i, note) in chord_settings.frets.iter().enumerate() {
        if note != &0 {
            let string: GuitarString = (i as i32).try_into().unwrap_or(GuitarString::E);
            notes += &svg_draw_note(note, string, &string_space, lowest_fret);
        }
    }

    let mut min_fret_marker = "".to_string();
    if *lowest_fret > 2 || *lowest_fret > 1 && !show_nut {
        min_fret_marker = svg_draw_min_fret(lowest_fret, &string_space);
    }

    let mut context = TeraContext::new();
    context.insert("name", &chord_settings.title);
    context.insert("padding", &margin);
    context.insert("nutWidth", &nut_width);
    context.insert("fingers", &fingers);
    context.insert("notes", &notes);
    context.insert("minFret", &min_fret_marker);

    match Tera::one_off(include_str!("../templates/chord.svg"), &context, false) {
        Ok(result) => {
            // println!("{:?}", result);
            let hashed_title = get_filename(&chord_settings);
            println!("{}", hashed_title);

            let path = Path::new(output_dir).join(format!("{}.svg", hashed_title));
            let mut output = File::create(path)?;
            write!(output, "{}", result)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())

    // output
}

pub fn get_filename(chord: &Chord) -> u64 {
    let mut s = DefaultHasher::new();
    chord.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use crate::{get_filename, Chord};

    #[test]
    fn filenames_should_use_chord_hash() {
        let chord = Chord {
            title: "",
            frets: vec![-1, -1, -1, -1, -1, -1],
            fingers: vec!["x", "x", "x", "x", "x", "x"],
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 4259046506241890749);
    }
}

// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872 +
