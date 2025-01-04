use std::sync::Arc;

use futures::Future;

use crate::{text, AsyncElement, Element, Node, StaticView};

pub trait IntoChildren {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>>;
}

impl IntoChildren for Node {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(std::iter::once(self))
    }
}

impl IntoChildren for Element {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(std::iter::once(Node::Element(self)))
    }
}

impl IntoChildren for Vec<Node> {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(self.into_iter())
    }
}

impl IntoChildren for &Vec<Node> {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(self.clone().into_iter())
    }
}

impl IntoChildren for Option<Node> {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(self.into_iter())
    }
}

impl<T> IntoChildren for T
where
    T: StaticView,
{
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(std::iter::once(self.to_node()))
    }
}

impl IntoChildren for String {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(std::iter::once(text(self)))
    }
}

impl<'a> IntoChildren for std::borrow::Cow<'a, str> {
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(std::iter::once(text(self)))
    }
}

impl<F> IntoChildren for (Node, F)
where
    F: Future<Output = Node> + 'static,
{
    fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
        let (pending, ready) = self;
        let ready = Box::pin(ready);
        Box::new(std::iter::once(Node::AsyncElement(AsyncElement {
            pending: Box::new(pending),
            ready: Some(ready),
        })))
    }
}

macro_rules! child_type {
    ($child_type:ty) => {
        impl IntoChildren for $child_type {
            fn into_children(self) -> Box<dyn Iterator<Item = Node>> {
                Box::new(std::iter::once(text(self.to_string())))
            }
        }
    };
}

child_type!(&String);
child_type!(&str);
child_type!(usize);
child_type!(u8);
child_type!(u16);
child_type!(u32);
child_type!(u64);
child_type!(u128);
child_type!(isize);
child_type!(i8);
child_type!(i16);
child_type!(i32);
child_type!(i64);
child_type!(i128);
child_type!(f32);
child_type!(f64);
child_type!(char);
