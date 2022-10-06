#![windows_subsystem = "windows"] // uncomment before build
use crate::keys::rdev_to_key;

// #[cfg(not(target_os = "linux"))]
use crate::keys::iced_to_key;

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
use serde::Deserialize;
use std::io::Cursor;
use toml::from_str;

mod keys;

#[derive(Debug, Deserialize)]
struct Config {
    position: Option<PositionConfig>,
    font_size: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            position: Some(PositionConfig::default()),
            font_size: Some(30),
        }
    }
}

#[derive(Debug, Deserialize)]
struct PositionConfig {
    x: i32,
    y: i32,
}

impl Default for PositionConfig {
    fn default() -> Self {
        Self { x: 1000, y: 1000 }
    }
}

#[derive(Default)]
struct ScreenKey {
    keys: String,
    key_frequency: u32,
    frequent_key: String,
    max_width: u32,
    width: u32,
    font_size: u32,
    line: u32,
    is_grabbing: bool,
    grab_location: (i32, i32),
    window_position: (i32, i32),
    extra_width: u32,
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
        let config_dir = dirs::config_dir().expect("No Config Directory");

        let config_file = config_dir.join("zr-alshasha/config.toml");

        let config_str = std::fs::read_to_string(config_file).unwrap_or_default();

        let config: Config = from_str(&config_str).unwrap_or_else(|e| {
            eprintln!("{e}");

            Config::default()
        });

        (
            Self {
                keys: "".to_string(),
                key_frequency: 0,
                frequent_key: "".to_string(),
                max_width: 1050,
                width: 0,
                font_size: config.font_size.unwrap_or(30),
                line: 1,
                is_grabbing: false,
                grab_location: (0, 0),
                window_position: (0, 0),
                extra_width: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Zr Alshaha")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.width = (self.keys.chars().count() / 2) as u32 * self.font_size + 50;

        if self.width >= self.max_width {
            if self.keys.starts_with("...") {
                self.keys.replace_range(3..6, "");
            } else {
                self.keys.replace_range(0..3, "...");
            }
        }

        match message {
            Message::RdevEvents(event) => match event {
                keys::Event::Ready => {
                    #[cfg(debug_assertions)]
                    println!("Ready to recieve!");
                }
                keys::Event::EventRecieved(rdev_event) => {
                    if let rdev::EventType::KeyPress(key) = rdev_event.event_type {
                        return self.add_key(&key, rdev_to_key);
                    }
                }
            },
            Message::IcedEvents(event) => match event {
                #[cfg(not(target_os = "linux"))]
                iced_native::Event::Keyboard(Event::KeyPressed {
                    key_code,
                    modifiers: _,
                }) => {
                    return self.add_key(&key_code, iced_to_key);
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

                    if self.is_grabbing {
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
                self.keys = new_value;
            }
        }
        Command::none()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }

    fn subscription(&self) -> Subscription<Message> {
        let iced_events = subscription::events().map(Message::IcedEvents);
        Subscription::batch(vec![keys::bind().map(Message::RdevEvents), iced_events])
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        container::Container::new(
            Text::new(self.keys.as_str())
                .size(self.font_size as u16)
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

fn image_to_icon() -> Result<Icon, std::io::Error> {
    let (icon_rgba, icon_width, icon_height) = {
        let image =
            image::io::Reader::new(Cursor::new(include_bytes!("../assets/peepoSalute.png")))
                .with_guessed_format()?
                .decode()
                .expect("Failed to open icon path")
                .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    Ok(icon)
}

impl ScreenKey {
    fn add_key<KS>(&mut self, key: &KS, key_to_string: impl Fn(&KS) -> String) -> Command<Message>
    where
        KS: keys::Keys,
    {
        let coming_key = key_to_string(&key);
        
        if self.frequent_key != coming_key {
            self.key_frequency = 0;
            self.frequent_key = "".to_string();
        }

        let frequent_key = format!("{}...x{} ", self.frequent_key, self.key_frequency);

        self.key_frequency += 1;

        self.frequent_key = coming_key.clone();

        let new_frequent_key = format!("{}...x{} ", self.frequent_key, self.key_frequency);

        // add extra width for long keys to fit in the same line
        let coming_key_char_count = coming_key.chars().count();
        
        if  coming_key_char_count > 1 && self.key_frequency < 3 && !self.keys.starts_with("...") {
            self.extra_width += coming_key_char_count as u32 * 2;
        }

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
                    self.width + self.extra_width
                },
                height: self.font_size * self.line,
            },
        ));
    }
}

fn main() -> Result<(), iced::Error> {
    let config_dir = dirs::config_dir().expect("No Config Directory");

    let config_file = config_dir.join("zr-alshasha/config.toml");

    let config_str = std::fs::read_to_string(config_file).unwrap_or_default();

    let config: Config = from_str(&config_str).unwrap_or_else(|e| {
        eprintln!("{e}");

        Config::default()
    });

    let position = config.position.unwrap_or_default();

    let height = config.font_size.unwrap_or(30);

    let settings = Settings {
        window: iced::window::Settings {
            size: (1, height),
            position: Position::Specific(position.x, position.y),
            decorations: false,
            transparent: true,
            always_on_top: true,
            icon: if let Ok(icon) = image_to_icon() {
                Some(icon)
            } else {
                None
            },
            ..Default::default()
        },
        ..Default::default()
    };

    ScreenKey::run(settings)
}
