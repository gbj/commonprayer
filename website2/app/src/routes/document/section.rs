use super::{breadcrumbs::breadcrumbs, NavType};
use leptos2::*;
use liturgy::SlugPath;

pub fn section_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    pages: &[(String, Vec<(NavType, String)>)],
) -> Vec<Node> {
    let sections = pages
        .iter()
        .flat_map(|(label, pages)| {
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

            let label = if !label.is_empty() {
                Some(view! { <h2>{label}</h2> })
            } else {
                None
            };

            view! {
                <>
                    <li>
                        {label}
                        <ul>
                            {pages}
                        </ul>
                    </li>
                </>
            }
        })
        .collect::<Vec<_>>();
    view! {
        <>
            <header><h1>{title}</h1></header>
            <main>
                {breadcrumbs(locale, base_slug)}
                <ul class="section-summary">
                    {sections}
                </ul>
            </main>
        </>
    }
}
