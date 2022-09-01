// #![windows_subsystem = "windows"]
use crate::keys::{iced_to_key, rdev_to_key};
use iced::{
    container::{Style, StyleSheet},
    executor,
    keyboard::Event,
    mouse,
    widget::container,
    window::{self, Icon, Position},
    Application, Background, Color, Command, Element, Font, Settings, Subscription,
};
use iced_native::{subscription, widget::Text, window as native_window};
use image;

mod keys;

#[derive(Default)]
struct ScreenKey {
    keys: String,
    key_frequency: u32,
    frequent_key: String,
    max_width: u32,
    width: u32,
    line: u32,
    is_grabbing: bool,
    grab_location: (i32, i32),
    window_position: (i32, i32),
}

#[derive(Debug, Clone)]
pub enum Message {
    RdevEvents(keys::Event),
    IcedEvents(iced_native::Event),
    InputChanged(String),
}

const FONT: Font = Font::External {
    name: "Nerd Font",
    bytes: include_bytes!("../fonts/Fura Code Bold Nerd Font Complete Mono.ttf"),
};

// const WIDTH: u32 = 400;

const HEIGHT: u16 = 30;

struct ContainerStyles;

impl StyleSheet for ContainerStyles {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            })),
            border_radius: 10.,
            ..Default::default()
        }
    }
}

impl Application for ScreenKey {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                keys: "".to_string(),
                key_frequency: 0,
                frequent_key: "".to_string(),
                max_width: 1050,
                width: 0,
                line: 1,
                is_grabbing: false,
                grab_location: (0, 0),
                window_position: (0, 0),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Zr Alshaha")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.width = ((self.keys.chars().count() / 2) as u32 * HEIGHT as u32 + 50)
            - (self.max_width * (self.line - 1));
        if self.width >= self.max_width {
            self.line += 1;
        }
        if self.line > 3 {
            self.line = 1;
            self.keys = "".to_string();
            return Command::single(iced_native::command::Action::Window(
                native_window::Action::Resize {
                    width: 1,
                    height: HEIGHT as u32 * self.line,
                },
            ));
        }
        match message {
            Message::RdevEvents(event) => match event {
                keys::Event::Ready => {
                    println!("Ready to recieve!");
                }
                keys::Event::EventRecieved(rdev_event) => match rdev_event.event_type {
                    rdev::EventType::KeyPress(key) => {
                        let coming_key = rdev_to_key(&key).to_string();
                        if self.frequent_key != coming_key {
                            self.key_frequency = 0;
                            self.frequent_key = "".to_string();
                        }
                        let frequent_key =
                            format!("{}...x{} ", self.frequent_key, self.key_frequency);
                        self.key_frequency += 1;

                        self.frequent_key = coming_key.clone();

                        let new_frequent_key =
                            format!("{}...x{} ", self.frequent_key, self.key_frequency);

                        if self.key_frequency > 3 {
                            let repeated_key = format!(
                                "{} {} {} ",
                                self.frequent_key, self.frequent_key, self.frequent_key
                            );
                            if self.keys.ends_with(repeated_key.as_str()) {
                                self.keys = format!(
                                    "{}{}",
                                    self.keys.trim_end_matches(repeated_key.as_str()),
                                    new_frequent_key
                                );
                            } else {
                                self.keys = format!(
                                    "{}{}",
                                    self.keys.trim_end_matches(frequent_key.as_str()),
                                    new_frequent_key
                                );
                            }
                        } else {
                            self.keys = format!("{}{} ", self.keys, coming_key);
                        }
                        return Command::single(iced_native::command::Action::Window(
                            native_window::Action::Resize {
                                width: if self.line > 1 {
                                    self.max_width
                                } else {
                                    self.width
                                },
                                height: HEIGHT as u32 * self.line,
                            },
                        ));
                    }
                    _ => {}
                },
            },
            Message::IcedEvents(event) => match event {
                #[cfg(target_os = "windows")]
                iced_native::Event::Keyboard(Event::KeyPressed {
                    key_code,
                    modifiers,
                }) => {
                    let coming_key = iced_to_key(&key_code).to_string();
                    if self.frequent_key != coming_key {
                        self.key_frequency = 0;
                        self.frequent_key = "".to_string();
                    }
                    let frequent_key = format!("{}...x{} ", self.frequent_key, self.key_frequency);
                    self.key_frequency += 1;

                    self.frequent_key = coming_key.clone();

                    let new_frequent_key =
                        format!("{}...x{} ", self.frequent_key, self.key_frequency);

                    if self.key_frequency > 3 {
                        let repeated_key = format!(
                            "{} {} {} ",
                            self.frequent_key, self.frequent_key, self.frequent_key
                        );
                        if self.keys.ends_with(repeated_key.as_str()) {
                            self.keys = format!(
                                "{}{}",
                                self.keys.trim_end_matches(repeated_key.as_str()),
                                new_frequent_key
                            );
                        } else {
                            self.keys = format!(
                                "{}{}",
                                self.keys.trim_end_matches(frequent_key.as_str()),
                                new_frequent_key
                            );
                        }
                    } else {
                        self.keys = format!("{}{} ", self.keys, coming_key);
                    }
                    return Command::single(iced_native::command::Action::Window(
                        native_window::Action::Resize {
                            width: if self.line > 1 {
                                self.max_width
                            } else {
                                self.width
                            },
                            height: HEIGHT as u32 * self.line,
                        },
                    ));
                }
                iced_native::Event::Mouse(mouse::Event::ButtonPressed(
                    iced::mouse::Button::Right,
                )) => {
                    self.is_grabbing = true;
                    self.grab_location = (0, 0);
                }
                iced_native::Event::Mouse(mouse::Event::ButtonReleased(
                    iced::mouse::Button::Right,
                )) => {
                    self.is_grabbing = false;
                }
                iced_native::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    if self.grab_location == (0, 0) {
                        self.grab_location = (position.x as i32, position.y as i32);
                    }
                    if self.is_grabbing == true {
                        let x = position.x as i32 + self.window_position.0 - self.grab_location.0;
                        let y = position.y as i32 + self.window_position.1 - self.grab_location.1;
                        // println!("{:?}", self.window_position);
                        return window::move_to(x, y);
                    }
                }
                iced_native::Event::Window(native_window::Event::Moved { x, y }) => {
                    self.window_position = (x, y);
                }
                _ => {
                    // println!("{event:?}");
                }
            },
            Message::InputChanged(new_value) => {
                self.keys = String::from(new_value);
            } // _ => {}
        }
        Command::none()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }

    fn subscription(&self) -> Subscription<Message> {
        let iced_events = subscription::events().map(Message::IcedEvents);
        Subscription::batch(vec![
            keys::bind().map(|x| Message::RdevEvents(x)),
            iced_events,
        ])
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        container::Container::new(
            Text::new(self.keys.as_str())
                .size(HEIGHT)
                .height(iced::Length::Fill)
                .color(Color::WHITE)
                .font(FONT)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .style(ContainerStyles)
        .into()
    }
}

fn make_window_icon(path: &str) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap()
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: iced::window::Settings {
            size: (1, 1),
            position: Position::Specific(464, 918),
            decorations: false,
            transparent: true,
            always_on_top: true,
            icon: Some(make_window_icon("assets/peepoSalute.png")),
            ..Default::default()
        },
        ..Default::default()
    };
    ScreenKey::run(settings)
}
