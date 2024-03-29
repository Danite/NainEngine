pub struct WindowProps {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "Nain Engine",
            width: 1280,
            height: 720,
        }
    }
}

impl WindowProps {
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
        }
    }
}

pub trait Window {
    fn on_update(&mut self);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync_enabled(&self) -> bool;
    fn create(props: WindowProps) -> Self;
}
