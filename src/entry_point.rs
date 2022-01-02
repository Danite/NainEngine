use crate::application::Application;
use crate::log;
use std::error::Error;
use crate::events::{Event, KeyPressedEvent, EventBus, dispatch_event, subscribe_event};

pub struct CreateApplication;

impl CreateApplication {
    pub fn new(application: &mut Application) -> Result<(), Box<dyn Error>> {
        log::init()?;
        let event_bus = EventBus::new("nain_engine");

        subscribe_event("nain_engine", nain, 0);

        application.run();

        Ok(())
    }
}

fn nain(event: &mut dyn Event) {
    println!("hahahahahahahah {}", event);
}