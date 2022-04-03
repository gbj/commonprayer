use std::{pin::Pin, rc::Rc};

use futures::{Stream, StreamExt};

use crate::View;

pub struct NaiveList<T, I>
where
    I: IntoIterator<Item = T>,
{
    stream: Pin<Box<dyn Stream<Item = I>>>,
    container_template: Rc<dyn Fn(View) -> View>,
    template: Rc<dyn Fn((usize, T)) -> View>,
}

impl<T, I> NaiveList<T, I>
where
    I: IntoIterator<Item = T> + 'static,
    T: 'static,
{
    pub fn new(
        container_template: impl Fn(View) -> View + 'static,
        items: impl Stream<Item = I> + 'static,
        template: impl Fn((usize, T)) -> View + 'static,
    ) -> Self {
        Self {
            stream: items.boxed_local(),
            template: Rc::new(template),
            container_template: Rc::new(container_template),
        }
    }

    pub fn view(self) -> View {
        View::ViewStream(
            self.stream
                .map(move |items| {
                    (self.container_template)(View::Fragment(
                        items
                            .into_iter()
                            .enumerate()
                            .map(|item| (self.template)(item))
                            .collect(),
                    ))
                })
                .boxed_local(),
        )
    }
}
