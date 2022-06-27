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
