use iced::{
    executor,
    keyboard::{Event, KeyCode},
    widget::container,
    Application, Command, Element, Settings, Subscription, Text,
};
use iced_native::subscription;
use my_text::MyText;
use keys::bind;

mod my_text {
    use iced::{alignment, keyboard, Point};
    use iced_native::layout::{self, Layout};
    use iced_native::renderer;
    use iced_native::text::Text;
    use iced_native::widget::Widget;
    use iced_native::{event, text, Clipboard, Shell};
    use iced_native::{Color, Element, Length, Rectangle, Size};

    use crate::keys::key_code_to_key;
    #[allow(missing_debug_implementations)]
    pub struct MyText<'a, Message, Renderer: text::Renderer> {
        content: String,
        size: Option<u16>,
        color: Option<Color>,
        font: Renderer::Font,
        width: Length,
        height: Length,
        on_change: Box<dyn Fn(String) -> Message + 'a>,
        horizontal_alignment: alignment::Horizontal,
        vertical_alignment: alignment::Vertical,
    }

    impl<'a, Message, Renderer: text::Renderer> MyText<'a, Message, Renderer> {
        /// Create a new fragment of [`Text`] with the given contents.
        pub fn new<F>(content: &str, on_change: F) -> Self
        where
            F: 'a + Fn(String) -> Message,
        {
            MyText {
                content: String::from(content),
                size: None,
                color: None,
                font: Default::default(),
                width: Length::Fill,
                height: Length::Units(20),
                on_change: Box::new(on_change),
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Top,
            }
        }

        /// Sets the size of the [`Text`].
        pub fn size(mut self, size: u16) -> Self {
            self.size = Some(size);
            self
        }

        /// Sets the [`Color`] of the [`Text`].
        pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
            self.color = Some(color.into());
            self
        }

        /// Sets the [`Font`] of the [`Text`].
        ///
        /// [`Font`]: crate::text::Renderer::Font
        pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
            self.font = font.into();
            self
        }

        /// Sets the width of the [`Text`] boundaries.
        pub fn width(mut self, width: Length) -> Self {
            self.width = width;
            self
        }

        /// Sets the height of the [`Text`] boundaries.
        pub fn height(mut self, height: Length) -> Self {
            self.height = height;
            self
        }

        /// Sets the [`alignment::Horizontal`] of the [`Text`].
        pub fn horizontal_alignment(mut self, alignment: alignment::Horizontal) -> Self {
            self.horizontal_alignment = alignment;
            self
        }

        /// Sets the [`alignment::Vertical`] of the [`Text`].
        pub fn vertical_alignment(mut self, alignment: alignment::Vertical) -> Self {
            self.vertical_alignment = alignment;
            self
        }

        pub fn content(mut self, new_content: String) -> Self {
            self.content = new_content;
            self
        }
    }

    // pub fn my_text(content: String) -> MyText<Renderer>
    //     where T: ToString
    // {
    //     MyText::new(content)
    // }

    impl<'a, Message, Renderer> Widget<Message, Renderer> for MyText<'a, Message, Renderer>
    where
        Message: Clone,
        Renderer: text::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
            let limits = limits.width(self.width).height(self.height);

            let size = self.size.unwrap_or(renderer.default_size());

            let bounds = limits.max();

            let (width, height) = renderer.measure(&self.content, size, self.font.clone(), bounds);

            let size = limits.resolve(Size::new(width, height));

            layout::Node::new(size)
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            style: &renderer::Style,
            layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) {
            draw(
                renderer,
                style,
                layout,
                &self.content.to_string(),
                self.font.clone(),
                self.size,
                self.color,
                self.horizontal_alignment,
                self.vertical_alignment,
            );
        }

        fn on_event(
            &mut self,
            event: iced_native::Event,
            layout: Layout<'_>,
            cursor_position: Point,
            renderer: &Renderer,
            _clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            update(
                event,
                layout,
                cursor_position,
                renderer,
                shell,
                &mut self.content,
                self.size,
                &self.font,
                self.on_change.as_ref(),
            )
        }
    }

    fn update<'a, Message, Renderer>(
        event: iced_native::Event,
        layout: Layout,
        _cursor_position: Point,
        renderer: &Renderer,
        shell: &mut Shell<Message>,
        content: &mut String,
        size: Option<u16>,
        font: &Renderer::Font,
        on_change: &dyn Fn(String) -> Message,
    ) -> event::Status
    where
        Message: Clone,
        Renderer: text::Renderer,
    {
        let text = content.to_string();
        let size = size.unwrap_or(renderer.default_size());

        let text_width = renderer.measure_width(&text, size, font.clone());

        let bounds = layout.bounds();

        match event {
            iced_native::Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                content.insert_str(
                    content.len(),
                    format!("{} ", key_code_to_key(&key_code)).as_str(),
                );
                let message = (on_change)(content.clone());
                shell.publish(message);
                return event::Status::Captured;
            }
            _ => {}
        }
        if text_width > bounds.width {
            let (_oldest_key, splitted_content) = content.split_once(" ").unwrap();
            *content = splitted_content.to_string();
            let message = (on_change)(content.clone());
            shell.publish(message);
            return event::Status::Captured;
        }
        event::Status::Ignored
    }

    impl<'a, Message, Renderer> From<MyText<'a, Message, Renderer>> for Element<'a, Message, Renderer>
    where
        Message: 'a + Clone,
        Renderer: 'a + text::Renderer,
    {
        fn from(my_text: MyText<'a, Message, Renderer>) -> Self {
            Self::new(my_text)
        }
    }
    pub fn draw<Renderer>(
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        content: &String,
        font: Renderer::Font,
        size: Option<u16>,
        color: Option<Color>,
        horizontal_alignment: alignment::Horizontal,
        vertical_alignment: alignment::Vertical,
    ) where
        Renderer: text::Renderer,
    {
        let bounds = layout.bounds();

        let x = match horizontal_alignment {
            alignment::Horizontal::Left => bounds.x,
            alignment::Horizontal::Center => bounds.center_x(),
            alignment::Horizontal::Right => bounds.x + bounds.width,
        };

        let y = match vertical_alignment {
            alignment::Vertical::Top => bounds.y,
            alignment::Vertical::Center => bounds.center_y(),
            alignment::Vertical::Bottom => bounds.y + bounds.height,
        };

        renderer.fill_text(Text {
            content: content.as_str(),
            size: f32::from(size.unwrap_or(renderer.default_size())),
            bounds: Rectangle { x, y, ..bounds },
            color: color.unwrap_or(style.text_color),
            font,
            horizontal_alignment,
            vertical_alignment,
        });
    }
}

