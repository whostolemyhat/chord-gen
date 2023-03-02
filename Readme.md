Rust library to create guitar chord diagrams.

![](https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/13217194300744275703.svg)

Use it online at [chordgenerator.xyz](https://chordgenerator.xyz)

## Usage

This crate contains a library and a command line binary.

Run the cli binary locally with

```
cargo run -- -f "x,x,x,2,3,2" -p "x,x,x,2,3,1" -t "D"
```

![](https://github.com/whostolemyhat/chord-gen/blob/main/fixtures/4095730029079104823.svg)

``` 
cargo run -- --help

-t title     title of the diagram as a string
-f frets     frets of the notes as comma-separated string. '-1' to skip a string and '0' to indicate and open string
-p fingers   suggested fingering to play the chord as comma-separated string. 'x' to skip a string and '0' for open string.

```
