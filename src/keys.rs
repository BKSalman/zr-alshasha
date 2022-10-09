use iced::{
    futures::{channel::mpsc, FutureExt, SinkExt, StreamExt, TryFutureExt},
    Subscription,
};

#[cfg(not(target_os = "linux"))]
use iced::keyboard::KeyCode;

use iced_native::subscription;
use rdev::listen;

pub trait Keys {
    
}

#[cfg(not(target_os = "linux"))]
impl Keys for KeyCode {
    
}

impl Keys for rdev::Key {
    
}

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

#[cfg(not(target_os = "linux"))]
pub fn iced_to_key(key_code: &KeyCode) -> String {
    match key_code {
        KeyCode::Key1 => "1".to_string(),
        KeyCode::Key2 => "2".to_string(),
        KeyCode::Key3 => "3".to_string(),
        KeyCode::Key4 => "4".to_string(),
        KeyCode::Key5 => "5".to_string(),
        KeyCode::Key6 => "6".to_string(),
        KeyCode::Key7 => "7".to_string(),
        KeyCode::Key8 => "8".to_string(),
        KeyCode::Key9 => "9".to_string(),
        KeyCode::Key0 => "0".to_string(),

        KeyCode::A => "A".to_string(),
        KeyCode::B => "B".to_string(),
        KeyCode::C => "C".to_string(),
        KeyCode::D => "D".to_string(),
        KeyCode::E => "E".to_string(),
        KeyCode::F => "F".to_string(),
        KeyCode::G => "G".to_string(),
        KeyCode::H => "H".to_string(),
        KeyCode::I => "I".to_string(),
        KeyCode::J => "J".to_string(),
        KeyCode::K => "K".to_string(),
        KeyCode::L => "L".to_string(),
        KeyCode::M => "M".to_string(),
        KeyCode::N => "N".to_string(),
        KeyCode::O => "O".to_string(),
        KeyCode::P => "P".to_string(),
        KeyCode::Q => "Q".to_string(),
        KeyCode::R => "R".to_string(),
        KeyCode::S => "S".to_string(),
        KeyCode::T => "T".to_string(),
        KeyCode::U => "U".to_string(),
        KeyCode::V => "V".to_string(),
        KeyCode::W => "W".to_string(),
        KeyCode::X => "X".to_string(),
        KeyCode::Y => "Y".to_string(),
        KeyCode::Z => "Z".to_string(),

        KeyCode::Apostrophe => "'".to_string(),
        KeyCode::Backslash => "\\".to_string(),
        KeyCode::Colon => ":".to_string(),
        KeyCode::Comma => ".to_string(),".to_string(),
        KeyCode::Equals => "=".to_string(),
        KeyCode::LBracket => "[".to_string(),
        KeyCode::Minus => "-".to_string(),
        KeyCode::Period => ".".to_string(),
        KeyCode::Plus => "+".to_string(),
        KeyCode::RBracket => "]".to_string(),
        KeyCode::Semicolon => ";".to_string(),
        KeyCode::Slash => "/".to_string(),

        KeyCode::Escape => "esc".to_string(),

        KeyCode::F1 => "F1".to_string(),
        KeyCode::F2 => "F2".to_string(),
        KeyCode::F3 => "F3".to_string(),
        KeyCode::F4 => "F4".to_string(),
        KeyCode::F5 => "F5".to_string(),
        KeyCode::F6 => "F6".to_string(),
        KeyCode::F7 => "F7".to_string(),
        KeyCode::F8 => "F8".to_string(),
        KeyCode::F9 => "F9".to_string(),
        KeyCode::F10 => "F10".to_string(),
        KeyCode::F11 => "F11".to_string(),
        KeyCode::F12 => "F12".to_string(),
        KeyCode::F13 => "F13".to_string(),
        KeyCode::F14 => "F14".to_string(),
        KeyCode::F15 => "F15".to_string(),
        KeyCode::F16 => "F16".to_string(),
        KeyCode::F17 => "F17".to_string(),
        KeyCode::F18 => "F18".to_string(),
        KeyCode::F19 => "F19".to_string(),
        KeyCode::F20 => "F20".to_string(),
        KeyCode::F21 => "F21".to_string(),
        KeyCode::F22 => "F22".to_string(),
        KeyCode::F23 => "F23".to_string(),
        KeyCode::F24 => "F24".to_string(),

        KeyCode::Snapshot => "Snapshot".to_string(),
        KeyCode::Scroll => "Scroll".to_string(),
        KeyCode::Pause => "Pause".to_string(),

        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Home => HOME.to_string(),
        KeyCode::Delete => DELETE.to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),

        KeyCode::Left => LEFT_ARROW.to_string(),
        KeyCode::Up => UP_ARROW.to_string(),
        KeyCode::Right => RIGHT_ARROW.to_string(),
        KeyCode::Down => DOWN_ARROW.to_string(),

        KeyCode::Backspace => "⌫".to_string(),
        KeyCode::Enter => ENTER.to_string(),
        KeyCode::Space => "_".to_string(),

        KeyCode::Compose => "Compose".to_string(),

        KeyCode::Caret => "Caret".to_string(),

        KeyCode::Numlock => "Numlock".to_string(),
        KeyCode::Numpad0 => "0".to_string(),
        KeyCode::Numpad1 => "1".to_string(),
        KeyCode::Numpad2 => "2".to_string(),
        KeyCode::Numpad3 => "3".to_string(),
        KeyCode::Numpad4 => "4".to_string(),
        KeyCode::Numpad5 => "5".to_string(),
        KeyCode::Numpad6 => "6".to_string(),
        KeyCode::Numpad7 => "7".to_string(),
        KeyCode::Numpad8 => "8".to_string(),
        KeyCode::Numpad9 => "9".to_string(),
        KeyCode::NumpadAdd => "+".to_string(),
        KeyCode::NumpadDivide => "/".to_string(),
        KeyCode::NumpadDecimal => ".".to_string(),
        KeyCode::NumpadComma => ".to_string(),".to_string(),
        KeyCode::NumpadEnter => ENTER.to_string(),
        KeyCode::NumpadEquals => "=".to_string(),
        KeyCode::NumpadMultiply => "*".to_string(),
        KeyCode::NumpadSubtract => "-".to_string(),

        // KeyCode::AbntC1 => "Alo".to_string(),
        // KeyCode::AbntC2 => "Alo".to_string(),
        // KeyCode::Apps.to_string(),
        KeyCode::Asterisk => "*".to_string(),
        KeyCode::At => "@".to_string(),
        // KeyCode::Ax => "Alo".to_string(),
        // KeyCode::Calculator.to_string(),
        KeyCode::Capital => "Caps".to_string(),
        KeyCode::Convert => "Convert".to_string(),
        KeyCode::Grave => "`".to_string(),
        KeyCode::Kana => "Kana".to_string(),
        KeyCode::Kanji => "Kanji".to_string(),
        KeyCode::LAlt => "Alt".to_string(),
        KeyCode::LControl => CONTROL.to_string(),
        KeyCode::LShift => SHIFT.to_string(),
        KeyCode::LWin => "Win".to_string(),
        // KeyCode::Mail.to_string(),
        // KeyCode::MediaSelect.to_string(),
        // KeyCode::MediaStop.to_string(),
        // KeyCode::Mute.to_string(),
        // KeyCode::MyComputer.to_string(),
        // KeyCode::NavigateForward.to_string(),  // also called "Next"
        // KeyCode::NavigateBackward.to_string(), // also called "Prior"
        // KeyCode::NextTrack.to_string(),
        // KeyCode::NoConvert.to_string(),
        KeyCode::OEM102 => "OEM102".to_string(),
        // KeyCode::PlayPause.to_string(),
        // KeyCode::Power.to_string(),
        // KeyCode::PrevTrack.to_string(),
        KeyCode::RAlt => "Alt".to_string(),
        KeyCode::RControl => CONTROL.to_string(),
        KeyCode::RShift => SHIFT.to_string(),
        KeyCode::RWin => "Win".to_string(),
        // KeyCode::Sleep.to_string(),
        // KeyCode::Stop.to_string(),
        KeyCode::Sysrq => "Sysrq".to_string(),
        KeyCode::Tab => TAB.to_string(),
        KeyCode::Underline => "Underline".to_string(),
        KeyCode::Unlabeled => "Unlabeled".to_string(),
        KeyCode::VolumeDown => "VolumeDown".to_string(),
        KeyCode::VolumeUp => "VolumeUp".to_string(),
        KeyCode::Wake => "Wake".to_string(),
        KeyCode::WebBack => "WebBack".to_string(),
        KeyCode::WebFavorites => "WebFav".to_string(),
        KeyCode::WebForward => "WebForward".to_string(),
        KeyCode::WebHome => "WebHome".to_string(),
        KeyCode::WebRefresh => "WebRefresh".to_string(),
        KeyCode::WebSearch => "WebSearch".to_string(),
        KeyCode::WebStop => "WebStop".to_string(),
        KeyCode::Yen => "Yen".to_string(),
        KeyCode::Copy => "Copy".to_string(),
        KeyCode::Paste => "Paste".to_string(),
        KeyCode::Cut => "Cut".to_string(),
        _ => "?".to_string(),
    }
}

