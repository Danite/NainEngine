pub use self::window::{Window as WindowTrait, WindowProps};

pub use self::platforms::WindowsWindow as Window;

mod platforms;
mod window;
