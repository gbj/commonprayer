use crate::{debug_warn, locales::locale_data};
use leptos::*;
use std::{collections::HashMap, rc::Rc};

pub type I18nArgs = HashMap<&'static str, String>;

#[derive(Clone)]
pub struct LocaleData(
    pub &'static str,
    pub HashMap<&'static str, Rc<dyn Fn(Option<I18nArgs>) -> String>>,
);

impl PartialEq for LocaleData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for LocaleData {}

impl std::fmt::Debug for LocaleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LocaleData").field(&self.0).finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locale(pub ReadSignal<String>, pub WriteSignal<String>);

#[component]
pub fn Localizer(cx: Scope, children: Box<dyn Fn() -> Vec<Element>>) -> Element {
    //impl IntoChild {
    let (locale, set_locale) = create_signal(cx, "en-US".to_string());
    let locale = Locale(locale, set_locale);
    provide_context::<Locale>(cx, locale);

    view! {<div>{children}</div>}
}

pub fn provide_localization(cx: Scope) {
    let (locale, set_locale) = create_signal(cx, "en-US".to_string());
    let locale = Locale(locale, set_locale);
    provide_context::<Locale>(cx, locale);
}

pub fn use_i18n(
    cx: Scope,
) -> (
    impl Fn(&str) -> String + Copy,
    impl Fn(&str, HashMap<&'static str, String>) -> String + Copy,
    impl Fn(&str),
) {
    match use_context::<Locale>(cx) {
        None => {
            log::warn!("use_i18n() should be called within a <Localizer/> component");
            unreachable!()
        }
        Some(locale) => {
            let Locale(locale, set_locale) = locale;
            let locale_data = create_memo(cx, move |_| locale_data(&locale()));

            let t = move |key: &str| {
                if let Some(val) = locale_data.with(|d| d.1.get(key).map(Rc::clone)) {
                    val(None)
                } else {
                    debug_warn!("(i18n::t) key not found: {key}");
                    key.to_string()
                }
            };
            let t_with_args = move |key: &str, args: I18nArgs| {
                if let Some(val) = locale_data.with(|d| d.1.get(key).map(Rc::clone)) {
                    val(Some(args))
                } else {
                    debug_warn!("(i18n::t_with_args) key not found: {key}");
                    key.to_string()
                }
            };
            let set_locale = move |locale: &str| {
                set_locale(|n| {
                    if let Ok(langid) = locale.parse() {
                        *n = langid;
                    } else {
                        debug_warn!("set_locale() couldn't parse locale {locale}");
                    }
                })
            };

            (t, t_with_args, set_locale)
        }
    }
}

pub fn use_language(cx: Scope) -> Memo<String> {
    match use_context::<Locale>(cx) {
        None => {
            debug_warn!("Called use_context but could not find a Locale context. Is this inside a <Localizer/>?");
            create_memo(cx, |_| "en-US".to_string())
        }
        Some(locale) => create_memo(cx, move |_| locale.0.get()),
    }
}
