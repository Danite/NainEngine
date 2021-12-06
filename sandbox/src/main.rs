use nain::{Application, CreateApplication};
use std::error::Error;

struct Sandbox;

impl Application for Sandbox {}

fn main() -> Result<(), Box<dyn Error>> {
    let sandbox = Sandbox {};

    CreateApplication::new(&sandbox)?;

    Ok(())
}
