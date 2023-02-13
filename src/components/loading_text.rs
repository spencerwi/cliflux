use super::super::tui::Message;
use tuirealm::{Component, MockComponent, State, tui::widgets::Paragraph, Props, props::Style, command::CmdResult, event::KeyEvent};

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

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, KeyEvent> for LoadingText {
    fn on(&mut self, ev: tuirealm::Event<KeyEvent>) -> Option<Message> {
        None
    }
}