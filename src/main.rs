mod checker;
mod output;

use reqwest;
use std::time::Duration;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = checker::App::new();

    loop {
        if let Some(update) = app.check() {
            output::send(update)
        }

        std::thread::sleep(Duration::from_secs(60))
    }
}
