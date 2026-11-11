//! Tử Vi chart engine.
//!
//! Pure computation: no IO, no network, no database. Apps call
//! [`build_chart`] with a [`BirthInput`] and render the returned [`Chart`].
//!
//! Modules, in the order the chart is built:
//! - `calendar`: solar date to lunar date, birth hour branch
//! - `can_chi`: Thiên Can and Địa Chi
//! - `cung`: the 12 cung roles, Mệnh and Thân
//! - `cuc`: Cục
//! - `chinh_tinh`: the 14 main stars
//! - `chart`: the public types and `build_chart`

mod calendar;
mod can_chi;
mod chart;
mod chinh_tinh;
mod cuc;
mod cung;

pub use chart::{BirthInput, Chart, ChartError, Cung, Gender, build_chart};
