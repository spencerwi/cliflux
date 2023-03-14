use tuirealm::{tui::text::Span, props::{Style, TextModifiers, TextSpan}};

use crate::libminiflux::{FeedEntry, ReadStatus, self};

pub struct EntryTitle {
    pub text: String,
    pub read_status: ReadStatus
}
impl EntryTitle {
    pub fn for_entry(entry : &FeedEntry) -> Self {
        let text = format!(
            " {}{} ",
            if entry.starred { "🟊 " } else { "" },
            entry.title.clone()
        );
        return Self {
            text,
            read_status: entry.status.clone()
        }
    }
}
impl From<EntryTitle> for Span<'_> {
    fn from(val: EntryTitle) -> Self {
        Span::styled(
            format!(" {} ", val.text.clone()), 
            Style::default().add_modifier(TextModifiers::BOLD)
        )
    }
}
impl From<EntryTitle> for TextSpan {
    fn from(value: EntryTitle) -> Self {
        if value.read_status == libminiflux::ReadStatus::Unread {
            TextSpan::new(value.text.clone()).bold()
        } else {
            TextSpan::new(value.text.clone()).italic()
        }
    }
}

pub fn to_window_title(text : &str) -> Span {
    Span::styled(
        format!(" {} ", text), 
        Style::default().add_modifier(TextModifiers::BOLD)
    )
}
