use api::summary::ObservanceSummary;
use lectionary::Reading;
use leptos2::*;
use liturgy::Psalm;

pub struct ReadingLinks<'a> {
    morning_office_psalms: Vec<&'a str>,
    evening_office_psalms: Vec<&'a str>,
    morning_30_psalms: Vec<&'a str>,
    evening_30_psalms: Vec<&'a str>,
    morning_readings: Vec<&'a str>,
    evening_readings: Vec<&'a str>,
}

pub fn reading_links<'a>(
    morning: &'a ObservanceSummary,
    evening: &'a ObservanceSummary,
    morning_thirty_day_psalms: &'a [Psalm],
    evening_thiry_day_psalms: &'a [Psalm],
) -> ReadingLinks<'a> {
    ReadingLinks {
        morning_office_psalms: psalm_links(&morning.daily_office_psalms),
        evening_office_psalms: psalm_links(&evening.daily_office_psalms),
        morning_30_psalms: psalm_links(morning_thirty_day_psalms),
        evening_30_psalms: psalm_links(evening_thiry_day_psalms),
        morning_readings: lectionary_reading_links(&morning.daily_office_readings),
        evening_readings: lectionary_reading_links(&evening.daily_office_readings),
    }
}

fn psalm_links(psalms: &[Psalm]) -> Vec<&str> {
    psalms
        .iter()
        .filter_map(|psalm| psalm.citation.as_deref())
        .collect()
}

fn lectionary_reading_links(readings: &[Reading]) -> Vec<&str> {
    readings
        .iter()
        .map(|reading| reading.citation.as_str())
        .collect()
}

pub fn reading_links_view(links: &ReadingLinks, thirty_day: bool) -> Node {
    let readings_different = links.morning_readings != links.evening_readings;

    view! {
        <table>
            <tr>
                <th>{t!("daily_readings.morning")}</th>
                <th>{t!("daily_readings.evening")}</th>
            </tr>
            <tr>
                <td>
                    {psalm_links_view(if thirty_day {
                        &links.morning_30_psalms
                    } else {
                        &links.morning_office_psalms
                    })}
                </td>
                <td>
                    {psalm_links_view(if thirty_day {
                        &links.evening_30_psalms
                    } else {
                        &links.evening_office_psalms
                    })}
                </td>
            </tr>
            <tr>
                <td colspan={if readings_different { "1" } else { "2" } }>
                    {reading_links_reading_view(&links.morning_readings)}
                </td>
                <td>
                    {if readings_different {
                        Some(reading_links_reading_view(&links.evening_readings))
                    } else {
                        None
                    }}
                </td>
            </tr>
        </table>
    }
}

fn reading_links_reading_view(readings: &Vec<&str>) -> Node {
    let reading_links = readings
        .iter()
        .map(|citation| {
            view! {
                <li>
                    <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <ul>{reading_links}</ul>
    }
}

fn psalm_links_view(psalms: &Vec<&str>) -> Vec<Node> {
    psalms
        .iter()
        .map(|citation| {
            view! {
                <li>
                    <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                </li>
            }
        })
        .collect::<Vec<_>>()
}
