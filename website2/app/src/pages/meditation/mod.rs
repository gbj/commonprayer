use crate::views::Header;
use leptos2::*;
use std::time::Duration;

pub struct MeditationPage {}

impl Page for MeditationPage {
    type Params = ();

    fn name() -> &'static str {
        "meditation"
    }

    fn paths() -> Vec<String> {
        vec!["".into()]
    }

    fn build_state(_locale: &str, _path: &str, _params: Self::Params) -> Option<Self> {
        Some(Self {})
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        view! {
            <>
                <title>{t!("meditation.title")} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        view! {
            <>
                {Header::new(locale, &t!("meditation.title")).to_node()}
                <main>
                    <MeditationTimer/>
                </main>
            </>
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, WebComponent)]
pub struct MeditationTimer {
    state: MeditationTimerState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MeditationTimerState {
    Setup {
        minutes: u16,
        seconds: u16,
        bell: bool,
    },
    Playing {
        initial: Duration,
        remaining: Duration,
        interval_handle: Option<IntervalHandle>,
        audio: Option<web_sys::HtmlAudioElement>,
        bell: bool,
    },
    Paused {
        initial: Duration,
        remaining: Duration,
        audio: Option<web_sys::HtmlAudioElement>,
        bell: bool,
    },
    Complete {
        remaining: Duration,
        audio: Option<web_sys::HtmlAudioElement>,
    },
    Error {
        error: MeditationTimerError,
    },
}

impl Default for MeditationTimerState {
    fn default() -> Self {
        MeditationTimerState::Setup {
            minutes: 5,
            seconds: 0,
            bell: true,
        }
    }
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
    Error(MeditationTimerError),
}

#[derive(Clone, Debug)]
pub enum MeditationTimerCmd {
    StartTimer,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MeditationTimerError {
    Bell,
    Interval,
}

#[async_trait(?Send)]
impl State for MeditationTimer {
    type Msg = MeditationTimerMsg;
    type Cmd = MeditationTimerCmd;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        leptos2::warn(&format!("[MeditationTimer::update] {:#?}", msg));
        match (&mut self.state, msg) {
            // Error handling
            (_, Self::Msg::Error(error)) => self.state = MeditationTimerState::Error { error },

            // Setup: Change initial settings
            (MeditationTimerState::Setup { minutes, .. }, Self::Msg::SetMinutes(m)) => *minutes = m,
            (MeditationTimerState::Setup { seconds, .. }, Self::Msg::SetSeconds(s)) => *seconds = s,
            (MeditationTimerState::Setup { bell, .. }, Self::Msg::SetBell(b)) => *bell = b,

            // Playing: tick decrements the total remaining
            (
                MeditationTimerState::Playing {
                    remaining,
                    audio,
                    bell,
                    ..
                },
                Self::Msg::Tick,
            ) => {
                let secs_remaining = remaining.as_secs();
                if secs_remaining == 1 {
                    if *bell {
                        play_bell();
                    }
                    self.state = MeditationTimerState::Complete {
                        remaining: Duration::from_secs(0),
                        audio: audio.clone(),
                    };
                } else {
                    *remaining = Duration::from_secs(secs_remaining - 1);
                }
            }

            // Transition from setup to playing
            (
                MeditationTimerState::Setup {
                    minutes,
                    seconds,
                    bell,
                },
                Self::Msg::Start,
            ) => {
                let initial = Duration::from_secs(*minutes as u64 * 60 + *seconds as u64);

                // create audio element and, if it can't be created, transition to an error state
                let audio = if *bell {
                    if let Ok(audio) = play_bell() {
                        Some(audio)
                    } else {
                        self.state = MeditationTimerState::Error {
                            error: MeditationTimerError::Bell,
                        };
                        return None;
                    }
                } else {
                    None
                };

                self.state = MeditationTimerState::Playing {
                    initial,
                    remaining: initial,
                    interval_handle: None,
                    audio,
                    bell: *bell,
                };

                return Some(MeditationTimerCmd::StartTimer);
            }

            // Pause playing
            (
                MeditationTimerState::Playing {
                    initial,
                    interval_handle,
                    remaining,
                    audio,
                    bell,
                    ..
                },
                Self::Msg::Pause,
            ) => {
                // clear timer
                if let Some(handle) = interval_handle.take() {
                    handle.clear();
                }

                // pause audio
                if let Some(audio) = audio {
                    audio.pause();
                }

                self.state = MeditationTimerState::Paused {
                    initial: *initial,
                    remaining: *remaining,
                    audio: audio.clone(),
                    bell: *bell,
                }
            }

            // Resume playing
            (
                MeditationTimerState::Paused {
                    initial,
                    remaining,
                    audio,
                    bell,
                    ..
                },
                Self::Msg::Resume,
            ) => {
                // pause audio
                if let Some(audio) = audio {
                    audio.play();
                }

                self.state = MeditationTimerState::Playing {
                    interval_handle: None,
                    initial: *initial,
                    remaining: *remaining,
                    audio: audio.clone(),
                    bell: *bell,
                };

                return Some(MeditationTimerCmd::StartTimer);
            }

            // Set interval handle, which has been returned by the command
            (
                MeditationTimerState::Playing {
                    interval_handle, ..
                },
                Self::Msg::SetIntervalHandle(handle),
            ) => *interval_handle = Some(handle),
            _ => {}
        }
        None
    }

    async fn cmd(
        cmd: Self::Cmd,
        _host: web_sys::HtmlElement,
        link: StateLink<Self>,
    ) -> Option<Self::Msg> {
        match cmd {
            MeditationTimerCmd::StartTimer => {
                match set_interval(
                    move || {
                        link.send(&MeditationTimerMsg::Tick);
                    },
                    Duration::from_secs(1),
                ) {
                    Ok(handle) => Some(MeditationTimerMsg::SetIntervalHandle(handle)),
                    Err(_) => Some(MeditationTimerMsg::Error(MeditationTimerError::Interval)),
                }
            }
        }
    }
}

impl Component for MeditationTimer {
    fn view(&self) -> Host {
        let state = if let MeditationTimerState::Setup {
            minutes,
            seconds,
            bell,
        } = &self.state
        {
            self.form(*minutes, *seconds, *bell)
        } else {
            // action buttons (Begin/Pause/Resume)
            let action_button = match &self.state {
                MeditationTimerState::Playing { .. } => Some(view! {
                    <input
                        type="button"
                        value={t!("meditation.pause")}
                        on:click=|_| MeditationTimerMsg::Pause
                    />
                }),
                MeditationTimerState::Paused { .. } => Some(view! {
                    <input
                        type="button"
                        value={t!("meditation.resume")}
                        on:click=|_| MeditationTimerMsg::Resume
                    />
                }),
                _ => None,
            };

            // wheel calculations
            let initial = self.initial_secs().unwrap_or_default();
            let remaining = self.remaining_secs().unwrap_or_default();
            let progress_amount = if initial == 0.0 {
                0.0
            } else {
                remaining / initial
            };
            let progress_percent = progress_amount * 100f32;
            let percent_remaining = 100f32 - progress_percent;

            let seconds = match &self.state {
                MeditationTimerState::Setup { .. } => None,
                MeditationTimerState::Playing { remaining, .. }
                | MeditationTimerState::Paused { remaining, .. }
                | MeditationTimerState::Complete { remaining, .. } => {
                    let minutes = remaining.as_secs() / 60;
                    let seconds = remaining.as_secs() % 60;
                    Some(view! {
                        <div class="seconds">
                            {minutes}
                            ":"
                            {format!("{:0>2}", seconds)}
                        </div>
                    })
                }
                MeditationTimerState::Error { error } => {
                    let error_text = match error {
                        MeditationTimerError::Bell => t!("meditation.bell_error"),
                        MeditationTimerError::Interval => t!("meditation.interval_error"),
                    };
                    Some(view! {
                        <div class="error">{error_text}</div>
                    })
                }
            };
            view! {
                <div class="progress-circle-container">
                    <div class="progress-circle"
                        style={format!("background: conic-gradient(var(--link-color) {}%, 0, #ecf0f1 {}%)", percent_remaining, progress_percent)}
                    >
                        <div class="circle-overlay">
                            {seconds}
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
            </Host>
        }
    }
}

fn play_bell() -> Result<web_sys::HtmlAudioElement, MeditationTimerError> {
    let audio = web_sys::HtmlAudioElement::new_with_src("/static/audio/singing-bowl.mp3")
        .map_err(|_| MeditationTimerError::Bell)?;
    audio.play().map_err(|_| MeditationTimerError::Bell)?;
    Ok(audio)
}

impl MeditationTimer {
    pub fn remaining_secs(&self) -> Option<f32> {
        match self.state {
            MeditationTimerState::Playing { remaining, .. }
            | MeditationTimerState::Paused { remaining, .. } => Some(remaining.as_secs_f32()),
            MeditationTimerState::Complete { .. } => Some(0.0),
            _ => None,
        }
    }

    pub fn initial_secs(&self) -> Option<f32> {
        match self.state {
            MeditationTimerState::Playing { initial, .. }
            | MeditationTimerState::Paused { initial, .. } => Some(initial.as_secs_f32()),
            MeditationTimerState::Complete { .. } => Some(1.0),
            _ => None,
        }
    }

    pub fn form(&self, minutes: u16, seconds: u16, bell: bool) -> Node {
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
