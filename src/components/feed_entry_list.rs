use tui_realm_stdlib::Select;
use tuirealm::{MockComponent, Component, event::{KeyEvent, Key, KeyModifiers}, command::{CmdResult, Cmd, Direction}, Event, Sub, SubClause, Attribute, AttrValue, props::{PropPayload, PropValue}, State, SubEventClause};
use crate::{libminiflux::FeedEntry, tui::ComponentIds};

use super::super::tui::Message;

struct FeedEntryListState {
    entries: Vec<FeedEntry>,
}

impl Default for FeedEntryListState {
    fn default() -> Self {
        return Self { 
            entries: vec![],
        };
    }
}

impl FeedEntryListState {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        return Self { 
            entries,
        }
    }
}

pub struct FeedEntryList {
    state: FeedEntryListState,
    component: Select,
}

impl Default for FeedEntryList {
    fn default() -> Self {
        return Self {
            state: FeedEntryListState::default(),
            component: Select::default()
                .title("Unread Entries", tuirealm::props::Alignment::Center),
        }
    }
}

impl FeedEntryList {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        let mut instance =  Self {
            state: FeedEntryListState::new(entries.clone()),
            component: Select::default()
        };
        instance.update_entries(&entries);
        return instance
    }

    fn update_entries(&mut self, entries: &Vec<FeedEntry>) {
        let choices = 
            entries.iter()
                .map(|e| e.title.to_string())
                .map(|title| PropValue::Str(title))
                .collect::<Vec<PropValue>>();
        self.component.attr(
            Attribute::Content, 
            AttrValue::Payload(PropPayload::Vec(choices))
        );
    }

    pub fn subscriptions() -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('q'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('k'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Up,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('j'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Down,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('r'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Enter,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Tick,
                SubClause::Always
            )
        ]
    }
}

impl MockComponent for FeedEntryList {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        match attr {
            Attribute::Content => {
                let unwrapped = value.unwrap_payload().unwrap_vec();
                let updated_entries = unwrapped.iter()
                    .map(|attr_value| attr_value.clone().unwrap_str())
                    .map(|json| serde_json::from_str::<FeedEntry>(&json).unwrap())
                    .collect::<Vec<FeedEntry>>();
                self.update_entries(&updated_entries)
            },
            _ => self.component.attr(attr, value)
        }
    }

    fn state(&self) -> tuirealm::State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            Cmd::Custom("quit") => {
                CmdResult::Custom("quit")
            }
            _ => self.component.perform(cmd)
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
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: _
            }) => Cmd::Move(Direction::Down),

            Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                modifiers: _
            }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: _
            }) => Cmd::Move(Direction::Up),

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
            CmdResult::Submit(State::One(selected_index)) => {
                let idx = selected_index.unwrap_usize();
                return self.state.entries.get(idx)
                    .map(|entry| Message::EntrySelected(entry.clone()))
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
