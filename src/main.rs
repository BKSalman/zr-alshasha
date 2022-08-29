// #![windows_subsystem = "windows"]
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
use crate::keys::{iced_to_key, rdev_to_key};

mod keys;

#[derive(Default)]
struct ScreenKey {
    keys: String,
    max_characters: u32,
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

const WIDTH: u32 = 400;

const HEIGHT: u16 = 30;

struct ContainerStyles;

impl StyleSheet for ContainerStyles {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color(Color::BLACK)),
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
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Zr Alshaha")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.max_characters = (WIDTH / 10) / 2;
        match message {
            Message::RdevEvents(event) => match event {
                keys::Event::Ready => {
                    println!("Ready to recieve!");
                }
                keys::Event::EventRecieved(rdev_event) => match rdev_event.event_type {
                    rdev::EventType::KeyPress(key) => {
                        if self.keys.chars().count() > self.max_characters as usize {
                            let (_oldest_key, splitted_content) =
                                self.keys.split_once(" ").unwrap();
                            self.keys = splitted_content.to_string();
                        }
                        self.keys = format!("{} {}", self.keys, rdev_to_key(&key));
                        return Command::none();
                    }
                    _ => {}
                },
            },
            Message::IcedEvents(event) => match event {
                iced_native::Event::Keyboard(Event::KeyPressed {
                    key_code,
                    modifiers,
                }) => {
                    if self.keys.chars().count() > self.max_characters as usize {
                        let (_oldest_key, splitted_content) = self.keys.split_once(" ").unwrap();
                        self.keys = splitted_content.to_string();
                    }
                    self.keys = format!("{} {}", self.keys, iced_to_key(&key_code));
                    return Command::none();
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
                        return window::move_to(
                            x,
                            y
                        );
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
                .color(Color::WHITE)
                .font(FONT),
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
            size: (WIDTH, HEIGHT.into()),
            resizable: false,
            position: Position::Specific(100, 100),
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
