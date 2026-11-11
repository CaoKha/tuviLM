//! `POST /charts` and the form that posts to it.

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use crate::features::{
    auth::require_user,
    charts::{NewChart, create as save_chart},
};

pub mod chart_id;

/// Saves a new chart, then redirects to it. On bad input, shows the form again
/// with the error.
#[page(POST)]
pub async fn create(cx: &Cx, Form(form): Form<NewChart>) -> Result<impl View> {
    let user = require_user(cx).await?;
    let error = match form.validate() {
        Ok(chart) => {
            let id = save_chart(cx, user.id, chart).await?;
            return Err(see_other(href!(chart_id::show, chart_id: id).resolve(cx)).into());
        }
        Err(error) => error.to_string(),
    };
    Ok(view! {
        <main class="mx-auto max-w-xl px-4 pb-16">
            new_chart_form(error: Some(error))
        </main>
    })
}

/// Classes shared by the text, date and select controls.
const CONTROL: &str = "h-11 w-full rounded-2xl border border-glass-line bg-white/5 px-4 text-sm text-ink outline-none transition-colors focus-visible:border-accent [color-scheme:dark]";

/// Classes for a field label.
const LABEL: &str = "text-[10px] font-bold tracking-[0.2em] text-muted uppercase";

/// The "new chart" form. `error` is shown above the button after a failed submit.
#[topcoat::view::component]
pub async fn new_chart_form(cx: &Cx, error: Option<String>) -> Result<impl View> {
    Ok(view! {
        <section class="glass p-6">
            <p class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase">
                "An sao"
            </p>
            <h2 class="mt-1 text-xl font-semibold">"Lập lá số"</h2>
            <p class="mt-1 text-sm font-light text-muted">
                "Ngày sinh dương lịch, giờ Việt Nam."
            </p>
            <form method="post" action=(href!(create)) class="mt-5 flex flex-col gap-4">
                <label class="flex flex-col gap-2">
                    <span class=(LABEL)>"Tên"</span>
                    <input name="name" required="" maxlength="80" class=(CONTROL)>
                </label>
                <div class="grid grid-cols-2 gap-3">
                    <label class="flex flex-col gap-2">
                        <span class=(LABEL)>"Ngày sinh"</span>
                        <input
                            name="birth_date"
                            type="date"
                            required=""
                            min="1800-01-25"
                            max="2199-12-31"
                            class=(CONTROL)
                        >
                    </label>
                    <label class="flex flex-col gap-2">
                        <span class=(LABEL)>"Giờ sinh"</span>
                        <select name="hour" class=(CONTROL)>
                            for hour in 0u8..24 {
                                <option value=(hour) class="bg-bg-1">
                                    (format!("{hour:02}:00 – {hour:02}:59"))
                                </option>
                            }
                        </select>
                    </label>
                </div>
                <fieldset class="flex flex-col gap-2">
                    <legend class=(LABEL)>"Giới tính"</legend>
                    <div class="mt-2 grid grid-cols-2 gap-2">
                        <label
                            class="pill-ghost flex h-11 cursor-pointer items-center justify-center rounded-full text-sm has-[:checked]:border-accent has-[:checked]:text-accent"
                        >
                            <input
                                type="radio"
                                name="gender"
                                value="Nam"
                                checked=""
                                class="sr-only"
                            >
                            "Nam"
                        </label>
                        <label
                            class="pill-ghost flex h-11 cursor-pointer items-center justify-center rounded-full text-sm has-[:checked]:border-accent has-[:checked]:text-accent"
                        >
                            <input
                                type="radio"
                                name="gender"
                                value="Nu"
                                class="sr-only"
                            >
                            "Nữ"
                        </label>
                    </div>
                </fieldset>
                if let Some(error) = &error {
                    <p role="alert" class="text-sm text-fire">(error.as_str())</p>
                }
                <button
                    type="submit"
                    class="pill-primary mt-1 h-12 rounded-full text-sm font-semibold"
                >
                    "Lập lá số"
                </button>
            </form>
        </section>
    })
}
