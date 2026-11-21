use ratatui_core::buffer::{Buffer, CellDiffOption};
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use ratatui_core::widgets::Widget as _;
use ratatui_textarea::{CursorMove, TextArea, WrapMode};
use std::num::NonZeroU16;

const FAMILY: &str = "👨\u{200d}👩\u{200d}👧";

// A terminal that draws a family emoji in 8 cells, and each other grapheme in 1 cell.
fn family_is_8(grapheme: &str) -> usize {
    if grapheme == FAMILY { 8 } else { 1 }
}

fn textarea(text: &str) -> TextArea<'static> {
    let mut textarea = TextArea::from([text]);
    textarea.set_cursor_style(Style::default());
    textarea.set_cursor_line_style(Style::default());
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    textarea
}

fn render(textarea: &TextArea<'_>, area: Rect, buffer: Rect) -> Buffer {
    let mut buf = Buffer::empty(buffer);
    textarea.render(area, &mut buf);
    buf
}

fn symbols(buf: &Buffer, y: u16) -> Vec<&str> {
    (0..buf.area.width).map(|x| buf[(x, y)].symbol()).collect()
}

#[test]
fn cursor_uses_the_width_function() {
    let mut textarea = textarea(&format!("{FAMILY}x"));
    textarea.set_grapheme_width(family_is_8);
    let area = Rect::new(0, 0, 20, 2);
    render(&textarea, area, area);
    textarea.move_cursor(CursorMove::End);
    assert_eq!(textarea.screen_cursor().col, 9);
}

#[test]
fn wrap_uses_the_width_function() {
    let mut textarea = textarea(&format!("{FAMILY}xxx"));
    textarea.set_wrap_mode(WrapMode::Glyph);
    textarea.set_grapheme_width(family_is_8);
    let area = Rect::new(0, 0, 10, 2);
    let buf = render(&textarea, area, area);
    assert_eq!(&symbols(&buf, 0)[8..], ["x", "x"]);
    assert_eq!(symbols(&buf, 1)[0], "x");
    assert_eq!(symbols(&buf, 1)[1], " ");
}

#[test]
fn split_of_a_long_word_uses_the_width_function() {
    let mut textarea = textarea("xxxxxxxxxxxx");
    textarea.set_grapheme_width(|_| 2);
    let area = Rect::new(0, 0, 10, 3);
    let buf = render(&textarea, area, area);
    let drawn = |y| symbols(&buf, y).iter().filter(|s| **s == "x").count();
    assert_eq!([drawn(0), drawn(1), drawn(2)], [5, 5, 2]);
}

#[test]
fn render_forces_the_width_of_each_cell() {
    let mut textarea = textarea(&format!("{FAMILY}x"));
    textarea.set_grapheme_width(family_is_8);
    let area = Rect::new(0, 0, 20, 1);
    let buf = render(&textarea, area, area);
    assert_eq!(buf[(0, 0)].symbol(), FAMILY);
    assert_eq!(
        buf[(0, 0)].diff_option,
        CellDiffOption::ForcedWidth(NonZeroU16::new(8).unwrap())
    );
    assert_eq!(buf[(8, 0)].symbol(), "x");
}

#[test]
fn default_widths_do_not_change() {
    let mut textarea = textarea(&format!("{FAMILY}x"));
    // The cursor span holds one char, so a cursor at the start draws the emoji in parts.
    textarea.move_cursor(CursorMove::End);
    let area = Rect::new(0, 0, 20, 1);
    let buf = render(&textarea, area, area);
    assert_eq!(buf[(0, 0)].symbol(), FAMILY);
    assert_eq!(buf[(0, 0)].diff_option, CellDiffOption::None);
    assert_eq!(buf[(2, 0)].symbol(), "x");
}

#[test]
fn grapheme_wider_than_the_area_draws_no_cell_outside_it() {
    let mut textarea = textarea(&format!("{FAMILY}x"));
    textarea.set_grapheme_width(family_is_8);
    let area = Rect::new(0, 0, 5, 2);
    let buf = render(&textarea, area, Rect::new(0, 0, 10, 2));
    for y in 0..2 {
        assert_eq!(&symbols(&buf, y)[5..], [" "; 5], "row {y}");
    }
}

#[test]
fn removed_function_gives_the_default_widths() {
    let mut textarea = textarea(&format!("{FAMILY}x"));
    textarea.set_grapheme_width(family_is_8);
    textarea.remove_grapheme_width();
    textarea.move_cursor(CursorMove::End);
    let area = Rect::new(0, 0, 20, 1);
    let buf = render(&textarea, area, area);
    assert_eq!(buf[(2, 0)].symbol(), "x");
}
