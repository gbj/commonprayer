use crate::{condition, dyn_attr_once, dyn_class_once};
use calendar::*;
use futures::StreamExt;
use language::Language;
use lectionary::ReadingType;
use leptos::*;
use liturgy::*;
use std::str::FromStr;
use strum::IntoEnumIterator;
pub struct ConditionEditor {
    pub condition: Behavior<Option<Condition>>,
}

impl From<Option<Condition>> for ConditionEditor {
    fn from(condition: Option<Condition>) -> Self {
        Self {
            condition: Behavior::new(condition),
        }
    }
}

impl ConditionEditor {
    pub fn view(&self) -> View {
        view! {
            <div class="condition-editor">
                <dyn:select
                    prop:value={self.condition.stream().map(|condition| Some(value_from_optional_condition_type(&condition).to_string())).boxed_local()}
                    on:change={
                        let condition = self.condition.clone();
                        move |ev: Event| {
                            let value = default_condition_from_type(&event_target_value(ev));
                            condition.set(value);
                        }
                    }
                >
                    <option>"—"</option>
                    <option>"Day"</option>
                    <option>"Feast"</option>
                    <option>"Season"</option>
                    <option>"Observed Season"</option>
                    <option>"Week"</option>
                    <option>"Weekday"</option>
                    <option>"Evening"</option>
                    <option>"Rank Gte"</option>
                    <option>"Date Lt"</option>
                    <option>"Date Lte"</option>
                    <option>"Date Gt"</option>
                    <option>"Date Gte"</option>
                    <option>"Day of Month"</option>
                    <option>"Preference"</option>
                    <option>"Not"</option>
                    <option>"And"</option>
                    <option>"Or"</option>
                    <option>"Any"</option>
                    <option>"All"</option>
                    <option>"None"</option>
                </dyn:select>
                {edit_condition(&self.condition)}
            </div>
        }
    }
}

macro_rules! edit_date_condition {
    ($root_condition:ident, $m:ident, $d:ident, $create_condition:expr) => {{
        let month = Behavior::new($m);
        let date = Behavior::new($d);

        (month.stream(), date.stream())
            .lift()
            .skip(2)
            .create_effect({
                let root_condition = $root_condition.clone();
                move |(month, date)| {
                    root_condition.set(Some($create_condition(
                        month.unwrap_or(1),
                        date.unwrap_or(1),
                    )))
                }
            });

        view! {
            <div>
                <dyn:select
                    prop:value={month.stream().map(|n| Some(n.to_string())).boxed_local()}
                    on:change=move |ev: Event| {
                        let val = event_target_value(ev).parse::<u8>().unwrap_or(1);
                        month.set(val);
                    }
                >
                    {View::Fragment(
                        (1..=12)
                            .map(|n: u8| view! { <option>{n.to_string()}</option> })
                            .collect()
                    )}
                </dyn:select>
                "/"
                <dyn:select
                    prop:value={date.stream().map(|n| Some(n.to_string())).boxed_local()}
                    on:change=move |ev: Event| {
                        let val = event_target_value(ev).parse::<u8>().unwrap_or(1);
                        date.set(val);
                    }
                >
                    {View::Fragment(
                        (1..=31)
                            .map(|n: u8| view! { <option>{n.to_string()}</option> })
                            .collect()
                    )}
                </dyn:select>
            </div>
        }
    }};
}

macro_rules! select_enum {
    ($root:ident, $value:ident, $enum_type:ty, $val_expr:expr) => {{
        let root = $root.clone();
        let value = Behavior::new($value);
        view! {
            <dyn:select
                prop:value={value.stream().map(|value| Some(value.to_string())).boxed_local()}
                on:change=move |ev: Event| {
                    if let Ok(val) = $enum_type::from_str(&event_target_value(ev)) {
                        value.set(val);
                        root.set(Some(($val_expr)(val)));
                    }
                }
            >
                {View::Fragment(
                    $enum_type::iter()
                        .map(|variant| view! { <option>{variant.to_string()}</option> })
                        .collect()
                )}
            </dyn:select>
        }
    }};
}

