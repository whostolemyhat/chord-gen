use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use std::fs::File;

enum GuitarString {
    E = 0,
    A = 1,
    D = 2,
    G = 3,
    B = 4,
    HighE = 5,
}

fn draw_note(context: &Context, fret: i32, string: GuitarString, string_space: f64, margin: f64) {
    context.arc(
        string as i32 as f64 * string_space + margin, // string
        fret as f64 * string_space + margin - (string_space / 2.), // fret
        15.,
        0.,
        360.,
    );
    context.fill().expect("failed to fill :(");
}

fn draw_grid(context: &Context, string_space: f64, margin: f64) {
    let end = margin + 5. * string_space;
    for i in 0..6 {
        context.move_to((i as f64 * string_space) + margin, margin);
        context.line_to((i as f64 * string_space) + margin, end);
        context.stroke().expect("Failed to draw");

        context.move_to(margin, margin + (string_space * i as f64));
        context.line_to(end, margin + (string_space * i as f64));
        context.stroke().expect("Failed to draw");
    }
}

fn draw_fingering(
    context: &Context,
    finger: &str,
    string: GuitarString,
    string_space: f64,
    margin: f64,
) {
    let end = margin + 5. * string_space;
    context.move_to(string as i32 as f64 * string_space + margin, end + 32.);
    context.show_text(finger).expect("Can't write");
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 800, 800).expect("oh no");
    let context = Context::new(&surface).expect("Failed to create context");

    context.set_source_rgb(0.6, 0.45, 0.75);
    context.paint().expect("Failed to fill background");
    // set paint
    context.set_source_rgb(0.8, 0.78, 0.644);

    context.new_path();

    let line_colour = (0.1, 0.1, 0.0);
    context.set_source_rgb(line_colour.0, line_colour.1, line_colour.1);
    context.set_line_width(2.0);

    let string_space = 40.;
    let margin = 30.;

    draw_grid(&context, string_space, margin);

    draw_note(&context, 2, GuitarString::G, string_space, margin);
    draw_note(&context, 3, GuitarString::B, string_space, margin);
    draw_note(&context, 2, GuitarString::HighE, string_space, margin);

    // fingering
    context.select_font_face("DejaVuSans.ttf", FontSlant::Normal, FontWeight::Bold);
    context.set_font_size(24 as f64);
    draw_fingering(&context, "x", GuitarString::E, string_space, margin);
    draw_fingering(&context, "x", GuitarString::A, string_space, margin);
    draw_fingering(&context, "0", GuitarString::D, string_space, margin);
    draw_fingering(&context, "2", GuitarString::G, string_space, margin);
    draw_fingering(&context, "3", GuitarString::B, string_space, margin);
    draw_fingering(&context, "1", GuitarString::HighE, string_space, margin);

    let mut file = File::create("debug.png").expect("Can't create file for some reason");
    surface
        .write_to_png(&mut file)
        .expect("Failed to draw image");
}
