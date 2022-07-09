use crate::components::MeditationTimer;
use leptos2::*;

pub struct MeditationView {}

#[async_trait(?Send)]
impl Loader for MeditationView {
    type Params = ();
    type Query = ();

    async fn loader(
        _locale: &str,
        _req: Arc<dyn Request>,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        Some(Self {})
    }
}

impl View for MeditationView {
    fn title(&self) -> String {
        format!("{} – {}", t!("meditation.title"), t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![include_str!("meditation.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <header><h1>{t!("meditation.title")}</h1></header>
                <main>
                    <MeditationTimer
                        title={t!("meditation.title")}
                        artist={t!("common_prayer")}
                        pauselabel={t!("meditation.pause")}
                        stoplabel={t!("meditation.stop")}
                        resumelabel={t!("meditation.resume")}
                        minuteslabel={t!("meditation.minutes")}
                        secondslabel={t!("meditation.seconds")}
                        usebelllabel={t!("meditation.use_bell")}
                        beginlabel={t!("meditation.begin")}
                    />
                    <noscript>{t!("meditation.no_script")}</noscript>
                </main>
            </div>
        }
    }
}
