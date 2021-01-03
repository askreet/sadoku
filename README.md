# Sadoku

_(Pronounced like Sudoku with a thick Boston accent.)_

A tiny command-line Sudoku game, written in Rust, using [pancurses].

What it does:

* Renders an editable game state allowing you to solve a sudoku.
* Works on my Mac.

What it doesn't do:

* Generate puzzles. Has one hard-coded puzzle I transcribed from NY Times the day
  I wrote it.
* Tell you when you won.
* Highlight all errors.

What it should do, but hasn't been tested:

* Work on devices other than my Mac.

## Usage

To run locally, have a Rust development environment and run:

    cargo run --release

Controls are as follows:

Move cursor: `hjkl` (Like vim).
Clear cell: `x`
Insert guess: `1` - `9`
Insert pencil marks: Shift + `1` - `9`.
Exit: `q`

