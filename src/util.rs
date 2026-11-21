use ratatui_core::buffer::CellWidth as _;
use std::fmt;
use std::sync::Arc;
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

/// A function from a grapheme to its width in cells, which the application sets with
/// [`TextArea::set_grapheme_width`](crate::TextArea::set_grapheme_width).
#[derive(Clone)]
pub struct GraphemeWidth(Arc<dyn Fn(&str) -> usize + Send + Sync>);

impl GraphemeWidth {
    pub fn new(width: impl Fn(&str) -> usize + Send + Sync + 'static) -> Self {
        Self(Arc::new(width))
    }

    /// The width of `grapheme` in cells. A grapheme with a control char is not drawn, so it
    /// takes no cells.
    pub fn of(&self, grapheme: &str) -> usize {
        if grapheme.contains(char::is_control) {
            0
        } else {
            (self.0)(grapheme)
        }
    }
}

impl fmt::Debug for GraphemeWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GraphemeWidth")
    }
}

/// Each char of `text` with its width in cells. Without `width`, a char has the width that
/// ratatui draws it in. With `width`, the first char of a grapheme has the width of the whole
/// grapheme, and the other chars of the grapheme have none.
pub fn char_widths<'t>(
    text: &'t str,
    width: Option<&'t GraphemeWidth>,
) -> impl Iterator<Item = (char, usize)> + 't {
    text.graphemes(true).flat_map(move |grapheme| {
        let cells = width.map(|width| width.of(grapheme));
        grapheme.chars().enumerate().map(move |(i, c)| {
            let cells = match cells {
                Some(cells) if i == 0 => cells,
                Some(_) => 0,
                None => char_width(c),
            };
            (c, cells)
        })
    })
}

/// The width of the mask char, which a masked line draws in place of each char.
pub fn mask_width(mask: char, width: Option<&GraphemeWidth>) -> usize {
    match width {
        Some(width) => width.of(mask.encode_utf8(&mut [0; 4])),
        None => char_width(mask),
    }
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
