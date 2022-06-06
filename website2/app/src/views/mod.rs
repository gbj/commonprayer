use language::Language;
use leptos2::{view, Node};
use liturgy::Version;

pub mod document;
pub mod header;
pub mod icon;
pub mod menu;
pub mod readings;

pub use document::*;
pub use header::*;
pub use icon::*;
pub use menu::*;
pub use readings::*;

pub fn bible_version_select(locale: &str, name: &str, value: Version) -> Node {
    view! {
        <select name={name}>
            {bible_version_select_options(locale, value)}
        </select>
    }
}

pub fn bible_version_select_options(locale: &str, value: Version) -> Vec<Node> {
    let versions = match Language::from_locale(locale) {
        Language::Es => [
            Version::RV09,
            Version::NRSV,
            Version::CEB,
            Version::ESV,
            Version::KJV,
        ],
        _ => [
            Version::NRSV,
            Version::CEB,
            Version::ESV,
            Version::KJV,
            Version::RV09,
        ],
    };

    versions
        .into_iter()
        .map(|version| {
            let value_str: &'static str = version.into();
            view! {
                <option value={value_str} selected={value == version}>{version.to_string()}</option>
            }
        })
        .collect::<Vec<_>>()
}
