use nain_events::{EventCategory, KeyPressedEvent};
use nain_log as log;

pub trait Application {
    fn run(&self) {
        println!("Welcome to the Nain engine!");

        let event = KeyPressedEvent::new(2, 5);
        log::trace!("{}", event);

        loop {}
    }
}
