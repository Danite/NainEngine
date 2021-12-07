use crate::event::{event_category_flags, event_type, Event, EventCategory, EventType};
use std::fmt;

// Mouse moved event

pub struct MouseMovedEvent {
    mouse_x: f32,
    mouse_y: f32,
}

impl Event for MouseMovedEvent {
    event_category_flags!(EventCategory::Mouse);
    event_type!(EventType::MouseMoved);
}

impl fmt::Display for MouseMovedEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formater,
            "MouseMovedEvent: x: {} - y: {}",
            self.mouse_x, self.mouse_y
        )
    }
}

impl MouseMovedEvent {
    pub fn new(mouse_x: f32, mouse_y: f32) -> Self {
        Self { mouse_x, mouse_y }
    }

    pub fn get_x(&self) -> f32 {
        self.mouse_x
    }

    pub fn get_y(&self) -> f32 {
        self.mouse_y
    }
}

// Mouse scrolled event

pub struct MouseScrolledEvent {
    mouse_x_offset: f32,
    mouse_y_offset: f32,
}

impl Event for MouseScrolledEvent {
    event_category_flags!(EventCategory::Mouse);
    event_type!(EventType::MouseScrolled);
}

impl fmt::Display for MouseScrolledEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formater,
            "MouseScrolledEvent: x-offset: {} - y-offset: {}",
            self.get_x_offset(),
            self.get_y_offset()
        )
    }
}

impl MouseScrolledEvent {
    pub fn new(mouse_x_offset: f32, mouse_y_offset: f32) -> Self {
        Self {
            mouse_x_offset,
            mouse_y_offset,
        }
    }

    pub fn get_x_offset(&self) -> f32 {
        self.mouse_x_offset
    }

    pub fn get_y_offset(&self) -> f32 {
        self.mouse_y_offset
    }
}

// Mouse button events

trait MouseButton: Event {
    fn get_mouse_button(&self) -> u32;
}

// Mouse button pressed event

pub struct MouseButtonPressedEvent {
    button: u32,
}

impl Event for MouseButtonPressedEvent {
    event_category_flags!(EventCategory::Keyboard);
    event_type!(EventType::MouseButtonPressed);
}

impl MouseButton for MouseButtonPressedEvent {
    fn get_mouse_button(&self) -> u32 {
        self.button
    }
}

impl fmt::Display for MouseButtonPressedEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "MouseButtonPressedEvent: {}", self.button)
    }
}

impl MouseButtonPressedEvent {
    pub fn new(button: u32) -> Self {
        Self { button }
    }
}

// Mouse button released event

pub struct MouseButtonReleasedEvent {
    button: u32,
}

impl Event for MouseButtonReleasedEvent {
    event_category_flags!(EventCategory::Keyboard);
    event_type!(EventType::MouseButtonReleased);
}

impl MouseButton for MouseButtonReleasedEvent {
    fn get_mouse_button(&self) -> u32 {
        self.button
    }
}

impl fmt::Display for MouseButtonReleasedEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "MouseButtonReleasedEvent: {}", self.button)
    }
}

impl MouseButtonReleasedEvent {
    pub fn new(button: u32) -> Self {
        Self { button }
    }
}
