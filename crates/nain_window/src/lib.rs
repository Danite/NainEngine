pub use self::window::{EventCallbackFn, Window as WindowTrait, WindowProps};

pub use self::platforms::WindowsWindow as Window;

mod platforms;
mod window;
