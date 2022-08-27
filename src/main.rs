// #![windows_subsystem = "windows"]
use iced::{
    executor, keyboard::Event, widget::container, Application, Command, Element, Settings,
    Subscription, Font, container::{StyleSheet, Style}, Background, Color,
};
use iced_native::{subscription, widget::Text};

use crate::keys::{iced_to_key, rdev_to_key};

mod keys;

#[derive(Default)]
struct ScreenKey {
    keys: String,
    max_characters: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    RdevEvents(keys::Event),
    IcedEvents(iced_native::Event),
    InputChanged(String),
}

const FONT: Font = Font::External {
    name: "Nerd Font",
    bytes: include_bytes!("../../screenkey/fonts/Fura Code Bold Nerd Font Complete Mono.ttf"),
};

const WIDTH: u32 = 400;

struct ContainerStyles;


impl StyleSheet for ContainerStyles {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
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
        self.max_characters = (WIDTH / 10)/2;
        match message {
            Message::RdevEvents(event) => match event {
                keys::Event::Ready => {
                    println!("Ready to recieve!");
                }
                keys::Event::EventRecieved(rdev_event) => match rdev_event.event_type {
                    rdev::EventType::KeyPress(key) => {
                        if self.keys.chars().count() > self.max_characters as usize {
                            let (_oldest_key, splitted_content) = self.keys.split_once(" ").unwrap();
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
                _ => {}
            },
            Message::InputChanged(new_value) => {
                self.keys = String::from(new_value);
            }
            // _ => {}
        }
        Command::none()
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
                Text::new(
                    self.keys.as_str()
                )
                    .font(FONT)
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .style(ContainerStyles)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: iced::window::Settings {
            size: (WIDTH, 20),
            resizable: false,
            // decorations: false,
            transparent: true,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    };
    ScreenKey::run(settings)
}
