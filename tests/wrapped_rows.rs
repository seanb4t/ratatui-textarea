use ratatui_textarea::{TextArea, WrapMode};

fn wrapped(lines: &[&str]) -> TextArea<'static> {
    let mut textarea = TextArea::from(lines.iter().copied());
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    textarea
}

#[test]
fn empty_text_takes_one_row() {
    assert_eq!(wrapped(&[]).wrapped_row_count(10), 1);
}

#[test]
fn short_line_takes_one_row() {
    assert_eq!(wrapped(&["hello"]).wrapped_row_count(10), 1);
}

#[test]
fn line_wider_than_the_width_takes_a_row_for_each_part() {
    assert_eq!(wrapped(&["one two three"]).wrapped_row_count(5), 3);
    assert_eq!(wrapped(&["xxxxxxxxxxxx"]).wrapped_row_count(5), 3);
}

#[test]
fn each_line_adds_its_rows() {
    assert_eq!(wrapped(&["one two", "", "three"]).wrapped_row_count(5), 4);
}

#[test]
fn width_of_zero_gives_the_count_of_the_lines() {
    assert_eq!(wrapped(&["one two", "three"]).wrapped_row_count(0), 2);
}

#[test]
fn no_wrap_gives_the_count_of_the_lines() {
    let textarea = TextArea::from(["one two three", "four"]);
    assert_eq!(textarea.wrapped_row_count(5), 2);
}
