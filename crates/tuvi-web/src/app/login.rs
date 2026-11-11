//! `/login`: log in with email and password.

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use crate::features::auth::{LoginForm, current_user, log_in};

/// Classes of a text field on the auth pages.
pub(super) const FIELD: &str = "h-12 w-full rounded-[14px] border border-glass-line bg-white/[0.07] px-3.5 text-[15px] text-ink caret-accent outline-none transition-colors placeholder:text-muted/70 focus:border-accent";

/// Classes of a field label on the auth pages.
pub(super) const LABEL: &str = "text-[10px] font-bold tracking-[0.2em] text-muted uppercase";

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    if current_user(cx).await?.is_some() {
        return Err(see_other(href!(super::home).resolve(cx)).into());
    }
    Ok(view! { login_form(email: String::new(), error: None) })
}

/// Logs in, then goes home. A wrong email or password shows the form again.
#[page(POST)]
pub async fn submit(cx: &Cx, Form(form): Form<LoginForm>) -> Result<impl View> {
    if log_in(cx, &form).await? {
        return Err(see_other(href!(super::home).resolve(cx)).into());
    }
    Ok(view! {
        login_form(
            email: form.email,
            error: Some("Email hoặc mật khẩu không đúng.")
        )
    })
}

#[topcoat::view::component]
async fn login_form(cx: &Cx, email: String, error: Option<&'static str>) -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-md px-4 pb-16">
            <section class="glass p-6">
                <p
                    class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
                >
                    "Tài khoản"
                </p>
                <h1 class="mt-1 text-2xl font-semibold">"Đăng nhập"</h1>
                <form
                    method="post"
                    action=(href!(submit))
                    class="mt-5 flex flex-col gap-4"
                >
                    <label class="flex flex-col gap-2">
                        <span class=(LABEL)>"Email"</span>
                        <input
                            name="email"
                            type="email"
                            required=""
                            autocomplete="email"
                            value=(email)
                            placeholder="ban@example.com"
                            class=(FIELD)
                        >
                    </label>
                    <label class="flex flex-col gap-2">
                        <span class=(LABEL)>"Mật khẩu"</span>
                        <input
                            name="password"
                            type="password"
                            required=""
                            autocomplete="current-password"
                            placeholder="Nhập mật khẩu"
                            class=(FIELD)
                        >
                    </label>
                    if let Some(error) = error {
                        <p role="alert" class="text-sm text-fire">(error)</p>
                    }
                    <button
                        type="submit"
                        class="pill-primary mt-1 h-12 cursor-pointer rounded-full text-sm font-semibold"
                    >
                        "Đăng nhập"
                    </button>
                </form>
                <p class="mt-5 text-center text-sm text-muted">
                    "Chưa có tài khoản? "
                    <a
                        href=(href!(super::register::page))
                        class="font-semibold text-accent"
                    >
                        "Tạo tài khoản"
                    </a>
                </p>
            </section>
        </main>
    })
}