macro_rules! list_of_conditions {
    ($root:ident, $conditions:ident, $constructor:expr) => {
        {
            let root = $root.clone();
            let conditions = Behavior::new($conditions);

            let list = NaiveList::new(
                |items| view! {
                    <ul class="conditions">{items}</ul>
                },
                conditions.stream(),
                {let root = root.clone(); let conditions = conditions.clone(); move |(idx, condition)| {
                    let editor = ConditionEditor::from(Some(condition));
                    editor.condition.stream().skip(1).create_effect({
                        let root = root.clone();
                        let conditions = conditions.clone();
                        move |condition| {
                            // TODO start here with updating the condition at this index
                            conditions.update(move |conditions: &mut Vec<Condition>| {
                                if let Some(l) = conditions.get_mut(idx) {
                                    if let Some(condition) = &condition {
                                        *l = condition.clone();
                                    }
                                };
                            });
                            let new_conditions = conditions.get();
                            root.set(Some(($constructor)(new_conditions)));
                        }
                    });
                    editor.view()
                }}
            );
            view! {
                <div class="condition multiple">
                    {list.view()}
                    <dyn:button
                        on:click={
                            let root = root.clone();
                            let conditions = conditions.clone();
                            move |_ev: Event| {
                                conditions.update(|c| c.push(Condition::Evening));
                                let new_conditions = conditions.get();
                                root.update(move |root| *root = Some(($constructor)(new_conditions.clone())));
                            }
                        }
                    >
                        "Add"
                    </dyn:button>
                </div>
            }
        }
    }
}

fn edit_condition(root_condition: &Behavior<Option<Condition>>) -> View {
    View::ViewStream(
        root_condition
            .stream()
            .map({
                let root_condition = root_condition.clone();
                move |condition| match condition {
                    None => View::Empty,
                    Some(condition) => match condition {
                        Condition::Day(day) => view! { <pre>{serde_json::to_string_pretty(&day).unwrap()}</pre> },
                        Condition::Feast(feast) => {
							select_enum!(root_condition, feast, Feast, Condition::Feast)
						},
                        Condition::Season(season) => {
							select_enum!(root_condition, season, Season, Condition::Season)
						},
                        Condition::ObservedSeason(season) => {
							select_enum!(root_condition, season, Season, Condition::ObservedSeason)
						},
                        Condition::Week(week) => select_enum!(root_condition, week, LiturgicalWeek, Condition::Week),
                        Condition::Weekday(weekday) => {
							select_enum!(root_condition, weekday, Weekday, Condition::Weekday)
						},
                        Condition::Evening => View::Empty,
                        Condition::RankGte(rank) => {
							select_enum!(root_condition, rank, Rank, Condition::RankGte)
						},
                        Condition::DateLt(m, d) => {
                            edit_date_condition!(root_condition, m, d, |month, date| {
                                Condition::DateLt(month, date)
                            })
                        }
                        Condition::DateLte(m, d) => {
                            edit_date_condition!(root_condition, m, d, |month, date| {
                                Condition::DateLte(month, date)
                            })
                        }
                        Condition::DateGt(m, d) => {
                            edit_date_condition!(root_condition, m, d, |month, date| {
                                Condition::DateGt(month, date)
                            })
                        }
                        Condition::DateGte(m, d) => {
                            edit_date_condition!(root_condition, m, d, |month, date| {
                                Condition::DateGte(month, date)
                            })
                        }
                        Condition::DayOfMonth(d) => {
                            let root_condition = root_condition.clone();
							let d = Behavior::new(d);

                            view! {
                                <dyn:select
                                    prop:value={d.stream().map(|n| Some(n.to_string())).boxed_local()}
                                    on:change=move |ev: Event| {
                                        let val = event_target_value(ev).parse::<u8>().unwrap_or(1);
                                        root_condition.set(Some(Condition::DayOfMonth(val)));
										d.set(val);
                                    }
                                >
                                    {View::Fragment(
                                        (1..=31)
                                            .map(|n: u8| view! { <option>{n.to_string()}</option> })
                                            .collect()
                                    )}
                                </dyn:select>
                            }
                        }
                        Condition::Preference(key, value) => preference_condition_editor(&root_condition, key, value),
                        Condition::Not(pref) => {
							let root = root_condition.clone();

							let subeditor = ConditionEditor::from(Some(*pref));
							subeditor.condition.stream().skip(1).create_effect(move |subcondition| {
								root.set(subcondition.map(|subcondition| Condition::Not(Box::new(subcondition))));
							});

							subeditor.view()
						},
                        Condition::And(a, b) => {
							let root = root_condition.clone();
							let a_editor = ConditionEditor::from(Some(*a));
							let b_editor = ConditionEditor::from(Some(*b));
							(a_editor.condition.stream(), b_editor.condition.stream()).lift().skip(2).create_effect(move |(a, b)| {
								if let (Some(Some(a)), Some(Some(b))) = (a, b) {
									root.set(Some(Condition::And(Box::new(a), Box::new(b))));
								}
							});
							view! {
								<div class="condition and">
									{a_editor.view()}
									" and "
									{b_editor.view()}
								</div>
							}
						},
                        Condition::Or(a, b) => {
							let root = root_condition.clone();
							let a_editor = ConditionEditor::from(Some(*a));
							let b_editor = ConditionEditor::from(Some(*b));
							(a_editor.condition.stream(), b_editor.condition.stream()).lift().skip(2).create_effect(move |(a, b)| {
								if let (Some(Some(a)), Some(Some(b))) = (a, b) {
									root.set(Some(Condition::Or(Box::new(a), Box::new(b))));
								}
							});
							view! {
								<div class="condition or">
									{a_editor.view()}
									" or "
									{b_editor.view()}
								</div>
							}
						},
                        Condition::Any(conditions) => list_of_conditions!(root_condition, conditions, Condition::Any),
                        Condition::All(conditions) => list_of_conditions!(root_condition, conditions, Condition::All),
                        Condition::None(conditions) => list_of_conditions!(root_condition, conditions, Condition::None),
                    },
                }
            })
            .boxed_local(),
    )
}

