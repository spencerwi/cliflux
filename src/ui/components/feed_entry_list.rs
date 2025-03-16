use std::vec;

use tui_realm_stdlib::List;
use tuirealm::{MockComponent, Component, event::{KeyEvent, Key, KeyModifiers}, command::{CmdResult, Cmd, Direction}, Event, Sub, SubClause, Attribute, AttrValue, props::{Alignment, TableBuilder, TextSpan}, State, SubEventClause};
use crate::{config::ThemeConfig, libminiflux::{FeedEntry, ReadStatus}, ui::{ComponentIds, Message, SubscribingComponent, SubClauses, utils::EntryTitle}};

#[derive(Copy, Debug, PartialEq, Clone)]
pub enum FeedListViewType {
    UnreadEntries,
    StarredEntries,
}
impl FeedListViewType {
    pub fn title(&self) -> String {
        match self {
            FeedListViewType::UnreadEntries => " Unread Entries ".to_string(),
            FeedListViewType::StarredEntries => " Starred Entries ".to_string()
        }
    }

    pub fn cycle(&self) -> FeedListViewType {
        match self {
            FeedListViewType::UnreadEntries => FeedListViewType::StarredEntries,
            FeedListViewType::StarredEntries => FeedListViewType::UnreadEntries
        }
    }
}

pub struct FeedEntryList {
    entries: Vec<FeedEntry>,
    component: List,
    view_type : FeedListViewType,
	theme_config : ThemeConfig
}

