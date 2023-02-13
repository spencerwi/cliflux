use std::collections::HashMap;

use tuirealm::{Props, MockComponent, Component, event::{KeyEvent, Key}, command::{CmdResult, Cmd, Direction}, StateValue, Event, tui::widgets::ListItem};
use crate::libminiflux::FeedEntry;

use super::super::tui::Message;

struct FeedEntryListState {
    entries: Vec<FeedEntry>,
    selected_index: usize
}

impl Default for FeedEntryListState {
    fn default() -> Self {
        return Self { 
            entries: vec![],
            selected_index: 0
        };
    }
}

impl FeedEntryListState {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        return Self { 
            entries,
            selected_index: 0
        }
    }
}

pub struct FeedEntryList {
    props: Props,
    state: FeedEntryListState
}

impl Default for FeedEntryList {
    fn default() -> Self {
        return Self {
            props: Props::default(),
            state: FeedEntryListState::default()
        }
    }
}

impl FeedEntryList {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        Self {
            props: Props::default(),
            state: FeedEntryListState::new(entries)
        }
    }
}

impl MockComponent for FeedEntryList {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        frame.render_widget(
            tuirealm::tui::widgets::List::new(
                self.state.entries.iter().map(|entry| {
                    ListItem::new(entry.title.to_string())
                }).collect::<Vec<_>>()
            ), 
            area
        )
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        return self.props.get(attr);
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> tuirealm::State {
        return tuirealm::State::Map(
            HashMap::from([
                ("entry_list".to_string(), StateValue::String(serde_json::to_string(&self.state.entries).expect("Error serializing FeedEntryList state"))),
                ("selected_index".to_string(), StateValue::Usize(self.state.selected_index))
            ])
        );
    }

    fn perform(&mut self, cmd: Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            Cmd::Scroll(Direction::Down) => {
                if (self.state.selected_index + 1) < self.state.entries.len().try_into().unwrap() {
                    self.state.selected_index = self.state.selected_index + 1;
                    return CmdResult::Changed(self.state())
                }
                return CmdResult::None
            }
            Cmd::Scroll(Direction::Up) => {
                if (self.state.selected_index) > 0 {
                    self.state.selected_index = self.state.selected_index - 1;
                    return CmdResult::Changed(self.state())
                }
                return CmdResult::None
            }
            Cmd::Submit => {
                CmdResult::Submit(self.state())
            }
            Cmd::Custom("quit") => {
                CmdResult::Custom("quit")
            }
            _ => CmdResult::None
        }
    }
}

impl Component<Message, KeyEvent> for FeedEntryList {
    fn on(&mut self, ev: Event<KeyEvent>) -> Option<Message> {
        // TODO: how do I catch events raised by the List widget?
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('j'),
                modifiers: _
            }) => Cmd::Scroll(Direction::Down),

            Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                modifiers: _
            }) => Cmd::Scroll(Direction::Up),

            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: _
            }) => Cmd::Submit,

            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: _
            }) => Cmd::Custom("quit"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: _
            }) => Cmd::Custom("refresh"),

            _ => Cmd::None
        };

        match self.perform(cmd) {
            CmdResult::Submit(new_state) => {
                return new_state.unwrap_map()
                    .get("selected_index")
                    .map(|state_value| state_value.to_owned().unwrap_usize())
                    .and_then(|idx| self.state.entries.get(idx))
                    .map(|entry| Message::EntrySelected(entry.clone()));
            }
            CmdResult::Custom("refresh") => {
                return Some(Message::RefreshRequested)
            }
            CmdResult::Custom("quit") => {
                return Some(Message::AppClose)
            }
            _ => None
        }
    }
}
