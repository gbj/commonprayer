use crate::preferences::{self, DisplaySettings};
use futures::StreamExt;
use leptos::*;
use wasm_bindgen::JsCast;

pub struct DisplaySettingsComponent {
    pub current_settings: Behavior<DisplaySettings>,
}

impl DisplaySettingsComponent {
    pub fn new() -> Self {
        let initial_settings = preferences::get_display_settings().unwrap_or_default();
        Self {
            current_settings: Behavior::new(initial_settings),
        }
    }

    pub fn view(&self) -> View {
        let initial_settings = self.current_settings.get();
        let bible_verses = Behavior::new(initial_settings.bible_verses);
        let psalm_verses = Behavior::new(initial_settings.psalm_verses);

        (bible_verses.stream(), psalm_verses.stream())
            .lift()
            .create_effect({
                let current_settings = self.current_settings.clone();
                move |(bible_verses, psalm_verses)| {
                    let new_settings = DisplaySettings {
                        bible_verses: bible_verses.unwrap_or_default(),
                        psalm_verses: psalm_verses.unwrap_or_default(),
                    };
                    preferences::set_display_settings(&new_settings);
                    current_settings.set(new_settings);
                }
            });

        view! {
            <section class="display-settings">
                <label>
                    {t!("settings.display_settings.bible_verses")}
                    <dyn:input type="checkbox"
                        checked={bible_verses.stream().map(|n| if n { Some("checked".to_string()) } else { None }).boxed_local()}
                        on:change={move |ev: Event| bible_verses.set(event_target_checked(ev))}
                    />
                </label>
                <label>
                    {t!("settings.display_settings.psalm_verses")}
                    <dyn:input type="checkbox"
                        checked={psalm_verses.stream().map(|n| if n { Some("checked".to_string()) } else { None }).boxed_local()}
                        on:change={move |ev: Event| psalm_verses.set(event_target_checked(ev))}
                    />
                </label>
            </section>
        }
    }
}

fn event_target_checked(ev: Event) -> bool {
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .checked()
}
