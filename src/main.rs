use std::cmp::{max, min};

use pancurses::{curs_set, endwin, initscr, noecho, Window};
use pancurses::Input::Character;
use serde;

use color::ColorPairs;
use puzzle::{GamePos, Puzzle};

use crate::puzzle::{Cell, CellState};

mod color;
mod puzzle;
mod download;

fn boolchar(b: bool, t: char, f: char) -> char {
    if b {
        t
    } else {
        f
    }
}

struct PlayerCursor {
    pos: GamePos,
}

impl PlayerCursor {
    fn new() -> Self {
        Self::at(0, 0)
    }

    fn at(row: i32, col: i32) -> Self {
        PlayerCursor { pos: GamePos { row, col } }
    }

    fn right(&mut self) { self.pos.col = min(self.pos.col + 1, 8); }

    fn left(&mut self) { self.pos.col = max(self.pos.col - 1, 0); }

    fn up(&mut self) { self.pos.row = max(self.pos.row - 1, 0); }

    fn down(&mut self) { self.pos.row = min(self.pos.row + 1, 8) }
}

fn main() {
    let mut puzzle = Puzzle::from([
        // NYTimes Medium Jan 2, 2021
        0, 3, 0, 0, 1, 0, 0, 5, 4,
        0, 0, 0, 7, 8, 0, 0, 0, 3,
        7, 0, 2, 0, 0, 0, 0, 6, 0,
        4, 1, 0, 0, 5, 0, 0, 8, 0,
        0, 0, 3, 0, 0, 2, 9, 0, 0,
        0, 0, 0, 0, 0, 3, 0, 4, 6,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 5, 0, 4, 0, 0, 0, 0, 0,
        9, 0, 0, 0, 0, 0, 0, 3, 0,
    ]);
    let window = initscr();
    let mut cursor = PlayerCursor::new();
    curs_set(0);
    noecho();
    color::setup();

    draw_grid(&window);

    for cell in puzzle.iter_cells() {
        draw_cell(&cell, &window, &cursor);
    }

    while let Some(key) = window.getch() {
        match key {
            // TODO clean up window correctly?
            Character('q') => break,

            Character('h') => cursor.left(),
            Character('j') => cursor.down(),
            Character('k') => cursor.up(),
            Character('l') => cursor.right(),

            Character('1') => puzzle.set_guess(&cursor.pos, 1),
            Character('2') => puzzle.set_guess(&cursor.pos, 2),
            Character('3') => puzzle.set_guess(&cursor.pos, 3),
            Character('4') => puzzle.set_guess(&cursor.pos, 4),
            Character('5') => puzzle.set_guess(&cursor.pos, 5),
            Character('6') => puzzle.set_guess(&cursor.pos, 6),
            Character('7') => puzzle.set_guess(&cursor.pos, 7),
            Character('8') => puzzle.set_guess(&cursor.pos, 8),
            Character('9') => puzzle.set_guess(&cursor.pos, 9),

            Character('!') => puzzle.toggle_candidate(&cursor.pos, 1),
            Character('@') => puzzle.toggle_candidate(&cursor.pos, 2),
            Character('#') => puzzle.toggle_candidate(&cursor.pos, 3),
            Character('$') => puzzle.toggle_candidate(&cursor.pos, 4),
            Character('%') => puzzle.toggle_candidate(&cursor.pos, 5),
            Character('^') => puzzle.toggle_candidate(&cursor.pos, 6),
            Character('&') => puzzle.toggle_candidate(&cursor.pos, 7),
            Character('*') => puzzle.toggle_candidate(&cursor.pos, 8),
            Character('(') => puzzle.toggle_candidate(&cursor.pos, 9),

            Character('d') => puzzle = download::fetch("nytimes-medium").unwrap(),

            Character('x') => puzzle.clear(&cursor.pos),
            _ => {}
        }

        for cell in puzzle.iter_cells() {
            draw_cell(&cell, &window, &cursor);
        }
    }

    endwin();
}

