use crate::event::{event_category_flags, event_type, Event, EventCategory, EventType};
use std::fmt;

trait KeyEvent: Event {
    fn get_key_code(&self) -> u32;
}

// Key pressed event

pub struct KeyPressedEvent {
    key_code: u32,
    repeat_count: u32,
}

impl Event for KeyPressedEvent {
    event_category_flags!(EventCategory::Keyboard);
    event_type!(EventType::KeyPressed);
}

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> u32 {
        self.key_code
    }
}

impl fmt::Display for KeyPressedEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formater,
            "KeyPressedEvent: {} ({} repeats)",
            self.key_code, self.repeat_count
        )
    }
}

impl KeyPressedEvent {
    pub fn new(key_code: u32, repeat_count: u32) -> KeyPressedEvent {
        Self {
            key_code,
            repeat_count,
        }
    }

    pub fn get_repeat_count(&self) -> u32 {
        self.repeat_count
    }
}
