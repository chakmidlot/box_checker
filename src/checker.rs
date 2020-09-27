use log::{info, warn};

const URL: &str = "https://www.amazon.de/-/en/dp/B08H93ZRLL";

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum State {
    SOLD,
    AVAILABLE,
    ERROR
}

pub struct App {
    state: State,
}

impl App {
    pub fn new() -> App {
        App {
            state: State::SOLD,
        }
    }

    pub fn check(&mut self) -> Option<State> {
        let resp = App::query();

        info!("Query result: {:?}", resp);

        if resp != self.state {
            info!("Status changed: {:?} -> {:?}", self.state, resp);
            self.state = resp;
            Some(resp)
        }
        else {
            info!("Status didn't change");
            self.state = resp;
            None
        }
    }

    fn query() -> State {
        let resp = reqwest::blocking::get(URL);

        match resp {
            Ok(resp) => {
                let status = resp.status();

                info!("Response status: {}", status);

                if status == reqwest::StatusCode::OK {
                    match resp.text() {
                        Ok(text) => {
                            if text.contains("Currently unavailable.") {
                                State::SOLD
                            } else {
                                State::AVAILABLE
                            }
                        },
                        Err(err) => {
                            warn!("Failed to get response text: {:?}", err);
                            State::ERROR
                        }
                    }
                } else {
                    warn!("Wrong response");
                    State::ERROR
                }
            },
            Err(err) => {
                warn!("Query failed: {:?}", err);
                State::ERROR
            }
        }
    }
}
