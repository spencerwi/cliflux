use std::time::Duration;

use tokio::sync::mpsc;
use tuirealm::{tui::layout::{Layout, Direction, Constraint}, Application, event::KeyEvent, terminal::TerminalBridge, EventListenerCfg, Update, props::{PropPayload, PropValue}};

use crate::{libminiflux::{FeedEntry, self}, ui::components::{loading_text::LoadingText, feed_entry_list::FeedEntryList, read_entry_view::ReadEntryView}};

use super::{ComponentIds, Message};

extern crate tuirealm;

pub struct Model {
    pub app: Application<ComponentIds, Message, KeyEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub miniflux_client: libminiflux::Client,
    pub entries_rx : tokio::sync::mpsc::Receiver<Option<Vec<FeedEntry>>>,
    entries_tx : tokio::sync::mpsc::Sender<Option<Vec<FeedEntry>>>,
    current_view : ComponentIds
}
impl Model { 
    pub fn new(miniflux_client : libminiflux::Client) -> Self {
        let (entries_tx, entries_rx) = mpsc::channel::<Option<Vec<FeedEntry>>>(32);

        let mut instance = Self {
            app: Self::init_app(),
            quit: false,
            redraw: false,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            miniflux_client,
            entries_tx,
            entries_rx,
            current_view: ComponentIds::LoadingText
        };
        instance.update(Some(Message::RefreshRequested));
        return instance
    }
    pub fn view(&mut self) {
        assert!(
            self.terminal.raw_mut().draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)].as_ref()) 
                    .split(f.size());
                self.app.view(&self.current_view.clone(), f, chunks[0]);
            }).is_ok()
        );
        let _ = self.app.active(&self.current_view);
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
                LoadingText::subscriptions()
            ).is_ok()
        );
        assert!(
            app.mount(
                ComponentIds::FeedEntryList, 
                Box::new(FeedEntryList::new(Vec::default())),
                FeedEntryList::subscriptions()
            ).is_ok()
        );
        assert!(
            app.mount(
                ComponentIds::ReadEntry, 
                Box::new(ReadEntryView::new(None)),
                ReadEntryView::subscriptions()
            ).is_ok()
        );

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
                    let miniflux_client = self.miniflux_client.clone();
                    let entries_tx = self.entries_tx.clone();
                    tokio::spawn(async move {
                        let entries = miniflux_client.get_unread_entries(100, 0).await;
                        let updated_entries = match entries {
                            Ok(e) => Some(e),
                            _ => None
                        };
                        let _ = entries_tx.send(updated_entries).await;
                    });
                    self.current_view = ComponentIds::LoadingText;
                    return None
                }
                Message::FeedEntriesReceived(entries) => {
                    let serialized_entries = entries.iter()
                        .map(|e| serde_json::to_string(e).unwrap())
                        .map(|json| PropValue::Str(json))
                        .collect::<Vec<PropValue>>();
                    assert!(
                        self.app.attr(
                            &ComponentIds::FeedEntryList, 
                            tuirealm::Attribute::Content, 
                            tuirealm::AttrValue::Payload(
                                PropPayload::Vec(serialized_entries)
                            )
                        ).is_ok()
                    );
                    self.current_view = ComponentIds::FeedEntryList;
                    return None
                }
                Message::EntrySelected(entry) => {
                    assert!(
                        self.app.attr(
                            &ComponentIds::ReadEntry, 
                            tuirealm::Attribute::Content, 
                            tuirealm::AttrValue::String(
                                serde_json::to_string(&entry).unwrap()
                            )
                        ).is_ok()
                    );
                    self.current_view = ComponentIds::ReadEntry;
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
                    self.current_view = ComponentIds::FeedEntryList;
                    return None
                },
            }
        }
        return None
    }
}
