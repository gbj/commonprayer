use leptos2::*;
use liturgy::{Slug, SlugPath};

pub fn breadcrumbs(locale: &str, path: &SlugPath) -> Node {
    let path_len = path.len();
    let crumbs = path
        .into_iter()
        .enumerate()
        .map(|(idx, slug)| {
            let preceding = SlugPath::from(path.as_slice()[0..=idx].to_vec());
            let label = match slug {
                Slug::Version(version) => version.to_string(),
                Slug::Canticle(id) => id.to_string(),
                _ => {
                    let key = format!("slug.{slug}");
                    t!(&key).to_string()
                }
            };
            view! {
                <li>
                    <a href={format!("/{}/document/{}", locale, preceding)}>
                        {label}
                    </a>
                    {if idx == path_len - 1 {
                        ""
                    } else {
                        " 〉"
                    }}
                </li>
            }
        })
        .collect::<Vec<_>>();
    view! {
        <nav class="breadcrumb">
            <ol>
                <li><a href={format!("/{}", locale)}>"⌂"</a>" 〉"</li>
                {crumbs}
            </ol>
        </nav>
    }
}
