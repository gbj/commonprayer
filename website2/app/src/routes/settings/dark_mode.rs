use leptos2::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, EnumString, Display)]
pub enum DarkMode {
    #[default]
    Auto,
    Dark,
    Light,
}
