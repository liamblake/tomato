use std::time::{Duration, Instant};

pub struct Block {
    text: String,
    duration: Duration,
    pub start: Option<Instant>,
    pub elapsed: Duration,
    done: bool,
}

impl Block {
    pub fn new(text: &str, duration: Duration) -> Block {
        Block {
            text: text.to_string(),
            duration: duration,
            start: None,
            elapsed: Duration::from_nanos(0),
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

        self.start = Some(Instant::now());

        // Run the time down
        let start_time = self.start.unwrap();
        while self.elapsed < self.duration {
            self.elapsed = Instant::now() - start_time;
        }

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
