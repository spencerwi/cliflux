use std::time::Duration;

use tokio::sync::mpsc;
use tuirealm::{tui::layout::{Layout, Direction, Constraint}, Application, event::KeyEvent, terminal::TerminalBridge, EventListenerCfg, Update, props::{PropPayload, PropValue}};

use crate::{libminiflux::{self, ReadStatus}, ui::components::{loading_text::LoadingText, feed_entry_list::FeedEntryList, read_entry_view::ReadEntryView}};

use super::{ComponentIds, Message};

extern crate tuirealm;

pub struct Model {
    pub app: Application<ComponentIds, Message, KeyEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub miniflux_client: libminiflux::Client,
    pub messages_rx : tokio::sync::mpsc::Receiver<Message>,
    messages_tx : tokio::sync::mpsc::Sender<Message>,
    current_view : ComponentIds
}
impl Model { 
    pub fn new(miniflux_client : libminiflux::Client) -> Self {
        let (messages_tx, messages_rx) = mpsc::channel::<Message>(32);

        let mut instance = Self {
            app: Self::init_app(),
            quit: false,
            redraw: false,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            miniflux_client,
            messages_tx,
            messages_rx,
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

    fn change_read_status(&mut self, entry_id : i32, new_status : ReadStatus) {
        let miniflux_client = self.miniflux_client.clone();
        tokio::spawn(async move {
            let _ = miniflux_client.change_entry_read_status(entry_id, new_status).await;
        });
    }

    fn do_refresh(&mut self) {
        let miniflux_client = self.miniflux_client.clone();
        let messages_tx = self.messages_tx.clone();
        tokio::spawn(async move {
            let entries = miniflux_client.get_unread_entries(100, 0).await;
            if let Ok(updated_entries) = entries {
                let _ = messages_tx.send(
                    Message::FeedEntriesReceived(updated_entries)
                ).await;
            }
        });
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
                    self.current_view = ComponentIds::LoadingText;
                    self.do_refresh();
                    return Some(Message::Tick)
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
                    return Some(Message::Tick)
                }
                Message::ChangeEntryReadStatus(entry_id, new_status) => {
                    self.change_read_status(entry_id, new_status);
                    return Some(Message::Tick)
                }
                Message::EntrySelected(mut entry) => {
                    assert!(
                        self.app.attr(
                            &ComponentIds::ReadEntry, 
                            tuirealm::Attribute::Content, 
                            tuirealm::AttrValue::String(
                                serde_json::to_string(&entry).unwrap()
                            )
                        ).is_ok()
                    );
                    entry.status = ReadStatus::Read;
                    self.change_read_status(entry.id, ReadStatus::Read);
                    self.current_view = ComponentIds::ReadEntry;
                    return Some(Message::Tick)
                },
                Message::ReadEntryViewClosed => {
                    self.current_view = ComponentIds::FeedEntryList;
                    return Some(Message::Tick)
                },
                _ => {}
            }
        }
        return None
    }

}
