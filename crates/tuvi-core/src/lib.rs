//! Tử Vi chart engine (Rust port of `src/refactored/`).
//!
//! Pure computation: no IO, no network, no database.

pub mod calendar;
pub mod elementary;
mod lunar_table;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartError {
    /// Date is not a real Gregorian date, or hour > 23.
    InvalidDate,
    /// Year is outside the lunar table (1800..=2199).
    YearOutOfRange,
}

impl std::fmt::Display for ChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidDate => "invalid date",
            Self::YearOutOfRange => "year out of supported range (1800-2199)",
        })
    }
}

impl std::error::Error for ChartError {}
