use std::time::Duration;
use crate::ui::{SubscribingComponent, components::{keyboard_help::KeyboardHelp, feed_entry_list::FeedListViewType, error_message::ErrorMessage}};

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
    current_view : ComponentIds,
    previous_view : Option<ComponentIds>,
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
            current_view: ComponentIds::LoadingText,
            previous_view: None
        };
        instance.update(Some(Message::RefreshRequested(FeedListViewType::UnreadEntries)));
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
                LoadingText::subscriptions(ComponentIds::LoadingText)
            ).is_ok()
        );

        assert!(
            app.mount(
                ComponentIds::FeedEntryList, 
                Box::new(FeedEntryList::new(Vec::default(), FeedListViewType::UnreadEntries)),
                FeedEntryList::subscriptions(ComponentIds::FeedEntryList)
            ).is_ok()
        );

        assert!(
            app.mount(
                ComponentIds::ReadEntry, 
                Box::new(ReadEntryView::new(None)),
                ReadEntryView::subscriptions(ComponentIds::ReadEntry)
            ).is_ok()
        );

        assert!(
            app.mount(
                ComponentIds::KeyboardHelp,
                Box::new(KeyboardHelp::default()),
                KeyboardHelp::subscriptions(ComponentIds::KeyboardHelp)
            ).is_ok()
        );

		assert!(
			app.mount(
				ComponentIds::ErrorMessage,
				Box::new(ErrorMessage::default()),
				ErrorMessage::subscriptions(ComponentIds::ErrorMessage)
			).is_ok()
		);

        return app;
    }

    fn change_read_status(&mut self, entry_id : i32, new_status : ReadStatus) {
        let miniflux_client = self.miniflux_client.clone();
		let messages_tx = self.messages_tx.clone();
        tokio::spawn(async move {
            match miniflux_client.change_entry_read_status(entry_id, new_status).await {
				Ok(_) => {}
				Err(e) => Self::handle_error_message(e, messages_tx).await
			}
        });
    }

    fn toggle_starred(&mut self, entry_id : i32) {
        let miniflux_client = self.miniflux_client.clone();
		let messages_tx = self.messages_tx.clone();
        tokio::spawn(async move {
            match miniflux_client.toggle_starred(entry_id).await {
				Ok(_) => {}
				Err(e) => Self::handle_error_message(e, messages_tx).await
			}
        });
    }

	fn save_entry(&self, entry_id: i32) {
		let miniflux_client = self.miniflux_client.clone();
		let messages_tx = self.messages_tx.clone();
		tokio::spawn(async move {
			match miniflux_client.save_entry(entry_id).await {
				Ok(_) => {}
				Err(e) => Self::handle_error_message(e, messages_tx).await
			}
		});
	}

	fn mark_all_as_read(&self, entry_ids: Vec<i32>) {
		let miniflux_client = self.miniflux_client.clone();
		let messages_tx = self.messages_tx.clone();
		tokio::spawn(async move {
			match miniflux_client.mark_all_as_read(entry_ids).await {
				Ok(_) => {}
				Err(e) => Self::handle_error_message(e, messages_tx).await
			}
		});
	}

    fn do_refresh(&mut self, view_type : FeedListViewType) {
        let miniflux_client = self.miniflux_client.clone();
        let messages_tx = self.messages_tx.clone();
        tokio::spawn(async move {
            // TODO: pagination
            let entries = match view_type {
                FeedListViewType::UnreadEntries => miniflux_client.get_unread_entries(100, 0).await,
                FeedListViewType::StarredEntries => miniflux_client.get_starred_entries(100, 0).await, 
            };
			match entries {
				Ok(updated_entries) => {
					let _ = messages_tx.send(
						Message::FeedEntriesReceived(updated_entries)
					).await;
				}
				Err(e) => Self::handle_error_message(e, messages_tx).await
			}
        });
    }

	async fn handle_error_message(e : reqwest::Error, messages_tx : tokio::sync::mpsc::Sender<Message>) {
		let _ = messages_tx.send(
			Message::RequestErrorEncountered(e.status(), e.to_string())
		).await;
	}
}

impl Update<Message> for Model {
    fn update(&mut self, msg: Option<Message>) -> Option<Message> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Message::Batch(msgs) => {
                    let results: Vec<Option<Message>> = msgs.iter()
                        .map(|msg| self.update(msg.to_owned()))
                        .filter(Option::is_some)
                        .collect();
                    return match results.len() {
                        0 => None,
                        1 => results[0].to_owned(),
                        _ => Some(
                            Message::Batch(results)
                        )
                    }
                }
                Message::AppClose => {
                    self.quit = true;
                    return None
                }
                Message::RefreshRequested(view_type) => {
                    self.current_view = ComponentIds::LoadingText;
                    self.do_refresh(view_type);
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
                Message::ToggleStarred(entry_id) => {
                    self.toggle_starred(entry_id);
                    return Some(Message::Tick)
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
                    return Some(Message::Tick)
                },

                Message::ReadEntryViewClosed => {
                    self.current_view = ComponentIds::FeedEntryList;
                    return Some(Message::Tick)
                },

                Message::ShowKeyboardHelp => {
                    self.previous_view = Some(self.current_view.clone());
                    self.current_view = ComponentIds::KeyboardHelp;
                    return Some(Message::Tick)
                }

                Message::HideKeyboardHelp => {
                    self.current_view = match &self.previous_view {
                        Some(v) => v.to_owned(),
                        None => ComponentIds::FeedEntryList
                    };
                    self.previous_view = None;
                    return Some(Message::Tick)

                }

				Message::RequestErrorEncountered(status_code_maybe, err_string) => {
					let status_code_str = match status_code_maybe {
						None => "UNKNOWN".to_owned(),
						Some(v) => v.to_string()
					};
					self.previous_view = Some(self.current_view.clone());
					assert!(
						self.app.attr(
							&ComponentIds::ErrorMessage,
							tuirealm::Attribute::Content,
							tuirealm::AttrValue::String(
								format!("Error {status_code_str}: {err_string}")
							)
						).is_ok()
					);
					self.current_view = ComponentIds::ErrorMessage;
					return Some(Message::Tick);
				}
				Message::DismissError => {
					self.current_view = match &self.previous_view {
						Some(v) => v.to_owned(),
						None => ComponentIds::LoadingText
					};
					self.previous_view = None;
					return Some(Message::Tick);
				}
				
				Message::SaveEntry(entry_id) => {
					self.save_entry(entry_id);
					return Some(Message::Tick);
				}

				Message::MarkAllAsRead(entry_ids) => {
					self.mark_all_as_read(entry_ids);
					return Some(Message::Tick);
				}

                _ => {}
            }
        }
        return None
    }
}
