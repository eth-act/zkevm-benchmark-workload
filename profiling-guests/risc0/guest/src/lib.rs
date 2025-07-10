use risc0_zkvm::guest::env;

/// Tracks the amount of cycles a region of code takes up
/// in a zkvm environment and is no-op otherwise.
#[macro_export]
macro_rules! track_cycles {
    ($name:expr, $body:expr) => {{
        #[cfg(target_os = "zkvm")]
        {
            let start_cycles = env::cycle_count();
            println!("\x1b[36m[START]\x1b[0m Cycle tracking for region: \x1b[33m{}\x1b[0m", $name);
            let result = $body;
            let end_cycles = env::cycle_count();
            let cycle_count = end_cycles - start_cycles;
            println!("\x1b[32m[END]\x1b[0m Cycle tracking for region: \x1b[33m{}\x1b[0m - cycles: \x1b[35m{}\x1b[0m", $name, cycle_count);
            (result, Some(cycle_count))
        }

        #[cfg(not(target_os = "zkvm"))]
        {
            let result = $body;
            (result, None)
        }
    }};
}

/// Example function demonstrating cycle tracking usage
pub fn example_cycle_tracking() {
    // Track cycles for a specific operation
    track_cycles!("example_operation", {
        // Simulate some computation-intensive operation
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        println!("Sum: {}", sum);
    });
} 