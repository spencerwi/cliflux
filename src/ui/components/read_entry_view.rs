use html2text::render::text_renderer::RichAnnotation;
use tuirealm::{MockComponent, event::{KeyEvent, Key, KeyModifiers}, Component, State, StateValue, tui::{layout::Alignment, widgets::{Paragraph, Block, Wrap}, text::{Text, Span, Spans}, style::{Style, Modifier, Color}}, command::{Cmd, CmdResult, Direction}, Event, Sub, SubClause, SubEventClause, Props};

use crate::{libminiflux::{FeedEntry, ReadStatus}, ui::{ComponentIds, Message, SubClauses, utils::StringPadding}};
use stringreader::StringReader;

// The number of lines to scroll when PageUp or PageDown is pressed
const PAGE_SCROLL_AMOUNT : u16 = 8;

pub struct RenderedEntry<'a> {
    rendered_text: Text<'a>,
    links: Vec<String>,
}
impl Default for RenderedEntry<'_> {
    fn default() -> Self {
        return Self {
            rendered_text: Text::default(),
            links: Vec::default()
        }
    }
}
impl RenderedEntry<'_> {
    pub fn new(entry: FeedEntry) -> Self {
        let mut links = Vec::default();
        let tagged_lines = html2text::from_read_rich(
            StringReader::new(&entry.content),
            120
        );
        let mut result = Text::default();
        for line in tagged_lines {
            let spans : Vec<Span> = line.tagged_strings()
                .into_iter()
                .flat_map(|element| {
                    let mut link_span : Option<Span> = None;
                    let mut contents = String::new();
                    contents += &element.s;
                    let mut style = Style::default();
                    for annotation in &element.tag {
                        match annotation {
                            RichAnnotation::Link(url) => {
                                links.extend(vec![url.to_owned()]);
                                link_span = Some(
                                    Span::styled(
                                        format!(" [{}]", links.len()),
                                        style.clone().fg(Color::Cyan)
                                    )
                                );
                            }
                            RichAnnotation::Image => {
                                style = style.add_modifier(Modifier::ITALIC)
                            }
                            RichAnnotation::Emphasis => {
                                style = style.add_modifier(Modifier::ITALIC);
                            }
                            RichAnnotation::Strong => {
                                style = style.add_modifier(Modifier::BOLD);
                            }
                            RichAnnotation::Strikeout => {
                                style = style.add_modifier(Modifier::CROSSED_OUT);
                            }
                            RichAnnotation::Code | RichAnnotation::Preformat(_) => {
                                style = style.fg(Color::Yellow);
                            }
                            RichAnnotation::Default => {}
                        }
                    }
                    let mut result = vec![
                        Span::styled(
                            format!("{}", element.s),
                            style
                        )
                    ];
                    if let Some(ls) = link_span {
                        result.extend(vec![ls]);
                    }
                    result
                })
                .collect();
            result.extend(
                Text::from(
                    Spans::from(spans)
                )
            )
        }
        result.extend(Text::from("\n")); // empty link before links
        for (idx, link) in links.iter().enumerate() {
            result.extend(
                Text::from(
                    Span::styled(
                        format!("[{}] {}", idx + 1, link),
                        Style::default().fg(Color::Cyan)
                    )
                )
            )
        }
        return Self {
            rendered_text: result.to_owned(),
            links
        }
    }
}

pub struct ReadEntryView<'a> {
    entry: Option<FeedEntry>,
    props: Props,
    rendered_entry : RenderedEntry<'a>,
    scroll : u16
}

impl Default for ReadEntryView<'_> {
    fn default() -> Self {
        Self {
            entry: None,
            props: Props::default(),
            rendered_entry: RenderedEntry::default(),
            scroll: 0
        }
    }
}

