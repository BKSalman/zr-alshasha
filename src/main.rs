use iced::{
    executor, keyboard::Event, widget::container, Application, Command, Element, Settings,
    Subscription,
};
use iced_native::{subscription, widget::Text};

use crate::keys::{iced_to_key, rdev_to_key};

mod keys;

#[derive(Default)]
struct ScreenKey {
    keys: String,
    max_characters: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    RdevEvents(keys::Event),
    IcedEvents(iced_native::Event),
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
        String::from("Zr Alshaha")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.max_characters = 20;
        match message {
            Message::RdevEvents(event) => match event {
                keys::Event::Ready => {
                    // println!("Ready to recieve!");
                }
                keys::Event::EventRecieved(rdev_event) => match rdev_event.event_type {
                    rdev::EventType::KeyPress(key) => {
                        if self.keys.len() > self.max_characters {
                            let (_oldest_key, splitted_content) = self.keys.split_once(" ").unwrap();
                            self.keys = splitted_content.to_string();
                        }
                        self.keys = format!("{} {}", self.keys, rdev_to_key(&key));
                        // println!("{key:?}");
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
                    if self.keys.len() > self.max_characters {
                        let (_oldest_key, splitted_content) = self.keys.split_once(" ").unwrap();
                        self.keys = splitted_content.to_string();
                    }
                    self.keys = format!("{} {}", self.keys, iced_to_key(&key_code));
                    // println!("{key_code:?}");
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
        container::Container::new(Text::new(self.keys.as_str()))
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
            resizable: false,
            decorations: false,
            transparent: true,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    };
    ScreenKey::run(settings)
}
