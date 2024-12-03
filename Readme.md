Rust library to create guitar chord diagrams in SVG format.

<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/13217194300744275703.svg" width="300" />

Use it online at [chordgenerator.xyz](https://chordgenerator.xyz)

## Usage

``` 
cargo run -- --help

Creates guitar chord diagrams

Usage: chord_cli [OPTIONS]

Options:
  -f, --frets <FRETS>      Notes to fret, 6 comma-separated values. 0 for open string, -1 to skip a string.
  -p, --fingers <FINGERS>  Suggested fingering, 6 comma-separated values. 0 for open string, x to skip a string.
  -t, --title <TITLE>      Name of chord. Optional.
  -s, --suffix <SUFFIX>    Chord suffix to use in title. Optional.
  -d, --hand <HANDEDNESS>  Left or right handedness. `left` or `right`. Optional, defaults to right.
  -r, --barres <BARRES>    Frets which should be barred. Comma-separated string. Optional.
  -m, --mode <MODE>        Light or dark mode `light` or `dark`. Optional, defaults to light.
  -b, --background         Add a background to image. Optional.
  -h, --help               Print help information
  -V, --version            Print version information
```

This crate contains a library and a command line binary.

Run the cli binary by installing: `cargo install chord-cli` or running locally.

## Features

### Barre chords
```
cargo run -- -f "x,9,8,9,9,9" -p "x,2,1,3,3,3" -t "B" -s "9" -r 9
```

<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/9333158008996547180.svg" width="300" />

### Dark mode/background
```
cargo run -- -f "x,6,5,6,x,x" -p "x,2,1,3,x,x" -t "E♭" -s "7" -b
```
<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/2476955617190468140.svg" width="300" />

```
cargo run -- -f "x,6,5,6,x,x" -p "x,2,1,3,x,x" -t "E♭" -s "7" -b -m "dark"
```
<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/1048205031866609166.svg" width="300" />

### Left-handed

```
cargo run -- -f "x,0,2,2,2,0" -p "x,0,2,1,3,0" -t "A" -d "left"
```
<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/left/12943706944351374242.svg" width="300" />

```
cargo run -- -f "x,7,6,7,8,x" -p "x,2,1,3,4,x" -t "Hendrix" -d "left"
```
<img src="https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/left/12438538594686784945.svg" width="300" />