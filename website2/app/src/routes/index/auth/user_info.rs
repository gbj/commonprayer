use leptos2::{Arc, Cookies, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::routes::index::auth::validate_token;

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

    pub async fn verified_id(req: Arc<dyn Request>) -> Option<String> {
        if let Some(unverified) = Self::get_untrusted(&req) {
            unverified.to_verified_id().await
        } else {
            None
        }
    }

    pub async fn to_verified_id(&self) -> Option<String> {
        let token = validate_token(&self.token).await;
        match token {
            Ok(token) => token
                .claims
                .get("user_id")
                .and_then(|user_id| match user_id {
                    Value::String(uid) => Some(uid.clone()),
                    _ => None,
                }),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}
