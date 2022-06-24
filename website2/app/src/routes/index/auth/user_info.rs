use leptos2::{Arc, Cookies, Request};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct UserInfo {
    #[serde(alias = "displayName")]
    pub display_name: Option<String>,
    pub email: Option<String>,
    #[serde(alias = "photoURL")]
    pub photo_url: Option<String>,
    #[serde(alias = "providerId")]
    pub provider_id: String,
    pub uid: String,
    pub token: String,
}

impl UserInfo {
    /// There is *always* the potential that this is fake user data set in a cookie by the client.
    /// It should never be trusted for actual authentication, but is convenient for SSR of display name
    /// or avatar URL. Actual, trusted user information needs to be handled with JWT authentication.
    pub fn get_untrusted(req: &Arc<dyn Request>) -> Option<Self> {
        req.headers()
            .cookies()
            .filter_map(|cookie| match cookie {
                Ok(cookie) => Some(cookie),
                Err(e) => {
                    eprintln!("invalid cookie: {:#?}", e);
                    None
                }
            })
            .find(|cookie| cookie.name() == "untrusted-user")
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
    }
}
