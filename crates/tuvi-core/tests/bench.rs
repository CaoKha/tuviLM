//! Timing for `build_chart`. Ignored by default; run in release:
//!
//! ```sh
//! cargo test --release -p tuvi-core --test bench -- --ignored --nocapture
//! ```

use std::hint::black_box;
use std::time::Instant;
use tuvi_core::{BirthInput, Gender, build_chart};

const ITERATIONS: u32 = 100_000;

#[test]
#[ignore = "timing only; run with --release --ignored --nocapture"]
fn build_chart_time() {
    let inputs: Vec<BirthInput> = (0..ITERATIONS)
        .map(|i| BirthInput {
            year: 1900 + (i % 200) as i32,
            month: (i % 12) as u8 + 1,
            day: (i % 28) as u8 + 1,
            hour: (i % 24) as u8,
            gender: if i % 2 == 0 { Gender::Nam } else { Gender::Nu },
        })
        .collect();

    let start = Instant::now();
    let mut stars = 0_usize;
    for input in &inputs {
        let chart = build_chart(black_box(*input)).expect("valid input");
        stars += black_box(&chart)
            .cungs
            .iter()
            .map(|c| c.chinh_tinh.len())
            .sum::<usize>();
    }
    let elapsed = start.elapsed();
    assert_eq!(stars, 14 * ITERATIONS as usize);
    println!(
        "build_chart: {ITERATIONS} iterations in {elapsed:?}, {:.0} ns per chart",
        elapsed.as_nanos() as f64 / f64::from(ITERATIONS)
    );
}
