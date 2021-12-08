use crate::{WindowProps, WindowTrait};
use glfw::{Context, SwapInterval};
use nain_log as log;

struct WindowData<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    vsync: bool,
    // event_callback: EventCallbackFn,
}

pub struct WindowsWindow<'a> {
    window: glfw::Window,
    data: WindowData<'a>,
}

impl<'a> WindowTrait for WindowsWindow<'a> {
    fn create(props: WindowProps) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let window = WindowsWindow::init(props, glfw);

        window
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
    }
}

impl<'a> WindowsWindow<'a> {
    fn init(props: WindowProps, glfw: glfw::Glfw) -> WindowsWindow<'a> {
        let width = props.width;
        let height = props.height;
        let title = props.title;

        log::info!("Creating window: {} ({}, {})", title, width, height);

        let (mut window, _) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        WindowsWindow {
            window,
            data: WindowData {
                title,
                width,
                height,
                vsync: true,
            },
        }
    }

    pub fn get_window(&mut self) -> &glfw::Window {
        &self.window
    }
}
