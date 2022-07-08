use leptos2::*;

mod user_info;
mod verify_jwk;
pub use verify_jwk::*;

pub fn auth_scripts() -> Vec<Node> {
    view! {
        <>
            <script src="https://www.gstatic.com/firebasejs/9.1.3/firebase-app-compat.js"></script>
            <script src="https://www.gstatic.com/firebasejs/9.1.3/firebase-auth-compat.js"></script>
            <script>{include_str!("firebase-config.js")}</script>
            <script src="https://www.gstatic.com/firebasejs/ui/6.0.1/firebase-ui-auth.js"></script>
            <link type="text/css" rel="stylesheet" href="/static/firebase-ui/firebase-ui-auth.css" />
            <script>{include_str!("firebase-ui-setup.js")}</script>
        </>
    }
}
