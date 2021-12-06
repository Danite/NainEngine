use crate::application::Application;
use nain_log as log;
use std::error::Error;

pub struct CreateApplication;

impl CreateApplication {
    pub fn new(application: &dyn Application) -> Result<(), Box<dyn Error>> {
        log::init()?;
        log::trace!("Eroror help me");

        application.run();

        Ok(())
    }
}
