//! A scroll-snap wheel of radios: the day, month or year.

use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{View, view},
};

/// One wheel. `options` are the form values, shown as they are. `picked`
/// holds the checked value; a change of radio updates it.
#[topcoat::view::component]
pub(super) async fn wheel(
    cx: &Cx,
    name: &'static str,
    label: &'static str,
    options: Vec<String>,
    picked: &Signal<String>,
) -> Result<impl View> {
    let checked = picked.get_untracked();
    Ok(view! {
        <div>
            <p class="dial-label">(label)</p>
            <div
                class="wheel"
                role="radiogroup"
                aria-label=(label)
                data-wheel=""
                @change=$(|e: Event| picked.set(e.target.value))
            >
                <div class="h-[55px]" aria-hidden="true"></div>
                for option in options {
                    <label>
                        <input
                            type="radio"
                            name=(name)
                            value=(option.as_str())
                            checked=(option == checked)
                            class="sr-only"
                        >
                        (option.as_str())
                    </label>
                }
                <div class="h-[55px]" aria-hidden="true"></div>
            </div>
        </div>
    })
}
