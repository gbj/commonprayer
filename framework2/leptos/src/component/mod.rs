use async_trait::async_trait;
use std::fmt::Debug;

use crate::{EventEmitter, Host};

pub mod link;

#[async_trait(?Send)]
pub trait Component
where
    Self: Default + Debug + Sized + PartialEq + 'static,
{
    type Msg: Debug;
    type Cmd: Debug;

    fn init(&self) -> Option<Self::Cmd> {
        None
    }

    fn update(&mut self, msg: &Self::Msg) -> Option<Self::Cmd>;

    async fn cmd(cmd: Self::Cmd, host: web_sys::HtmlElement) -> Option<Self::Msg>;

    fn view(&self) -> Host;
}
