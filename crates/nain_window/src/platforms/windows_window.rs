use crate::{EventCallbackFn, Window, WindowProps};
use glfw::{Action, Context, Key};

struct WindowData<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    vsync: bool,
    event_callback: EventCallbackFn,
}

pub struct WindowsWindow<'a> {
    window: &'static glfw::Window,
    data: WindowData<'a>,
}
