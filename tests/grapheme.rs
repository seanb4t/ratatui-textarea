use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::Widget as _;
use ratatui_textarea::{CursorMove, TextArea};

const FAMILY: &str = "👨\u{200d}👩\u{200d}👧";
const FLAG: &str = "🇺🇸";
const ACCENT: &str = "e\u{301}";
const SKIN_TONE: &str = "👍🏽";

const GRAPHEMES: [&str; 4] = [FAMILY, FLAG, ACCENT, SKIN_TONE];

fn chars(text: &str) -> usize {
    text.chars().count()
}

// Renders the textarea, so the moves that use the screen rows have an area.
fn render(textarea: &TextArea<'_>) {
    let area = Rect::new(0, 0, 40, 10);
    textarea.render(area, &mut Buffer::empty(area));
}

#[test]
fn forward_moves_over_a_whole_grapheme() {
    for grapheme in GRAPHEMES {
        let mut textarea = TextArea::from([format!("a{grapheme}b")]);
        textarea.move_cursor(CursorMove::Forward);
        textarea.move_cursor(CursorMove::Forward);
        assert_eq!(textarea.cursor(), (0, 1 + chars(grapheme)), "{grapheme}");
    }
}

#[test]
fn back_moves_over_a_whole_grapheme() {
    for grapheme in GRAPHEMES {
        let mut textarea = TextArea::from([format!("a{grapheme}b")]);
        textarea.move_cursor(CursorMove::End);
        textarea.move_cursor(CursorMove::Back);
        textarea.move_cursor(CursorMove::Back);
        assert_eq!(textarea.cursor(), (0, 1), "{grapheme}");
    }
}

#[test]
fn delete_char_deletes_a_whole_grapheme() {
    for grapheme in GRAPHEMES {
        let mut textarea = TextArea::from([format!("a{grapheme}")]);
        textarea.move_cursor(CursorMove::End);
        assert!(textarea.delete_char());
        assert_eq!(textarea.lines(), ["a"], "{grapheme}");
        assert_eq!(textarea.cursor(), (0, 1), "{grapheme}");
    }
}

#[test]
fn delete_next_char_deletes_a_whole_grapheme() {
    for grapheme in GRAPHEMES {
        let mut textarea = TextArea::from([format!("{grapheme}b")]);
        assert!(textarea.delete_next_char());
        assert_eq!(textarea.lines(), ["b"], "{grapheme}");
        assert_eq!(textarea.cursor(), (0, 0), "{grapheme}");
    }
}

#[test]
fn undo_brings_back_a_whole_grapheme() {
    for grapheme in GRAPHEMES {
        let mut textarea = TextArea::from([format!("a{grapheme}")]);
        textarea.move_cursor(CursorMove::End);
        textarea.delete_char();
        assert!(textarea.undo());
        assert_eq!(textarea.lines(), [format!("a{grapheme}")], "{grapheme}");
    }
}

#[test]
fn up_stops_at_a_grapheme_boundary() {
    let mut textarea = TextArea::from([format!("a{FAMILY}"), "xxx".to_string()]);
    render(&textarea);
    textarea.move_cursor(CursorMove::Bottom);
    textarea.move_cursor(CursorMove::End);
    textarea.move_cursor(CursorMove::Up);
    let cursor = textarea.cursor();
    let (row, col) = (cursor.0, cursor.1);
    assert_eq!(row, 0);
    assert!([1, 1 + chars(FAMILY)].contains(&col), "column {col}");
}

#[test]
fn down_stops_at_a_grapheme_boundary() {
    let mut textarea = TextArea::from(["xxx".to_string(), format!("a{FAMILY}")]);
    render(&textarea);
    textarea.move_cursor(CursorMove::End);
    textarea.move_cursor(CursorMove::Down);
    let cursor = textarea.cursor();
    let (row, col) = (cursor.0, cursor.1);
    assert_eq!(row, 1);
    assert!([1, 1 + chars(FAMILY)].contains(&col), "column {col}");
}

#[test]
fn jump_stops_at_a_grapheme_boundary() {
    let mut textarea = TextArea::from([format!("a{FAMILY}b")]);
    textarea.move_cursor(CursorMove::Jump(0, 3));
    assert_eq!(textarea.cursor(), (0, 1));
}

#[test]
fn text_inserted_after_a_move_keeps_the_grapheme_whole() {
    let mut textarea = TextArea::from([format!("a{FAMILY}"), "xxx".to_string()]);
    render(&textarea);
    textarea.move_cursor(CursorMove::Bottom);
    textarea.move_cursor(CursorMove::End);
    textarea.move_cursor(CursorMove::Up);
    textarea.insert_char('z');
    assert!(
        textarea.lines()[0].contains(FAMILY),
        "{:?}",
        textarea.lines()
    );
}
