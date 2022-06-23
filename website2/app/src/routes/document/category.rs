use super::breadcrumbs::breadcrumbs;
use crate::WebView;
use leptos2::*;
use liturgy::SlugPath;

use super::NavType;

pub fn category_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    pages: &[(NavType, String)],
) -> Vec<Node> {
    let pages = pages
        .iter()
        .map(|(nav_type, label)| {
            let href = match nav_type {
                NavType::Slug(slug) => {
                    format!("/{}/document/{}/{}", locale, base_slug, slug.slugify())
                }
                NavType::Url(url) => format!("/{}/{}", locale, url),
            };
            view! {
                <li>
                    <a href={href}>{label}</a>
                </li>
            }
        })
        .collect::<Vec<_>>();
    view! {
        <>
            <header><h1>{title}</h1></header>
            <main>
                {breadcrumbs(locale, base_slug)}
                <ul class="category-summary">
                    {pages}
                </ul>
            </main>
        </>
    }
}
