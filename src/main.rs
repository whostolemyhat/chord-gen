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
    context.move_to(20., 20.);
    context.line_to(20., 200.);

    let line_colour = (0.1, 0.1, 0.0);
    context.set_line_width(2.0);
    context.set_source_rgb(line_colour.0, line_colour.1, line_colour.1);
    context.stroke().expect("Failed to draw");

    let mut file = File::create("debug.png").expect("Can't create file for some reason");
    surface
        .write_to_png(&mut file)
        .expect("Failed to draw image");
}
