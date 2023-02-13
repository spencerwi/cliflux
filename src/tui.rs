use std::time::Duration;

use tuirealm::{tui::layout::{Layout, Direction, Constraint}, Application, event::KeyEvent, terminal::TerminalBridge, EventListenerCfg, Update, props::{PropPayload, PropValue}};

use crate::{libminiflux::{FeedEntry, self}, components::{loading_text::LoadingText, feed_entry_list::FeedEntryList, read_entry_view::ReadEntryView}};

extern crate tuirealm;


#[derive(Debug, PartialEq)]
pub enum Message {
    AppClose,
    FetchStarted,
    FeedEntriesReceived(Vec<FeedEntry>),
    EntrySelected(FeedEntry),
    RefreshRequested,
    ReadEntryViewClosed,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentIds {
    LoadingText,
    FeedEntryList,
    ReadEntry
}

pub struct Model {
    pub app: Application<ComponentIds, Message, KeyEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub miniflux_client: libminiflux::Client,
}
impl Model { 
    pub fn new(miniflux_client : libminiflux::Client) -> Self {
        return Self {
            app: Self::init_app(),
            quit: false,
            redraw: false,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            miniflux_client
        }
    }
    pub fn view(&mut self) {
        assert!(
            self.terminal.raw_mut().draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ].as_ref())
                    .split(f.size());
                self.app.view(&ComponentIds::LoadingText, f, chunks[0]);
                self.app.view(&ComponentIds::FeedEntryList, f, chunks[0]);
                self.app.view(&ComponentIds::ReadEntry, f, chunks[0]);
            }).is_ok()
        );
    }

    fn init_app() -> Application<ComponentIds, Message, KeyEvent> {
        let mut app: Application<ComponentIds, Message, KeyEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1))
        );
        assert!(
            app.mount(
                ComponentIds::LoadingText, 
                Box::new(LoadingText::new()),
                Vec::default()
            ).is_ok()
        );
        assert!(
            app.mount(
                ComponentIds::FeedEntryList, 
                Box::new(FeedEntryList::new(Vec::default())),
                Vec::default()
            ).is_ok()
        );
        assert!(
            app.mount(
                ComponentIds::ReadEntry, 
                Box::new(ReadEntryView::new(None)),
                Vec::default()
            ).is_ok()
        );

        assert!(app.active(&ComponentIds::LoadingText).is_ok());

        return app;
    }
}

impl Update<Message> for Model {
    fn update(&mut self, msg: Option<Message>) -> Option<Message> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Message::AppClose => {
                    self.quit = true;
                    return None
                }
                Message::RefreshRequested => {
                    // TODO: spawn background thread to fetch messages
                    return None
                }
                Message::FeedEntriesReceived(entries) => {
                    assert!(
                        self.app.attr(
                            &ComponentIds::FeedEntryList, 
                            tuirealm::Attribute::Content, 
                            tuirealm::AttrValue::Payload(
                                PropPayload::One(
                                    PropValue::Str(
                                        serde_json::to_string(&entries).unwrap()
                                    )
                                )
                            )
                        ).is_ok()
                    );
                    return None
                }
                Message::FetchStarted => {
                    return None
                }
                Message::EntrySelected(entry) => {
                    assert!(
                        self.app.attr(
                            &ComponentIds::ReadEntry, 
                            tuirealm::Attribute::Content, 
                            tuirealm::AttrValue::Payload(
                                PropPayload::One(
                                    PropValue::Str(
                                        serde_json::to_string(&entry).unwrap()
                                    )
                                )
                            )
                        ).is_ok()
                    );
                    assert!(self.app.active(&ComponentIds::ReadEntry).is_ok());
                    return None
                },
                Message::ReadEntryViewClosed => {
                    assert!(
                        self.app.attr(
                            &ComponentIds::ReadEntry, 
                            tuirealm::Attribute::Content, 
                            tuirealm::AttrValue::Payload(PropPayload::None)
                        ).is_ok()
                    );
                    assert!(self.app.active(&ComponentIds::FeedEntryList).is_ok());
                    return None
                },
            }
        }
        return None
    }
}