mod keys {
    use iced_native::subscription;
    use iced::{
        futures::{channel::mpsc, FutureExt, SinkExt, StreamExt, TryFutureExt},
        Subscription, keyboard::KeyCode,
    };
    use rdev::listen;

    pub enum State {
        Starting,
        Ready(mpsc::UnboundedReceiver<rdev::Event>),
    }
    #[derive(Debug, Clone)]
    pub enum Event {
        Ready,
        EventRecieved(rdev::Event),
    }

    pub fn bind() -> Subscription<Event> {
        struct Keys;
        
        subscription::unfold(
            std::any::TypeId::of::<Keys>(),
            State::Starting,
            |state| async move {
                match state {
                    State::Starting => {
                        let (mut sender, receiver) = mpsc::unbounded();
                        std::thread::spawn(move || {
                            listen(move |event| {
                                sender
                                    .send(event)
                                    .unwrap_or_else(|e| println!("Could not send event {:?}", e))
                                    .now_or_never();
                            })
                            .expect("Could not listen");
                        });
                        (Some(Event::Ready), State::Ready(receiver))
                    }
                    State::Ready(mut input) => {
                        let received = input.next().await;
                        match received {
                            Some(key) => (Some(Event::EventRecieved(key)), State::Ready(input)),
                            None => (None, State::Ready(input)),
                        }
                    }
                }
            },
        )
    }
    
