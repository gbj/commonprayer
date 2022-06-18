use crate::components::MediaSession;
use crate::views::{Header, Icon};
use leptos2::*;
use std::time::Duration;

pub struct MeditationView {}

#[async_trait(?Send)]
impl Loader for MeditationView {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        Some(Self {})
    }
}

impl View for MeditationView {
    fn title(&self) -> String {
        format!("{} – {}", t!("meditation.title"), t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![include_str!("meditation.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        view! {
            <div>
                <header><h1>{t!("meditation.title")}</h1></header>
                <main>
                    <MeditationTimer/>
                    <noscript>{t!("meditation.no_script")}</noscript>
                </main>
            </div>
        }
    }
}

#[derive(Clone, Debug, PartialEq, WebComponent)]
pub struct MeditationTimer {
    /// URL of the bell audio file
    pub bell: String,
    total_time: Duration,
    elapsed_time: Duration,
    state: MeditationTimerState,
    audio: Option<web_sys::HtmlAudioElement>,
    error: Option<MeditationTimerError>,
}

impl Default for MeditationTimer {
    fn default() -> Self {
        // total time defaults to 5 mins
        let total_time = Duration::from_secs(5 * 60);
        let elapsed_time = Duration::from_secs(0);
        Self {
            bell: "/static/audio/singing-bowl.mp3".to_string(),
            total_time,
            elapsed_time,
            audio: None,
            state: MeditationTimerState::Setup { bell: true },
            error: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MeditationTimerState {
    Setup {
        bell: bool,
    },
    Playing {
        interval_handle: Option<IntervalHandle>,
    },
    Paused,
    Finished,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MeditationTimerMsg {
    SetMinutes(u16),
    SetSeconds(u16),
    SetBell(bool),
    SetIntervalHandle(IntervalHandle),
    Tick,
    Start,
    Edit,
    Pause,
    Resume,
    Stop,
    Error(MeditationTimerError),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MeditationTimerError {
    Bell,
    Interval,
}

#[async_trait(?Send)]
impl State for MeditationTimer {
    type Msg = MeditationTimerMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        leptos2::warn(&format!("[MeditationTimer::update] {:#?}", msg));
        match (&mut self.state, msg) {
            // Setup: Change initial settings
            (MeditationTimerState::Setup { .. }, Self::Msg::SetMinutes(new_minutes)) => {
                let current_secs = self.total_time.as_secs() % 60;
                self.total_time = Duration::from_secs((new_minutes as u64) * 60 + current_secs);
            }
            (MeditationTimerState::Setup { .. }, Self::Msg::SetSeconds(new_seconds)) => {
                let current_mins = self.total_time.as_secs() / 60;
                self.total_time = Duration::from_secs(current_mins * 60 + (new_seconds as u64));
            }
            (MeditationTimerState::Setup { bell, .. }, Self::Msg::SetBell(b)) => *bell = b,

            // Transition from setup to playing
            (MeditationTimerState::Setup { bell }, Self::Msg::Start) => {
                // create audio element and, if it can't be created, note that error
                self.audio = if *bell {
                    if let Ok(audio) = self.create_bell() {
                        Some(audio)
                    } else {
                        self.error = Some(MeditationTimerError::Bell);
                        None
                    }
                } else {
                    None
                };

                // this will do nothing if we've chosen not to use the bell, or have an error
                self.play_bell();

                // interval handle will be created by the command below
                self.state = MeditationTimerState::Playing {
                    interval_handle: None,
                };

                return Some(self.start_timer());
            }

            // Playing: tick decrements the total remaining
            (MeditationTimerState::Playing { .. }, Self::Msg::Tick) => {
                // increment elapsed time
                self.elapsed_time += Duration::from_secs(1);

                // if full time has elapsed, play bell and transition
                if self.elapsed_time >= self.total_time {
                    self.play_bell();
                }
            }

            // Pause playing
            (MeditationTimerState::Playing { interval_handle }, Self::Msg::Pause) => {
                // clear timer
                if let Some(handle) = interval_handle.take() {
                    handle.clear();
                }

                self.pause_bell();

                self.state = MeditationTimerState::Paused;
            }

            // Resume playing
            (MeditationTimerState::Paused, Self::Msg::Resume) => {
                self.play_bell();

                // interval handle created by command
                self.state = MeditationTimerState::Playing {
                    interval_handle: None,
                };

                return Some(self.start_timer());
            }

            // Finish session
            (MeditationTimerState::Playing { interval_handle }, Self::Msg::Stop) => {
                // clear timer
                if let Some(handle) = interval_handle.take() {
                    handle.clear();
                }
                self.state = MeditationTimerState::Finished;
            }

            // Set interval handle, which has been returned by the command
            (
                MeditationTimerState::Playing { interval_handle },
                Self::Msg::SetIntervalHandle(handle),
            ) => *interval_handle = Some(handle),
            _ => {}
        }
        None
    }
}

impl Component for MeditationTimer {
    fn view(&self) -> Host {
        let state = if let MeditationTimerState::Setup { bell } = &self.state {
            self.form(*bell)
        } else {
            // action buttons (Begin/Pause/Resume)
            let action_button = match &self.state {
                MeditationTimerState::Playing { .. } if self.elapsed_time <= self.total_time => {
                    Some(view! {
                        <button on:click=|_| MeditationTimerMsg::Pause>
                            <img src={Icon::Pause} alt=""/>
                            {t!("meditation.pause")}
                        </button>
                    })
                }
                MeditationTimerState::Playing { .. } if self.elapsed_time > self.total_time => {
                    Some(view! {
                        <button on:click=|_| MeditationTimerMsg::Stop>
                            <img src={Icon::Stop} alt=""/>
                            {t!("meditation.stop")}
                        </button>
                    })
                }
                MeditationTimerState::Paused => Some(view! {
                    <button on:click=|_| MeditationTimerMsg::Resume>
                        <img src={Icon::Play} alt=""/>
                        {t!("meditation.resume")}
                    </button>
                }),
                _ => None,
            };

            let remaining = if self.elapsed_time > self.total_time {
                self.elapsed_time - self.total_time
            } else {
                self.total_time - self.elapsed_time
            }
            .as_secs();
            let minutes = remaining / 60;
            let seconds = remaining % 60;
            let progress_percent = self.percent_progress();
            let remaining_percent = 100.0 - progress_percent;

            view! {
                <div class="progress-circle-container">
                    <div class="progress-circle"
                        style={format!("background: conic-gradient(var(--link-color) {}%, 0, #ecf0f1 {}%)", progress_percent, remaining_percent)}
                    >
                        <div class="circle-overlay">
                            <div class="seconds">
                                {if self.elapsed_time > self.total_time { "+" } else { "" }}
                                {minutes}
                                ":"
                                {format!("{:0>2}", seconds)}
                            </div>
                            <div class="controls">
                                {action_button}
                            </div>
                        </div>
                    </div>
                </div>
            }
        };

        view! {
            <Host>
                <style>{include_str!("meditation_timer.css")}</style>
                {state}
                <MediaSession
                    active={matches!(self.state, MeditationTimerState::Playing { .. } | MeditationTimerState::Paused { .. })}
                    actions={"play,pause"}
                    title={t!("meditation.title")}
                    artist={t!("common_prayer")}
                    duration={self.total_time.as_secs()}
                    position={self.elapsed_time.as_secs()}
                    playback-rate={1.0}
                    playback-state={if matches!(self.state, MeditationTimerState::Playing { .. }) { "playing" } else if self.state == MeditationTimerState::Paused { "paused" } else { "none" }}
                    on:play=|_| Self::Msg::Resume
                    on:pause=|_| Self::Msg::Pause
                    on:stop=|_| Self::Msg::Stop
                />
            </Host>
        }
    }
}

impl MeditationTimer {
    fn start_timer(&self) -> Cmd<Self> {
        Cmd::new(|_, link| {
            match set_interval(
                {
                    let link = link.clone();
                    move || {
                        link.send(&MeditationTimerMsg::Tick);
                    }
                },
                Duration::from_secs(1),
            ) {
                Ok(handle) => link.send(&MeditationTimerMsg::SetIntervalHandle(handle)),
                Err(_) => link.send(&MeditationTimerMsg::Error(MeditationTimerError::Interval)),
            };
        })
    }

    pub fn create_bell(&self) -> Result<web_sys::HtmlAudioElement, MeditationTimerError> {
        let audio = web_sys::HtmlAudioElement::new_with_src(&self.bell)
            .map_err(|_| MeditationTimerError::Bell)?;
        Ok(audio)
    }

    pub fn play_bell(&self) {
        if let Some(audio) = &self.audio {
            audio.play();
        }
    }

    pub fn pause_bell(&self) {
        if let Some(audio) = &self.audio {
            audio.pause();
        }
    }

    pub fn percent_progress(&self) -> f32 {
        let initial = self.total_time.as_secs_f32();
        let elapsed = self.elapsed_time.as_secs_f32();
        let progress_amount = if initial == 0.0 {
            0.0
        } else {
            elapsed / initial
        };
        progress_amount * 100f32
    }

    pub fn form(&self, bell: bool) -> Node {
        // storing these both in the total_time field rather than as separate
        // minutes and seconds values has the convenient side effect of automatically
        // translating seconds > 60 into minutes + seconds
        let minutes = self.total_time.as_secs() / 60;
        let seconds = self.total_time.as_secs() % 60;

        view! {
            <form
                on:submit=|_| MeditationTimerMsg::Start
            >
                <fieldset class="time">
                    <label>
                        {t!("meditation.minutes")}
                        <input
                            type="number"
                            min="0"
                            max="300"
                            value={minutes}
                            on:input=|ev| MeditationTimerMsg::SetMinutes(
                                event_target_value(&ev)
                                    .parse::<u16>()
                                    .unwrap_or_default()
                                )
                        />
                    </label>
                    <span class="colon">": "</span>
                    <label>
                        {t!("meditation.seconds")}
                        <input
                            type="number"
                            min="0"
                            max="60"
                            value={format!("{:0>2}", seconds)}
                            prop:value={format!("{:0>2}", seconds)}
                            on:input=|ev| MeditationTimerMsg::SetSeconds(
                                event_target_value(&ev)
                                    .parse::<u16>()
                                    .unwrap_or_default()
                                )
                        />
                    </label>
                </fieldset>
                <label class="bell">
                    {t!("meditation.use_bell")}
                    <input type="checkbox"
                        checked={bell}
                        on:change=|ev| MeditationTimerMsg::SetBell(event_target_checked(&ev))
                    />
                </label>
                 <input
                    type="submit"
                    value={t!("meditation.begin")}
                />
            </form>
        }
    }
}
