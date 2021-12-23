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

    pub fn run(&mut self) {
        log::info!("Welcome to the Nain engine!");

        while self.is_running {
            self.window.on_update();
        }
    }
}
