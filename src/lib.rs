use std::fs::File;
use std::io::Write;
use std::path::Path;
use svg::{
    svg_draw_barres, svg_draw_bg, svg_draw_finger, svg_draw_min_fret, svg_draw_note, svg_draw_title,
};
use tera::{Context as TeraContext, Tera};
use types::{Chord, GuitarString, Hand};
use utils::{get_filename, get_palette};

mod svg;
pub mod types;
mod utils;

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

#[cfg(test)]
mod tests {
    use crate::{
        generate_svg,
        types::{Chord, Hand, Mode},
    };

    #[test]
    fn should_render_svg_correctly() {
        let title = String::from("Hendrix");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 8, -1],
            fingers: vec!["x", "2", "1", "3", "4", "x"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/8429847222939097413.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("E");
        let chord = Chord {
            title: Some(&title),
            frets: vec![0, 2, 2, 1, 0, 0],
            fingers: vec!["0", "2", "3", "1", "0", "0"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/6264984114944231516.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("C");
        let suffix = String::from("°7");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 3, 4, 2, 3, -1],
            suffix: Some(&suffix),
            fingers: vec!["x", "2", "3", "1", "4", "x"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/14747384251356703516.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("E9");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 7, 6, 7, 7, 7],
            fingers: vec!["x", "2", "1", "3", "3", "3"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/1119813197946994512.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("D7");
        let chord = Chord {
            title: Some(&title),
            frets: vec![10, 12, 10, 11, 10, 10],
            fingers: vec!["1", "3", "1", "2", "1", "1"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/14132601197530680953.svg")
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
        let expected = std::fs::read_to_string("fixtures/14960314899480148130.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_render_barres() {
        // B9 barre regression
        let suffix = String::from("9");
        let title = String::from("B");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 9, 8, 9, 9, 9],
            fingers: vec!["x", "2", "1", "3", "3", "3"],
            suffix: Some(&suffix),
            barres: Some(vec![9]),
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/9333158008996547180.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        let title = String::from("C");
        let suffix = String::from("m");
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 3, 5, 5, 4, 3],
            fingers: vec!["x", "1", "3", "4", "2", "1"],
            barres: Some(vec![3]),
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/12230973133991337290.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_render_barre_at_nut() {
        let title = String::from("F");
        let chord = Chord {
            title: Some(&title),
            frets: vec![1, 3, 3, 2, 1, 1],
            fingers: vec!["1", "3", "4", "2", "1", "1"],
            barres: Some(vec![1]),
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/18011157745197688127.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_not_show_nut_when_min_is_higher_than_3() {
        // nut regression
        let title = String::from("D");
        let suffix = String::from("m69");
        let chord = Chord {
            title: Some(&title),
            frets: vec![-1, 5, 3, 4, 5, 0],
            fingers: vec!["x", "3", "1", "2", "4", "0"],
            suffix: Some(&suffix),
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/13801489451752273984.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_render_bg() {
        // no bg rh
        let title = String::from("E♭");
        let suffix = String::from("7");

        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/14385705171269355287.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // no bg lh
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            hand: Hand::Left,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/16884747429378599218.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // bg rh
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            use_background: true,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/2476955617190468140.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // bg lh
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            hand: Hand::Left,
            use_background: true,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/3472658043626440162.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }

    #[test]
    fn should_use_dark_mode() {
        // light no bg rh
        let title = String::from("E♭");
        let suffix = String::from("7");

        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/14385705171269355287.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // light bg rh
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            use_background: true,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/2476955617190468140.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // dark no bg rh
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            mode: Mode::Dark,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/13340061165425864902.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);

        // dark bg rh
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            frets: vec![-1, 6, 5, 6, -1, -1],
            fingers: vec!["x", "2", "1", "3", "x", "x"],
            use_background: true,
            mode: Mode::Dark,
            ..Default::default()
        };
        let image = generate_svg(chord);
        let expected = std::fs::read_to_string("fixtures/1048205031866609166.svg")
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
        let expected = std::fs::read_to_string("fixtures/left/12943706944351374242.svg")
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
        let expected = std::fs::read_to_string("fixtures/left/12438538594686784945.svg")
            .expect("couldn't open fixture");
        assert_eq!(image.unwrap(), expected);
    }
}

// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872 +
