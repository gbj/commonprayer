use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Color {
    Purple,
    Blue,
    Gold,
    White,
    Green,
    Red,
}
