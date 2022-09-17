use crate::debug_warn;
use leptos::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locale(
    ReadSignal<LanguageIdentifier>,
    WriteSignal<LanguageIdentifier>,
);

#[component]
pub fn Localizer(cx: Scope, children: Box<dyn Fn() -> Vec<Element>>) -> impl IntoChild {
    let (locale, set_locale) = create_signal(cx, "en-US".parse::<LanguageIdentifier>().unwrap());
    let locale = Locale(locale, set_locale);
    provide_context::<Locale>(cx, locale);

    children
}

use fluent_templates::{LanguageIdentifier, Loader};
fluent_templates::static_loader! {
    static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "./locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
    };
}

pub fn use_i18n(
    cx: Scope,
) -> (
    impl Fn(&str) -> String + Copy,
    //impl Fn(&str, &HashMap<String, String>) -> String + Clone,
    impl Fn(&str),
) {
    match use_context::<Locale>(cx) {
        None => {
            log::warn!("use_i18n() should be called within a <Localizer/> component");
            unreachable!()
        }
        Some(locale) => {
            let Locale(locale, set_locale) = locale;

            let t = move |key: &str| {
                if let Some(val) = LOCALES.lookup(&locale.get(), key) {
                    val
                } else {
                    debug_warn!("(i18n) key not found: {key}");
                    key.to_string()
                }
            };
            /* let t_with_args = |key: &str, args: &HashMap<String, String>| {
                LOCALES.lookup_with_args(&locale.get(), key, args)
            }; */
            let set_locale = move |locale: &str| {
                set_locale(|n| {
                    if let Ok(langid) = locale.parse() {
                        *n = langid;
                    } else {
                        debug_warn!("set_locale() couldn't parse locale {locale}");
                    }
                })
            };

            (t, /* t_with_args, */ set_locale)
        }
    }
}

pub fn use_language(cx: Scope) -> Memo<String> {
    match use_context::<Locale>(cx) {
        None => {
            debug_warn!("Called use_context but could not find a Locale context. Is this inside a <Localizer/>?");
            create_memo(cx, |_| "en-US".to_string())
        }
        Some(locale) => create_memo(cx, move |_| locale.0.get().language.as_str().to_string()),
    }
}
