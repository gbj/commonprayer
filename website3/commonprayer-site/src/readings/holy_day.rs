use calendar::{
    lff2018::LFF_BIOS, Date, Feast, LiturgicalDayId, Time, BCP1979_CALENDAR, LFF2018_CALENDAR,
};
use language::Language;
use lectionary::Reading;
use leptos::*;
use library::{
    lff2018::collects::{LFF_COLLECTS_CONTEMPORARY, LFF_COLLECTS_TRADITIONAL},
    CollectId,
};
use liturgy::{Content, Document, Version};

use crate::{
    i18n::use_language,
    time::{get_timezone_offset, today},
};

#[derive(Clone, Debug)]
pub struct HolyDayReadingsData {
    date: Memo<Date>,
    version: Memo<Version>,
    feast: Memo<Option<Feast>>,
    name: Memo<String>,
    bio: Memo<String>,
    collect_traditional: Memo<Document>,
    collect_contemporary: Memo<Document>,
    first_lesson: Memo<Vec<Reading>>,
    psalm: Vec<Psalm>,
    epistle: Memo<Vec<Reading>>,
    gospel: Memo<Vec<Reading>>,
}

pub fn holy_day_readings_data(
    cx: Scope,
    _params: Memo<ParamsMap>,
    location: Location,
) -> HolyDayReadingsData {
    let language = move || Language::from_locale(&use_language(cx)());

    let tzoffset = get_timezone_offset(cx);

    let date = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("date")
                .and_then(|date| date.parse::<Date>().ok())
                .unwrap_or_else(|| today(&tzoffset))
        })
    });

    let version = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("version")
                .and_then(|version| version.parse::<Version>().ok())
                .unwrap_or(Version::NRSV)
        })
    });

    // TODO below here: server for bundle size
    let feast = create_memo(cx, move |_| {
        location.query.with(|q| {
            let id = q.get("id").and_then(|id| id.parse::<Feast>().ok());
            let date = q.get("date").and_then(|date| date.parse::<Date>().ok());

            match (id, date) {
                (Some(feast), _) => Some(feast),
                (None, Some(date)) => {
                    let day = LFF2018_CALENDAR.liturgical_day(date, false);
                    if let LiturgicalDayId::Feast(feast) = day.observed {
                        Some(feast)
                    } else if let LiturgicalDayId::TransferredFeast(feast) = day.observed {
                        Some(feast)
                    } else {
                        day.holy_days.get(0).copied()
                    }
                }
                _ => panic!(),
            }
        })
    });

    let eve_of = move || {
        let feast = feast();

        feast.and_then(|feast| {
            BCP1979_CALENDAR
                .holy_days
                .iter()
                .find(|(_, s_feast, _, _)| *s_feast == feast)
                .and_then(|(_, _, time, _)| {
                    if let Time::EveningOnly(eve_of) = time {
                        eve_of.as_ref().copied()
                    } else {
                        None
                    }
                })
        })
    };

    let name = create_memo(cx, move |_| {
        let eve_of = eve_of();

        feast().and_then(|feast| {
            LFF_BIOS
                .iter()
                .find(|(s_feast, _)| *s_feast == feast || Some(*s_feast) == eve_of)
                .map(|(_, bio)| bio.to_string())
        })
    });

    let bio = create_memo(cx, move |_| {
        feast().and_then(|feast| {
            LFF2018_CALENDAR
                .feast_name(feast, language())
                // or, search in BCP calendar if can't find in LFF (i.e., for Eve of ___)
                .or_else(|| BCP1979_CALENDAR.feast_name(feast, language()))
        })
    });

    let collect_traditional = create_memo(cx, move |_| {
        let eve_of = eve_of();
        LFF_COLLECTS_TRADITIONAL
            .iter()
            .find(|(id, _)| {
                *id == CollectId::Feast(feast) || Some(id) == eve_of.map(CollectId::Feast).as_ref()
            })
            .map(|(_, data)| data.document.clone())
            .unwrap_or_else(|| Document::from(Content::Empty));
    });

    let collect_contemporary = create_memo(cx, move |_| {
        let eve_of = eve_of();
        LFF_COLLECTS_CONTEMPORARY
            .iter()
            .find(|(id, _)| {
                *id == CollectId::Feast(feast) || Some(id) == eve_of.map(CollectId::Feast).as_ref()
            })
            .map(|(_, data)| data.document.clone())
            .unwrap_or_else(|| Document::from(Content::Empty));
    });

    HolyDayReadingsData {
        date,
        version,
        name,
        bio,
        collect_traditional,
        collect_contemporary,
        first_lesson: (),
        psalm: (),
        epistle: (),
        gospel: (),
    }
}

#[component]
pub fn HolyDayReadings(_cx: Scope) -> Element {
    view! { cx, 
        <div>
            "Holy Day TODO"
        </div>
    }
}
