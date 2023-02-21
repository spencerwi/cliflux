use tui_realm_stdlib::Textarea;
use tuirealm::{MockComponent, event::{KeyEvent, Key, KeyModifiers}, Component, State, StateValue, tui::layout::Alignment, command::{Cmd, CmdResult, Direction}, Event, Sub, SubClause, props::{TextSpan, PropPayload, PropValue}, Attribute, AttrValue, SubEventClause};

use crate::{libminiflux::{FeedEntry, ReadStatus}, ui::{ComponentIds, Message, SubClauses, utils::StringPadding}};
use stringreader::StringReader;

pub struct ReadEntryView {
    entry: Option<FeedEntry>,
    component: Textarea,
}

impl ReadEntryView {
    pub fn new(entry: Option<FeedEntry>) -> Self {
        let (title, text_rows) = match &entry {
            Some(e) => (
                StringPadding::spaces_around(e.title.clone(), 1),
                ReadEntryView::format_entry_text(e)
            ),
            None => (
                "".to_string(),
                vec![]
            )
        };
        let component = Textarea::default()
            .title(title, Alignment::Center)
            .text_rows(&text_rows)
        ;
        return Self {
            entry,
            component,
        }
    }

    pub fn subscriptions(component_id : ComponentIds) -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('q'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('?'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('k'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Up,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('j'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Down,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),


            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('b'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('u'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('o'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Tick,
                SubClauses::when_focused(&component_id)
            )
        ]
    }

    fn format_entry_text(entry: &FeedEntry) -> Vec<TextSpan> {
        html2text::from_read(
            StringReader::new(&entry.content), 
            120
        ).lines()
            .into_iter()
            .map(TextSpan::from)
            .collect()
    }
}

impl MockComponent for ReadEntryView {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        if let Some(_) = &self.entry {
            self.component.view(frame, area)
        }
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        match attr {
            tuirealm::Attribute::Content => {
                let unwrapped = value.clone().unwrap_string();
                let new_entry = serde_json::from_str::<FeedEntry>(&unwrapped).unwrap();
                self.entry = Some(new_entry.clone());
                self.component.attr(
                    Attribute::Text,
                    AttrValue::Payload(
                        PropPayload::Vec(
                            ReadEntryView::format_entry_text(&new_entry)
                                .into_iter()
                                .map(PropValue::TextSpan)
                                .collect()
                        )
                    )
                );
                self.component.attr(
                    Attribute::Title,
                    AttrValue::Title((StringPadding::spaces_around(new_entry.title, 1), Alignment::Center))
                )
            }
            _ => {}
        }
        self.component.attr(attr, value)
    }

    fn state(&self) -> tuirealm::State {
        match &self.entry {
            Some(e) => State::One(
                StateValue::I32(e.id)
            ),
            None => State::None
        }
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        return match cmd {
            Cmd::Custom("quit") => CmdResult::Custom("quit"),
            Cmd::Custom("show_keyboard_help") => CmdResult::Custom("show_keyboard_help"),

            Cmd::Custom("back") => {
                self.entry = None;
                CmdResult::Custom("back")
            }

            Cmd::Custom("mark_as_unread") => {
                CmdResult::Custom("mark_as_unread")
            }

            Cmd::Custom("open_in_browser") => {
                if let Some(e) = &self.entry {
                    let _ = open::that(&e.url);
                }
                CmdResult::Custom("open_in_browser")
            }

            Cmd::Scroll(direction) => {
                self.component.perform(Cmd::Scroll(direction));
                CmdResult::Custom("scrolled")
            }
            _ => CmdResult::None
        }
    }
}

impl Component<Message, KeyEvent> for ReadEntryView {
    fn on(&mut self, ev: tuirealm::Event<KeyEvent>) -> Option<Message> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('?'),
                ..
            }) => Cmd::Custom("show_keyboard_help"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                ..
            }) => Cmd::Custom("back"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('u'),
                ..
            }) => Cmd::Custom("mark_as_unread"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('o'),
                ..
            }) => Cmd::Custom("open_in_browser"),

            Event::Keyboard(KeyEvent { 
                code: Key::Char('k'),
                ..
            }) => Cmd::Scroll(Direction::Up),
            Event::Keyboard(KeyEvent { 
                code: Key::Up,
                ..
            }) => Cmd::Scroll(Direction::Up),

            Event::Keyboard(KeyEvent { 
                code: Key::Char('j'),
                ..
            }) => Cmd::Scroll(Direction::Down),
            Event::Keyboard(KeyEvent { 
                code: Key::Down,
                ..
            }) => Cmd::Scroll(Direction::Down),


            _ => Cmd::None
        };

        return match self.perform(cmd) {
            CmdResult::Custom("quit") => Some(Message::AppClose),
            CmdResult::Custom("show_keyboard_help") => Some(Message::ShowKeyboardHelp),

            CmdResult::Custom("back") => Some(Message::ReadEntryViewClosed),

            CmdResult::Custom("mark_as_unread") => {
                match &mut self.entry {
                    Some(e) => {
                        e.status = ReadStatus::Unread;
                        return Some(Message::ChangeEntryReadStatus(e.id, ReadStatus::Unread))
                    }
                    None => None
                }
            }

            CmdResult::Custom("scrolled") => Some(Message::Tick),

            CmdResult::Changed(_) => Some(Message::Tick),

            _ => None
        }
    }
}
