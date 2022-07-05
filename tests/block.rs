use approx::abs_diff_eq;
use std::time::{Duration, Instant};
use tomato::block::*;

#[test]
fn test_block() {
    let mut block = Block::new("Some block", Duration::from_secs(1));

    assert!(!block.done());

    // Run the timer - should return OK and the block should be done
    // The duration should have elapsed
    let start = Instant::now();
    let res = block.run();
    let duration = start.elapsed();

    assert!(res.is_ok());
    assert!(block.done());
    assert!(block.elapsed == block.duration());
    // TODO: Improve precision of timers
    assert!(abs_diff_eq!(
        duration.as_millis() as f64,
        block.duration().as_millis() as f64,
        epsilon = 100.0
    ));

    // Try to run the timer - should return an error
    let res = block.run();
    assert!(res.is_err());
}

#[test]
fn test_default_task() {
    let block = Block::default_task("some task");
    assert!(block.text() == "Task: some task");
    assert!(block.duration() == Duration::from_secs(25 * 60));
}

#[test]
fn test_default_short_break() {
    let block = Block::default_short_break();
    assert!(block.text() == "Short break");
    assert!(block.duration() == Duration::from_secs(5 * 60));
}

#[test]
fn test_default_long_break() {
    let block = Block::default_long_break();
    assert!(block.text() == "Long break");
    assert!(block.duration() == Duration::from_secs(15 * 60));
}
