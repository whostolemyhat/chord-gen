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
    pub barres: Option<Vec<i32>>,
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

const LIGHT_COLOUR: &str = "#FBF6E2";
const DARK_COLOUR: &str = "#160c1c";

#[derive(Hash, Default, Copy, Clone)]
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

fn get_note_coords(
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
    let radius = 13;

    let (x, y) = get_note_coords(note, string, string_space, min_fret);
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

fn svg_draw_title(chord_settings: &Chord, palette: &Palette) -> String {
    match (chord_settings.title, chord_settings.suffix) {
        (Some(title), Some(suffix)) => format!(
            "<text x=\"150px\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\"
        text-anchor=\"middle\" font-size=\"24\" fill=\"{}\" font-weight=\"400\">{}<tspan font-size=\"18\" fill=\"{}\" font-weight=\"300\">{}</tspan></text>",
            18,
            palette.fg,
            title,
            palette.fg,
            suffix
        ),
        (Some(title), None) => format!(
            "<text x=\"150px\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\"
  text-anchor=\"middle\" font-size=\"24\" fill=\"{}\" font-weight=\"400\">{}</text>",
            18,
            palette.fg,
            title,
        ),
        _ => String::from(""),
    }
}

fn find_all(frets: &Vec<i32>, search: &i32) -> Vec<usize> {
    frets
        .iter()
        .enumerate()
        .filter(|(index, &ref fret)| {
            // the E9 check!
            // does next fret exist and is played?
            if index + 1 < frets.len() && frets[index + 1] != -1 {
                // is next fret higher or eq?
                return fret == search && frets[index + 1] >= *fret;
            }
            return fret == search;
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
}

fn svg_draw_barres(
    barre_fret: &i32,
    frets: &Vec<i32>,
    string_space: &i32,
    min_fret: &i32,
    palette: &Palette,
) -> String {
    // get first instance of fret
    // check first doesn't have lower neighbour
    // start at lowest string
    // get last instance of fret
    // draw curve
    let strings = find_all(frets, barre_fret);
    if strings.len() < 2 {
        return String::from("");
    }

    let first = get_note_coords(
        barre_fret,
        (*strings.first().unwrap_or(&0)).into(),
        string_space,
        min_fret,
    );
    let last = get_note_coords(
        barre_fret,
        (*strings.last().unwrap_or(&0)).into(),
        string_space,
        min_fret,
    );

    // move curve out of centre of frets
    let y_offset = if barre_fret == &1 { 27 } else { 23 };

    // amount to move the controls
    // controls the angle of curve
    let control_y_offset = y_offset + 10;
    let control_x_offset = 8;

    let origin_control = (first.0 + control_x_offset, first.1 - control_y_offset);
    let end_control = (last.0 - control_x_offset, last.1 - control_y_offset);

    format!(
        "<path d=\"M {} {} C {} {}, {} {}, {} {}\" stroke=\"{}\" stroke-width=\"3\" fill=\"transparent\" stroke-linecap=\"round\" />",
        first.0,
        first.1 - y_offset,
        origin_control.0,
        origin_control.1,
        end_control.0,
        end_control.1,
        last.0,
        last.1 - y_offset,
        palette.fg
    )
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

    let show_nut = (chord_settings.frets.contains(&0) && lowest_fret < &3)
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

    let chord_title = svg_draw_title(&chord_settings, &palette);
    // if barre
    // for each barre
    let barres = match chord_settings.barres {
        Some(barres) => svg_draw_barres(
            &barres[0],
            &chord_settings.frets,
            &string_space,
            &lowest_fret,
            &palette,
        ),
        None => String::from(""),
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
    context.insert("barres", &barres);

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
    use crate::{generate_svg, get_filename, svg_draw_note, Chord, Hand, Palette};

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
        assert_eq!(filename, 16484474795306184489);

        let title = String::from("Hendrix♮");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 13635989049377548228);
        let title = String::from("Hendrix");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            hand: Hand::Left,
            ..Default::default()
        };
        let filename = get_filename(&chord);
        assert_eq!(filename, 16803325980362310838);
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
        let expected = std::fs::read_to_string("fixtures/13635989049377548228.svg")
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
        let expected = std::fs::read_to_string("fixtures/6468161314235284664.svg")
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
        let expected = std::fs::read_to_string("fixtures/17174280223802521722.svg")
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
        let expected = std::fs::read_to_string("fixtures/13685234863090620098.svg")
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
        let expected = std::fs::read_to_string("fixtures/14451764860938368709.svg")
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
        let expected = std::fs::read_to_string("fixtures/14020184813522087305.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // nut regression
        let title = String::from("D");
        let suffix = String::from("m69");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 5, 3, 4, 5, 0],
            fingers: vec!["x", "3", "1", "2", "4", "0"],
            hand: Hand::Right,
            suffix: Some(&suffix),
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/13801489451752273984.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // B9 barre regression
        let suffix = String::from("aug69");
        let title = String::from("B");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 9, 8, 9, 9, 9],
            fingers: vec!["x", "2", "1", "3", "3", "3"],
            hand: Hand::Right,
            suffix: Some(&suffix),
            barres: Some(vec![9]),
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/2945394272046755899.svg")
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
        let expected = std::fs::read_to_string("fixtures/left/1682699279882065446.svg")
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
        let expected = std::fs::read_to_string("fixtures/left/16803325980362310838.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_render_note() {
        let palette = Palette {
            fg: "#fff",
            bg: "#111",
        };
        let note = svg_draw_note(&6, crate::GuitarString::D, &10, &0, &palette);
        let expected = "<circle cx=\"70\" cy=\"105\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&2, crate::GuitarString::E, &12, &1, &palette);
        let expected = "<circle cx=\"50\" cy=\"68\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&7, crate::GuitarString::A, &14, &2, &palette);
        let expected = "<circle cx=\"64\" cy=\"141\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&4, crate::GuitarString::G, &20, &3, &palette);
        let expected = "<circle cx=\"110\" cy=\"100\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&9, crate::GuitarString::B, &30, &5, &palette);
        let expected = "<circle cx=\"170\" cy=\"215\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&12, crate::GuitarString::HighE, &32, &10, &palette);
        let expected = "<circle cx=\"210\" cy=\"162\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);
    }
}

// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872 +
