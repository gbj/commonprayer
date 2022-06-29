use leptos2::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, EnumString, Display, Hash,
)]
pub enum DarkMode {
    #[default]
    Auto,
    Dark,
    Light,
}
