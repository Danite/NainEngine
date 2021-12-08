use crate::application::Application;
use crate::log;
use std::error::Error;

pub struct CreateApplication;

impl CreateApplication {
    pub fn new(application: &mut Application) -> Result<(), Box<dyn Error>> {
        log::init()?;

        application.run();

        Ok(())
    }
}
