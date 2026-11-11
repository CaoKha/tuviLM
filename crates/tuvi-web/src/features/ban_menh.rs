//! The Bản mệnh screen: a deck of three cards from the chart. A new chart
//! lands here.
//!
//! - `content`: the card text of the old app, as compact tables
//! - `facts`: what the cards read from the chart (the old selectors)
//! - `card`: the frame of the two lá bài phụ
//! - `destiny`, `luck`, `advice`: Lá Bài Bản Mệnh, Vận May, Lời Khuyên
//! - `screen`: the header and the deck

mod advice;
mod card;
mod content;
mod destiny;
mod facts;
mod luck;
mod screen;

pub use screen::ban_menh_screen;
