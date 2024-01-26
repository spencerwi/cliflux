use tuirealm::{Props, Sub, event::{KeyEvent, KeyModifiers, Key}, SubClause, MockComponent, Component, State, command::{Cmd, CmdResult}, Event, tui::widgets::{Block, Borders, Paragraph}, props::Alignment};

use crate::ui::{SubscribingComponent, ComponentIds, SubClauses, Message};

pub struct ErrorMessage {
    props: Props,
	message: Option<String>,
}

impl Default for ErrorMessage {
    fn default() -> Self {
        Self {
            props: Props::default(),
			message: None
        }
    }
}

impl ErrorMessage {}

impl SubscribingComponent for ErrorMessage {
    fn subscriptions(component_id : ComponentIds) -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('q'),
                    modifiers: KeyModifiers::NONE
                }),
                SubClause::Always
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('b'),
                    modifiers: KeyModifiers::NONE
                }),
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Esc,
                    modifiers: KeyModifiers::NONE
                }),
                SubClauses::when_focused(&component_id)
            )
        ]
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
						.borders(Borders::ALL)
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

            Event::Keyboard(KeyEvent { 
                code: Key::Esc,
                .. 
            }) => Some(Message::DismissError),

            _ => None
        }
    }
}
