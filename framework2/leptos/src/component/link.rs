use std::{any::Any, rc::Rc};

use futures::channel::mpsc::UnboundedSender;

use crate::{Component, ComponentError};

#[derive(Clone)]
pub struct Link(Rc<dyn AnyLink>);

impl Link {
    pub fn send(&self, msg: &dyn Any) -> Result<(), ComponentError> {
        self.0.send_any(msg)
    }
}

impl<T> From<T> for Link
where
    T: AnyLink + 'static,
{
    fn from(link: T) -> Self {
        Self(Rc::new(link))
    }
}

pub trait AnyLink {
    fn send_any(&self, msg: &dyn Any) -> Result<(), ComponentError>;
}

impl<T> AnyLink for ComponentLink<T>
where
    T: Component,
    T::Msg: Clone,
{
    fn send_any(&self, msg: &dyn Any) -> Result<(), ComponentError> {
        if let Some(msg) = msg.downcast_ref::<T::Msg>() {
            self.send(msg)
        } else {
            Err(ComponentError::MsgMismatch)
        }
    }
}

pub struct ComponentLink<T>
where
    T: Component,
{
    pub(crate) tx: UnboundedSender<Option<T::Msg>>,
}

impl<T> Clone for ComponentLink<T>
where
    T: Component,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<T> ComponentLink<T>
where
    T: Component,
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

impl<T> From<UnboundedSender<Option<T::Msg>>> for ComponentLink<T>
where
    T: Component,
{
    fn from(tx: UnboundedSender<Option<T::Msg>>) -> Self {
        Self { tx }
    }
}
