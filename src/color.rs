use pancurses::{ColorPair, init_color, init_pair, start_color, Window};

enum Colors {
    WhiteText = 17,
    BrightWhiteText,
    GreenText,
    RedText,

    ClueBg,
    HighlightBg,
    ClueHighlightBg,
    CursorHighlightBg,
    GuessBg,
    PencilmarksBg,
    PencilmarksHighlightBg,
}

pub const COLOR_TABLE: [(i16, (i16, i16, i16)); 11] = [
    (Colors::WhiteText as i16, (800, 800, 800)),
    (Colors::BrightWhiteText as i16, (1000, 1000, 1000)),
    (Colors::GreenText as i16, (700, 900, 700)),
    (Colors::RedText as i16, (1000, 0, 0)),
    (Colors::ClueBg as i16, (300, 300, 300)),
    (Colors::HighlightBg as i16, (200, 200, 200)),
    (Colors::ClueHighlightBg as i16, (400, 400, 400)),
    (Colors::CursorHighlightBg as i16, (300, 300, 800)),
    (Colors::PencilmarksBg as i16, (0, 0, 0)),
    (Colors::PencilmarksHighlightBg as i16, (300, 300, 300)),
    (Colors::GuessBg as i16, (0, 0, 0)),
];

pub enum ColorPairs {
    Clue = 1,
    ClueErr,
    RowHighlight,
    RowHighlightErr,
    Pencilmarks,
    PencilmarksRowHighlight,
    ClueRowHighlight,
    ClueRowHighlightErr,
    CursorHighlight,
    CursorHighlightErr,
    CursorPencilmarksHighlight,
    Guess,
    GuessErr,
}

impl ColorPairs {
    pub fn set_in(self, window: &Window) {
        window.attrset(ColorPair(self as u8));
    }
}

pub const COLOR_PAIRS_TABLE: [(i16, i16, i16); 13] = [
    (ColorPairs::Clue as i16, Colors::WhiteText as i16, Colors::ClueBg as i16),
    (ColorPairs::ClueErr as i16, Colors::RedText as i16, Colors::ClueBg as i16),
    (ColorPairs::RowHighlight as i16, Colors::BrightWhiteText as i16, Colors::HighlightBg as i16),
    (ColorPairs::RowHighlightErr as i16, Colors::RedText as i16, Colors::HighlightBg as i16),
    (ColorPairs::Pencilmarks as i16, Colors::GreenText as i16, Colors::PencilmarksBg as i16),
    (ColorPairs::PencilmarksRowHighlight as i16, Colors::GreenText as i16, Colors::HighlightBg as i16),
    (ColorPairs::ClueRowHighlight as i16, Colors::WhiteText as i16, Colors::ClueHighlightBg as i16),
    (ColorPairs::ClueRowHighlightErr as i16, Colors::RedText as i16, Colors::ClueHighlightBg as i16),
    (ColorPairs::CursorHighlight as i16, Colors::BrightWhiteText as i16, Colors::CursorHighlightBg as i16),
    (ColorPairs::CursorHighlightErr as i16, Colors::RedText as i16, Colors::CursorHighlightBg as i16),
    (ColorPairs::CursorPencilmarksHighlight as i16, Colors::GreenText as i16, Colors::CursorHighlightBg as i16),
    (ColorPairs::Guess as i16, Colors::WhiteText as i16, Colors::GuessBg as i16),
    (ColorPairs::GuessErr as i16, Colors::RedText as i16, Colors::GuessBg as i16),
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
