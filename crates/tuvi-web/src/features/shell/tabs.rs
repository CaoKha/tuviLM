//! The five tabs, in the old app's order, for the current chart.

use std::fmt::Display;

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, iconify::iconify_icon},
    router::{Href, HrefParams, HrefQueries, HrefTarget, href, try_raw_path_params},
};

use crate::{
    app::{
        charts::chart_id::{chart_id, hoi_ai, show as la_so},
        ho_so,
    },
    features::charts::recent,
};

/// One tab. A locked tab has no `href`.
pub(super) struct Tab {
    pub label: &'static str,
    pub icon: IconData,
    pub href: Option<String>,
    pub current: bool,
    /// Locked because its screen is not ported yet.
    pub soon: bool,
}

impl Tab {
    /// A tab for `href`, locked when `href` is `None`.
    fn new<T, P, Q, F>(
        cx: &Cx,
        label: &'static str,
        icon: IconData,
        href: Option<Href<T, P, Q, F>>,
    ) -> Self
    where
        T: HrefTarget,
        P: HrefParams,
        Q: HrefQueries,
        F: Display,
    {
        Self {
            label,
            icon,
            current: href.as_ref().is_some_and(|href| href.is_current(cx)),
            href: href.map(|href| href.resolve(cx)),
            soon: false,
        }
    }

    fn soon(label: &'static str, icon: IconData) -> Self {
        Self {
            label,
            icon,
            href: None,
            current: false,
            soon: true,
        }
    }
}

/// The tabs for `chart`. Without a chart, the chart tabs are locked.
pub(super) fn tabs(cx: &Cx, chart: Option<u64>) -> [Tab; 5] {
    [
        Tab::soon("Bản mệnh", iconify_icon!("lucide:star")),
        Tab::soon("Vận hạn", iconify_icon!("lucide:orbit")),
        Tab::new(
            cx,
            "Hỏi AI",
            iconify_icon!("lucide:message-circle"),
            chart.map(|id| href!(hoi_ai::show, chart_id: id)),
        ),
        Tab::new(
            cx,
            "Nâng cao",
            iconify_icon!("lucide:compass"),
            chart.map(|id| href!(la_so, chart_id: id)),
        ),
        Tab::new(
            cx,
            "Hồ sơ",
            iconify_icon!("lucide:circle-user-round"),
            Some(href!(ho_so::page)),
        ),
    ]
}

/// The chart in the URL under `/charts/{chart_id}`, else the newest chart of
/// `user_id`, else `None`.
pub(super) async fn current_chart(cx: &Cx, user_id: u64) -> Result<Option<u64>> {
    let in_url = try_raw_path_params(cx)
        .and_then(|mut params| params.find(|(name, _)| *name == "chart_id"))
        .and_then(|(_, id)| id.as_str().parse().ok());
    Ok(match in_url {
        Some(id) => Some(id),
        None => recent(cx, user_id, 1).await?.first().map(|saved| saved.id),
    })
}
