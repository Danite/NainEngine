use nain_events::{Event, EventCategory, KeyPressedEvent};
use nain_log as log;

pub trait Application {
    fn run(&self) {
        println!("Welcome to the Nain engine!");

        let event = KeyPressedEvent::new(2, 5);

        log::trace!("{}", event);
        log::trace!("{}", event.get_name());

        if event.is_in_category(EventCategory::Keyboard) {
            log::trace!("Event category: {}", stringify!(EventCategory::Keyboard));
        }

        loop {}
    }
}