    pub fn key_code_to_key(key_code: &KeyCode) -> &str {
        match key_code {
            KeyCode::Key1 => "1",
            KeyCode::Key2 => "2",
            KeyCode::Key3 => "3",
            KeyCode::Key4 => "4",
            KeyCode::Key5 => "5",
            KeyCode::Key6 => "6",
            KeyCode::Key7 => "7",
            KeyCode::Key8 => "8",
            KeyCode::Key9 => "9",
            KeyCode::Key0 => "0",

            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",

            KeyCode::Apostrophe => "'",
            KeyCode::Backslash => "\\",
            KeyCode::Colon => ":",
            KeyCode::Comma => ",",
            KeyCode::Equals => "=",
            KeyCode::LBracket => "[",
            KeyCode::Minus => "-",
            KeyCode::Period => ".",
            KeyCode::Plus => "+",
            KeyCode::RBracket => "]",
            KeyCode::Semicolon => ";",
            KeyCode::Slash => "/",

            KeyCode::Escape => "esc",

            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            KeyCode::F13 => "F13",
            KeyCode::F14 => "F14",
            KeyCode::F15 => "F15",
            KeyCode::F16 => "F16",
            KeyCode::F17 => "F17",
            KeyCode::F18 => "F18",
            KeyCode::F19 => "F19",
            KeyCode::F20 => "F20",
            KeyCode::F21 => "F21",
            KeyCode::F22 => "F22",
            KeyCode::F23 => "F23",
            KeyCode::F24 => "F24",

            KeyCode::Snapshot => "Snapshot",
            KeyCode::Scroll => "Scroll",
            KeyCode::Pause => "Pause",

            KeyCode::Insert => "Insert",
            KeyCode::Home => "Home",
            KeyCode::Delete => "Delete",
            KeyCode::End => "End",
            KeyCode::PageDown => "PageDown",
            KeyCode::PageUp => "PageUp",

            KeyCode::Left => "<-",
            KeyCode::Up => "Up",
            KeyCode::Right => "->",
            KeyCode::Down => "Down",

            KeyCode::Backspace => "Backspace",
            KeyCode::Enter => "Enter",
            KeyCode::Space => "Space",

            KeyCode::Compose => "Compose",

            KeyCode::Caret => "Caret",

            KeyCode::Numlock => "Numlock",
            KeyCode::Numpad0 => "Numpad0",
            // KeyCode::Numpad1 => "Numpad1",
            // KeyCode::Numpad2 => "Numpad2",
            // KeyCode::Numpad3 => "Numpad3",
            // KeyCode::Numpad4 => "Numpad",
            // KeyCode::Numpad5 => "Numpad",
            // KeyCode::Numpad6 => "Numpad",
            // KeyCode::Numpad7 => "Numpad",
            // KeyCode::Numpad8 => "Numpad",
            // KeyCode::Numpad9 => "Numpad",
            // KeyCode::NumpadAdd => "NumpadAdd",
            // KeyCode::NumpadDivide => "NumpadDivide",
            // KeyCode::NumpadDecimal => "Numpad",
            // KeyCode::NumpadComma => "Numpad",
            // KeyCode::NumpadEnter => "Numpad",
            // KeyCode::NumpadEquals => "Numpad",
            // KeyCode::NumpadMultiply => "Numpad",
            // KeyCode::NumpadSubtract => "Numpad",

            // KeyCode::AbntC1,
            // KeyCode::AbntC2,
            // KeyCode::Apps,
            // KeyCode::Asterisk,
            // KeyCode::At,
            // KeyCode::Ax,
            // KeyCode::Calculator,
            // KeyCode::Capital,
            // KeyCode::Convert,
            // KeyCode::Grave,
            // KeyCode::Kana,
            // KeyCode::Kanji,
            // KeyCode::LAlt,
            // KeyCode::LControl,
            // KeyCode::LShift,
            // KeyCode::LWin,
            // KeyCode::Mail,
            // KeyCode::MediaSelect,
            // KeyCode::MediaStop,
            // KeyCode::Mute,
            // KeyCode::MyComputer,
            // KeyCode::NavigateForward,  // also called "Next"
            // KeyCode::NavigateBackward, // also called "Prior"
            // KeyCode::NextTrack,
            // KeyCode::NoConvert,
            // KeyCode::OEM102,
            // KeyCode::PlayPause,
            // KeyCode::Power,
            // KeyCode::PrevTrack,
            // KeyCode::RAlt,
            // KeyCode::RControl,
            // KeyCode::RShift,
            // KeyCode::RWin,
            // KeyCode::Sleep,
            // KeyCode::Stop,
            // KeyCode::Sysrq,
            // KeyCode::Tab,
            // KeyCode::Underline,
            // KeyCode::Unlabeled,
            // KeyCode::VolumeDown,
            // KeyCode::VolumeUp,
            // KeyCode::Wake,
            // KeyCode::WebBack,
            // KeyCode::WebFavorites,
            // KeyCode::WebForward,
            // KeyCode::WebHome,
            // KeyCode::WebRefresh,
            // KeyCode::WebSearch,
            // KeyCode::WebStop,
            // KeyCode::Yen,
            // KeyCode::Copy,
            // KeyCode::Paste,
            // KeyCode::Cut,
            _ => "?",
        }
    }
}

#[derive(Default)]
struct ScreenKey {
    keys: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    RdevKeys(keys::Event),
    IcedKeys(KeyCode),
    InputChanged(String),
}

impl Application for ScreenKey {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Cool Title")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::RdevKeys(event) => {
                match event {
                    keys::Event::Ready => {
                        println!("Ready to recieve!");
                    },
                    keys::Event::EventRecieved(rdev_event) => {
                        match rdev_event.event_type {
                            rdev::EventType::KeyPress(key) => {
                                println!("{key:?}");
                            }
                            _=> {}
                        }
                    }
                }
            }
            Message::IcedKeys(key) => {
                todo!()
            }
            Message::InputChanged(new_value) => {
                self.keys = String::from(new_value);
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        keys::bind().map(|x| Message::RdevKeys(x))
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        container::Container::new(Text::new("Hello"))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: iced::window::Settings {
            size: (200, 20),
            // resizable: false,
            // decorations: false,
            transparent: true,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    };
    ScreenKey::run(settings)
}
