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

    println!("{}", duration.as_millis() as f64);
    println!("{}", block.duration().as_millis() as f64);

    assert!(res.is_ok());
    assert!(block.done());
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