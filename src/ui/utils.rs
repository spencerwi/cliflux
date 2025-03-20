use std::str::FromStr;

use tuirealm::{tui::{style::ParseColorError, text::Span}, props::{Color, Style, TextModifiers, TextSpan}};

use crate::{config::ThemeConfig, libminiflux::{FeedEntry, ReadStatus}};

pub struct EntryTitle {
    pub text: String,
    pub read_status: ReadStatus,
	pub color: Color,
}
impl EntryTitle {
    pub fn for_entry(
		entry : &FeedEntry, 
		theme_config : &ThemeConfig 
	) -> Self {
        let text = format!(
            " {}{} ",
            if entry.starred { "🟊 " } else { "" },
            entry.title.clone()
        );
		let color = EntryTitle::resolve_color(entry, theme_config)
			.unwrap_or_else(|err| {
				eprintln!("Error parsing color; using defaults: {}", err);
				let default_config = &ThemeConfig::default();
				EntryTitle::resolve_color(entry, default_config).unwrap()
			});
        return Self {
            text,
            read_status: entry.status.clone(),
			color
        }
    }

	fn resolve_color(entry : &FeedEntry, theme_config : &ThemeConfig) -> Result<Color, ParseColorError> {
		let color_str = 
			match entry.status {
				ReadStatus::Read => &theme_config.read_color,
				ReadStatus::Unread => &theme_config.unread_color
			};
		Color::from_str(color_str)
	}

}
impl From<EntryTitle> for Span<'_> {
    fn from(val: EntryTitle) -> Self {
        Span::styled(
            format!(" {} ", val.text.clone()), 
            Style::default()
				.fg(val.color)
				.add_modifier(TextModifiers::BOLD)
        )
    }
}
impl From<EntryTitle> for TextSpan {
    fn from(value: EntryTitle) -> Self {
        if value.read_status == ReadStatus::Unread {
            TextSpan::new(value.text.clone())
				.fg(value.color)
				.bold()
        } else {
            TextSpan::new(value.text.clone())
				.fg(value.color)
				.italic()
        }
    }
}

pub fn to_window_title(text : &str) -> Span {
    Span::styled(
        format!(" {} ", text), 
        Style::default().add_modifier(TextModifiers::BOLD)
    )
}
