// cargo run -- -f "x,x,x,0,2,0" -p "x,x,x,2,3,1" -t "D♭7"
use chord_gen::{render_svg, Chord};
use clap::{arg, Command};

// https://en.wikiversity.org/wiki/Template:Music_symbols
// https://en.wikipedia.org/wiki/Chord_notation
// ♭ \u266D
// ♯ \u266F
// natural ♮ \u266E
// dim o U+E870
// aug + U+E872

fn main() -> Result<(), std::io::Error> {
    let matches = Command::new("ChordGenerator")
        .version("0.1")
        .author("James Baum <james@jamesbaum.co.uk>")
        .about("Creates guitar chord diagrams")
        .arg(arg!(-f --frets <VALUE> "Notes to fret")) // comma-separated string x,x,0,2,3,2
        .arg(arg!(-p --fingers <VALUE> "Suggested fingering")) // comma-separated string x,x,0,2,3,1
        .arg(arg!(-t --title <VALUE> "Name of chord"))
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

    let default_title = "".to_string();
    let title = matches.get_one::<String>("title").unwrap_or(&default_title);

    // let settings = Chord {
    //     frets: vec![5, 7, 7, 6, 5, 5],
    //     fingers: vec!['1', '3', '4', '2', '1', '1'],
    //     size: 1,
    //     title: "A",
    // };
    // let settings = Chord {
    //     frets: vec![-1, -1, 4, 5, 3, -1],
    //     fingers: vec!['x', 'x', '2', '3', '1', 'x'],
    //     size: 1,
    //     title: "../P7",
    // };
    // let settings = Settings {
    //     frets: vec![0, 0, 0, 2, 3, 2],
    //     fingers: vec!['x', 'x', '0', '2', '3', '1'],
    //     size: 1,
    //     title: "D",
    // };

    // TODO palette etc
    let output_dir = "./output";

    let chord = Chord {
        frets,
        fingers,
        title,
    };

    render_svg(chord, output_dir)?;
    Ok(())
}
