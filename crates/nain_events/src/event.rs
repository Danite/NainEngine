use std::fmt::Display;

pub enum EventType {
    None,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

#[derive(PartialEq)]
pub enum EventCategory {
    None,
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

pub trait Event: Display + 'static {
    fn get_type(&self) -> EventType;
    fn get_name(&self) -> String;
    fn get_category_flags(&self) -> EventCategory;
    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() == category
    }
}

#[macro_export]
macro_rules! event_category_flags {
    ($category:expr) => {
        fn get_category_flags(&self) -> EventCategory {
            $category
        }
    };
}

pub(crate) use event_category_flags;

#[macro_export]
macro_rules! event_type {
    ($type:expr) => {
        fn get_type(&self) -> EventType {
            $type
        }

        fn get_name(&self) -> String {
            stringify!($type).to_string()
        }
    };
}

pub(crate) use event_type;
