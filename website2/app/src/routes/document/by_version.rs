use leptos2::*;
use liturgy::{Slug, SlugPath, Version};
use crate::views::Header;
use super::breadcrumbs::breadcrumbs;

pub fn by_version_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    versions: &[Version],
) -> Vec<Node> {
    let pages = 
        versions
            .iter()
            .map(|version| {
                view! {
                    <li>
                        <a href={format!("/{}/document/{}/{}", locale, base_slug, Slug::Version(*version).slugify())}>{version.to_string()}</a>
                    </li>
                }
            })
            .collect::<Vec<_>>();
    view! {
        <>
            {Header::new(locale, title).to_node()}
            <main>
                {breadcrumbs(locale, base_slug)}
                <ul class="by-version-summary">
                    {pages}
                </ul>
            </main>
        </>
    }
}