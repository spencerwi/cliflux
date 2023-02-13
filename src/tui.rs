use std::sync::mpsc::{Receiver, channel, Sender};

use cursive::{Cursive, views::TextView, traits::Nameable};

use crate::libminiflux::{FeedEntry, self};

extern crate cursive;

enum UiMessage {
    LoadingStarted,
    FeedEntriesFetched(Vec<FeedEntry>),
    EntrySelected(FeedEntry)
}
enum ControllerMessage {
    RefreshRequested
}

pub struct Controller {
    miniflux_client : libminiflux::Client,
    rx: Receiver<ControllerMessage>,
    ui: Ui
}
impl Controller {
    pub fn new(miniflux_client : libminiflux::Client) -> Self {
        let (tx, rx) = channel::<ControllerMessage>();
        return Self {
            miniflux_client,
            rx,
            ui: Ui::new(tx)
        }
    }
}

pub struct Ui {
    cursive : Cursive,
    ui_tx : Sender<UiMessage>,
    ui_rx : Receiver<UiMessage>,
    controller_tx : Sender<ControllerMessage>,
}
impl Ui {
    pub fn new(controller_tx : Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = channel::<UiMessage>();
        let mut ui = Ui {
            cursive: Cursive::new(),
            ui_tx,
            ui_rx,
            controller_tx
        };
        ui.show_loading();
        return ui;
    }

    fn show_loading(&mut self) { 
        self.cursive.add_layer(
            TextView::new("Loading...").with_name("loading_text")
        );
    }

    pub fn step(&mut self) -> bool {
        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UiMessage::LoadingStarted => todo!(),
                UiMessage::FeedEntriesFetched(_) => todo!(),
                UiMessage::EntrySelected(_) => todo!(),
            }
        }

        self.cursive.step();

        true
    }
}
