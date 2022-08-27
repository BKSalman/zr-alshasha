use iced::{
    futures::{channel::mpsc, FutureExt, SinkExt, StreamExt, TryFutureExt},
    keyboard::KeyCode,
    Subscription,
};
use iced_native::subscription;
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

const TAB: &str = "";
const ENTER: &str = "⏎";
const SHIFT: &str = "וּ";
const LEFT_ARROW: &str = "ﰯ";
const RIGHT_ARROW: &str = "ﰲ";
const UP_ARROW: &str = "ﰵ";
const DOWN_ARROW: &str = "ﰬ";
const CONTROL: &str = "דּ";
const DELETE: &str = "﫧";
const HOME: &str = "";

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

pub fn iced_to_key(key_code: &KeyCode) -> &str {
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
        KeyCode::Home => HOME,
        KeyCode::Delete => DELETE,
        KeyCode::End => "End",
        KeyCode::PageDown => "PageDown",
        KeyCode::PageUp => "PageUp",

        KeyCode::Left => LEFT_ARROW,
        KeyCode::Up => UP_ARROW,
        KeyCode::Right => RIGHT_ARROW,
        KeyCode::Down => DOWN_ARROW,

        KeyCode::Backspace => "⌫",
        KeyCode::Enter => ENTER,
        KeyCode::Space => "_",

        KeyCode::Compose => "Compose",

        KeyCode::Caret => "Caret",

        KeyCode::Numlock => "Numlock",
        KeyCode::Numpad0 => "0",
        KeyCode::Numpad1 => "1",
        KeyCode::Numpad2 => "2",
        KeyCode::Numpad3 => "3",
        KeyCode::Numpad4 => "4",
        KeyCode::Numpad5 => "5",
        KeyCode::Numpad6 => "6",
        KeyCode::Numpad7 => "7",
        KeyCode::Numpad8 => "8",
        KeyCode::Numpad9 => "9",
        KeyCode::NumpadAdd => "+",
        KeyCode::NumpadDivide => "/",
        KeyCode::NumpadDecimal => ".",
        KeyCode::NumpadComma => ",",
        KeyCode::NumpadEnter => ENTER,
        KeyCode::NumpadEquals => "=",
        KeyCode::NumpadMultiply => "*",
        KeyCode::NumpadSubtract => "-",

        // KeyCode::AbntC1 => "Alo",
        // KeyCode::AbntC2 => "Alo",
        // KeyCode::Apps,
        KeyCode::Asterisk => "*",
        KeyCode::At => "@",
        // KeyCode::Ax => "Alo",
        // KeyCode::Calculator,
        KeyCode::Capital => "Caps",
        KeyCode::Convert => "Convert",
        KeyCode::Grave => "`",
        KeyCode::Kana => "Kana",
        KeyCode::Kanji => "Kanji",
        KeyCode::LAlt => "Alt",
        KeyCode::LControl => CONTROL,
        KeyCode::LShift => SHIFT,
        KeyCode::LWin => "Win",
        // KeyCode::Mail,
        // KeyCode::MediaSelect,
        // KeyCode::MediaStop,
        // KeyCode::Mute,
        // KeyCode::MyComputer,
        // KeyCode::NavigateForward,  // also called "Next"
        // KeyCode::NavigateBackward, // also called "Prior"
        // KeyCode::NextTrack,
        // KeyCode::NoConvert,
        KeyCode::OEM102 => "OEM102",
        // KeyCode::PlayPause,
        // KeyCode::Power,
        // KeyCode::PrevTrack,
        KeyCode::RAlt => "Alt",
        KeyCode::RControl => CONTROL,
        KeyCode::RShift => SHIFT,
        KeyCode::RWin => "Win",
        // KeyCode::Sleep,
        // KeyCode::Stop,
        KeyCode::Sysrq => "Sysrq",
        KeyCode::Tab => TAB,
        KeyCode::Underline => "Underline",
        KeyCode::Unlabeled => "Unlabeled",
        KeyCode::VolumeDown => "VolumeDown",
        KeyCode::VolumeUp => "VolumeUp",
        KeyCode::Wake => "Wake",
        KeyCode::WebBack => "WebBack",
        KeyCode::WebFavorites => "WebFav",
        KeyCode::WebForward => "WebForward",
        KeyCode::WebHome => "WebHome",
        KeyCode::WebRefresh => "WebRefresh",
        KeyCode::WebSearch => "WebSearch",
        KeyCode::WebStop => "WebStop",
        KeyCode::Yen => "Yen",
        KeyCode::Copy => "Copy",
        KeyCode::Paste => "Paste",
        KeyCode::Cut => "Cut",
        _ => "?",
    }
}

