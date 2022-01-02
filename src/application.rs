use crate::events::{dispatch_event, subscribe_event, Event, EventBus, KeyPressedEvent};
use crate::log;
use crate::window::{Window, WindowProps, WindowTrait};

pub struct Application {
    window: Window<'static>,
    is_running: bool,
}

impl Application {
    pub fn new() -> Self {
        Self {
            window: Window::create(WindowProps::default()),
            is_running: true,
        }
    }

    fn on_event(&self, event: &dyn Event) {
        log::info!("event: {}", event);
    }

    pub fn run(&mut self) {
        log::info!("Welcome to the Nain engine!");

        let mut event = KeyPressedEvent::new(1, 0);

        dispatch_event("nain_engine", &mut event);

        while self.is_running {
            self.window.on_update();
        }
    }
}
