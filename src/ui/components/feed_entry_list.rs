use tui_realm_stdlib::List;
use tuirealm::{MockComponent, Component, event::{KeyEvent, Key, KeyModifiers}, command::{CmdResult, Cmd, Direction}, Event, Sub, SubClause, Attribute, AttrValue, props::{Alignment, TableBuilder, TextSpan}, State, SubEventClause};
use crate::{libminiflux::{FeedEntry, self}, ui::{ComponentIds, Message, SubscribingComponent, SubClauses}};

pub struct FeedEntryList {
    entries: Vec<FeedEntry>,
    component: List,
}

impl FeedEntryList {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        let mut instance =  Self {
            entries: entries.clone(),
            component: List::default()
                .title(" Unread Entries ", Alignment::Center)
                .rows(
                    TableBuilder::default()
                        .add_row()
                        .add_col(TextSpan::from("Loading..."))
                        .build()
                )
                .rewind(true)
                .scroll(true)
                .highlighted_str(">> ")
        };
        instance.update_entries(&entries);
        return instance
    }

    fn spans_for_entry(entry : &FeedEntry) -> Vec<TextSpan> {
        let title_line =
            if entry.status == libminiflux::ReadStatus::Unread {
                TextSpan::from(entry.title.to_string()).bold()
            } else {
                TextSpan::from(entry.title.to_string()).italic()
            };
        return vec![
            title_line,
            TextSpan::from(" »» "),
            TextSpan::from(entry.feed.title.to_string()).italic()
        ]
    }

    fn update_entries(&mut self, entries: &Vec<FeedEntry>) {
        self.entries = entries.to_vec();
        self.redraw_entries();
    }

    fn redraw_entries(&mut self) {
        let choices = 
            self.entries.iter()
                .map(FeedEntryList::spans_for_entry)
                .collect::<Vec<Vec<TextSpan>>>();
        if choices.is_empty() {
            self.component.attr(
                Attribute::Content, 
                AttrValue::Table(vec![
                    vec![TextSpan::from("No unread feed items. Press r to refresh.")]
                ])
            );
        } else {
            self.component.attr(
                Attribute::Content, 
                AttrValue::Table(choices)
            );
        }
    }

    fn toggle_read_status(&mut self, idx: usize) -> Option<Message> {
        if idx < self.entries.len() {
            {
                let mut entry = &mut self.entries[idx];
                entry.status = entry.status.toggle();
            }
            self.redraw_entries();
            let entry = &self.entries[idx];
            return Some(Message::ChangeEntryReadStatus(entry.id, entry.status.clone()))
        }
        return None
    }
}

impl SubscribingComponent for FeedEntryList {
    fn subscriptions(component_id : ComponentIds) -> Vec<Sub<ComponentIds, KeyEvent>> {
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
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Up,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('j'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Down,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('m'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('r'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Enter,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Tick,
                SubClauses::when_focused(&component_id)
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
            },
            Cmd::Custom("toggle_read_status") => {
                CmdResult::Custom("toggle_read_status")
            }
            Cmd::Submit => {
                CmdResult::Submit(self.component.state())
            },
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
                code: Key::Char('m'),
                ..
            }) => Cmd::Custom("toggle_read_status"),

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
                if idx < self.entries.len() {
                    let entry = &self.entries[idx];
                    return Some(Message::EntrySelected(entry.clone()));
                }
                return None;
            }
            CmdResult::Custom("refresh") => {
                return Some(Message::RefreshRequested)
            }
            CmdResult::Custom("quit") => {
                return Some(Message::AppClose)
            },
            CmdResult::Custom("toggle_read_status") => {
                let idx = self.component.state()
                    .unwrap_one()
                    .unwrap_usize();
                return self.toggle_read_status(idx);
            }
            CmdResult::Submit(state) => {
                let idx = state.unwrap_one().unwrap_usize();
                let _ = self.toggle_read_status(idx);
                return Some(
                    Message::EntrySelected(
                        self.entries[idx].clone()
                    )
                )
            },
            CmdResult::Changed(_) => Some(Message::Tick),
            _ => None
        }
    }
}
