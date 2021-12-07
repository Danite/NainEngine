use crate::event::{event_category_flags, event_type, Event, EventCategory, EventType};
use std::fmt;

// Window resize event

pub struct WindowResizeEvent {
    width: u32,
    height: u32,
}

impl Event for WindowResizeEvent {
    event_category_flags!(EventCategory::Application);
    event_type!(EventType::WindowResize);
}

impl fmt::Display for WindowResizeEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formater,
            "WindowResizeEvent: {} - {}",
            self.width, self.height
        )
    }
}

impl WindowResizeEvent {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

// Window close event

pub struct WindowCloseEvent;

impl Event for WindowCloseEvent {
    event_category_flags!(EventCategory::Application);
    event_type!(EventType::WindowClose);
}

impl fmt::Display for WindowCloseEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "WindowCloseEvent")
    }
}

// App tick event

pub struct AppTickEvent;

impl Event for AppTickEvent {
    event_category_flags!(EventCategory::Application);
    event_type!(EventType::AppTick);
}

impl fmt::Display for AppTickEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "AppTickEvent")
    }
}

// App update event

pub struct AppUpdateEvent;

impl Event for AppUpdateEvent {
    event_category_flags!(EventCategory::Application);
    event_type!(EventType::AppUpdate);
}

impl fmt::Display for AppUpdateEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "AppUpdateEvent")
    }
}

// App render event

pub struct AppRenderEvent;

impl Event for AppRenderEvent {
    event_category_flags!(EventCategory::Application);
    event_type!(EventType::AppRender);
}

impl fmt::Display for AppRenderEvent {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        write!(formater, "AppRenderEvent")
    }
}
