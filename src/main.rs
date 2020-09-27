mod checker;
mod output;

use std::time::Duration;
use log::info;
use simple_logger::SimpleLogger;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    info!("Start");

    let mut app = checker::App::new();

    loop {
        if let Some(update) = app.check() {
            output::send(update)
        }

        std::thread::sleep(Duration::from_secs(60))
    }
}
