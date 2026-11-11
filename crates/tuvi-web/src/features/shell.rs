//! The app shell around every page of a logged-in user.
//!
//! - `logo`: the logo and the name, in the page header and in the rail
//! - `tabs`: the five tabs in order, the current chart, the active tab and
//!   the locks
//! - `nav`: one `<nav>`: a bottom tab bar below `lg`, a left rail from `lg`
//!
//! The shell keeps no state. The current chart is the chart in the URL, else
//! the user's newest chart.

mod logo;
mod nav;
mod tabs;

pub use logo::{LOGO, brand};
pub use nav::app_nav;