fn value_from_optional_condition_type(condition: &Option<Condition>) -> &'static str {
    match condition {
        None => "—",
        Some(condition) => value_from_condition_type(condition),
    }
}

fn default_condition_from_type(condition_type: &str) -> Option<Condition> {
    match condition_type {
        "Day" => Some(Condition::Day(LiturgicalDayId::WeekAndDay(
            LiturgicalWeek::Advent1,
            Weekday::Sun,
        ))),
        "Feast" => Some(Condition::Feast(Feast::AllSaintsDay)),
        "Season" => Some(Condition::Season(Season::Advent)),
        "Observed Season" => Some(Condition::ObservedSeason(Season::Advent)),
        "Week" => Some(Condition::Week(LiturgicalWeek::Advent1)),
        "Weekday" => Some(Condition::Weekday(Weekday::Sun)),
        "Evening" => Some(Condition::Evening),
        "Rank Gte" => Some(Condition::RankGte(Rank::HolyDay)),
        "Date Lt" => Some(Condition::DateLt(1, 1)),
        "Date Lte" => Some(Condition::DateLte(1, 1)),
        "Date Gt" => Some(Condition::DateGt(1, 1)),
        "Date Gte" => Some(Condition::DateGte(1, 1)),
        "Day of Month" => Some(Condition::DayOfMonth(1)),
        "Preference" => Some(Condition::Preference(
            PreferenceKey::from("key"),
            PreferenceValue::Local("value".into()),
        )),
        "Not" => Some(Condition::Not(Box::new(Condition::Evening))),
        "And" => Some(Condition::And(
            Box::new(Condition::Evening),
            Box::new(Condition::Evening),
        )),
        "Or" => Some(Condition::Or(
            Box::new(Condition::Evening),
            Box::new(Condition::Evening),
        )),
        "Any" => Some(Condition::Any(Vec::new())),
        "All" => Some(Condition::All(Vec::new())),
        "None" => Some(Condition::None(Vec::new())),
        _ => None,
    }
}

fn value_from_condition_type(condition: &Condition) -> &'static str {
    match condition {
        Condition::Day(_) => "Day",
        Condition::Feast(_) => "Feast",
        Condition::Season(_) => "Season",
        Condition::ObservedSeason(_) => "Observed Season",
        Condition::Week(_) => "Week",
        Condition::Weekday(_) => "Weekday",
        Condition::Evening => "Evening",
        Condition::RankGte(_) => "Rank Gte",
        Condition::DateLt(_, _) => "Date Lt",
        Condition::DateLte(_, _) => "Date Lte",
        Condition::DateGt(_, _) => "Date Gt",
        Condition::DateGte(_, _) => "Date Gte",
        Condition::DayOfMonth(_) => "Day of Month",
        Condition::Preference(_, _) => "Preference",
        Condition::Not(_) => "Not",
        Condition::And(_, _) => "And",
        Condition::Or(_, _) => "Or",
        Condition::Any(_) => "Any",
        Condition::All(_) => "All",
        Condition::None(_) => "None",
    }
}

macro_rules! enum_preference_value {
    ($value:ident, $variant:expr, $match:expr, $inner:ty) => {
        view! {
            <dyn:select class:hidden={$value.stream().map(|(_, value)| (($match)(value)).is_none()).boxed_local()}
                prop:value={
                    $value.stream()
                            .map(|(_, value)| ($match)(value))
                            .boxed_local()
                }
                on:change={
                    let value = $value.clone();
                    move |ev: Event| {
                        let val = event_target_value(ev);
                        value.update(|(_, value)| {
                            let val = $inner::from_str(&val).unwrap();
                            *value = $variant(val);
                        });
                    }
                }
            >
                {View::Fragment(
                    $inner::iter()
                        .map(|variant| view! { <option>{variant.to_string()}</option> })
                        .collect()
                )}
            </dyn:select>
        }
    }
}

