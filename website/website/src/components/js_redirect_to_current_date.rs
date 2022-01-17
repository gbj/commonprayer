use leptos::*;

pub fn redirect_script(locale: &str, page: &str, should_redirect: bool) -> View {
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
        View::Empty
    }
}
