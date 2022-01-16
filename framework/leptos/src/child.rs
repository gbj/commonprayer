use std::pin::Pin;

use futures::Stream;

use crate::View;

pub trait IntoViewChild {
    fn into_view_child(self) -> View;
}

impl IntoViewChild for &str {
    fn into_view_child(self) -> View {
        View::StaticText(self.to_string())
    }
}

impl IntoViewChild for &String {
    fn into_view_child(self) -> View {
        View::StaticText(self.clone())
    }
}

impl IntoViewChild for String {
    fn into_view_child(self) -> View {
        View::StaticText(self)
    }
}

impl IntoViewChild for View {
    fn into_view_child(self) -> View {
        self
    }
}

impl IntoViewChild for Pin<Box<dyn Stream<Item = View>>> {
    fn into_view_child(self) -> View {
        View::ViewStream(self)
    }
}

pub trait IntoDynamicViewChild {
    fn into_dynamic_view_child(self) -> View;
}

impl IntoDynamicViewChild for String {
    fn into_dynamic_view_child(self) -> View {
        View::StaticText(self)
    }
}

impl IntoDynamicViewChild for &str {
    fn into_dynamic_view_child(self) -> View {
        View::StaticText(self.to_string())
    }
}

impl IntoDynamicViewChild for Pin<Box<dyn Stream<Item = String>>> {
    fn into_dynamic_view_child(self) -> View {
        View::DynamicText(self)
    }
}

impl IntoDynamicViewChild for View {
    fn into_dynamic_view_child(self) -> View {
        self
    }
}

impl IntoDynamicViewChild for Pin<Box<dyn Stream<Item = View>>> {
    fn into_dynamic_view_child(self) -> View {
        View::ViewStream(self)
    }
}