fn preference_condition_editor(
    root_condition: &Behavior<Option<Condition>>,
    key: PreferenceKey,
    value: PreferenceValue,
) -> View {
    let root = root_condition.clone();

    let value = Behavior::new((key, value));
    value.stream().skip(1).create_effect({
        move |(key, value)| {
            root.set(Some(Condition::Preference(key, value)));
        }
    });

    view! {
        <div class="condition preference">
            <fieldset>
                <legend>"Key"</legend>
                <label>
                    "Global"
                    <dyn:select
                        prop:value={
                            value.stream()
                                .map(|(key, _)| if let PreferenceKey::Global(val) = key {
                                    let s: &'static str = val.into();
                                    Some(s.to_string())
                                } else {
                                    None
                                })
                                .boxed_local()
                        }
                        on:change={
                            let value = value.clone();
                            move |ev: Event| {
                                let val = event_target_value(ev);
                                value.update(move |(key, _)| *key = PreferenceKey::Global(GlobalPref::from_str(&val).unwrap()))
                            }
                        }
                    >
                        {View::Fragment(
                            GlobalPref::iter()
                                .map(|variant| {
                                    let s: &'static str = variant.into();
                                    view! { <option>{s}</option> }
                                })
                                .collect()
                        )}
                    </dyn:select>
                </label>
                <label>
                    "Local"
                    <dyn:input
                        prop:value={
                            value.stream()
                                .map(|(key, _)| if let PreferenceKey::Local(val) = key {
                                    Some(val)
                                } else {
                                    None
                                })
                                .boxed_local()
                        }
                        on:change={
                            let value = value.clone();
                            move |ev: Event| {
                                let val = event_target_value(ev);
                                value.update(move |(key, _)| *key = PreferenceKey::Local(val.clone()))
                            }
                        }
                    />
                </label>
            </fieldset>
            <fieldset>
                <legend>"Value"</legend>
                <dyn:select
                    prop:value={
                        value.stream()
                            .map(|(_, value)| {
                                let s: &'static str = value.into();
                                Some(s.to_string())
                            })
                            .boxed_local()
                    }
                    on:change={
                        let value = value.clone();
                        move |ev: Event| {
                            let val = PreferenceValue::from_str(&event_target_value(ev)).unwrap();
                            value.update(|(_, value)| * value = val.clone());
                        }
                    }
                >
                        {View::Fragment(
                            PreferenceValue::iter()
                                .map(|variant| {
                                    let s: &'static str = variant.into();
                                    view! { <option>{s}</option> }
                                })
                            .collect()
                        )}
                </dyn:select>

                // Local/String preference value
                <dyn:input class:hidden={value.stream().map(|(_, value)| !matches!(value, PreferenceValue::Local(_))).boxed_local()}
                    prop:value={
                        value.stream()
                                .map(|(_, value)| if let PreferenceValue::Local(val) = value {
                                    Some(val)
                                } else {
                                    None
                                })
                                .boxed_local()
                    }
                    on:change={
                        let value = value.clone();
                        move |ev: Event| {
                            let val = event_target_value(ev);
                            value.update(|(_, value)| *value = PreferenceValue::Local(val.clone()));
                        }
                    }
                />

                // Enum values
                {enum_preference_value!(
                    value,
                    PreferenceValue::Language,
                    |value| match value {
                        PreferenceValue::Language(v) => Some(v.to_string()),
                        _ => None
                    },
                    Language)
                }
                {enum_preference_value!(
                    value,
                    PreferenceValue::Version,
                    |value| match value {
                        PreferenceValue::Version(v) => Some(v.to_string()),
                        _ => None
                    },
                    Version)
                }
                {enum_preference_value!(
                    value,
                    PreferenceValue::Lectionary,
                    |value| match value {
                        PreferenceValue::Lectionary(v) => Some(v.to_string()),
                        _ => None
                    },
                    Lectionaries)
                }
                {enum_preference_value!(
                    value,
                    PreferenceValue::CanticleTable,
                    |value| match value {
                        PreferenceValue::CanticleTable(v) => Some(v.to_string()),
                        _ => None
                    },
                    CanticleTables)
                }
                {enum_preference_value!(
                    value,
                    PreferenceValue::ReadingType,
                    |value| match value {
                        PreferenceValue::ReadingType(v) => Some(v.to_string()),
                        _ => None
                    },
                    ReadingType)
                }
            </fieldset>
        </div>
    }
}
