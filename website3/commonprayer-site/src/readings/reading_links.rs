use leptos::*;
use serde::{Deserialize, Serialize};

use crate::i18n::use_i18n;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadingLinks {
    morning_psalms: Vec<String>,
    evening_psalms: Vec<String>,
    morning_readings: Vec<String>,
    evening_readings: Vec<String>,
}

pub fn reading_links(
    morning_readings: &[lectionary::Reading],
    evening_readings: &[lectionary::Reading],
    morning_psalms: &[liturgy::Psalm],
    evening_psalms: &[liturgy::Psalm],
) -> ReadingLinks {
    ReadingLinks {
        morning_psalms: psalm_links(morning_psalms),
        evening_psalms: psalm_links(evening_psalms),
        morning_readings: lectionary_reading_links(morning_readings),
        evening_readings: lectionary_reading_links(evening_readings),
    }
}

fn psalm_links(psalms: &[liturgy::Psalm]) -> Vec<String> {
    psalms
        .iter()
        .filter_map(|psalm| psalm.citation.clone())
        .collect()
}

fn lectionary_reading_links(readings: &[lectionary::Reading]) -> Vec<String> {
    readings
        .iter()
        .map(|reading| reading.citation.clone())
        .collect()
}

#[component]
pub fn ReadingLinksView(cx: Scope, reading_links: Memo<ReadingLinks>) -> Element {
    let (t, _, _) = use_i18n(cx);
    let morning_readings = move || reading_links.with(|l| l.morning_readings.clone());
    let evening_readings = move || reading_links.with(|l| l.evening_readings.clone());
    let morning_psalms = move || reading_links.with(|l| l.morning_psalms.clone());
    let evening_psalms = move || reading_links.with(|l| l.evening_psalms.clone());
    let readings_different = move || morning_readings() != evening_readings();

    view! {
        <table class="reading-link-table">
            <thead>
                <tr>
                    <th>{t("daily-readings-morning")}</th>
                    <th>{t("daily-readings-evening")}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <LinksView links=Box::new(morning_psalms) evening=false />
                    </td>
                    <td>
                        <LinksView links=Box::new(evening_psalms) evening=true />
                    </td>
                </tr>
                <tr>
                    <td colspan={move || if readings_different() { "1" } else { "2" } }>
                        <LinksView links=Box::new(morning_readings) evening=false />
                    </td>
                    {move || readings_different().then(move || {
                        view! {
                            <td>
                                <LinksView links=Box::new(evening_readings) evening=false />
                            </td>
                        }
                    })}
                </tr>
            </tbody>
        </table>
    }
}

#[component]
pub fn LinksView(cx: Scope, links: Box<dyn Fn() -> Vec<String>>, evening: bool) -> Element {
    view! {
        <ul>
            <For each=links key=|v| v.clone()>
            {|cx, citation: &String| view! {
                <li>
                    <a href=format!("#{citation}")>{citation}</a>
                </li>
            }}
            </For>
        </ul>
    }
}
