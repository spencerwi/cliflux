use tuirealm::{tui::text::Span, props::{Style, TextModifiers}};

pub struct StringPadding {}
impl StringPadding {
    pub fn spaces_around(s : String, count : usize) -> String {
        return format!(
            "{}{}{}",
            " ".repeat(count),
            s,
            " ".repeat(count) 
        );
    }
}

pub fn to_title(text : &str) -> Span {
    Span::styled(
        format!(" {} ", text), 
        Style::default().add_modifier(TextModifiers::BOLD)
    )
}
