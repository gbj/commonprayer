use futures::StreamExt;
use leptos::*;

pub struct Alert {
    pub is_open: Behavior<bool>,
    pub content: View,
}

impl Alert {
    pub fn new(content: View) -> Self {
        Self {
            is_open: Behavior::new(false),
            content,
        }
    }

    pub fn view(self) -> View {
        view! {
            <>
                <dyn:div class="alert alert-overlay"
                    class:hidden={self.is_open.stream().map(|open| !open).boxed_local()}
                    on:click={
                        let is_open = self.is_open.clone();
                        move |_ev: Event| is_open.set(false)
                    }
                ></dyn:div>
                <dyn:div class="alert alert-content"
                    class:hidden={self.is_open.stream().map(|open| !open).boxed_local()}
                >
                    {self.content}
                </dyn:div>
            </>
        }
    }
}
