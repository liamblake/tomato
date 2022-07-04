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

    pub fn default_task(text: &str) -> Block {
        Block::new(&format!("Task: {}", text), Duration::from_secs(25 * 60))
    }

    pub fn default_short_break() -> Block {
        Block::new("Short break", Duration::from_secs(5 * 60))
    }

    pub fn default_long_break() -> Block {
        Block::new("Long break", Duration::from_secs(15 * 60))
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
