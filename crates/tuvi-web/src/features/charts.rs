//! Saved charts.
//!
//! - `form`: the "new chart" form data and its checks
//! - `store`: the database model and queries
//! - `grid`: the chart drawing and the cung popups
//! - `accent`: the chart's accent color from cung Mệnh
//!
//! The database keeps only the birth data. Pages build the chart again with
//! `tuvi_core` on each read. This takes microseconds, so engine changes need no
//! data migration.

mod accent;
mod form;
mod grid;
mod store;

pub use accent::accent_style;
pub use form::{NewChart, ValidChart};
pub use grid::chart_grid;
pub use store::{create, load, recent};
