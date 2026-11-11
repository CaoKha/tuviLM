//! The An sao screen: the home page form that casts a new chart.
//!
//! - `screen`: the form, put together from the parts below
//! - `wheel`, `segment`, `clock`: the date wheels, the AM/PM and gender
//!   segments, and the birth clock
//! - `preview`: the shard that shows the chính tinh of cung Mệnh while the
//!   user picks, and tints the page accent
//! - `chom_sao`: the constellation of each chính tinh over its ghost portrait
//! - `confirm`, `loader`: the confirm dialog and the casting loader
//! - `an_sao.js`: what runtime expressions cannot do: wheel scroll, clock
//!   drag, the confirm dialog and the loader delay
//!
//! The form is a plain POST to `/charts`, so it works without JavaScript and
//! the server checks every field (`features::charts::NewChart`).

mod chom_sao;
mod clock;
mod confirm;
mod loader;
mod preview;
mod screen;
mod segment;
mod wheel;

pub use preview::menh_preview;
pub use screen::an_sao_screen;
