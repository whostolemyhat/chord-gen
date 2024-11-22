use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use tera::{Context as TeraContext, Tera};

#[derive(Hash, Default)]
pub struct Chord<'a> {
    pub frets: Vec<i32>,       // -1 = skip
    pub fingers: Vec<&'a str>, // 'x' = skip
    pub title: Option<&'a String>,
    pub hand: Hand,
    pub suffix: Option<&'a String>,
    pub mode: Mode,
    pub use_background: bool,
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

impl From<usize> for GuitarString {
    fn from(value: usize) -> Self {
        match value {
            1 => GuitarString::A,
            2 => GuitarString::D,
            3 => GuitarString::G,
            4 => GuitarString::B,
            5 => GuitarString::HighE,
            _ => GuitarString::E,
        }
    }
}

const LIGHT_COLOUR: &'static str = "#FBF6E2";
const DARK_COLOUR: &'static str = "#160c1c";

#[derive(Hash, Default)]
pub enum Mode {
    #[default]
    Light,
    Dark,
}

#[derive(PartialEq, Hash, Default)]
pub enum Hand {
    #[default]
    Right,
    Left,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Hand::Left),
            _ => Ok(Hand::Right),
        }
    }
}

fn svg_draw_bg(use_background: bool, palette: &Palette) -> String {
    if use_background {
        format!(
            "<rect fill=\"{}\" width=\"300\" height=\"310\" rx=\"10\" />",
            palette.bg
        )
    } else {
        "".into()
    }
}

fn svg_draw_finger(finger: &str, i: GuitarString, string_space: &i32, palette: &Palette) -> String {
    let x = 50 + (i as i32 * string_space);
    let y = if finger == "0" || finger == "x" {
        35
    } else {
        265
    };

    format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"16\" fill=\"{}\" font-weight=\"400\">{}</text>",
        x, y, palette.fg, finger
    )
}

fn svg_draw_min_fret(min_fret: &i32, string_space: &i32, palette: &Palette) -> String {
    let offset_top = 50;

    let x = 32;
    let y = string_space * 2 + offset_top - (string_space / 2);
    format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"end\" font-size=\"16\" fill=\"{}\" font-weight=\"400\">{}</text>",
        x, y, palette.fg, min_fret
    )
}

fn svg_draw_note(
    note: &i32,
    string: GuitarString,
    string_space: &i32,
    min_fret: &i32,
    palette: &Palette,
) -> String {
    if note <= &0 {
        return "".to_string();
    }

    let offset_left = 50;
    let offset_top = 50;
    let radius = 13;

    let mut offset_fret = *note;
    if min_fret > &1 {
        offset_fret = (note - min_fret) + 2; // 1=first playable pos
    }

    let x = offset_left + string as i32 * string_space;
    let y = offset_fret * string_space + offset_top - (string_space / 2); // fret
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />",
        x, y, radius, palette.fg
    )
}

struct Palette<'a> {
    fg: &'a str,
    bg: &'a str,
}

fn get_palette<'a>(mode: Mode) -> Palette<'a> {
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

fn generate_svg(chord_settings: Chord) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let string_space = 40;
    let margin = 30;

    let palette = get_palette(chord_settings.mode);

    // var for switching between handedness
    let total_strings = 5;

    let mut fingers = "".to_string();
    for (i, finger) in chord_settings.fingers.iter().enumerate() {
        let string: GuitarString = if chord_settings.hand == Hand::Right {
            i.into()
        } else {
            (total_strings - i).into()
        };
        fingers += &svg_draw_finger(finger, string, &string_space, &palette);
    }

    let lowest_fret: &i32 = chord_settings
        .frets
        .iter()
        .filter(|fret| **fret > 0)
        .min()
        .unwrap_or(&0);

    let show_nut = (chord_settings.frets.contains(&0) && lowest_fret < &5)
        || chord_settings.frets.contains(&1);
    let nut_width = if show_nut { 9 } else { 2 };
    let nut_shape = if show_nut { "round" } else { "butt" };

    let mut notes = "".to_string();
    for (i, note) in chord_settings.frets.iter().enumerate() {
        if note != &0 {
            let string: GuitarString = if chord_settings.hand == Hand::Right {
                i.into()
            } else {
                (total_strings - i).into()
            };
            notes += &svg_draw_note(note, string, &string_space, lowest_fret, &palette);
        }
    }

    let mut min_fret_marker = "".to_string();
    if *lowest_fret > 2 || *lowest_fret > 1 && !show_nut {
        min_fret_marker = svg_draw_min_fret(lowest_fret, &string_space, &palette);
    }

    let chord_title = match (chord_settings.title, chord_settings.suffix) {
        (Some(title), Some(suffix)) => format!(
            "<text x=\"150px\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\"
        text-anchor=\"middle\" font-size=\"24\" fill=\"{}\" font-weight=\"400\">{}<tspan font-size=\"18\" fill=\"{}\" font-weight=\"300\">{}</tspan></text>",
            20,
            palette.fg,
            title,
            palette.fg,
            suffix
        ),
        (Some(title), None) => format!(
            "<text x=\"150px\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\"
  text-anchor=\"middle\" font-size=\"24\" fill=\"{}\" font-weight=\"400\">{}</text>",
            20,
            palette.fg,
            title,
        ),
        _ => String::from(""),
    };

    let mut context = TeraContext::new();
    context.insert("name", &chord_title);
    context.insert("padding", &margin);
    context.insert("nutWidth", &nut_width);
    context.insert("nutShape", &nut_shape);
    context.insert("fingers", &fingers);
    context.insert("notes", &notes);
    context.insert("minFret", &min_fret_marker);
    context.insert("foreground", &palette.fg);
    context.insert(
        "background",
        &svg_draw_bg(chord_settings.use_background, &palette),
    );

    match Tera::one_off(include_str!("../templates/chord.svg"), &context, false) {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("{:?}", e);
            Err(Box::new(e))
        }
    }
}

