use leptos2::*;
use wasm_bindgen::UnwrapThrowExt;

pub enum RedirectMode {
    DailyOffice,
    Eucharist,
}

impl std::fmt::Display for RedirectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RedirectMode::DailyOffice => "office",
                RedirectMode::Eucharist => "lectionary",
            }
        )
    }
}

pub fn redirect_to_date(locale: &str, date: String, mode: RedirectMode) {
    location()
        .set_href(&format!("/{locale}/readings/{mode}/{date}"))
        .unwrap_throw();
}

pub fn redirect_script(locale: &str, page: &str, should_redirect: bool) -> Node {
    if should_redirect {
        let js = format!(
            r#"
            const now = new Date(),
                formatted = `${{now.getFullYear()}}-${{now.getMonth() + 1}}-${{now.getDate()}}`;
            window.location.href = `/{}/{}/${{formatted}}`;
        "#,
            locale, page
        );
        view! {
            <script>{js}</script>
        }
    } else {
        text("")
    }
}
