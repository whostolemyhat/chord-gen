Rust library to create guitar chord diagrams.

<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/13217194300744275703.svg" width="300" />

Use it online at [chordgenerator.xyz](https://chordgenerator.xyz)

## Usage

This crate contains a library and a command line binary.

Run the cli binary locally with

```
cargo run -- -f "x,x,x,2,3,2" -p "x,x,x,2,3,1" -t "D"
```

<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/4095730029079104823.svg" width="300" />

``` 
cargo run -- --help

Creates guitar chord diagrams

Usage: chord_cli [OPTIONS]

Options:
  -f, --frets <FRETS>      Notes to fret, 6 comma-separated values. 0 for open string, -1 to skip a string.
  -p, --fingers <FINGERS>  Suggested fingering, 6 comma-separated values. 0 for open string, x to skip a string.
  -t, --title <TITLE>      Name of chord
```
