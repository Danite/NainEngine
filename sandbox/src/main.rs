use nain::{log, Application};
use std::error::Error;

struct Sandbox;

impl Application for Sandbox {}

fn main() -> Result<(), Box<dyn Error>> {
    log::init()?;
    log::trace!("Eroror help me");

    let sandbox = Sandbox {};

    sandbox.run();

    Ok(())
}
