use std::{
    any::{type_name, Any},
    rc::Rc,
};

use futures::channel::mpsc::UnboundedSender;

use crate::{Component, ComponentError, State};

#[derive(Clone)]
pub struct Link(Option<Rc<dyn AnyLink>>);

impl Link {
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn send(&self, msg: &dyn Any) -> Result<(), ComponentError> {
        if let Some(link) = &self.0 {
            link.send_any(msg)
        } else {
            Err(ComponentError::EmptyLink)
        }
    }

    pub fn type_name(&self) -> &'static str {
        if let Some(link) = &self.0 {
            link.type_name()
        } else {
            "<Empty Link>"
        }
    }
}

impl<T> From<T> for Link
where
    T: AnyLink + 'static,
{
    fn from(link: T) -> Self {
        Self(Some(Rc::new(link)))
    }
}

pub trait AnyLink {
    fn send_any(&self, msg: &dyn Any) -> Result<(), ComponentError>;

    fn type_name(&self) -> &'static str;
}

impl<T> AnyLink for StateLink<T>
where
    T: State,
    T::Msg: Clone,
{
    fn send_any(&self, msg: &dyn Any) -> Result<(), ComponentError> {
        if let Some(msg) = msg.downcast_ref::<T::Msg>() {
            self.send(msg)
        } else {
            Err(ComponentError::MsgMismatch)
        }
    }

    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}

pub struct StateLink<T>
where
    T: State,
{
    pub(crate) tx: UnboundedSender<Option<T::Msg>>,
}

impl<T> Clone for StateLink<T>
where
    T: State,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<T> StateLink<T>
where
    T: State,
    T::Msg: Clone,
{
    pub fn attributes_changed(&self) -> Result<(), ComponentError> {
        self.tx
            .unbounded_send(None)
            .map_err(|e| ComponentError::Link(e.to_string()))
    }

    pub fn send(&self, msg: &T::Msg) -> Result<(), ComponentError> {
        self.tx
            .unbounded_send(Some(msg.clone()))
            .map_err(|e| ComponentError::Link(e.to_string()))
    }
}

impl<T> From<UnboundedSender<Option<T::Msg>>> for StateLink<T>
where
    T: State,
{
    fn from(tx: UnboundedSender<Option<T::Msg>>) -> Self {
        Self { tx }
    }
}
