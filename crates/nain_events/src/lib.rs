pub use self::{
    application_event::{WindowResizeEvent, WindowCloseEvent},
    event::{Event, EventCategory, EventType},
    key_event::{KeyPressedEvent, KeyReleasedEvent},
    mouse_event::{
        MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrolledEvent,
    },
};

mod application_event;
mod event;
mod key_event;
mod mouse_event;
