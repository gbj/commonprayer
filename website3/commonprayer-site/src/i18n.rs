use leptos::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locale(ReadSignal<String>, WriteSignal<String>);

#[component]
pub fn Localizer(cx: Scope) -> impl IntoChild {
    let navigate = use_navigate(cx);

    let params = use_params_map(cx);
    let (locale, set_locale) = create_signal(
        cx,
        params
            .get("lang")
            .map(String::from)
            .unwrap_or_else(|| "en".to_string()),
    );
    let locale = Locale(locale, set_locale);
    provide_context(cx, locale);

    view! {
        <Outlet/>
    }
}

pub fn use_i18n(cx: Scope) -> (impl Fn(&str) -> String + Copy, impl Fn(&str)) {
    match use_context::<Locale>(cx) {
        None => {
            log::warn!("use_i18n() should be called within a <Localizer/> component");
            let t = |key: &str| key.to_string();

            let set_locale = move |locale: &str| {};

            (t, set_locale)
        }
        Some(locale) => {
            let Locale(locale, set_locale) = locale;

            // TODO actual localization
            /* let t = |key: &str| key.to_string();

            let set_locale = move |locale: &str| set_locale(|n| *n = locale.to_string());

            (t, set_locale) */
            todo!()
        }
    }
}

pub fn use_locale(cx: Scope) -> ReadSignal<String> {
    match use_context::<Locale>(cx) {
        None => {
            log::warn!("Called use_context but could not find a Locale context. Is this inside a <Localizer/>?");
            create_signal(cx, "en".to_string()).0
        }
        Some(locale) => locale.0,
    }
}
