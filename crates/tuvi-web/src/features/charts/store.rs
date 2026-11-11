//! The saved charts table and its queries.

use topcoat::{Result, context::Cx, router::error::not_found};
use tuvi_core::{BirthInput, Chart, Gender};

use super::ValidChart;
use crate::db::db;

/// One row: a name and the birth data. The chart is not stored.
#[derive(Debug, toasty::Model)]
pub struct SavedChart {
    #[key]
    #[auto]
    pub id: u64,

    pub name: String,

    pub year: i32,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    /// `"Nam"` or `"Nu"`.
    pub gender: String,
}

impl SavedChart {
    /// The birth data, or `None` if the row holds values the engine cannot use.
    fn birth_input(&self) -> Option<BirthInput> {
        Some(BirthInput {
            year: self.year,
            month: u8::try_from(self.month).ok()?,
            day: u8::try_from(self.day).ok()?,
            hour: u8::try_from(self.hour).ok()?,
            gender: match self.gender.as_str() {
                "Nam" => Gender::Nam,
                "Nu" => Gender::Nu,
                _ => return None,
            },
        })
    }
}

/// Saves a checked form and returns the new id.
pub async fn create(cx: &Cx, chart: ValidChart<'_>) -> Result<u64> {
    let birth = chart.birth;
    let gender = match birth.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nu",
    };
    let saved = toasty::create!(SavedChart {
        name: chart.name,
        year: birth.year,
        month: i16::from(birth.month),
        day: i16::from(birth.day),
        hour: i16::from(birth.hour),
        gender: gender,
    })
    .exec(&mut db(cx))
    .await?;
    Ok(saved.id)
}

/// The newest charts first.
pub async fn recent(cx: &Cx, limit: usize) -> Result<Vec<SavedChart>> {
    Ok(SavedChart::all()
        .order_by(SavedChart::fields().id().desc())
        .limit(limit)
        .exec(&mut db(cx))
        .await?)
}

/// Loads a saved chart and builds it again. A missing or unusable row is a 404;
/// a database failure is a 500.
pub async fn load(cx: &Cx, id: u64) -> Result<(SavedChart, Chart)> {
    let Some(saved) = SavedChart::filter_by_id(id)
        .first()
        .exec(&mut db(cx))
        .await?
    else {
        return Err(not_found().into());
    };
    let Some(chart) = saved
        .birth_input()
        .and_then(|birth| tuvi_core::build_chart(birth).ok())
    else {
        return Err(not_found().into());
    };
    Ok((saved, chart))
}
