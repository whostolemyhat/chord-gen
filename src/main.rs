use crate::render::{render, Chord};

use clap::{arg, Command};

mod render;

// https://en.wikiversity.org/wiki/Template:Music_symbols
// https://en.wikipedia.org/wiki/Chord_notation
// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872

fn main() -> Result<(), cairo::IoError> {
    // let settings = Settings {
    //     frets: vec![0, 0, 0, 2, 3, 2],
    //     fingers: vec!['x', 'x', '0', '2', '3', '1'],
    //     size: 1,
    //     title: "D",
    // };

    let matches = Command::new("ChordGenerator")
        .version("0.1")
        .author("James Baum <james@jamesbaum.co.uk>")
        .about("Creates guitar chord diagrams")
        .arg(arg!(-f --frets <VALUE> "Notes to fret"))
        .arg(arg!(-p --fingers <VALUE> "Suggested fingering"))
        .arg(arg!(-t --title <VALUE> "Name of chord"))
        .get_matches();

    let default_frets = "x,x,x,x,x,x".to_string();
    let frets: Vec<&str> = matches
        .get_one::<String>("frets")
        .unwrap_or(&default_frets)
        .split(',')
        .collect();

    println!("frets {:?}", frets);
    println!("fingers {:?}", matches.get_one::<String>("fingers"));
    println!("title {:?}", matches.get_one::<String>("title"));

    let settings = Chord {
        frets: vec![5, 7, 7, 6, 5, 5],
        fingers: vec!['1', '3', '4', '2', '1', '1'],
        size: 1,
        title: "A",
    };
    let output_dir = "./output";

    // let settings = Chord {
    //     frets: vec![-1, -1, 4, 5, 3, -1],
    //     fingers: vec!['x', 'x', '2', '3', '1', 'x'],
    //     size: 1,
    //     title: "../P7",
    // };

    render(settings, output_dir)?;
    Ok(())
}