pub fn render_svg(
    chord_settings: Chord,
    output_dir: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let hashed_title = get_filename(&chord_settings);

    match generate_svg(chord_settings) {
        Ok(result) => {
            let path = Path::new(output_dir).join(format!("{}.svg", hashed_title));
            let mut output = File::create(path)?;
            write!(output, "{}", result)?;
            Ok(hashed_title)
        }

        Err(e) => {
            println!("Failed to create SVG: {:?}", e);
            Err(e)
        }
    }
}

pub fn get_filename(chord: &Chord) -> u64 {
    let mut s = DefaultHasher::new();
    chord.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use crate::{generate_svg, get_filename, svg_draw_note, Chord, Hand};

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
        assert_eq!(filename, 13639735217056851883);

        let title = String::from("Hendrix♮");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 427001437295530661);
        let title = String::from("Hendrix");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            hand: Hand::Left,
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 6949637816226260381);
    }

    #[test]
    fn should_render_svg_correctly() {
        let title = String::from("Hendrix♮");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/13592681158382067823.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("E");
        let chord = Chord {
            title: Some(&title),
            frets: vec![0, 2, 2, 1, 0, 0],
            fingers: vec!["0", "2", "3", "1", "0", "0"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/18436534002643003894.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("C°7");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 3, 4, 2, 3, -1],
            fingers: vec!["x", "2", "3", "1", "4", "x"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/15615698213659243213.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("E9");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 7, 7],
            fingers: vec!["x", "2", "1", "3", "3", "3"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/13724104169966017016.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("D7");
        let chord = Chord {
            title: Some(&title),
            frets: vec![10, 12, 10, 11, 10, 10],
            fingers: vec!["1", "3", "1", "2", "1", "1"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/13518970828834701382.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("Bond");
        let chord = Chord {
            title: Some(&title),
            frets: vec![0, 10, 9, 8, 7, -1],
            fingers: vec!["0", "4", "3", "2", "1", "x"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/12540277254987366366.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("G");
        let chord = Chord {
            title: Some(&title),
            frets: vec![3, 2, 0, 0, 0, 3],
            fingers: vec!["2", "1", "0", "0", "0", "3"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/8535511527932517360.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("A");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 0, 2, 2, 2, 0],
            fingers: vec!["x", "0", "2", "1", "3", "0"],
            hand: Hand::Right,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/6374786531096975228.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_render_left_handed() {
        let title = String::from("A");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 0, 2, 2, 2, 0],
            fingers: vec!["x", "0", "2", "1", "3", "0"],
            hand: Hand::Left,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/left/5698515120374566304.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("Hendrix");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            hand: Hand::Left,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/left/10083194593582405925.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_render_note() {
        let note = svg_draw_note(&6, crate::GuitarString::D, &10, &0);
        let expected = "<circle cx=\"70\" cy=\"105\" r=\"13\" fill=\"#FBF6E2\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&2, crate::GuitarString::E, &12, &1);
        let expected = "<circle cx=\"50\" cy=\"68\" r=\"13\" fill=\"#FBF6E2\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&7, crate::GuitarString::A, &14, &2);
        let expected = "<circle cx=\"64\" cy=\"141\" r=\"13\" fill=\"#FBF6E2\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&4, crate::GuitarString::G, &20, &3);
        let expected = "<circle cx=\"110\" cy=\"100\" r=\"13\" fill=\"#FBF6E2\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&9, crate::GuitarString::B, &30, &5);
        let expected = "<circle cx=\"170\" cy=\"215\" r=\"13\" fill=\"#FBF6E2\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&12, crate::GuitarString::HighE, &32, &10);
        let expected = "<circle cx=\"210\" cy=\"162\" r=\"13\" fill=\"#FBF6E2\" />";
        assert_eq!(note, expected);
    }
}

// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872 +