impl FeedEntryList {
    pub fn new(entries: Vec<FeedEntry>, view_type : FeedListViewType, theme_config : ThemeConfig) -> Self {
        let mut instance =  Self {
            view_type,
            entries: entries.clone(),
			theme_config,
            component: List::default()
                .title(view_type.title(), Alignment::Center)
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
        instance.update_entries(&entries, view_type);
        return instance
    }

    fn spans_for_entry(&self, entry : &FeedEntry) -> Vec<TextSpan> {
        let title_line = TextSpan::from(EntryTitle::for_entry(entry, &self.theme_config));
        return vec![
            title_line,
            TextSpan::from(" »» "),
            TextSpan::from(entry.feed.title.to_string()).italic()
        ]
    }

    fn update_entries(&mut self, entries: &Vec<FeedEntry>, view_type : FeedListViewType) {
        self.view_type = view_type;
        self.entries = entries.to_vec();
        self.redraw();
    }

    fn redraw(&mut self) {
        let contents = 
            if self.entries.is_empty() {
                FeedEntryList::zero_state_contents()
            } else {
                self.entries.iter()
                    .map(|entry| self.spans_for_entry(entry))
                    .collect::<Vec<Vec<TextSpan>>>()
            };

        self.component.attr(
            Attribute::Content, 
            AttrValue::Table(contents)
        );
        self.component.attr(
            Attribute::Title,
            AttrValue::Title((self.view_type.title(), Alignment::Center))
        );
    }

    fn toggle_read_status(&mut self, idx: usize) -> Option<Message> {
        if idx < self.entries.len() {
            {
                let entry = &mut self.entries[idx];
                entry.status = entry.status.toggle();
            }
            self.redraw();
            let entry = &self.entries[idx];
            return Some(Message::ChangeEntryReadStatus(entry.id, entry.status.clone()))
        }
        return None
    }

    fn toggle_starred(&mut self, idx: usize) -> Option<Message> {
        if idx < self.entries.len() {
            {
                let entry = &mut self.entries[idx];
                entry.starred = !entry.starred;
            }
            self.redraw();
            let entry = &self.entries[idx];
            return Some(Message::ToggleStarred(entry.id))
        }
        return None
    }

	fn save_entry(&mut self, idx: usize) -> Option<Message> {
		if idx < self.entries.len() {
			let entry = &self.entries[idx];
			return Some(Message::SaveEntry(entry.id));
		}
		return None
	}

    fn mark_as_read(&mut self, idx: usize) -> Option<Message> {
        if idx < self.entries.len() {
            {
                let entry = &mut self.entries[idx];
                if entry.status == ReadStatus::Read {
                    return None;
                }
                entry.status = ReadStatus::Read;
            }
            self.redraw();
            let entry = &self.entries[idx];
            return Some(Message::ChangeEntryReadStatus(entry.id, entry.status.clone()))
        }
        return None
    }

	fn mark_all_as_read(&mut self) -> Option<Message> {
		if self.entries.is_empty() {
			return None
		}
		let mut entry_ids = vec![];
		for entry in &mut self.entries {
			entry.status = ReadStatus::Read;
			entry_ids.push(entry.id);
		}
		self.redraw();
		return Some(Message::MarkAllAsRead(entry_ids))
	}

    fn zero_state_contents() -> Vec<Vec<TextSpan>> {
        vec![
            vec![TextSpan::from("No unread feed items. Press r to refresh.")]
        ]
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
                    code: Key::Char('?'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
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
					code: Key::Char('a'),
					modifiers: KeyModifiers::NONE
				}),
				SubClauses::when_focused(&component_id)
			),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('s'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('e'),
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
                    code: Key::Char('v'),
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
                self.update_entries(&updated_entries, self.view_type)
            },
            _ => self.component.attr(attr, value)
        }
    }

    fn state(&self) -> tuirealm::State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            Cmd::Custom("quit") => CmdResult::Custom("quit"),

            Cmd::Custom("show_keyboard_help") => CmdResult::Custom("show_keyboard_help"),

            Cmd::Custom("refresh") => CmdResult::Custom("refresh"),
            Cmd::Custom("force_refresh") => CmdResult::Custom("force_refresh"),

            Cmd::Custom("change_view") => {
                self.view_type = self.view_type.cycle();
                CmdResult::Custom("refresh")
            }

            Cmd::Custom("toggle_read_status") => CmdResult::Custom("toggle_read_status"),

            Cmd::Custom("toggle_starred") => CmdResult::Custom("toggle_starred"),

            Cmd::Custom("save_entry") => CmdResult::Custom("save_entry"),

			Cmd::Custom("mark_all_as_read") => CmdResult::Custom("mark_all_as_read"),

            Cmd::Submit => CmdResult::Submit(self.component.state()),

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
                code: Key::Char('a'),
				..
            }) => Cmd::Custom("mark_all_as_read"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
				..
            }) => Cmd::Custom("toggle_starred"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
				..
            }) => Cmd::Custom("save_entry"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => Cmd::Custom("quit"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::NONE
            }) => Cmd::Custom("refresh"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('R'),
                modifiers: KeyModifiers::SHIFT
            }) => {
                Cmd::Custom("force_refresh")
            },

            Event::Keyboard(KeyEvent {
                code: Key::Char('v'),
                ..
            }) => Cmd::Custom("change_view"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('?'),
                ..
            }) => Cmd::Custom("show_keyboard_help"),

            _ => Cmd::None
        };

        return match self.perform(cmd) {
            CmdResult::Submit(State::One(selected_index)) => {
                let idx = selected_index.unwrap_usize();
                if idx < self.entries.len() {
                    let change_state_message = self.mark_as_read(idx);
                    let entry = &self.entries[idx];
                    return Some(
                        Message::Batch(vec![
                            change_state_message,
                            Some(Message::EntrySelected(entry.clone()))
                        ])
                    );
                }
                None
            }

            CmdResult::Custom("quit") => return Some(Message::AppClose),
            CmdResult::Custom("show_keyboard_help") => Some(Message::ShowKeyboardHelp),

            CmdResult::Custom("refresh") => Some(Message::RefreshRequested(self.view_type)),
            CmdResult::Custom("force_refresh") => Some(Message::ForceRefreshRequested(self.view_type)),

            CmdResult::Custom("toggle_read_status") => {
                let idx = self.component.state()
                    .unwrap_one()
                    .unwrap_usize();
                self.toggle_read_status(idx)
            }

            CmdResult::Custom("toggle_starred") => {
                let idx = self.component.state()
                    .unwrap_one()
                    .unwrap_usize();
                self.toggle_starred(idx)
            }

            CmdResult::Custom("save_entry") => {
                let idx = self.component.state()
                    .unwrap_one()
                    .unwrap_usize();
                self.save_entry(idx)
            }

			CmdResult::Custom("mark_all_as_read") => {
				self.mark_all_as_read()
			}

            CmdResult::Changed(_) => Some(Message::Tick),

            _ => None
        }
    }
}
