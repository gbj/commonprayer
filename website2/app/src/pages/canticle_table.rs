use crate::views::Header;
use crate::WebView;
use leptos2::*;
use liturgy::{Reference, Source};

pub fn canticle_table() -> Page<(), ()> {
    Page::new("canticle-table")
        .body_fn(body)
        .head_fn(head)
        .state(|_, _, _| Some(()))
        .incremental_generation()
        .static_page()
}

pub fn head(_locale: &str, _props: &()) -> Vec<Node> {
    view! {
        <>
            <title>{t!("menu.canticle_table")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
            <link rel="stylesheet" href="/static/canticle-table.css"/>
        </>
    }
}

pub fn body(locale: &str, _props: &()) -> Vec<Node> {
    view! {
        <>
            {Header::new(locale, &t!("menu.canticle_table")).to_node()}
            <main>
                <h1>{t!("menu.canticle_table")}</h1>

                // BCP 1979 Canticle Table
                <section id="BCP1979">
                    <h2>{t!("bcp_1979")}</h2>
                    {Reference {
                        source: Source::BCP1979,
                        page: 144,
                    }.view(locale)}

                    <h3>{t!("canticle_table.canticles_mp")}</h3>

                    <table class="canticle-table">
                        <tr>
                            <td></td>
                            <td><em>{t!("canticle_table.after_ot")}</em></td>
                            <td><em>{t!("canticle_table.after_nt")}</em></td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.sunday_abbrev")}</td>
                            {a_or_b(locale, ("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus"))}
                            {a_or_b(locale, ("7", "RiteI", "7."), ("21", "RiteII", "21. Te Deum laudamus"))}
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>{t!("canticle_table.advent")} ": "</em>
                                <br/>
                                {canticle_link(locale, "11", "RiteII", "11. Surge, illuminare")}
                            </td>
                            <td>
                                <em>{t!("canticle_table.advent_and_lent")} ": "</em>
                                <br/>
                                {a_or_b(locale, ("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus"))}
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>{t!("canticle_table.lent")} ": "</em>
                                <br/>
                                {canticle_link(locale, "14", "RiteII", "14. Kyrie Pantokrator")}
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>{t!("canticle_table.easter")} ": "</em>
                                <br/>
                                {canticle_link(locale, "8", "RiteII", "8. Cantemus Domino")}
                            </td>
                            <td></td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.monday_abbrev")}</td>
                            <td>{canticle_link(locale, "9", "RiteII", "9. Ecce, Deus")}</td>
                            <td>{canticle_link(locale, "19", "RiteII", "19. Magna et mirabilia")}</td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.tuesday_abbrev")}</td>
                            {a_or_b(locale, ("2", "RiteI", "2."), ("13", "RiteII", "13. Benedictus es"))}
                            <td>{canticle_link(locale, "18", "RiteII", "18. Dignus es")}</td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.wednesday_abbrev")}</td>
                            <td>{canticle_link(locale, "11", "RiteII", "11. Surge, illuminare")}</td>
                            {a_or_b(locale, ("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus"))}
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>{t!("canticle_table.lent")} ": "</em>
                                <br/>
                                {canticle_link(locale, "14", "RiteII", "14. Kyrie Pantokrator")}
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.thursday_abbrev")}</td>
                            <td>{canticle_link(locale, "8", "RiteII", "8. Cantemus Domino")}</td>
                            {a_or_b(locale, ("6", "RiteI", "6."), ("20", "RiteII", "20. Gloria in excelsis"))}
                        </tr>
                        <tr>
                            <td></td>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.advent_and_lent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "19", "RiteII", "19. Magna et mirabilia")}
                            </td>
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.friday_abbrev")}</td>
                            <td>{canticle_link(locale, "10", "RiteII", "10. Quaerite Dominums")}</td>
                            <td>{canticle_link(locale, "18", "RiteII", "18. Dignus es")}</td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "14", "RiteII", "14. Kyrie Pantokrator")}
                            </td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.saturday_abbrev")}</td>
                            {a_or_b(locale, ("1", "RiteI", "1."), ("12", "RiteII", "12. Benedicite"))}
                            <td>{canticle_link(locale, "19", "RiteII", "19. Magna et mirabilia")}</td>
                        </tr>
                        <tr>
                            <td colspan="3">
                                <p>
                                    <em class="rubric">{t!("canticle_table.on_feasts")}</em>
                                </p>
                            </td>
                        </tr>
                         <tr>
                            <td></td>
                            {a_or_b(locale, ("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus"))}
                            {a_or_b(locale, ("7", "RiteI", "7."), ("21", "RiteII", "21. Te Deum laudamus"))}
                        </tr>
                    </table>

                    <h3>{t!("canticle_table.canticles_ep")}</h3>
                    <table class="canticle-table">
                        <tr>
                            <td></td>
                            <td><em>{t!("canticle_table.after_ot")}</em></td>
                            <td><em>{t!("canticle_table.after_nt")}</em></td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.sunday_abbrev")}</td>
                            {a_or_b(locale, ("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat"))}
                            {a_or_b(locale, ("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.monday_abbrev")}</td>
                            <td>{canticle_link(locale, "8", "RiteII", "8. Cantemus, Domino")}</td>
                            {a_or_b(locale, ("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis"))}
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "14", "RiteII", "14. Kyrie Pantrokrator")}
                            </td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.tuesday_abbrev")}</td>
                            <td>{canticle_link(locale, "10", "RiteII", "10. Quaerite Dominum")}</td>
                            {a_or_b(locale, ("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.wednesday_abbrev")}</td>
                            {a_or_b(locale, ("1", "RiteI", "1."), ("12", "RiteII", "12. Benedicite"))}
                            {a_or_b(locale, ("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.thursday_abbrev")}</td>
                            <td>{canticle_link(locale, "11", "RiteII", "11. Surge, illuminare")}</td>
                            {a_or_b(locale, ("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.friday_abbrev")}</td>
                            {a_or_b(locale, ("2", "RiteI", "2."), ("13", "RiteII", "13. Benedictus es"))}
                            {a_or_b(locale, ("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.saturday_abbrev")}</td>
                            <td>{canticle_link(locale, "9", "RiteII", "9. Ecce, Deus")}</td>
                            {a_or_b(locale, ("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat"))}
                        </tr>
                        <tr>
                            <td colspan="3">
                                <p>
                                    <em class="rubric">{t!("canticle_table.on_feasts")}</em>
                                </p>
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            {a_or_b(locale, ("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat"))}
                            {a_or_b(locale, ("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis"))}
                        </tr>
                    </table>
                </section>

                // EOW 1 Canticle Table
                <section id="EOW1">
                    <h2>{t!("eow_1")}</h2>
                    {Reference {
                        source: Source::EOW1,
                        page: 44,
                    }.view(locale)}
                    <details>
                        <summary>{t!("canticle_table.please_note")}</summary>
                        <p>{t!("canticle_table.eow_canticle_table_note")}</p>
                    </details>
                    <h3>{t!("canticle_table.canticles_mp")}</h3>
                    <h4>{t!("canticle_table.supplemental_materials")}</h4>
                    <table class="canticle-table">
                        <tr>
                            <td></td>
                            <td><em>{t!("canticle_table.after_ot")}</em></td>
                            <td><em>{t!("canticle_table.after_nt")}</em></td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.sunday_abbrev")}</td>
                            {a_or_b(locale, ("e", "EOW", "E. A Song of Jerusalem Our Mother"), ("16", "EOW", "16. The Song of Zechariah"))}
                            {a_or_b(locale, ("k", "EOW", "K. A Song of Our Adoption"), ("21", "EOW", "21. We Praise You O Go"))}
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.advent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "d", "EOW", "D. A Song of the Wilderness")}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.advent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "p", "EOW", "P. A Song of the Spirit")}
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.christmas")}
                                    ": "
                                </em>
                                <br/>
                                {a_or_b(locale, ("c", "EOW", "C. A Song of Hannah"), ("9", "RiteII", "9. The First Song of Isaiah"))}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.christmas")}
                                    ": "
                                </em>
                                <br/>
                                {a_or_b(locale, ("n", "EOW", "N. A Song of God’s Love"), ("20", "RiteII", "20. Glory to God"))}
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "h", "EOW", "H. A Song of Hosea")}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "l", "EOW", "L. A Song of Christ’s Humility")}
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.easter")}
                                    ": "
                                </em>
                                <br/>
                                {a_or_b(locale, ("a", "EOW", "A. A Song of Wisdom"), ("8", "RiteII", "8. The Song of Moses"))}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.easter")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "m", "EOW", "M. A Song of Faith")}
                            </td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.monday_abbrev")}</td>
                            {a_or_b(locale, ("c", "EOW", "C. A Song of Hannah"), ("11", "RiteII", "11. The Third Song of Isaiah"))}
                            {a_or_b(locale, ("l", "EOW", "L. A Song of Christ’s Humility"), ("q", "EOW", "Q. A Song of Christ’s Goodness"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.tuesday_abbrev")}</td>
                            {a_or_b(locale, ("b", "EOW", "B. A Song of Pilgrimage"), ("13", "EOW", "13. A Song of Praise"))}
                            {a_or_b(locale, ("m", "EOW", "M. A Song of Faith"), ("n", "EOW", "N. A Song of God’s Love"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.wednesday_abbrev")}</td>
                            {a_or_b(locale, ("g", "EOW", "G. A Song of Ezekiel"), ("h", "EOW", "H. A Song of Hosea"))}
                            {a_or_b(locale, ("p", "EOW", "P. A Song of the Spirit"), ("s", "EOW", "S. A Song of Our True Nature"))}
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {a_or_b(locale, ("i", "EOW", "I. A Song of Jonah"), ("10", "RiteII", "10. The Second Song of Isaiah"))}
                            </td>
                            <td></td>
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.thursday_abbrev")}</td>
                            {a_or_b(locale, ("a", "EOW", "A. A Song of Wishdom"), ("j", "EOW", "J. A Song of Judith"))}
                            {a_or_b(locale, ("r", "EOW", "R. A Song of True Motherhood"), ("16", "RiteII", "16. A Song of Zechariah"))}
                        </tr>
                        <tr class="day">
                            <td class="day-name">{t!("canticle_table.friday_abbrev")}</td>
                            <td>{canticle_link(locale, "i", "EOW", "I. A Song of Jonah")}</td>
                            <td>{canticle_link(locale, "18", "EOW", "18. Song to the Lamb")}</td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.christmas")}
                                    ":* "
                                </em>
                                <br/>
                                {canticle_link(locale, "j", "EOW", "J. A Song of Judith")}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.christmas")}
                                    ":* "
                                </em>
                                <br/>
                                {canticle_link(locale, "r", "EOW", "R. A Song of True Motherhood")}
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {a_or_b(locale, ("f", "EOW", "F. A Song of Lamentation"), ("14", "RiteII", "14. A Song of Penitence"))}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.lent")}
                                    ": "
                                </em>
                                <br/>
                                {canticle_link(locale, "s", "EOW", "S. A Song of Our True Nature")}
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <em>
                                    {t!("canticle_table.easter")}
                                    ":* "
                                </em>
                                <br/>
                                {canticle_link(locale, "g", "EOW", "G. A Song of Ezekiel")}
                            </td>
                            <td>
                                <em>
                                    {t!("canticle_table.easter")}
                                    ":* "
                                </em>
                                <br/>
                                {canticle_link(locale, "k", "EOW", "K. A Song of Our Adoption")}
                            </td>
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.saturday_abbrev")}</td>
                            {a_or_b(locale, ("12", "EOW", "12. A Song of Creation"), ("d", "EOW", "D. A Song of the Wilderness"))}
                            {a_or_b(locale, ("o", "EOW", "O. A Song of the Heavenly City"), ("19", "RiteII", "19. The Song of the Redeemed"))}
                        </tr>
                        <tr>
                            <td colspan="3">
                                <p>
                                    <em class="rubric">{t!("canticle_table.on_feasts")}</em>
                                </p>
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            {a_or_b(locale, ("16", "EOW", "16. A Song of Zechariah"), ("e", "EOW", "E. A Song of Jerusalem Our Mother"))}
                            {a_or_b(locale, ("21", "EOW", "21. We Praise You O GOd"), ("K", "EOW", "K. A Song of Our Adoption"))}
                        </tr>
                        <tr>
                            <td colspan="3">
                                <p>
                                    <em class="rubric">{t!("canticle_table.canticles_appointed_for_christmas")}</em>
                                </p>
                            </td>
                        </tr>
                    </table>

                    <h3>{t!("canticle_table.canticles_ep")}</h3>
                    <table class="canticle-table">
                        <tr>
                            <td></td>
                            <td><em>{t!("canticle_table.after_ot")}</em></td>
                            <td><em>{t!("canticle_table.after_nt")}</em></td>
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.sunday_abbrev")}</td>
                            <td>{canticle_link(locale, "15", "EOW", "15. The Song of Mary")}</td>
                            {a_or_b(locale, ("17", "RiteII", "The Song of Simeon**"), ("m", "EOW", "M. A Song of Faith"))}
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.monday_abbrev")}</td>
                            <td>{canticle_link(locale, "a", "EOW", "A. A Song of Wisdom")}</td>
                            {a_or_b(locale, ("n", "EOW", "N. A Song of God’s Love"), ("17", "RiteII", "The Song of Simeon"))}
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.tuesday_abbrev")}</td>
                            <td>{canticle_link(locale, "d", "EOW", "D. A Song of the Wilderness")}</td>
                            {a_or_b(locale, ("15", "EOW", "15. The Song of Mary"), ("p", "EOW", "P. A Song of the Spirit"))}
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.wednesday_abbrev")}</td>
                            <td>{canticle_link(locale, "c", "EOW", "C. The Song of Hannah")}</td>
                            {a_or_b(locale, ("l", "EOW", "L. A Song of Christ’s Humility"), ("17", "RiteII", "The Song of Simeon"))}
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.thursday_abbrev")}</td>
                            <td>{canticle_link(locale, "j", "EOW", "J. A Song of Judith")}</td>
                            {a_or_b(locale, ("15", "EOW", "15. The Song of Mary"), ("s", "EOW", "S. A Song of Our True Nature"))}
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.friday_abbrev")}</td>
                            <td>{canticle_link(locale, "g", "EOW", "G. A Song of Ezekiel")}</td>
                            {a_or_b(locale, ("q", "EOW", "Q. A Song of Christ’s Goodness"), ("17", "RiteII", "The Song of Simeon"))}
                        </tr>
                        <tr>
                            <td class="day-name">{t!("canticle_table.saturday_abbrev")}</td>
                            <td>{canticle_link(locale, "b", "EOW", "B. A Song of Pilgrimage")}</td>
                            {a_or_b(locale, ("15", "EOW", "15. The Song of Mary"), ("r", "EOW", "R. A Song of True Motherhood"))}
                        </tr>
                        <tr>
                            <td colspan="3">
                                <p>
                                    <em class="rubric">{t!("canticle_table.on_feasts")}</em>
                                </p>
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>{canticle_link(locale, "15", "EOW", "15. The Song of Mary")}</td>
                            {a_or_b(locale, ("o", "EOW", "O. A Song of the Heavenly City**"), ("17", "RiteII", "The Song of Simeon**"))}
                        </tr>
                        <tr>
                            <td colspan="3">
                                <p>
                                    <em class="rubric">{t!("canticle_table.magnificat_note_eow")}</em>
                                </p>
                            </td>
                        </tr>
                    </table>
                </section>
            </main>
        </>
    }
}

fn a_or_b(
    locale: &str,
    a: (&'static str, &'static str, &'static str),
    b: (&'static str, &'static str, &'static str),
) -> Node {
    let (a_number, a_version, a_label) = a;
    let (b_number, b_version, b_label) = b;
    let separator = if a_label.len() < 5 {
        None
    } else {
        Some(view! { <br/> })
    };
    view! {
        <td>
            {canticle_link(locale, a_number, a_version, a_label)}
            " "
            {separator}
            {t!("canticle_table.or")}
            " "
            {canticle_link(locale, b_number, b_version, b_label)}
        </td>
    }
}

fn canticle_link(
    locale: &str,
    number: &'static str,
    version: &'static str,
    label: &'static str,
) -> Node {
    view! {
        <a href={format!("/{}/document/canticle/{}/{}", locale, number, version)}>
            {label}
        </a>
    }
}
