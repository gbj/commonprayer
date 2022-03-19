use std::{cell::RefCell, pin::Pin, rc::Rc, task::Poll};

use futures::{
    channel::mpsc::{self, UnboundedSender},
    Stream, StreamExt,
};
use pin_project_lite::pin_project;
use wasm_bindgen_futures::spawn_local;

use crate::is_server;

// Reactive signals
#[derive(Clone)]
pub struct Behavior<T>
where
    T: Clone,
{
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<UnboundedSender<T>>>>,
}

impl<T> Behavior<T>
where
    T: Clone,
{
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(initial_value)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    pub fn set(&self, value: T) {
        for subscriber in self.subscribers.borrow().iter() {
            subscriber.unbounded_send(value.clone()).unwrap();
        }
        *self.value.borrow_mut() = value;
    }

    pub fn update(&self, update_fn: impl Fn(&mut T)) {
        let mut current = self.get();
        (update_fn)(&mut current);
        self.set(current);
    }

    pub fn stream(&self) -> impl Stream<Item = T>
    where
        T: 'static,
    {
        let (tx, rx) = mpsc::unbounded();
        self.subscribers.borrow_mut().push(tx);
        rx.start_with(self.get())
    }
}

// Trait to start a stream with a particular value, so that it emits immediately when first polled
pub trait StartWithStreamExt {
    type Item;

    fn start_with(self, initial_value: Self::Item) -> StartWith<Self::Item>;
}

impl<T, S> StartWithStreamExt for S
where
    S: Stream<Item = T> + 'static,
{
    type Item = T;

    fn start_with(self, initial_value: Self::Item) -> StartWith<Self::Item> {
        StartWith {
            stream: Box::pin(self),
            initial_value: Some(initial_value),
        }
    }
}

impl<T> Stream for StartWith<T> {
    type Item = T;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if let Some(initial_value) = this.initial_value.take() {
            Poll::Ready(Some(initial_value))
        } else {
            this.stream.as_mut().poll_next(cx)
        }
    }
}

pin_project! {
    #[must_use="streams do nothing unless polled"]
    pub struct StartWith<T> {
        stream: Pin<Box<dyn Stream<Item = T>>>,
        initial_value: Option<T>
    }
}

// Trait to combine two streams, yielding a new stream with a tuple of their latest values
pub trait LiftStreamExt {
    type A;
    type B;

    fn lift(self) -> Lift<Self::A, Self::B>;
}

impl<A, B, T, U> LiftStreamExt for (T, U)
where
    T: Stream<Item = A> + 'static,
    U: Stream<Item = B> + 'static,
    A: Clone,
    B: Clone,
{
    type A = A;
    type B = B;

    fn lift(self) -> Lift<Self::A, Self::B> {
        Lift::new(self.0, self.1)
    }
}

pin_project! {
    #[must_use="streams do nothing unless polled"]
    pub struct Lift<A, B> {
        has_sent_initial_value: bool,
        last_a: Option<A>,
        last_b: Option<B>,
        stream_a: Pin<Box<dyn Stream<Item = A>>>,
        stream_b: Pin<Box<dyn Stream<Item = B>>>,
    }
}

impl<A, B> Lift<A, B>
where
    A: Clone,
    B: Clone,
{
    pub fn new(
        stream_a: impl Stream<Item = A> + 'static,
        stream_b: impl Stream<Item = B> + 'static,
    ) -> Self {
        Self {
            has_sent_initial_value: false,
            last_a: None,
            last_b: None,
            stream_a: stream_a.boxed_local(),
            stream_b: stream_b.boxed_local(),
        }
    }
}

impl<A, B> Stream for Lift<A, B>
where
    A: Clone,
    B: Clone,
{
    type Item = (Option<A>, Option<B>);

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match (
            this.stream_a.as_mut().poll_next(cx),
            this.stream_b.as_mut().poll_next(cx),
        ) {
            (Poll::Ready(new_a), Poll::Ready(new_b)) => {
                *this.last_a = new_a.clone();
                *this.last_b = new_b.clone();
                Poll::Ready(Some((new_a, new_b)))
            }
            (Poll::Ready(new_a), Poll::Pending) => {
                *this.last_a = new_a.clone();
                Poll::Ready(Some((new_a, this.last_b.clone())))
            }
            (Poll::Pending, Poll::Ready(new_b)) => {
                *this.last_b = new_b.clone();
                Poll::Ready(Some((this.last_a.clone(), new_b)))
            }
            (Poll::Pending, Poll::Pending) => {
                if *this.has_sent_initial_value {
                    Poll::Pending
                } else {
                    *this.has_sent_initial_value = true;
                    Poll::Ready(Some((this.last_a.clone(), this.last_b.clone())))
                }
            }
        }
    }
}

pub trait EffectExt {
    type Item;

    fn create_effect(self, cb: impl Fn(Self::Item) + 'static);
}

impl<T, U> EffectExt for T
where
    T: Stream<Item = U> + 'static,
    U: 'static,
{
    type Item = U;

    fn create_effect(self, cb: impl Fn(Self::Item) + 'static) {
        let mut stream = self.boxed_local();
        if !is_server!() {
            spawn_local(async move {
                while let Some(value) = stream.next().await {
                    (cb)(value);
                }
            })
        }
    }
}
