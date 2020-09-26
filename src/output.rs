use crate::checker::State;
use reqwest::blocking::Client;

static TOKEN: &str = std::env!("TELEGRAM_BOT_TOKEN");
static CHAT_ID: &str = std::env!("TELEGRAM_CHAT_ID");


pub fn send(message: State) {
    let text = match message {
        State::AVAILABLE => "XBOOOOX!!!",
        State::SOLD => "Too late",
        State::ERROR => "x_x"
    };

    let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                      TOKEN, CHAT_ID, text);
    let resp = reqwest::blocking::get(&url);
}
