use leptos2::*;
use liturgy::{
    BiblicalCitation, BiblicalReading, GlobalPref, PreferenceKey, PreferenceValue, Version,
};
use reference_parser::{BibleVerse, BibleVersePart, Book};

use crate::{
    preferences,
    utils::{
        fetch::{fetch, Fetch, FetchError, FetchMsg, FetchStatus},
        reading_loader::*,
    },
    views::document::biblical_reading_verses,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct BiblicalCitationLoader {
    pub locale: String,
    #[prop]
    pub citation: BiblicalCitation,
    pub version: Version,
    #[prop]
    pub path: Vec<usize>,
    state: FetchStatus<BiblicalReading>,
}

#[derive(Clone, Debug)]
pub enum BiblicalCitationMsg {
    LoadReading,
    FetchError(FetchError),
    FetchResult(BiblicalReading),
}

impl State for BiblicalCitationLoader {
    type Msg = BiblicalCitationMsg;

    fn init(&self) -> Option<Cmd<Self>> {
        Some(Cmd::new(|_, link| {
            link.send(&Self::Msg::LoadReading);
        }))
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            BiblicalCitationMsg::LoadReading => {
                if !self.citation.citation.is_empty() {
                    self.state = FetchStatus::Loading;
                    return Some(self.load_reading(&self.citation, self.version));
                }
            }
            BiblicalCitationMsg::FetchError(e) => self.state = FetchStatus::Error(e),
            BiblicalCitationMsg::FetchResult(r) => self.state = FetchStatus::Success(Box::new(r)),
        };
        None
    }
}

impl Component for BiblicalCitationLoader {
    fn view(&self) -> Host {
        let content = match &self.state {
            FetchStatus::Idle => view! { <p class="loading">{t!("loading")}</p> },
            FetchStatus::Loading => view! { <p class="loading">{t!("loading")}</p> },
            FetchStatus::Error(_) => {
                view! { <p class="error">{t!("biblical_citation.error", citation = &self.citation.citation)}</p> }
            }
            FetchStatus::Success(reading) => {
                let verses = biblical_reading_verses(reading);
                view! {
                    <div>{verses}</div>
                }
            }
        };

        view! {
            <Host>
                {content}
            </Host>
        }
    }
}

impl BiblicalCitationLoader {
    fn load_reading(&self, citation: &BiblicalCitation, version: Version) -> Cmd<Self> {
        let version = if version.is_bible_translation() {
            version
        } else {
            preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
                .and_then(|value| match value {
                    PreferenceValue::Version(version) => Some(version),
                    _ => None,
                })
                .unwrap_or(Version::NRSV)
        };
        let url = reading_url(&citation.citation, version);
        let citation = citation.clone();
        Cmd::new(move |_, link| {
            let citation = citation.clone();
            let link = link.clone();
            spawn_local(async move {
                match fetch::<BibleReadingFromAPI>(&url, None).await {
                    Ok(res) => link.send(&BiblicalCitationMsg::FetchResult(
                        res.api_data_to_biblical_reading(&citation.citation, citation.intro),
                    )),
                    Err(e) => link.send(&BiblicalCitationMsg::FetchError(e)),
                };
            });
        })
    }
}
