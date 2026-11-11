//! Parity with the Python engine. Fixtures come from
//! `scripts/gen_fixtures.py` (run against `src/refactored`).

use serde::Deserialize;
use serde_json::Value;
use tuvi_core::{BirthInput, ChartError, build_chart};

#[derive(Deserialize)]
struct Case {
    input: BirthInput,
    expected: Option<Value>,
    error: Option<String>,
}

fn error_name(error: &ChartError) -> &'static str {
    match error {
        ChartError::InvalidDate => "InvalidDate",
        ChartError::YearOutOfRange => "YearOutOfRange",
    }
}

#[test]
fn charts_match_python() {
    let raw = include_str!("fixtures/charts.json");
    let cases: Vec<Case> = serde_json::from_str(raw).expect("valid fixture JSON");
    assert!(cases.len() >= 300, "expected at least 300 cases");

    let mut failures = Vec::new();
    for case in &cases {
        let actual = match build_chart(case.input) {
            Ok(chart) => serde_json::to_value(&chart).expect("chart serializes"),
            Err(error) => Value::String(error_name(&error).to_owned()),
        };
        let expected = match (&case.expected, &case.error) {
            (Some(chart), None) => chart.clone(),
            (None, Some(error)) => Value::String(error.clone()),
            _ => panic!(
                "case must have exactly one of expected/error: {:?}",
                case.input
            ),
        };
        if actual != expected {
            failures.push(format!(
                "{:?}\n  expected: {expected}\n  actual:   {actual}",
                case.input
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} cases differ:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
