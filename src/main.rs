use chord_gen::{
    render_svg,
    types::{Chord, Hand, Mode},
};
use clap::{arg, Command};

// https://en.wikiversity.org/wiki/Template:Music_symbols
// https://en.wikipedia.org/wiki/Chord_notation
// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ChordGenerator")
        .version("2.1.1")
        .author("James Baum <james@jamesbaum.co.uk>")
        .about("Creates guitar chord diagrams")
        .arg(arg!(-f --frets <FRETS> "Notes to fret, 6 comma-separated values. 0 for open string, -1 to skip a string.")) // comma-separated string x,x,0,2,3,2
        .arg(arg!(-p --fingers <FINGERS> "Suggested fingering, 6 comma-separated values. 0 for open string, x to skip a string.")) // comma-separated string x,x,0,2,3,1
        .arg(arg!(-t --title <TITLE> "Name of chord. Optional."))
        .arg(arg!(-s --suffix <SUFFIX> "Chord suffix to use in title. Optional."))
        .arg(arg!(-d --hand <HANDEDNESS> "Left or right handedness. `left` or `right`. Optional, defaults to right."))
        .arg(arg!(-r --barres <BARRES> "Frets which should be barred. Comma-separated string. Optional."))
        .arg(arg!(-m --mode <MODE> "Light or dark mode `light` or `dark`. Optional, defaults to light."))
        .arg(arg!(-b --background "Add a background to image. Optional."))
        .get_matches();

    let default_frets = "x,x,x,x,x,x".to_string();
    let frets: Vec<i32> = matches
        .get_one::<String>("frets")
        .unwrap_or(&default_frets)
        .split(',')
        .map(|letter| letter.parse::<i32>().unwrap_or(-1))
        .collect();

    let fingers: Vec<&str> = matches
        .get_one::<String>("fingers")
        .unwrap_or(&default_frets)
        .split(',')
        .collect();

    let barres: Option<Vec<i32>> = matches.get_one::<String>("barres").map(|frets| {
        frets
            .split(',')
            .map(|f| f.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<_>>()
    });

    let mut hand = Hand::Right;
    if let Some(h) = matches.get_one::<String>("hand") {
        if h == "left" {
            hand = Hand::Left;
        }
    }
    let mut mode = Mode::Light;
    if let Some(m) = matches.get_one::<String>("mode") {
        if m == "dark" {
            mode = Mode::Dark;
        }
    }
    let title = matches.get_one::<String>("title");
    let suffix = matches.get_one::<String>("suffix");

    let use_background = matches.get_one::<bool>("background").unwrap_or(&false);

    // examples
    // cargo run -- -f "x,0,2,2,2,0" -p "x,0,2,1,3,0" -t "A" -d "left"
    // cargo run -- -f "x,0,2,2,2,0" -p "x,0,2,1,3,0" -t "A"
    // cargo run -- -f "x,7,6,7,8,x" -p "x,2,1,3,4,x" -t "Hendrix" -d "left"
    // cargo run -- -f "x,7,6,7,8,x" -p "x,2,1,3,4,x" -t "Hendrix" -d "right"
    // cargo run -- -f "4,3,1,1,1,x" -p "3,2,1,1,1,x" -t "Broken"

    let output_dir = "./output/";

    let chord = Chord {
        frets,
        fingers,
        title,
        hand,
        suffix,
        mode,
        use_background: *use_background,
        barres,
    };

    let filename = render_svg(chord, output_dir)?;
    println!("{}", filename);

    Ok(())
}
