use tuirealm::{Update, SubClause, Attribute, AttrValue, event::KeyEvent, Sub};

use crate::libminiflux::{FeedEntry, self, ReadStatus};

use self::{model::Model, components::feed_entry_list::FeedListViewType};

pub mod model;
pub mod components;
pub mod utils;

#[derive(Debug, PartialEq, Clone)]
pub enum Message {
    Tick,
    AppClose,
    FeedEntriesReceived(Vec<FeedEntry>),
    EntrySelected(FeedEntry),
    RefreshRequested(FeedListViewType),
    ReadEntryViewClosed,
    ChangeEntryReadStatus(i32, ReadStatus),
    ToggleStarred(i32),
    ShowKeyboardHelp,
    HideKeyboardHelp,
    Batch(Vec<Option<Message>>),
    RequestErrorEncountered(Option<reqwest::StatusCode>, String),
    DismissError,
    SaveEntry(i32),
    MarkAllAsRead(Vec<i32>),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentIds {
    LoadingText,
    FeedEntryList,
    ReadEntry,
    KeyboardHelp,
    ErrorMessage,
}

trait SubscribingComponent {
    fn subscriptions(component_id : ComponentIds) -> Vec<Sub<ComponentIds, KeyEvent>>;
}


// Convenient factories for common subscription "clauses"
pub struct SubClauses {}
impl SubClauses {
    pub fn when_focused(component_id : &ComponentIds) -> SubClause<ComponentIds> {
        SubClause::And(
            Box::new(SubClause::IsMounted(component_id.clone())), 
            Box::new(
                SubClause::HasAttrValue(
                    component_id.clone(),
                    Attribute::Focus,
                    AttrValue::Flag(true)
                )
            )
        )
    }
}

pub struct Ui {
    model: Model
}
impl Ui {
    pub fn new(miniflux_client : libminiflux::Client) -> Self {
        let model = Model::new(miniflux_client);
        return Self {
            model
        }
    }
    pub fn run(&mut self) {
        let _ = self.model.terminal.enter_alternate_screen();
        let _ = self.model.terminal.enable_raw_mode();
        while !self.model.quit {
            // When RefreshRequested events are processed, a new thread fetches updated entries, and
            // throws them into a channel. We should periodically check that channel to see if messages 
            // have finished fetching, and if so, update the model with them.
            match self.model.messages_rx.try_recv() {
                Ok(msg) => {
                    self.model.redraw = true;
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = self.model.update(msg);
                    }
                },
                _ => {}
            }
            match self.model.app.tick(tuirealm::PollStrategy::Once) {
                Err(err) => {
                    panic!("{}", err)
                },
                Ok(messages) if messages.len() > 0 => {
                    self.model.redraw = true;
                    for msg in messages.into_iter() {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = self.model.update(msg)
                        }
                    }
                },
                _ => {}
            }
            if self.model.redraw {
                self.model.view();
                self.model.redraw = false;
            }
        }
        let _ = self.model.terminal.leave_alternate_screen();
        let _ = self.model.terminal.disable_raw_mode();
    }
}
