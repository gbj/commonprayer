use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Names for events fired by different components
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Display)]
pub enum Events {
    DatePicker,
    ModalState,
    ToggleChange,
}
