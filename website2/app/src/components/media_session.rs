use leptos2::*;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MediaSession {
    pub active: bool,
    pub actions: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: f32,
    pub playback_rate: f32,
    pub playback_state: String,
    pub position: f32,
    host: Option<web_sys::HtmlElement>,
}

impl DeclarativeWebApi for MediaSession {
    fn tag() -> &'static str {
        "l-media-session"
    }

    fn attributes() -> &'static [&'static str] {
        &[
            "active",
            "actions",
            "artist",
            "title",
            "album",
            "duration",
            "playback_rate",
            "playback-rate",
            "playback_state",
            "playback-state",
            "position",
        ]
    }

    fn init(&mut self, host: web_sys::HtmlElement) {
        leptos2::warn("MediaSession init fn");
        self.host = Some(host);
    }

    fn set_attribute(&mut self, attr_name: String, new_value: Option<String>) {
        if !is_server!() {
            match attr_name.as_str() {
                "active" => {
                    if new_value.is_some() {
                        self.active = true;
                    } else {
                        self.active = false;
                    }
                }
                "actions" => {
                    leptos2::warn(&format!("loading actions: {:?}", new_value));
                    if let Some(host) = &self.host {
                        leptos2::warn(&format!("has host {}", host.node_name()));

                        let actions = new_value.unwrap_or_default();
                        for action in actions.split(',') {
                            leptos2::warn(&format!("setting up action {}", action));

                            let emitter = EventEmitter::new(host);
                            let action_name = action.to_string();
                            let handler = move || {
                                leptos2::warn(&format!("action handler {} firing", action_name));
                                emitter.emit(CustomEvent::<()>::new(action_name.clone()).bubbles())
                            };
                            let closure =
                                Closure::wrap(Box::new(handler) as Box<dyn Fn()>).into_js_value();
                            set_media_session_action_handler(action, closure);
                        }
                    }
                }
                "artist" => {
                    self.artist = new_value.unwrap_or_default();
                    if self.active {
                        set_media_metadata(&self.title, &self.artist, &self.album);
                    }
                }
                "title" => {
                    self.title = new_value.unwrap_or_default();
                    if self.active {
                        set_media_metadata(&self.title, &self.artist, &self.album);
                    }
                }
                "album" => {
                    self.album = new_value.unwrap_or_default();
                    if self.active {
                        set_media_metadata(&self.title, &self.artist, &self.album);
                    }
                }
                "duration" => {
                    if let Some(value) = new_value.and_then(|value| value.parse::<f32>().ok()) {
                        self.duration = value;
                        if self.active {
                            set_position_state(self.duration, self.playback_rate, self.position);
                        }
                    }
                }
                "playback-rate" | "playback_rate" => {
                    if let Some(value) = new_value.and_then(|value| value.parse::<f32>().ok()) {
                        self.playback_rate = value;
                        if self.active {
                            set_position_state(self.duration, self.playback_rate, self.position);
                        }
                    }
                }
                "playback-state" | "playback_state" => {
                    self.playback_state = new_value.unwrap_or_else(|| String::from("none"));
                    leptos2::warn(&format!("setting playbackState to {}", self.playback_state));
                    set_playback_state(&self.playback_state);
                }
                "position" => {
                    if let Some(value) = new_value.and_then(|value| value.parse::<f32>().ok()) {
                        self.position = value;
                        if self.active {
                            set_position_state(self.duration, self.playback_rate, self.position);
                        }
                    }
                }
                _ => leptos2::debug_warn(&format!(
                    "[MediaSession API Custom Element] attribute not supported: {}",
                    attr_name
                )),
            }
        }
    }

    fn properties() -> &'static [&'static str] {
        &[]
    }

    fn set_property(&mut self, _attr_name: String, _new_value: JsValue) {}

    fn state_to_attributes(&self) -> Vec<Attribute> {
        [
            if self.active {
                Some(Attribute::Attribute(
                    "active".to_string(),
                    Some(String::new()),
                ))
            } else {
                None
            },
            Some(Attribute::Attribute(
                "actions".to_string(),
                Some(self.actions.clone()),
            )),
            Some(Attribute::Attribute(
                "artist".to_string(),
                Some(self.artist.clone()),
            )),
            Some(Attribute::Attribute(
                "title".to_string(),
                Some(self.title.clone()),
            )),
            Some(Attribute::Attribute(
                "album".to_string(),
                Some(self.album.clone()),
            )),
            Some(Attribute::Attribute(
                "duration".to_string(),
                Some(self.duration.to_string()),
            )),
            Some(Attribute::Attribute(
                "playback-rate".to_string(),
                Some(self.playback_rate.to_string()),
            )),
            Some(Attribute::Attribute(
                "playback-state".to_string(),
                Some(self.playback_state.clone()),
            )),
            Some(Attribute::Attribute(
                "position".to_string(),
                Some(self.position.to_string()),
            )),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[wasm_bindgen(module = "/src/components/media_session.js")]
extern "C" {
    fn set_media_metadata(title: &str, artist: &str, album: &str);

    fn set_media_session_action_handler(action: &str, handler: JsValue);

    fn clear_position_state();

    fn set_position_state(duration: f32, playback_rate: f32, position: f32);

    fn set_playback_state(state: &str);
}
