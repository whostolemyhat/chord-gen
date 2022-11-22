use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use std::fs::File;

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
) {
    let sizes = [16, 24, 32, 40];
    let offset_top = 64.;
    let radius = sizes[size as usize] as f64;
    context.arc(
        string as i32 as f64 * string_space + margin, // string
        fret as f64 * string_space + margin - (string_space / 2.) + offset_top, // fret
        radius,
        0.,
        360.,
    );
    context.fill().expect("failed to fill :(");
}

fn draw_grid(context: &Context, string_space: f64, margin: f64, has_open: bool) {
    let offset_top = 64.;

    let end = margin + 5. * string_space;
    for i in 0..6 {
        context.move_to((i as f64 * string_space) + margin, margin + offset_top);
        context.line_to((i as f64 * string_space) + margin, end + offset_top);
        context.stroke().expect("Failed to draw");

        context.move_to(margin, margin + offset_top + (string_space * i as f64));
        context.line_to(end, margin + offset_top + (string_space * i as f64));
        // draw thick line for nut
        if i == 0 && has_open {
            context.set_line_width(12.0);
        }
        context.stroke().expect("Failed to draw");
        if i == 0 && has_open {
            context.set_line_width(2.0);
        }
    }
}

fn draw_fingering(
    context: &Context,
    finger: &str,
    string: GuitarString,
    string_space: f64,
    margin: f64,
) {
    let offset_top = 64.;
    let font_offset = 32.;

    // 5 = last string index
    let end = margin + 5. * string_space;

    context.move_to(
        string as i32 as f64 * string_space + margin,
        end + font_offset + offset_top,
    );
    context.show_text(finger).expect("Can't write fingering");
}

struct Settings<'a> {
    fingers: Vec<&'a str>,
    frets: Vec<i32>,
    size: i32,
    title: &'a str,
}

fn main() {
    // let d_settings = Settings {
    //     frets: vec![0, 0, 0, 2, 3, 2],
    //     fingers: vec!["x", "x", "0", "2", "3", "1"],
    //     size: 1,
    //     title: "D",
    // };

    let settings = Settings {
        frets: vec![5, 7, 7, 6, 5, 5],
        fingers: vec!["1", "3", "4", "2", "1", "1"],
        size: 1,
        title: "A",
    };

    let selected_size = std::cmp::min(settings.size, 4) - 1;

    let sizes = [40., 60., 80., 100.];
    let string_space = sizes[selected_size as usize];
    let margins = [30., 40., 50., 60.];
    let margin = margins[selected_size as usize];
    let title_offset = 64.;
    let fingering_height = 64.;
    let font_sizes = [24., 28., 32., 36.];
    let font_size = font_sizes[selected_size as usize];

    let width = (2. * margin) + (string_space as f64) * 5.;
    let height = title_offset + (2. * margin) + fingering_height + (string_space * 5.);

    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).expect("oh no");
    let context = Context::new(&surface).expect("Failed to create context");

    context.set_source_rgb(0.6, 0.45, 0.75);
    context.paint().expect("Failed to fill background");
    // set paint
    context.set_source_rgb(0.8, 0.78, 0.644);

    let line_colour = (0.1, 0.1, 0.0);
    context.set_source_rgb(line_colour.0, line_colour.1, line_colour.1);
    context.set_line_width(2.0);

    // fingering
    context.select_font_face("DejaVuSans.ttf", FontSlant::Normal, FontWeight::Bold);
    context.set_font_size(64.);

    // 32 = font-size / 2
    context.move_to(100., margin + 32.);
    context
        .show_text(settings.title)
        .expect("Can't write title");

    context.new_path();

    let has_open = settings.fingers.contains(&"0");
    let lowest_fret = settings.fingers.filter(|finger| ).map(|finger | {

    })

    draw_grid(&context, string_space, margin, has_open);

    for (i, fret) in settings.frets.iter().enumerate() {
        if fret != &0 {
            let string: GuitarString = (i as i32).try_into().unwrap_or(GuitarString::E);
            draw_note(&context, *fret, string, string_space, margin, selected_size);
        }
    }

    // fingering
    context.select_font_face("DejaVuSans.ttf", FontSlant::Normal, FontWeight::Bold);
    context.set_font_size(font_size);

    for (i, finger) in settings.fingers.iter().enumerate() {
        let string: GuitarString = (i as i32).try_into().unwrap_or(GuitarString::E);
        draw_fingering(&context, finger, string, string_space, margin);
    }

    let mut file = File::create("debug.png").expect("Can't create file for some reason");
    surface
        .write_to_png(&mut file)
        .expect("Failed to draw image");
}
