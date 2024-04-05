use tuirealm::{Props, Sub, event::{KeyEvent, KeyModifiers, Key}, SubClause, MockComponent, Component, State, command::{Cmd, CmdResult}, Event, tui::{widgets::{Table, Row, Block, Borders}, layout::Constraint}, props::{Style, Alignment}};
use tuirealm::tui::style::Modifier;

use crate::ui::{SubscribingComponent, ComponentIds, SubClauses, Message, utils::to_window_title};

pub struct KeyboardHelp {
    props: Props
}

impl Default for KeyboardHelp {
    fn default() -> Self {
        Self {
            props: Props::default()
        }
    }
}

impl KeyboardHelp {}

impl SubscribingComponent for KeyboardHelp {
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

impl MockComponent for KeyboardHelp {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
		let rows = vec![
                Row::new(vec!["", "Global"]).style(Style::default().add_modifier(Modifier::BOLD)),
                Row::new(vec!["", "q", "Quit", ""]),
                Row::new(vec!["", "?", "Show keyboard help", ""]),
                Row::new(vec![""]),

                Row::new(vec!["", "Unread/Starred Entries view"]).style(Style::default().add_modifier(Modifier::BOLD)),
                Row::new(vec!["", "j", "Scroll down"]),
                Row::new(vec!["", "Down arrow", "Scroll down"]),
                Row::new(vec!["", "k", "Scroll up"]),
                Row::new(vec!["", "Up arrow", "Scroll up"]),
                Row::new(vec!["", "m", "Mark as read/unread"]),
                Row::new(vec!["", "s", "Toggle starred"]),
                Row::new(vec!["", "Enter", "Read entry"]),
                Row::new(vec!["", "v", "Swap view (Unread Entries/Starred Entries)"]),
                Row::new(vec![""]),

                Row::new(vec!["", "Read entry view"]).style(Style::default().add_modifier(Modifier::BOLD)),
                Row::new(vec!["", "j", "Scroll down"]),
                Row::new(vec!["", "Down arrow", "Scroll down"]),
                Row::new(vec!["", "k", "Scroll up"]),
                Row::new(vec!["", "Up arrow", "Scroll up"]),
                Row::new(vec!["", "u", "Mark as unread"]),
                Row::new(vec!["", "s", "Toggle starred"]),
                Row::new(vec!["", "o", "Open in browser"]),
                Row::new(vec!["", "b", "Back to Unread Entries view"]),
                Row::new(vec!["", "", ""]),

                Row::new(vec!["", "Keyboard help view"]).style(Style::default().add_modifier(Modifier::BOLD)),
                Row::new(vec!["", "Esc", "Close keyboard help"]),
            ];
        let widget = Table::new(
			rows,
			vec![Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1,3)]
        ).block(
            Block::default()
                .title(to_window_title("Keyboard Help"))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
        ).widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
        ;
        frame.render_widget(
            widget, 
            area
        );
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, KeyEvent> for KeyboardHelp {
    fn on(&mut self, ev: tuirealm::Event<KeyEvent>) -> Option<Message> {
        return match ev {
            Event::Keyboard(KeyEvent { 
                code: Key::Char('q'),
                .. 
            }) => Some(Message::AppClose),

            Event::Keyboard(KeyEvent { 
                code: Key::Char('b'),
                .. 
            }) => Some(Message::HideKeyboardHelp),

            Event::Keyboard(KeyEvent { 
                code: Key::Esc,
                .. 
            }) => Some(Message::HideKeyboardHelp),

            _ => None
        }
    }
}
