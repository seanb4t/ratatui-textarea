use ratatui_core::buffer::CellWidth as _;
use unicode_segmentation::UnicodeSegmentation as _;

pub fn spaces(size: u8) -> &'static str {
    const SPACES: &str = "                                                                                                                                                                                                                                                                ";
    &SPACES[..size as usize]
}

/// The number of cells ratatui draws `c` in. Unlike `UnicodeWidthChar::width`, this counts the
/// halfwidth katakana sound marks `ﾞ` and `ﾟ` as one cell each. Control chars are not drawn, so
/// they take no cells.
pub fn char_width(c: char) -> usize {
    if c.is_control() {
        0
    } else {
        c.encode_utf8(&mut [0; 4]).cell_width().into()
    }
}

/// The char offset of the grapheme boundary of `line` at `col` or before it. A `col` past the
/// end of the line gives the end of the line.
pub fn grapheme_floor(line: &str, col: usize) -> usize {
    let mut start = 0;
    for grapheme in line.graphemes(true) {
        let end = start + grapheme.chars().count();
        if col < end {
            return start;
        }
        start = end;
    }
    start
}

/// The char offset of the grapheme boundary of `line` at `col` or after it. A `col` past the
/// end of the line gives the end of the line.
pub fn grapheme_ceil(line: &str, col: usize) -> usize {
    let mut end = 0;
    for grapheme in line.graphemes(true) {
        if col <= end {
            return end;
        }
        end += grapheme.chars().count();
    }
    end
}

pub fn num_digits(i: usize) -> u8 {
    f64::log10(i as f64) as u8 + 1
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
    pub offset: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize, offset: usize) -> Self {
        Self { row, col, offset }
    }
}
