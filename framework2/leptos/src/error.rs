use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("failed while attempting to send a Msg via Link")]
    Link(String),
    #[error("tried to pass a Msg of the wrong type")]
    MsgMismatch,
    #[error("called AnyView::update with a state of the wrong type")]
    StateTypeMismatch,
}
