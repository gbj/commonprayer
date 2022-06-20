mod controller;

pub use controller::CalendarController;

use crate::views::{Header, Icon};
use calendar::{
    feasts::KalendarEntry, Calendar, Feast, HolyDayId, Rank, Time, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use language::Language;
use leptos2::{view::*, *};
use rust_i18n::t;

#[derive(Debug)]
pub struct CalendarView {
    lff: bool,
    data: CalendarListing,
    locale: String,
}

#[derive(Params)]
pub struct CalendarParams {
    locale: String,
}

type CalendarListing = Vec<(HolyDayId, Feast, String)>;

fn summarize_calendar(
    language: Language,
    calendar: &Calendar,
    holy_days: impl Iterator<Item = KalendarEntry>,
) -> CalendarListing {
    holy_days
        .filter_map(|(id, feast, time, _)| {
            if matches!(id, HolyDayId::Date(_, _)) {
                let rank = calendar.feast_day_rank(&feast);
                // include black-letter and red-letter days, but not weird Daily Office lectionary days like December 29
                // and don't include the Eve of ___ days
                if (rank == Rank::OptionalObservance || rank >= Rank::HolyDay)
                    && !matches!(time, Time::EveningOnly(_))
                {
                    let name = calendar.feast_name(feast, language);
                    Some((id, feast, name.map(String::from).unwrap_or_default()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
#[async_trait(?Send)]
impl Loader for CalendarView {
    type Params = ();
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let language = Language::from_locale(locale);

        let lff = req.path().ends_with("lff2018");
        let data = if lff {
            summarize_calendar(
                language,
                &LFF2018_CALENDAR,
                LFF2018_CALENDAR.holy_days.iter().cloned(),
            )
        } else {
            summarize_calendar(
                language,
                &BCP1979_CALENDAR,
                BCP1979_CALENDAR.holy_days.iter().cloned(),
            )
        };

        Some(Self {
            locale: locale.to_string(),
            lff,
            data,
        })
    }
}

impl View for CalendarView {
    fn title(&self) -> String {
        format!("{} – {}", t!("menu.calendar"), t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![
            include_str!("calendar.css").into(),
            include_str!("../../styles/toggle-links.css").into(),
        ]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        // Main view
        view! {
            <div>
                <header>
                    <span></span> // Exists only to balance span with buttons
                    <h1>{t!("menu.calendar")}</h1>
                    <CalendarController
                        locale={&self.locale}
                        prop:lff={self.lff}
                    />
                </header>
                <main>
                    {calendar_toggle_links(&self.locale, self.lff)}

                    {if self.lff {
                        view! {  <h2>{t!("lff_2018")}</h2> }
                    } else {
                        view! { <h2>{t!("bcp_1979")}</h2> }
                    }}

                    {self.calendar_view(&self.locale)}
                </main>
            </div>
        }
    }
}

pub fn calendar_toggle_links(locale: &str, use_lff: bool) -> Node {
    view! {
        <div class="toggle-links">
            <a href={format!("/{}/calendar", locale)} class:current={!use_lff}>{{t!("bcp_1979")}}</a>
            <a href={format!("/{}/calendar/lff2018", locale)} class:current={use_lff}>{{t!("lff_2018")}}</a>
        </div>
    }
}

impl CalendarView {
    fn calendar_view(&self, locale: &str) -> Vec<Node> {
        let language = Language::from_locale(locale);

        MONTHS
            .iter()
            .flat_map(move |(month, days)| {
                let name = language.month_name(*month);

                let rows = (1..=*days)
                    .map(|day_of_month| {
                        let feast = self
                            .data
                            .iter()
                            .find(|(id, _, _)| *id == HolyDayId::Date(*month, day_of_month))
                            .map(|(_, feast, name)| (feast, name.clone()));
                        let link = feast
                            .map(|(feast, name)| {
                                let link = format!("/{}/readings/holy-day/?id={}", locale, feast);
                                view! {
                                    <a href={link}>{name}</a>
                                }
                            })
                            .unwrap_or_default();
                        let id = format!("{}-{}", month, day_of_month);
                        view! {
                            <tr id={id}>
                                <td>{day_of_month.to_string()}</td>
                                <td>{link}</td>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>();

                view! {
                    <>
                        <h3>{name}</h3>
                        <table id={month}>{rows}</table>
                    </>
                }
            })
            .collect()
    }
}

const MONTHS: [(u8, u8); 12] = [
    (1, 31),
    (2, 28),
    (3, 31),
    (4, 30),
    (5, 31),
    (6, 30),
    (7, 31),
    (8, 31),
    (9, 30),
    (10, 31),
    (11, 30),
    (12, 31),
];
