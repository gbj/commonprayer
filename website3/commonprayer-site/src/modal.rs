use crate::{i18n::use_i18n, icon::Icon};
use leptos::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ModalProps<O, C, D>
where
    O: Fn() -> bool + 'static,
    C: Fn() + Clone,
    D: IntoVec<Element> + 'static,
{
    open: O,
    on_close: C,
    children: Box<dyn Fn() -> D>,
}

#[allow(non_snake_case)]
pub fn Modal<O, C, D>(cx: Scope, props: ModalProps<O, C, D>) -> Element
where
    O: Fn() -> bool,
    C: Fn() + Clone + 'static,
    D: IntoVec<Element> + 'static,
{
    let ModalProps {
        open,
        on_close,
        children,
    } = props;

    let (t, _, _) = use_i18n(cx);

    let dialog: Element;

    let tpl = view! { cx,
        <dialog ref=dialog
            class="Modal"
            // call the on_close callback when the close event fires
            on:close={
                let on_close = on_close.clone();
                move |_| { on_close() }
            }
            // clicking on ::backdrop should dismiss modal
            on:click={
                let dialog = dialog.clone();
                move |ev| {
                    let ev = ev.unchecked_into::<web_sys::MouseEvent>();
                    let rect = dialog
                        .unchecked_ref::<web_sys::HtmlElement>()
                        .get_bounding_client_rect();
                    let click_is_in_dialog = rect.top() <= ev.client_y() as f64
                        && ev.client_y() as f64 <= rect.top() + rect.height()
                        && rect.left() <= ev.client_x() as f64
                        && ev.client_x() as f64 <= rect.left() + rect.width();
                    if !click_is_in_dialog {
                        ev.target().unwrap().unchecked_into::<web_sys::HtmlDialogElement>().close();
                    }
                }
            }
        >
            <header class="Modal-header">
                <form method="dialog"
                    on:submit=move |_| on_close()
                >
                    <button
                        class="Modal-header-close"
                        aria-label=t("close")
                    >
                        <img src=Icon::Close.to_string() alt="X" />
                    </button>
                </form>
            </header>
            <main class="Modal-content">
                {move || children().into_vec()}
            </main>
        </dialog>
    };

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let dialog = dialog.unchecked_into::<web_sys::HtmlDialogElement>();

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    create_effect(cx, move |_| {
        if open() {
            if dialog.show_modal().is_err() {
                log::warn!("<Modal/> error while calling HTMLDialogElement.showModal()");
                dialog.set_open(true);
            }
        } else {
            dialog.close();
        }
    });

    tpl
}
