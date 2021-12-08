use nain::{Application, CreateApplication};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sandbox = Application::new();

    CreateApplication::new(&mut sandbox)?;

    Ok(())
}
