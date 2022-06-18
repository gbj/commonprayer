use std::sync::Arc;

use leptos2::*;

mod display;
mod general;
mod liturgy;

pub use self::liturgy::*;
pub use display::*;
pub use general::*;

pub struct SettingsView {
    locale: String,
    path: String,
}

#[async_trait(?Send)]
impl Loader for SettingsView {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        Some(Self {
            locale: locale.to_string(),
            path: req.path().to_string(),
        })
    }
}

impl View for SettingsView {
    fn title(&self) -> String {
        t!("settings.title")
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("../../styles/toggle-links.css").into(),
            include_str!("settings.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        view! {
            <>
                <header><h1>{t!("settings.title")}</h1></header>
                <main>
                    <div class="toggle-links">
                        <a
                            href={format!("/{}/settings", self.locale)}
                            class:current={self.path.ends_with("settings")}
                        >
                            {t!("settings.general")}
                        </a>
                         <a
                            href={format!("/{}/settings/display", self.locale)}
                            class:current={self.path.ends_with("display")}
                        >
                            {t!("settings.display_settings.title")}
                        </a>
                        <a
                            href={format!("/{}/liturgy", self.locale)}
                            class:current={self.path.contains("/liturgy")}
                        >
                            {t!("settings.liturgy")}
                        </a>
                    </div>
                    {nested_view}
                </main>
            </>
        }
    }
}