impl ReadEntryView<'_> {
    pub fn new(entry: Option<FeedEntry>) -> Self {
        if let Some(e) = entry {
            let rendered_entry = RenderedEntry::new(e.clone());
            return Self {
                entry: Some(e),
                props: Props::default(),
                rendered_entry,
                scroll: 0
            };
        } 
        Self::default()
    }

    pub fn subscriptions(component_id : ComponentIds) -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('q'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('?'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('k'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Up,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('j'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Down,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::PageUp,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::PageDown,
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
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
                    code: Key::Char('u'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('o'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClauses::when_focused(&component_id)
            ),

            Sub::new(
                tuirealm::SubEventClause::Tick,
                SubClauses::when_focused(&component_id)
            )
        ]
    }
}

impl MockComponent for ReadEntryView<'_> {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        if let Some(e) = &self.entry {
            let title = StringPadding::spaces_around(e.title.clone(), 1);
            let widget = Paragraph::new(self.rendered_entry.rendered_text.clone())
                .scroll((self.scroll, 0))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .title(Span::styled(title.clone(), Style::default().add_modifier(Modifier::BOLD)))
                        .title_alignment(Alignment::Center)
                        .borders(tuirealm::tui::widgets::Borders::ALL)
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
                let new_entry = serde_json::from_str::<FeedEntry>(&unwrapped).unwrap();
                self.entry = Some(new_entry.clone());
                self.rendered_entry = RenderedEntry::new(new_entry);
                self.scroll = 0;
            }
            _ => {}
        }
        self.props.set(attr, value)
    }

    fn state(&self) -> tuirealm::State {
        match &self.entry {
            Some(e) => State::One(
                StateValue::I32(e.id)
            ),
            None => State::None
        }
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        return match cmd {
            Cmd::Custom("quit") => CmdResult::Custom("quit"),
            Cmd::Custom("show_keyboard_help") => CmdResult::Custom("show_keyboard_help"),

            Cmd::Custom("back") => {
                self.entry = None;
                CmdResult::Custom("back")
            }

            Cmd::Custom("mark_as_unread") => {
                CmdResult::Custom("mark_as_unread")
            }

            Cmd::Custom("open_in_browser") => {
                if let Some(e) = &self.entry {
                    let _ = open::that(&e.url);
                }
                CmdResult::Custom("open_in_browser")
            }

            Cmd::Scroll(direction) => {
                self.scroll = 
                    match direction {
                        Direction::Up if self.scroll > 0 => self.scroll - 1,
                        Direction::Down => self.scroll + 1,
                        _ => 0
                    };
                CmdResult::Custom("scrolled")
            }

            Cmd::Custom("PageUp") => {
                if self.scroll > PAGE_SCROLL_AMOUNT {
                    self.scroll -= PAGE_SCROLL_AMOUNT;
                } else {
                    self.scroll = 0;
                }
                CmdResult::Custom("scrolled")
            }

            Cmd::Custom("PageDown") => {
                self.scroll += PAGE_SCROLL_AMOUNT;
                CmdResult::Custom("scrolled")
            }

            _ => CmdResult::None
        }
    }
}

impl Component<Message, KeyEvent> for ReadEntryView<'_> {
    fn on(&mut self, ev: tuirealm::Event<KeyEvent>) -> Option<Message> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('?'),
                ..
            }) => Cmd::Custom("show_keyboard_help"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                ..
            }) => Cmd::Custom("back"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('u'),
                ..
            }) => Cmd::Custom("mark_as_unread"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('o'),
                ..
            }) => Cmd::Custom("open_in_browser"),

            Event::Keyboard(KeyEvent { 
                code: Key::Char('k'),
                ..
            }) => Cmd::Scroll(Direction::Up),
            Event::Keyboard(KeyEvent { 
                code: Key::Up,
                ..
            }) => Cmd::Scroll(Direction::Up),

            Event::Keyboard(KeyEvent { 
                code: Key::Char('j'),
                ..
            }) => Cmd::Scroll(Direction::Down),
            Event::Keyboard(KeyEvent { 
                code: Key::Down,
                ..
            }) => Cmd::Scroll(Direction::Down),

            Event::Keyboard(KeyEvent { 
                code: Key::PageUp,
                ..
            }) => Cmd::Custom("PageUp"),

            Event::Keyboard(KeyEvent { 
                code: Key::PageDown,
                ..
            }) => Cmd::Custom("PageDown"),

            _ => Cmd::None
        };

        return match self.perform(cmd) {
            CmdResult::Custom("quit") => Some(Message::AppClose),
            CmdResult::Custom("show_keyboard_help") => Some(Message::ShowKeyboardHelp),

            CmdResult::Custom("back") => Some(Message::ReadEntryViewClosed),

            CmdResult::Custom("mark_as_unread") => {
                match &mut self.entry {
                    Some(e) => {
                        e.status = ReadStatus::Unread;
                        return Some(Message::ChangeEntryReadStatus(e.id, ReadStatus::Unread))
                    }
                    None => None
                }
            }

            CmdResult::Custom("scrolled") => Some(Message::Tick),

            CmdResult::Changed(_) => Some(Message::Tick),

            _ => None
        }
    }
}
