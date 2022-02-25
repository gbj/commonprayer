use std::collections::HashMap;

use crate::components::{self, *};
use episcopal_api::{
    calendar::LiturgicalDayId,
    lectionary::{ReadingType, BCP1979_30_DAY_PSALTER},
    liturgy::Psalm,
    psalter::{bcp1979::BCP1979_PSALTER, loc::LOC_PSALTER, Psalter},
};
use itertools::Itertools;
use leptos::*;

pub fn psalter() -> Page<(), (), PsalterRenderState> {
    Page::new("psalter")
        .body_fn(body)
        .head_fn(head)
        .render_state(render_state)
        .hydration_state(|_, _, _| Some(()))
        .static_page()
        .incremental_generation()
}

fn render_state(locale: &str, _path: &str, _params: &()) -> Option<PsalterRenderState> {
    Some(PsalterRenderState {
        psalms: match locale {
            "es" => &*LOC_PSALTER,
            _ => &*BCP1979_PSALTER,
        }
        .psalms
        .iter()
        .map(|(_, psalm)| (*psalm).clone())
        .collect(),
        layout: BCP1979_30_DAY_PSALTER
            .readings
            .iter()
            .filter_map(|(id, _, reading_type, citation)| match id {
                LiturgicalDayId::DayOfMonth(day) => {
                    Some(((*day, *reading_type), citation.to_string()))
                }
                _ => None,
            })
            .group_by(|(id, _)| *id)
            .into_iter()
            .map(|(id, grouped_citations)| {
                (
                    id,
                    grouped_citations
                        .into_iter()
                        .map(|(_, string)| string)
                        .collect(),
                )
            })
            .collect(),
    })
}

#[derive(Default, Clone)]
pub struct PsalterRenderState {
    psalms: Vec<Psalm>,
    layout: HashMap<(u8, ReadingType), Vec<String>>,
}

pub fn head(_locale: &str, _props: &(), _render_state: &PsalterRenderState) -> View {
    view! {
        <>
            <title>{t!("menu.psalter")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
        </>
    }
}

pub fn body(locale: &str, _props: &(), render_state: &PsalterRenderState) -> View {
    let render_psalter = Psalter {
        psalms: render_state
            .psalms
            .iter()
            .map(|psalm| (psalm.number, psalm))
            .collect(),
    };

    view! {
        <>
            {header(locale, &t!("menu.psalter"))}
            <main>
                <h2>{t!("psalter.full_title")}</h2>
                <h3>{t!("psalter.book_one")}</h3>
                {day_view(&render_psalter, &render_state.layout, 1, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 1, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 2, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 2, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 3, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 3, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 4, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 4, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 5, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 5, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 6, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 6, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 7, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 7, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 8, ReadingType::MorningPsalm)}

                // Book Two is the only one that begins mid-day, so needs to be rendered in a different way
                <h4>{t!("psalter.day", nth = &t!("psalter.ord_8"))} ": " {t!("psalter.evening")}</h4>
                {psalm_view(&render_psalter.psalms_by_citation("Psalm 41")[0])}

                <h3>{t!("psalter.book_two")}</h3>
                {psalm_view(&render_psalter.psalms_by_citation("Psalm 42")[0])}
                {psalm_view(&render_psalter.psalms_by_citation("Psalm 43")[0])}

                {day_view(&render_psalter, &render_state.layout, 9, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 9, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 10, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 10, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 11, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 11, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 12, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 12, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 13, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 13, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 14, ReadingType::MorningPsalm)}

                <h3>{t!("psalter.book_three")}</h3>
                {day_view(&render_psalter, &render_state.layout, 14, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 15, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 15, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 16, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 16, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 17, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 17, ReadingType::EveningPsalm)}

                <h3>{t!("psalter.book_four")}</h3>
                {day_view(&render_psalter, &render_state.layout, 18, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 18, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 19, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 19, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 20, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 20, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 21, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 21, ReadingType::EveningPsalm)}

                <h3>{t!("psalter.book_five")}</h3>
                {day_view(&render_psalter, &render_state.layout, 22, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 22, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 23, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 23, ReadingType::EveningPsalm)}				{day_view(&render_psalter, &render_state.layout, 20, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 24, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 24, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 25, ReadingType::EveningPsalm)}				{day_view(&render_psalter, &render_state.layout, 20, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 25, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 26, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 26, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 27, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 27, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 28, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 28, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 29, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 29, ReadingType::EveningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 30, ReadingType::MorningPsalm)}
                {day_view(&render_psalter, &render_state.layout, 30, ReadingType::EveningPsalm)}
            </main>
        </>
    }
}

fn day_view(
    psalter: &Psalter,
    layout: &HashMap<(u8, ReadingType), Vec<String>>,
    day: u8,
    reading_type: ReadingType,
) -> View {
    let time = match reading_type {
        ReadingType::EveningPsalm => t!("psalter.evening"),
        _ => t!("psalter.morning"),
    };
    view! {
        <>
            <h4>{t!("psalter.day", nth = &t!(&format!("psalter.ord_{}", day)))} ": " {time}</h4>
            {psalms_view(psalter, layout, day, reading_type)}
        </>
    }
}

fn psalms_view(
    psalter: &Psalter,
    layout: &HashMap<(u8, ReadingType), Vec<String>>,
    day: u8,
    reading_type: ReadingType,
) -> View {
    View::Fragment(
        layout
            .get(&(day, reading_type))
            .unwrap()
            .iter()
            .flat_map(|citation| psalter.psalms_by_citation(citation))
            .map(|psalm| psalm_view(&psalm))
            .collect(),
    )
}

fn psalm_view(psalm: &Psalm) -> View {
    let (header, main) = components::psalm(psalm);
    view! {
        <article class="document psalm">
            {header.unwrap()}
            {main}
        </article>
    }
}
