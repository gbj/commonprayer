use std::collections::HashMap;

use canticle_table::{CanticleId, CanticleNumber};
use itertools::Itertools;
use leptos2::*;
use library::{CommonPrayer, Contents, Library};
use liturgy::{Content, GlobalPref, PreferenceKey, PreferenceValue, Slug, SlugPath, Version};

pub struct CanticleChoice {
    locale: String,
    canticle_number: CanticleNumber,
    raw_redirect: String,
    redirect: url::Url,
    version: Option<Version>,
    canticles: Vec<(Option<CanticleId>, Version, SlugPath, String)>,
    search: String,
}

#[derive(Params)]
pub struct CanticleChoiceParams {
    version: Option<Version>,
}

#[derive(Params)]
pub struct CanticleChoiceQuery {
    canticle: CanticleNumber,
    redirect: String,
    search: Option<String>,
}

#[async_trait(?Send)]
impl Loader for CanticleChoice {
    type Params = CanticleChoiceParams;
    type Query = CanticleChoiceQuery;

    async fn loader(
        locale: &str,
        _req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let redirect = base64::decode(&query.redirect).ok()?;
        let redirect = String::from_utf8(redirect).ok()?;
        let redirect =
            url::Url::parse(&format!("https://www.commonprayeronline.org{redirect}")).ok()?;

        let search = query.search.unwrap_or_default();
        let version = params.version;

        let canticles_path = SlugPath::from([Slug::Office, Slug::Canticles]);
        let canticles = CommonPrayer::contents().contents_at_path(&canticles_path)?;
        let canticles = canticles
            .flatten_with_starting_path(&[Slug::Office, Slug::Canticles])
            .filter_map(|(path, contents)| {
                if let Contents::Document(document) = contents {
                    let canticle_id = match &document.content {
                        Content::Canticle(canticle) => Some(canticle.number),
                        _ => None,
                    };

                    ((version.is_none() || version == Some(document.version))
                        && (search.is_empty() || document.contains_case_insensitive(&search)))
                    .then(|| {
                        (
                            canticle_id,
                            document.version,
                            path,
                            document.best_label().unwrap_or_default(),
                        )
                    })
                } else {
                    None
                }
            })
            .dedup_by(|(a_id, a_version, _, _), (b_id, b_version, _, _)| {
                (a_id, a_version) == (b_id, b_version)
            })
            .collect::<Vec<_>>();

        Some(Self {
            locale: locale.to_string(),
            canticle_number: query.canticle,
            raw_redirect: query.redirect,
            redirect,
            canticles,
            search,
            version,
        })
    }
}

impl View for CanticleChoice {
    fn title(&self) -> String {
        format!("{} – {}", t!("category.canticle"), t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("../../styles/toggle-links.css").into(),
            include_str!("canticle_choice.css").into(),
        ]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        let search_query = if self.search.is_empty() {
            "".to_string()
        } else {
            format!("&search={}", self.search)
        };

        let redirect_query = self.redirect.query_pairs().collect::<HashMap<_, _>>();
        let redirect_prefs = redirect_query
            .get("prefs")
            .and_then(|prefs| {
                serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(&prefs).ok()
            })
            .unwrap_or_default();
        let canticle_pref_key = match self.canticle_number {
            CanticleNumber::One => PreferenceKey::Global(GlobalPref::CanticleOne),
            CanticleNumber::Two => PreferenceKey::Global(GlobalPref::CanticleTwo),
        };
        let other_prefs = redirect_prefs
            .iter()
            .filter(|(key, _)| key != &canticle_pref_key)
            .collect::<Vec<_>>();
        let other_query = itertools::Itertools::intersperse_with(
            redirect_query
                .into_iter()
                .filter(|(k, _)| k != "prefs")
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(&v))),
            || String::from("&"),
        )
        .collect::<String>();

        view! {
            <div>
                <header><h1>{t!("category.canticle")}</h1></header>
                <main>
                    <div class="toggle-links">
                        <a
                            class:current={self.version == None}
                            href={format!("/{}/canticle-choice?canticle={}{}&redirect={}", self.locale, self.canticle_number, search_query, self.raw_redirect)}
                        >
                            "—"
                        </a>
                        <a
                            class:current={self.version == Some(Version::RiteI)}
                            href={format!("/{}/canticle-choice/RiteI?canticle={}{}&redirect={}", self.locale, self.canticle_number, search_query, self.raw_redirect)}
                        >
                            {t!("rite_i")}
                        </a>
                        <a
                            class:current={self.version == Some(Version::RiteII)}
                            href={format!("/{}/canticle-choice/RiteII?canticle={}{}&redirect={}", self.locale, self.canticle_number, search_query, self.raw_redirect)}
                        >
                            {t!("rite_ii")}
                        </a>
                        <a
                            class:current={self.version == Some(Version::EOW)}
                            href={format!("/{}/canticle-choice/EOW?canticle={}{}&redirect={}", self.locale, self.canticle_number, search_query, self.raw_redirect)}
                        >
                            {t!("eow")}
                        </a>
                    </div>
                    <form>
                        <input type="hidden" name="canticle" value={self.canticle_number} />
                        <input type="hidden" name="redirect" value={self.raw_redirect} />
                        <input type="search" placeholder={t!("search")} value={self.search} name="search" />
                    </form>
                    <ul class="canticle-links">
                        {self.canticles
                            .into_iter()
                            .map(|(_, version, path, label)| {
                                let href = build_redirect_url(&self.redirect, path, &other_query, &other_prefs, &canticle_pref_key);
                                let version = match (self.version, version) {
                                    (None, Version::RiteI) => t!("rite_i"),
                                    (None, Version::RiteII) => t!("rite_ii"),
                                    (None, Version::EOW) => t!("eow"),
                                    _ => "".to_string()
                                };

                                view! {
                                    <li>
                                        <a href={href}>{label}</a> " " {version}
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </main>
            </div>
        }
    }
}

fn build_redirect_url(
    redirect_url: &url::Url,
    canticle_choice: SlugPath,
    other_query: &str,
    other_prefs: &[&(PreferenceKey, PreferenceValue)],
    canticle_pref_key: &PreferenceKey,
) -> String {
    let mut prefs = other_prefs.to_vec();
    let pref = (
        canticle_pref_key.clone(),
        PreferenceValue::Canticle(canticle_choice),
    );
    prefs.push(&pref);
    let prefs = serde_json::to_string(&prefs).unwrap_or_default();
    let prefs = urlencoding::encode(&prefs);
    let prefs = if other_query.is_empty() {
        format!("?prefs={prefs}")
    } else {
        format!("&prefs={prefs}")
    };
    let path = redirect_url.path();
    let query = format!("{other_query}{prefs}");
    let fragment = match redirect_url.fragment() {
        Some(fragment) => format!("#{fragment}"),
        None => "".into(),
    };
    format!("{path}?{query}{fragment}")
}
