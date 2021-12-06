use chrono;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub use log::{debug, error, info, trace, warn};

pub fn init() -> Result<(), fern::InitError> {
    let log_colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Yellow)
        .trace(Color::BrightBlue)
        .error(Color::BrightRed);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                log_colors.color(record.level()),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("log.log")?)
        .apply()?;

    Ok(())
}
