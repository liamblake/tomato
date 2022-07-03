use std::thread::sleep;
use std::time::Duration;

pub struct Block {
    text: String,
    duration: Duration,
    done: bool,
}

impl Block {
    pub fn new(text: &str, duration: Duration) -> Block {
        Block {
            text: text.to_string(),
            duration: duration,
            done: false,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.done {
            return Err("Block already done".to_string());
        }

        // Run the time down
        // TODO: This is almost certainly not the best way to do this.
        sleep(self.duration);

        self.done = true;
        return Ok(());
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
