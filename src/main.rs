use cairo::{Context, Format, ImageSurface};
use std::fs::File;

fn main() {
    println!("Hello, world!");
    // 1. draw an image
    // 2. draw a circle
    //   3. draw 6 lines
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
    // let start = 20.;
    let end = 230.;

    let margin = 30.;

    for col in 0..6 {
        context.move_to((col as f64 * string_space) + margin, margin);
        context.line_to((col as f64 * string_space) + margin, end);
        context.stroke().expect("Failed to draw");
    }

    for row in 0..6 {
        context.move_to(margin, margin + (string_space * row as f64));
        context.line_to(end, margin + (string_space * row as f64));
        context.stroke().expect("Failed to draw");
    }

    // context.move_to(40., 20.);
    // context.line_to(40., 200.);
    // context.stroke().expect("Failed to draw");

    // context.move_to(60., 20.);
    // context.line_to(60., 200.);
    // context.stroke().expect("Failed to draw");

    // context.move_to(80., 20.);
    // context.line_to(80., 200.);
    // context.stroke().expect("Failed to draw");

    // context.move_to(100., 20.);
    // context.line_to(100., 200.);
    // context.stroke().expect("Failed to draw");

    // context.move_to(120., 20.);
    // context.line_to(120., 200.);
    // context.stroke().expect("Failed to draw");

    let mut file = File::create("debug.png").expect("Can't create file for some reason");
    surface
        .write_to_png(&mut file)
        .expect("Failed to draw image");
}
