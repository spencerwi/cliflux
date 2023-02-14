use crate::tui::ComponentIds;

use super::super::tui::Message;
use tuirealm::{Component, MockComponent, State, tui::widgets::Paragraph, Props, props::Style, command::CmdResult, event::{KeyEvent, Key, KeyModifiers}, Event, Sub, SubClause};

pub struct LoadingText { 
    props: Props
}

impl Default for LoadingText {
    fn default() -> Self {
        Self {
            props: Props::default()
        }
    }
}

impl LoadingText {
    pub fn new() -> Self {
        LoadingText::default()
    }

    pub fn subscriptions() -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('q'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            )
        ]
    }
}

impl MockComponent for LoadingText {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        frame.render_widget(
            Paragraph::new("Loading...")
                .style(Style::default()), 
            area
        )
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> tuirealm::State {
        State::None
    }

    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, KeyEvent> for LoadingText {
    fn on(&mut self, ev: Event<KeyEvent>) -> Option<Message> {
        return match ev {
            Event::Keyboard(KeyEvent { 
                code: Key::Char('q'),
                modifiers: _ 
            }) => Some(Message::AppClose),
            _ => None
        };
    }
}
