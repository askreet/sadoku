use pancurses::{ColorPair, init_color, init_pair, start_color, Window};

enum Colors {
    ClueFg = 17,
    ClueBg,
    HighlightFg,
    HighlightBg,
    ClueHighlightFg,
    ClueHighlightBg,
    CursorHighlightFg,
    CursorHighlightBg,
    GuessFg,
    GuessBg,
    ErrorFg,
}

pub const COLOR_TABLE: [(i16, (i16, i16, i16)); 11] = [
    (Colors::ClueFg as i16, (800, 800, 800)),
    (Colors::ClueBg as i16, (300, 300, 300)),
    (Colors::HighlightFg as i16, (1000, 1000, 1000)),
    (Colors::HighlightBg as i16, (200, 200, 200)),
    (Colors::ClueHighlightFg as i16, (1000, 1000, 1000)),
    (Colors::ClueHighlightBg as i16, (400, 400, 400)),
    (Colors::CursorHighlightFg as i16, (1000, 1000, 1000)),
    (Colors::CursorHighlightBg as i16, (300, 300, 800)),
    (Colors::GuessFg as i16, (800, 800, 800)),
    (Colors::GuessBg as i16, (0, 0, 0)),
    (Colors::ErrorFg as i16, (1000, 0, 0)),
];

pub enum ColorPairs {
    Clue = 1,
    ClueErr,
    RowHighlight,
    RowHighlightErr,
    ClueRowHighlight,
    ClueRowHighlightErr,
    CursorHighlight,
    CursorHighlightErr,
    Guess,
    GuessErr,
}

impl ColorPairs {
    pub fn set_in(self, window: &Window) {
        window.attrset(ColorPair(self as u8));
    }
}

pub const COLOR_PAIRS_TABLE: [(i16, i16, i16); 10] = [
    (ColorPairs::Clue as i16, Colors::ClueFg as i16, Colors::ClueBg as i16),
    (ColorPairs::ClueErr as i16, Colors::ErrorFg as i16, Colors::ClueBg as i16),
    (ColorPairs::RowHighlight as i16, Colors::HighlightFg as i16, Colors::HighlightBg as i16),
    (ColorPairs::RowHighlightErr as i16, Colors::ErrorFg as i16, Colors::HighlightBg as i16),
    (ColorPairs::ClueRowHighlight as i16, Colors::ClueHighlightFg as i16, Colors::ClueHighlightBg as i16),
    (ColorPairs::ClueRowHighlightErr as i16, Colors::ErrorFg as i16, Colors::ClueHighlightBg as i16),
    (ColorPairs::CursorHighlight as i16, Colors::CursorHighlightFg as i16, Colors::CursorHighlightBg as i16),
    (ColorPairs::CursorHighlightErr as i16, Colors::ErrorFg as i16, Colors::CursorHighlightBg as i16),
    (ColorPairs::Guess as i16, Colors::GuessFg as i16, Colors::GuessBg as i16),
    (ColorPairs::GuessErr as i16, Colors::ErrorFg as i16, Colors::GuessBg as i16),
];

pub fn setup() {
    start_color();
    for (id, (r, g, b)) in &COLOR_TABLE {
        init_color(*id, *r, *g, *b);
    }
    for (id, fg, bg) in &COLOR_PAIRS_TABLE {
        init_pair(*id, *fg, *bg);
    }
}
