use crate::{WindowProps, WindowTrait};
use glfw::{Action, Context, SwapInterval};
use nain_events::{
    KeyPressedEvent, KeyReleasedEvent, MouseButtonPressedEvent, MouseButtonReleasedEvent,
    MouseMovedEvent, MouseScrolledEvent, WindowCloseEvent, WindowResizeEvent,
};
use nain_log as log;
use std::cell::Cell;

struct WindowData<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    vsync: bool,
}

pub struct WindowsWindow<'a> {
    window: glfw::Window,
    data: WindowData<'a>,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl<'a> WindowTrait for WindowsWindow<'a> {
    fn create(props: WindowProps) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let mut windows_window = WindowsWindow::init(props, glfw);

        windows_window
            .window
            .glfw
            .set_error_callback(Some(glfw::Callback {
                f: error_callback,
                data: Cell::new(0),
            }));

        windows_window
    }

    fn get_width(&self) -> u32 {
        self.data.width
    }

    fn get_height(&self) -> u32 {
        self.data.height
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.window.glfw.set_swap_interval(SwapInterval::Sync(1))
        } else {
            self.window.glfw.set_swap_interval(SwapInterval::None)
        }

        self.data.vsync = enabled;
    }

    fn is_vsync_enabled(&self) -> bool {
        self.data.vsync
    }

    fn on_update(&mut self) {
        self.window.glfw.poll_events();
        self.window.swap_buffers();

        let mut events = vec![];
        for (_, event) in glfw::flush_messages(&self.events) {
            events.push(event);
        }

        for event in events {
            self.handle_window_events(event);
        }
    }
}

impl<'a> WindowsWindow<'a> {
    fn init(props: WindowProps, glfw: glfw::Glfw) -> WindowsWindow<'a> {
        let width = props.width;
        let height = props.height;
        let title = props.title;

        log::info!("Creating window: {} ({}, {})", title, width, height);

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window.");

        window.set_all_polling(true);
        window.make_current();

        WindowsWindow {
            window,
            data: WindowData {
                title,
                width,
                height,
                vsync: true,
            },
            events,
        }
    }

    pub fn get_window(&mut self) -> &glfw::Window {
        &self.window
    }

    pub fn get_title(&self) -> &'a str {
        &self.data.title
    }

    // TODO: Break into sub-handlers
    fn handle_window_events(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Size(width, height) => {
                self.data.height = height as u32;
                self.data.height = width as u32;

                let event = WindowResizeEvent::new(width as u32, height as u32);
                log::info!("{}", event);
            }
            glfw::WindowEvent::Close => {
                let event = WindowCloseEvent;
                log::info!("{}", event);
            }
            glfw::WindowEvent::Key(_, scancode, action, _) => match action {
                Action::Press => {
                    let event = KeyPressedEvent::new(scancode as u32, 0);
                    log::info!("{}", event);
                }
                Action::Release => {
                    let event = KeyReleasedEvent::new(scancode as u32);
                    log::info!("{}", event);
                }
                Action::Repeat => {
                    let event = KeyPressedEvent::new(scancode as u32, 1);
                    log::info!("{}", event);
                }
            },
            glfw::WindowEvent::MouseButton(mouse_button, action, _) => match action {
                Action::Press => {
                    let event = MouseButtonPressedEvent::new(mouse_button as u32);
                    log::info!("{}", event);
                }
                Action::Release => {
                    let event = MouseButtonReleasedEvent::new(mouse_button as u32);
                    log::info!("{}", event);
                }
                _ => {}
            },
            glfw::WindowEvent::Scroll(x_offset, y_offset) => {
                let event = MouseScrolledEvent::new(x_offset, y_offset);
                log::info!("{}", event);
            }
            glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                let event = MouseMovedEvent::new(x_pos, y_pos);
                log::info!("{}", event);
            }
            _ => {}
        }
    }
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    log::error!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}