pub fn rdev_to_key(key_code: &rdev::Key) -> String {
    match key_code {
        rdev::Key::Alt => "Alt".to_string(),
        rdev::Key::AltGr => "Alt".to_string(),
        rdev::Key::Backspace => "⌫".to_string(),
        rdev::Key::CapsLock => "Caps".to_string(),
        rdev::Key::ControlLeft => CONTROL.to_string(),
        rdev::Key::ControlRight => CONTROL.to_string(),
        rdev::Key::Delete => DELETE.to_string(),
        rdev::Key::End => "End".to_string(),
        rdev::Key::Escape => "esc".to_string(),
        rdev::Key::F1 => "F1".to_string(),
        rdev::Key::F2 => "F2".to_string(),
        rdev::Key::F3 => "F3".to_string(),
        rdev::Key::F4 => "F4".to_string(),
        rdev::Key::F5 => "F5".to_string(),
        rdev::Key::F6 => "F6".to_string(),
        rdev::Key::F7 => "F7".to_string(),
        rdev::Key::F8 => "F8".to_string(),
        rdev::Key::F9 => "F9".to_string(),
        rdev::Key::F10 => "F10".to_string(),
        rdev::Key::F11 => "F11".to_string(),
        rdev::Key::F12 => "F12".to_string(),
        rdev::Key::Home => HOME.to_string(),
        rdev::Key::LeftArrow => LEFT_ARROW.to_string(),
        rdev::Key::RightArrow => RIGHT_ARROW.to_string(),
        rdev::Key::UpArrow => UP_ARROW.to_string(),
        rdev::Key::DownArrow => DOWN_ARROW.to_string(),
        rdev::Key::MetaLeft => "Win".to_string(), // TODO: make it platform dependant
        rdev::Key::MetaRight => "Win".to_string(), // TODO: make it platform dependant
        rdev::Key::PageDown => "PageDown".to_string(),
        rdev::Key::PageUp => "PageUp".to_string(),
        rdev::Key::Return => ENTER.to_string(),
        rdev::Key::ShiftLeft => SHIFT.to_string(),
        rdev::Key::ShiftRight => SHIFT.to_string(),
        rdev::Key::Space => "_".to_string(),
        rdev::Key::Tab => TAB.to_string(),
        rdev::Key::PrintScreen => "PrtSc".to_string(),
        rdev::Key::ScrollLock => "ScrLk".to_string(),
        rdev::Key::Pause => "Pause".to_string(),
        rdev::Key::NumLock => "NumLock".to_string(),
        rdev::Key::BackQuote => "`".to_string(),
        rdev::Key::Num1 => "1".to_string(),
        rdev::Key::Num2 => "2".to_string(),
        rdev::Key::Num3 => "3".to_string(),
        rdev::Key::Num4 => "4".to_string(),
        rdev::Key::Num5 => "5".to_string(),
        rdev::Key::Num6 => "6".to_string(),
        rdev::Key::Num7 => "7".to_string(),
        rdev::Key::Num8 => "8".to_string(),
        rdev::Key::Num9 => "9".to_string(),
        rdev::Key::Num0 => "0".to_string(),
        rdev::Key::Minus => "-".to_string(),
        rdev::Key::Equal => "=".to_string(),
        rdev::Key::KeyQ => "Q".to_string(),
        rdev::Key::KeyW => "W".to_string(),
        rdev::Key::KeyE => "E".to_string(),
        rdev::Key::KeyR => "R".to_string(),
        rdev::Key::KeyT => "T".to_string(),
        rdev::Key::KeyY => "Y".to_string(),
        rdev::Key::KeyU => "U".to_string(),
        rdev::Key::KeyI => "I".to_string(),
        rdev::Key::KeyO => "O".to_string(),
        rdev::Key::KeyP => "P".to_string(),
        rdev::Key::LeftBracket => "[".to_string(),
        rdev::Key::RightBracket => "]".to_string(),
        rdev::Key::KeyA => "A".to_string(),
        rdev::Key::KeyS => "S".to_string(),
        rdev::Key::KeyD => "D".to_string(),
        rdev::Key::KeyF => "F".to_string(),
        rdev::Key::KeyG => "G".to_string(),
        rdev::Key::KeyH => "H".to_string(),
        rdev::Key::KeyJ => "J".to_string(),
        rdev::Key::KeyK => "K".to_string(),
        rdev::Key::KeyL => "L".to_string(),
        rdev::Key::SemiColon => ";".to_string(),
        rdev::Key::Quote => "'".to_string(),
        rdev::Key::BackSlash => "\\".to_string(),
        rdev::Key::IntlBackslash => "|".to_string(),
        rdev::Key::KeyZ => "Z".to_string(),
        rdev::Key::KeyX => "X".to_string(),
        rdev::Key::KeyC => "C".to_string(),
        rdev::Key::KeyV => "V".to_string(),
        rdev::Key::KeyB => "B".to_string(),
        rdev::Key::KeyN => "N".to_string(),
        rdev::Key::KeyM => "M".to_string(),
        rdev::Key::Comma => ",".to_string(),
        rdev::Key::Dot => ".".to_string(),
        rdev::Key::Slash => "/".to_string(),
        rdev::Key::Insert => "Ins".to_string(),
        rdev::Key::KpReturn => "⏎".to_string(),
        rdev::Key::KpMinus => "-".to_string(),
        rdev::Key::KpPlus => "+".to_string(),
        rdev::Key::KpMultiply => "*".to_string(),
        rdev::Key::KpDivide => "/".to_string(),
        rdev::Key::Kp0 => "0".to_string(),
        rdev::Key::Kp1 => "1".to_string(),
        rdev::Key::Kp2 => "2".to_string(),
        rdev::Key::Kp3 => "3".to_string(),
        rdev::Key::Kp4 => "4".to_string(),
        rdev::Key::Kp5 => "5".to_string(),
        rdev::Key::Kp6 => "6".to_string(),
        rdev::Key::Kp7 => "7".to_string(),
        rdev::Key::Kp8 => "8".to_string(),
        rdev::Key::Kp9 => "9".to_string(),
        rdev::Key::KpDelete => DELETE.to_string(),
        rdev::Key::Function => "Function".to_string(),
        rdev::Key::Unknown(unknown) => unknown_to_key(unknown),
    }
}

fn unknown_to_key(unknown: &u32) -> String {
    println!("{unknown}");
    format!("{unknown}")
}
