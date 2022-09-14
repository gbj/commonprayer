use crate::{i18n::use_i18n, icon::Icon};
use leptos::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ModalProps<O, C>
where
    O: Fn() -> bool + 'static,
    C: Fn() + Clone,
{
    open: O,
    on_close: C,
    children: Vec<Element>,
}

#[allow(non_snake_case)]
pub fn Modal<O, C>(cx: Scope, props: ModalProps<O, C>) -> Element
where
    O: Fn() -> bool,
    C: Fn() + Clone,
{
    let ModalProps {
        open,
        on_close,
        children,
    } = props;

    let (t, _) = use_i18n(cx);

    let dialog: Element;

    let tpl = view! {
        <dialog ref=dialog
            on:close={
                let on_close = on_close.clone();
                move |_| { on_close() }
            }
        >
            <form method="dialog"
                on:submit=move |_| { on_close() }
            >
                <button
                    aria-label={t("close")}
                >
                    <img src={Icon::Close.to_string()} alt="X" />
                </button>
            </form>
            <main>{children}</main>
        </dialog>
    };

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let dialog = dialog.unchecked_into::<web_sys::HtmlDialogElement>();

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    create_effect(cx, move |_| {
        if open() {
            dialog.set_open(true);
        } else {
            dialog.set_open(false);
        }
    });

    tpl
}
