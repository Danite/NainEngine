use crate::log;
use crate::window::{Window, WindowProps, WindowTrait};

pub struct Application<'a> {
    window: Window<'a>,
    is_running: bool,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        Self {
            window: Window::create(WindowProps {
                title: "Nain Engine",
                width: 1280,
                height: 720,
            }),
            is_running: true,
        }
    }

    pub fn run(&mut self) {
        log::info!("Welcome to the Nain engine!");

        while self.is_running {
            self.window.on_update();
        }
    }
}
