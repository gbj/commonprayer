use leptos2::{Arc, Cookies, Deserialize, Request, Serialize};
use serde_json::Value;

use crate::routes::index::auth::validate_token;
use crate::UserInfo;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UserId(String);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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

    pub async fn verified_id(req: Arc<dyn Request>) -> Option<UserId> {
        if let Some(unverified) = Self::get_untrusted(&req) {
            eprintln!("unverified = {:#?}", unverified);
            unverified.to_verified_id().await
        } else {
            eprintln!("no unverified user found");
            None
        }
    }

    pub async fn to_verified_id(&self) -> Option<UserId> {
        let token = validate_token(&self.token).await;
        match token {
            Ok(token) => token
                .claims
                .get("user_id")
                .and_then(|user_id| match user_id {
                    Value::String(uid) => Some(UserId(uid.clone())),
                    _ => None,
                }),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}
