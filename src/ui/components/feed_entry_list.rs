use tui_realm_stdlib::List;
use tuirealm::{MockComponent, Component, event::{KeyEvent, Key, KeyModifiers}, command::{CmdResult, Cmd, Direction}, Event, Sub, SubClause, Attribute, AttrValue, props::{Alignment, Color, TableBuilder, TextSpan}, State, SubEventClause};
use crate::{libminiflux::FeedEntry, ui::{ComponentIds, Message}};

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
    component: List,
}

impl FeedEntryList {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        let mut instance =  Self {
            state: FeedEntryListState::new(entries.clone()),
            component: List::default()
                .title("Unread Entries", Alignment::Center)
                .rows(
                    TableBuilder::default()
                        .add_row()
                        .add_col(TextSpan::from("Loading..."))
                        .build()
                )
                .highlighted_str(">> ")
                .highlighted_color(Color::White)
        };
        instance.update_entries(&entries);
        return instance
    }

    fn update_entries(&mut self, entries: &Vec<FeedEntry>) {
        let choices = 
            entries.iter()
                .map(|e| e.title.to_string())
                .map(|title| vec![TextSpan::from(title)])
                .collect::<Vec<Vec<TextSpan>>>();
        self.component.attr(
            Attribute::Content, 
            AttrValue::Table(choices)
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
            },
            Cmd::Custom("refresh") => {
                CmdResult::Custom("refresh")
            }
            _ => self.component.perform(cmd)
        }
    }
}

impl Component<Message, KeyEvent> for FeedEntryList {
    fn on(&mut self, ev: Event<KeyEvent>) -> Option<Message> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('j'),
                ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                ..
            }) => Cmd::Move(Direction::Down),

            Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                ..
            }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                ..
            }) => Cmd::Move(Direction::Up),

            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                ..
            }) => Cmd::Submit,

            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => Cmd::Custom("quit"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                ..
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
