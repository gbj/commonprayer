use async_trait::async_trait;
use std::fmt::Debug;

use crate::{Host, State};

pub mod link;

#[async_trait(?Send)]
pub trait Component
where
    Self: State + Default + Debug + Sized + PartialEq + 'static,
{
    fn view(&self) -> Host;

    fn should_render(&self, msg: &Self::Msg) -> bool {
        true
    }
}
