const URL: &str = "https://www.amazon.de/-/en/dp/B08H93ZRLL";

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    SOLD,
    AVAILABLE,
    ERROR
}

pub struct App {
    state: State,
    error: bool,
}

impl App {
    pub fn new() -> App {
        App {
            state: State::SOLD,
            error: false,
        }
    }

    pub fn check(&mut self) -> Option<State> {
        let resp = App::query();

        if resp != self.state {
            self.state = resp;
            Some(resp)
        }
        else {
            self.state = resp;
            None
        }
    }

    fn query() -> State {
        let resp = reqwest::blocking::get(URL);

        match resp {
            Ok(resp) => {
                if resp.status() == reqwest::StatusCode::OK {
                    match resp.text() {
                        Ok(text) => {
                            if text.contains("Currently unavailable.") {
                                State::SOLD
                            } else {
                                State::AVAILABLE
                            }
                        },
                        Err(e) => {
                            State::ERROR
                        }
                    }
                } else {
                    State::ERROR
                }
            },
            Err(e) => State::ERROR
        }
    }
}
