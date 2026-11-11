//! `/register`: create an account, then log in.

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use super::login::{FIELD, LABEL};
use crate::features::auth::{RegisterForm, current_user, register};

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    if current_user(cx).await?.is_some() {
        return Err(see_other(href!(super::home).resolve(cx)).into());
    }
    Ok(view! { register_form(name: String::new(), email: String::new(), error: None) })
}

/// Creates the account and goes home. A problem shows the form again; the
/// passwords are never sent back.
#[page(POST)]
pub async fn submit(cx: &Cx, Form(form): Form<RegisterForm>) -> Result<impl View> {
    let error = match register(cx, &form).await? {
        Ok(()) => return Err(see_other(href!(super::home).resolve(cx)).into()),
        Err(error) => error,
    };
    Ok(view! {
        register_form(name: form.display_name, email: form.email, error: Some(error))
    })
}

#[topcoat::view::component]
async fn register_form(
    cx: &Cx,
    name: String,
    email: String,
    error: Option<String>,
) -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-md px-4 pb-16">
            <section class="glass p-6">
                <p
                    class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
                >
                    "Tài khoản"
                </p>
                <h1 class="mt-1 text-2xl font-semibold">"Tạo tài khoản"</h1>
                <form
                    method="post"
                    action=(href!(submit))
                    class="mt-5 flex flex-col gap-4"
                >
                    <label class="flex flex-col gap-2">
                        <span class=(LABEL)>"Tên hiển thị"</span>
                        <input
                            name="display_name"
                            required=""
                            minlength="2"
                            maxlength="50"
                            autocomplete="nickname"
                            value=(name)
                            placeholder="Tên của bạn"
                            class=(FIELD)
                        >
                    </label>
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
                            minlength="8"
                            maxlength="128"
                            autocomplete="new-password"
                            placeholder="Ít nhất 8 ký tự"
                            class=(FIELD)
                        >
                    </label>
                    <label class="flex flex-col gap-2">
                        <span class=(LABEL)>"Nhập lại mật khẩu"</span>
                        <input
                            name="confirm_password"
                            type="password"
                            required=""
                            minlength="8"
                            maxlength="128"
                            autocomplete="new-password"
                            placeholder="Nhập lại mật khẩu"
                            class=(FIELD)
                        >
                    </label>
                    if let Some(error) = &error {
                        <p role="alert" class="text-sm text-fire">(error.as_str())</p>
                    }
                    <button
                        type="submit"
                        class="pill-primary mt-1 h-12 cursor-pointer rounded-full text-sm font-semibold"
                    >
                        "Tạo tài khoản"
                    </button>
                </form>
                <p class="mt-5 text-center text-sm text-muted">
                    "Đã có tài khoản? "
                    <a
                        href=(href!(super::login::page))
                        class="font-semibold text-accent"
                    >
                        "Đăng nhập"
                    </a>
                </p>
            </section>
        </main>
    })
}
