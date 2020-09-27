use log::{info, warn};
use crate::checker::State;

pub fn send(message: State) {
    let token = get_env("TELEGRAM_BOT_TOKEN");
    let chat_id = get_env("TELEGRAM_CHAT_ID");

    let text = match message {
        State::AVAILABLE => "XBOOOOX!!!",
        State::SOLD => "Too late",
        State::ERROR => "x_x"
    };

    info!("Send notification with {}", text);


    let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                      token, chat_id, text);
    let response = reqwest::blocking::get(&url);

    match response {
        Ok(resp) if resp.status() == reqwest::StatusCode::OK =>
            warn!("Telegram notification failed: {}", resp.status()),
        Err(_) => warn!("Telegram request failed"),
        _ => {}
    }
}


pub fn get_env(name: &str) -> String {
     std::env::var(name).expect(&*format!("Env variable {} is not set", name))
}