fn draw_grid(window: &Window) {
    window.mv(0, 0);
    let heading = "+-------------+-------------+-------------+\n";
    let middle_ = "|             |             |             |\n";
    for _ in 1..=3 {
        window.addstr(heading);
        for _ in 1..=9 {
            window.addstr(middle_);
        }
    }
    window.addstr(heading);
}

fn draw_cell(cell: &Cell, window: &Window, cursor: &PlayerCursor) {
    let color = match (cell.state, cursor.pos == cell.pos, cursor.pos.aligned_with(cell.pos), cell.error) {
        (_, true, _, false) => ColorPairs::CursorHighlight,
        (_, true, _, true) => ColorPairs::CursorHighlightErr,
        (CellState::Clue(_), _, true, false) => ColorPairs::ClueRowHighlight,
        (CellState::Clue(_), _, true, true) => ColorPairs::ClueRowHighlightErr,
        (CellState::Clue(_), _, false, false) => ColorPairs::Clue,
        (CellState::Clue(_), _, false, true) => ColorPairs::ClueErr,
        (_, _, true, false) => ColorPairs::RowHighlight,
        (_, _, true, true) => ColorPairs::RowHighlightErr,
        (_, _, false, false) => ColorPairs::Guess,
        (_, _, false, true) => ColorPairs::GuessErr,
    };

    color.set_in(window);

    match cell.state {
        CellState::Clue(value) => draw_clue(cell, window, value),
        CellState::Guess(value) => draw_guess(cell, window, value),
        CellState::Pencilmarks(values) => draw_pencilmarks(cell, window, values),
    }
}

fn win_coords(cell: &Cell) -> (i32, i32) {
    let y = 1 + (cell.pos.row * 3) + (cell.pos.row / 3);
    let x = 2 + (cell.pos.col * 4) + ((cell.pos.col / 3) * 2);

    (y, x)
}

#[test]
fn test_cell_loc() {
    let tests = [
        (0, 1, 2), // Top-left-most cell.
        (20, 7, 10), // Bottom-right cell of top-left block.
        (80, 27, 38), // Bottom-right-most cell.
    ];

    for (idx, y, x) in tests.iter() {
        let cell = Cell::from_index(*idx, CellState::Guess(0), false);
        assert_eq!((*y, *x), win_coords(&cell));
    }
}


fn draw_clue(cell: &Cell, window: &Window, value: u8) {
    let (y, x) = win_coords(cell);

    // Write the number to the center of the shaded area.
    window.mvaddstr(y, x, "   ");
    window.mvaddstr(y + 1, x, format!(" {} ", value));
    window.mvaddstr(y + 2, x, "   ");
}

fn draw_guess(cell: &Cell, window: &Window, value: u8) {
    let (y, x) = win_coords(cell);

    // Write the number to the center of the shaded area.
    window.mvaddstr(y, x, "   ");
    window.mvaddstr(y + 1, x, format!(" {} ", value));
    window.mvaddstr(y + 2, x, "   ");
}

fn draw_pencilmarks(cell: &Cell, window: &Window, values: [bool; 9]) {
    let (y, x) = win_coords(cell);

    // Write the number to the center of the shaded area.
    // TODO: Ugh.
    window.mvaddstr(y, x, format!("{}{}{}",
                                  crate::boolchar(values[0], '1', ' '),
                                  crate::boolchar(values[1], '2', ' '),
                                  crate::boolchar(values[2], '3', ' ')));
    window.mvaddstr(y + 1, x, format!("{}{}{}",
                                      crate::boolchar(values[3], '4', ' '),
                                      crate::boolchar(values[4], '5', ' '),
                                      crate::boolchar(values[5], '6', ' ')));
    window.mvaddstr(y + 2, x, format!("{}{}{}",
                                      crate::boolchar(values[6], '7', ' '),
                                      crate::boolchar(values[7], '8', ' '),
                                      crate::boolchar(values[8], '9', ' ')));
}
