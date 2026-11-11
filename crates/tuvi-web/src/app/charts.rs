//! `POST /charts` and the form that posts to it.

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, attributes, view},
};

use crate::{
    components::{
        button::button,
        card::{card, card_content, card_description, card_header, card_title},
        field::{field, field_group, field_label},
        input::input,
    },
    features::charts::{NewChart, create as save_chart},
};

pub mod chart_id;

/// Saves a new chart, then redirects to it. On bad input, shows the form again
/// with the error.
#[page(POST)]
pub async fn create(cx: &Cx, Form(form): Form<NewChart>) -> Result<impl View> {
    let error = match form.validate() {
        Ok(chart) => {
            let id = save_chart(cx, chart).await?;
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

/// The "new chart" form. `error` is shown above the button after a failed submit.
#[topcoat::view::component]
pub async fn new_chart_form(cx: &Cx, error: Option<String>) -> Result<impl View> {
    Ok(view! {
        card(
            card_header(
                card_title("Lập lá số")
                card_description("Ngày sinh dương lịch, giờ Việt Nam.")
            )
            card_content(
                <form method="post" action=(href!(create))>
                    field_group(
                        field(
                            field_label(attrs: attributes! { for="name" }, "Tên")
                            input(
                                attrs: attributes! { id="name" name="name" required="" maxlength="80" }
                            )
                        )
                        field(
                            field_label(
                                attrs: attributes! { for="birth_date" },
                                "Ngày sinh"
                            )
                            input(
                                attrs: attributes! {
                                    id="birth_date"
                                    name="birth_date"
                                    type="date"
                                    required=""
                                    min="1800-01-25"
                                    max="2199-12-31"
                                }
                            )
                        )
                        field(
                            field_label(attrs: attributes! { for="hour" }, "Giờ sinh")
                            <select
                                id="hour"
                                name="hour"
                                class="h-9 rounded-md border border-border bg-transparent px-3 text-sm"
                            >
                                for hour in 0u8..24 {
                                    <option value=(hour)>
                                        (format!("{hour:02}:00 – {hour:02}:59"))
                                    </option>
                                }
                            </select>
                        )
                        <fieldset class="flex gap-6">
                            <legend class="mb-2 text-sm font-medium">
                                "Giới tính"
                            </legend>
                            <label class="flex items-center gap-2 text-sm">
                                <input type="radio" name="gender" value="Nam" checked="">
                                "Nam"
                            </label>
                            <label class="flex items-center gap-2 text-sm">
                                <input type="radio" name="gender" value="Nu">
                                "Nữ"
                            </label>
                        </fieldset>
                        if let Some(error) = &error {
                            <p role="alert" class="text-sm text-destructive">
                                (error.as_str())
                            </p>
                        }
                        button(attrs: attributes! { type="submit" }, "Lập lá số")
                    )
                </form>
            )
        )
    })
}
