use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;

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

fn draw_note(
    context: &Context,
    fret: i32,
    string: GuitarString,
    string_space: f64,
    margin: f64,
    size: i32,
    min_fret: &i32,
) {
    if fret <= 0 {
        return;
    }

    let mut offset_fret = fret;
    if min_fret > &0 {
        offset_fret = (fret - min_fret) + 2;
    }

    let sizes = [14, 22, 30, 38];
    let offset_top = 78.;
    let radius = sizes[size as usize] as f64;
    context.arc(
        string as i32 as f64 * string_space + (margin * 2.), // string
        offset_fret as f64 * string_space + margin - (string_space / 2.) + offset_top, // fret
        radius,
        0.,
        360.,
    );
    context.fill().expect("failed to fill :(");
}

fn draw_grid(context: &Context, string_space: f64, margin: f64, show_nut: bool) {
    let offset_top = 78.;

    let end = margin + 5. * string_space;
    for i in 0..6 {
        // vert
        context.move_to(
            (i as f64 * string_space) + (margin * 2.),
            margin + offset_top,
        );
        context.line_to((i as f64 * string_space) + (margin * 2.), end + offset_top);
        context.stroke().expect("Failed to draw");

        // horz
        // draw thick line for nut
        if i == 0 && show_nut {
            context.move_to(
                (margin * 2.) - 1.,
                margin + offset_top + (string_space * i as f64),
            );
            context.line_to(
                end + margin + 1.,
                margin + offset_top + (string_space * i as f64),
            );

            context.set_line_width(10.0);
            context.stroke().expect("Failed to draw");
        } else {
            context.move_to(margin * 2., margin + offset_top + (string_space * i as f64));
            context.line_to(
                end + margin,
                margin + offset_top + (string_space * i as f64),
            );

            context.set_line_width(2.0);
            context.stroke().expect("Failed to draw");
        }
        context.set_line_width(2.0);
    }
}

fn draw_fingering(
    context: &Context,
    finger: &str,
    string: GuitarString,
    string_space: f64,
    margin: f64,
) {
    let offset_top = 78.;
    let font_offset = 32.;
    let font_offset_top = 14.;

    // 5 = last string index
    // let end = margin + 5. * string_space;
    let end = if finger == "x" || finger == "0" {
        margin + offset_top - font_offset_top
    } else {
        margin + (5. * string_space) + font_offset + offset_top
    };

    let font_width = 8.;
    context.move_to(
        string as i32 as f64 * string_space + (margin * 2.) - font_width,
        end,
    );
    context
        .show_text(&finger.to_string()[..])
        .expect("Can't write fingering");
}

fn draw_min_fret(context: &Context, min_fret: &i32, string_space: f64, margin: f64) {
    let offset_top = 78.;
    let margin_left = if min_fret >= &10 {
        margin * 0.5
    } else {
        margin * 0.75
    };
    context.move_to(
        margin_left,
        (string_space * 2.) + offset_top + string_space / 2.,
    );
    context
        .show_text(&min_fret.to_string()[..])
        .expect("Can't write min fret");
}

pub fn render(chord_settings: Chord, output_dir: &str) -> Result<(), cairo::IoError> {
    let chord_size = 1;
    let selected_size = std::cmp::min(chord_size, 4) - 1;

    let sizes = [40., 60., 80., 100.];
    let string_space = sizes[selected_size as usize];
    let margins = [30., 40., 50., 60.];
    let margin = margins[selected_size as usize];
    let title_offset = 64.;
    let fingering_height = 64.;
    let font_sizes = [24., 28., 32., 36.];
    let font_size = font_sizes[selected_size as usize];

    let width = (4. * margin) + (string_space as f64) * 5.;
    let height = title_offset + (2. * margin) + fingering_height + (string_space * 5.);

    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).expect("oh no");
    let context = Context::new(&surface).expect("Failed to create context");

    // transparent white
    context.set_source_rgba(1., 1., 1., 0.);
    context.paint().expect("Failed to fill background");
    // set paint
    context.set_source_rgb(0.8, 0.78, 0.644);

    let line_colour = (0.1, 0.1, 0.0);
    context.set_source_rgb(line_colour.0, line_colour.1, line_colour.1);
    context.set_line_width(2.0);

    // fingering
    // note font needs to be installed globally
    context.select_font_face("DejaVuSans", FontSlant::Normal, FontWeight::Normal);
    context.set_font_size(36.);

    let title_left_offset = 24.;
    let title_len = chord_settings.title.chars().count();
    let char_width = 24.;
    context.move_to(
        width / 2. - ((title_len / 2) as f64 * char_width),
        margin + title_left_offset,
    );
    context
        .show_text(chord_settings.title)
        .expect("Can't write title");

    // footer watermark
    context.move_to(margin * 2. + string_space - 10., height - 16.);
    context.select_font_face("DejaVuSans", FontSlant::Normal, FontWeight::Normal);
    context.set_font_size(14.);
    context
        .show_text("chordgenerator.xyz")
        .expect("Can't write watermark");

    context.new_path();

    let lowest_fret: &i32 = chord_settings
        .frets
        .iter()
        .filter(|fret| **fret > 1)
        .min()
        .unwrap_or(&0);

    let show_nut = (chord_settings.frets.contains(&0) && lowest_fret < &5)
        || chord_settings.frets.contains(&1);

    draw_grid(&context, string_space, margin, show_nut);

    for (i, fret) in chord_settings.frets.iter().enumerate() {
        if fret != &0 {
            let string: GuitarString = (i as i32).try_into().unwrap_or(GuitarString::E);
            draw_note(
                &context,
                *fret,
                string,
                string_space,
                margin,
                selected_size,
                lowest_fret,
            );
        }
    }

    // fingering
    context.set_font_size(font_size);

    for (i, finger) in chord_settings.fingers.iter().enumerate() {
        let string: GuitarString = (i as i32).try_into().unwrap_or(GuitarString::E);
        draw_fingering(&context, finger, string, string_space, margin);
    }

    if *lowest_fret > 2 || *lowest_fret > 1 && !show_nut {
        draw_min_fret(&context, lowest_fret, string_space, margin);
    }

    let hashed_title = get_filename(&chord_settings);

    // println!("hashed {}", hashed_title);

    let mut file = File::create(Path::new(output_dir).join(format!("{}.png", hashed_title)))
        .expect("Can't create file for some reason");
    surface.write_to_png(&mut file)
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