pub fn rdev_to_key(key_code: &rdev::Key) -> &str {
    match key_code {
        rdev::Key::Alt => "Alt",
        rdev::Key::AltGr => "Alt",
        rdev::Key::Backspace => "⌫",
        rdev::Key::CapsLock => "Caps",
        rdev::Key::ControlLeft => CONTROL,
        rdev::Key::ControlRight => CONTROL,
        rdev::Key::Delete => DELETE,
        rdev::Key::End => "End",
        rdev::Key::Escape => "esc",
        rdev::Key::F1 => "F1",
        rdev::Key::F2 => "F2",
        rdev::Key::F3 => "F3",
        rdev::Key::F4 => "F4",
        rdev::Key::F5 => "F5",
        rdev::Key::F6 => "F6",
        rdev::Key::F7 => "F7",
        rdev::Key::F8 => "F8",
        rdev::Key::F9 => "F9",
        rdev::Key::F10 => "F10",
        rdev::Key::F11 => "F11",
        rdev::Key::F12 => "F12",
        rdev::Key::Home => HOME,
        rdev::Key::LeftArrow => LEFT_ARROW,
        rdev::Key::RightArrow => RIGHT_ARROW,
        rdev::Key::UpArrow => UP_ARROW,
        rdev::Key::DownArrow => DOWN_ARROW,
        rdev::Key::MetaLeft => "Win", // TODO: make it platform dependant
        rdev::Key::MetaRight => "Win", // TODO: make it platform dependant
        rdev::Key::PageDown => "PageDown",
        rdev::Key::PageUp => "PageUp",
        rdev::Key::Return => ENTER,
        rdev::Key::ShiftLeft => SHIFT,
        rdev::Key::ShiftRight => SHIFT,
        rdev::Key::Space => "_",
        rdev::Key::Tab => TAB,
        rdev::Key::PrintScreen => "PrtSc",
        rdev::Key::ScrollLock => "ScrLk",
        rdev::Key::Pause => "Pause",
        rdev::Key::NumLock => "NumLock",
        rdev::Key::BackQuote => "`",
        rdev::Key::Num1 => "1",
        rdev::Key::Num2 => "2",
        rdev::Key::Num3 => "3",
        rdev::Key::Num4 => "4",
        rdev::Key::Num5 => "5",
        rdev::Key::Num6 => "6",
        rdev::Key::Num7 => "7",
        rdev::Key::Num8 => "8",
        rdev::Key::Num9 => "9",
        rdev::Key::Num0 => "0",
        rdev::Key::Minus => "-",
        rdev::Key::Equal => "=",
        rdev::Key::KeyQ => "Q",
        rdev::Key::KeyW => "W",
        rdev::Key::KeyE => "E",
        rdev::Key::KeyR => "R",
        rdev::Key::KeyT => "T",
        rdev::Key::KeyY => "Y",
        rdev::Key::KeyU => "U",
        rdev::Key::KeyI => "I",
        rdev::Key::KeyO => "O",
        rdev::Key::KeyP => "P",
        rdev::Key::LeftBracket => "[",
        rdev::Key::RightBracket => "]",
        rdev::Key::KeyA => "A",
        rdev::Key::KeyS => "S",
        rdev::Key::KeyD => "D",
        rdev::Key::KeyF => "F",
        rdev::Key::KeyG => "G",
        rdev::Key::KeyH => "H",
        rdev::Key::KeyJ => "J",
        rdev::Key::KeyK => "K",
        rdev::Key::KeyL => "L",
        rdev::Key::SemiColon => ";",
        rdev::Key::Quote => "'",
        rdev::Key::BackSlash => "\\",
        rdev::Key::IntlBackslash => "|",
        rdev::Key::KeyZ => "Z",
        rdev::Key::KeyX => "X",
        rdev::Key::KeyC => "C",
        rdev::Key::KeyV => "V",
        rdev::Key::KeyB => "B",
        rdev::Key::KeyN => "N",
        rdev::Key::KeyM => "M",
        rdev::Key::Comma => ",",
        rdev::Key::Dot => ".",
        rdev::Key::Slash => "/",
        rdev::Key::Insert => "Ins",
        rdev::Key::KpReturn => "⏎",
        rdev::Key::KpMinus => "KpMinus",
        rdev::Key::KpPlus => "KpPlus",
        rdev::Key::KpMultiply => "KpMultiply",
        rdev::Key::KpDivide => "KpDivide",
        rdev::Key::Kp0 => "Kp0",
        rdev::Key::Kp1 => "Kp1",
        rdev::Key::Kp2 => "Kp2",
        rdev::Key::Kp3 => "Kp3",
        rdev::Key::Kp4 => "Kp4",
        rdev::Key::Kp5 => "Kp5",
        rdev::Key::Kp6 => "Kp6",
        rdev::Key::Kp7 => "Kp7",
        rdev::Key::Kp8 => "Kp8",
        rdev::Key::Kp9 => "Kp9",
        rdev::Key::KpDelete => DELETE,
        rdev::Key::Function => "Function",
        rdev::Key::Unknown(unknown) => unknown_to_key(unknown),
    }
}

fn unknown_to_key(unknown: &u32) -> &str {
    println!("{unknown}");
    "alo"
}
