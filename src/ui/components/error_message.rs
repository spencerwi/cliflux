use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::Alignment,
    tui::widgets::{Block, Borders, Paragraph},
    Component, Event, MockComponent, Props, State, Sub, SubClause,
};

use crate::ui::{ComponentIds, Message, SubClauses, SubscribingComponent};

pub struct ErrorMessage {
    props: Props,
    message: Option<String>,
}

impl Default for ErrorMessage {
    fn default() -> Self {
        Self {
            props: Props::default(),
            message: None,
        }
    }
}

impl ErrorMessage {}

impl SubscribingComponent for ErrorMessage {
    fn subscriptions(component_id: ComponentIds) -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Self::key_sub(Key::Char('q'), KeyModifiers::NONE, SubClause::Always),
            Self::key_sub(
                Key::Char('b'),
                KeyModifiers::NONE,
                SubClauses::when_focused(&component_id),
            ),
            Self::key_sub(
                Key::Esc,
                KeyModifiers::NONE,
                SubClauses::when_focused(&component_id),
            ),
        ];
    }
}

impl MockComponent for ErrorMessage {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        if let Some(msg) = &self.message {
            let widget = Paragraph::new(msg.clone())
                .wrap(tuirealm::tui::widgets::Wrap { trim: false })
                .block(
                    Block::default()
                        .title("Error")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL),
                );
            frame.render_widget(widget, area);
        }
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        match attr {
            tuirealm::Attribute::Content => {
                let unwrapped = value.clone().unwrap_string();
                self.message = Some(unwrapped);
            }
            _ => {}
        }
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, KeyEvent> for ErrorMessage {
    fn on(&mut self, ev: tuirealm::Event<KeyEvent>) -> Option<Message> {
        return match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => Some(Message::AppClose),

            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                ..
            }) => Some(Message::DismissError),

            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Message::DismissError),

            _ => None,
        };
    }
}
