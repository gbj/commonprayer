use crate::document::reference::*;
use crate::header::*;
use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use liturgy::Source;

#[component]
pub fn CanticleTable(cx: Scope) -> Vec<Element> {
    let (t, _, _) = use_i18n(cx);

    view! {
        <>
            <Header label=t("canticle-table-title")/>
            <main class="CanticleTable">
                <Title text=t("canticle-table-title").into()/>
                <div class="toggle-links">
                    <NavLink to="" exact=true>{t("version-BCP1979")}</NavLink>
                    <NavLink to="eow">{t("version-EOW")}</NavLink>
                </div>
                <Outlet/>
            </main>
        </>
    }
}

#[component]
pub fn BCPTable(cx: Scope) -> Element {
    let (t, _, _) = use_i18n(cx);

    view! {
        // BCP 1979 Canticle Table
        <section id="BCP1979">
        <h2>{t("bcp_1979")}</h2>
        <Reference reference=liturgy::Reference {
            source: Source::BCP1979,
            page: 144,
        }/>

        <h3>{t("canticle-table-canticles_mp")}</h3>

        <table class="CanticleTable-table">
            <thead>
                <tr>
                    <th></th>
                    <th><em>{t("canticle-table-after_ot")}</em></th>
                    <th><em>{t("canticle-table-after_nt")}</em></th>
                </tr>
            </thead>
            <tbody>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-sunday_abbrev")}</td>
                    <AorB a=("4", "RiteI", "4.") b=("16", "RiteII", "16. Benedictus Dominus")/>
                    <AorB a=("7", "RiteI", "7.") b=("21", "RiteII", "21. Te Deum laudamus")/>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <em>{t("canticle-table-advent")} ": "</em>
                        <br/>
                        <CanticleLink number="11" version="RiteII" label="11. Surge, illuminare" />
                    </td>
                    <td>
                        <em>{t("canticle-table-advent_and_lent")} ": "</em>
                        <br/>
                        <AorB a=("4", "RiteI", "4.") b=("16", "RiteII", "16. Benedictus Dominus")/>
                    </td>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <em>{t("canticle-table-lent")} ": "</em>
                        <br/>
                        <CanticleLink number="14" version="RiteII" label="14. Kyrie Pantokrator" />
                    </td>
                    <td></td>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <em>{t("canticle-table-easter")} ": "</em>
                        <br/>
                        <CanticleLink number="8" version="RiteII" label="8. Cantemus Domino" />
                    </td>
                    <td></td>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-monday_abbrev")}</td>
                    <td><CanticleLink number="9" version="RiteII" label="9. Ecce, Deus" /></td>
                    <td><CanticleLink number="19" version="RiteII" label="19. Magna et mirabilia" /></td>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-tuesday_abbrev")}</td>
                    <AorB a=("2", "RiteI", "2.") b=("13", "RiteII", "13. Benedictus es")/>
                    <td><CanticleLink number="18" version="RiteII" label="18. Dignus es" /></td>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-wednesday_abbrev")}</td>
                    <td><CanticleLink number="11" version="RiteII" label="11. Surge, illuminare" /></td>
                    <AorB a=("4", "RiteI", "4.") b=("16", "RiteII", "16. Benedictus Dominus")/>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <em>{t("canticle-table-lent")} ": "</em>
                        <br/>
                        <CanticleLink number="14" version="RiteII" label="14. Kyrie Pantokrator" />
                    </td>
                    <td></td>
                </tr>
                <tr>
                    <td class="day-name">{t("canticle-table-thursday_abbrev")}</td>
                    <td><CanticleLink number="8" version="RiteII" label="8. Cantemus Domino" /></td>
                    <AorB a=("6", "RiteI", "6.") b=("20", "RiteII", "20. Gloria in excelsis")/>
                </tr>
                <tr>
                    <td></td>
                    <td></td>
                    <td>
                        <em>
                            {t("canticle-table-advent_and_lent")}
                            ": "
                        </em>
                        <br/>
                        <CanticleLink number="19" version="RiteII" label="19. Magna et mirabilia" />
                    </td>
                </tr>
                <tr>
                    <td class="day-name">{t("canticle-table-friday_abbrev")}</td>
                    <td><CanticleLink number="10" version="RiteII" label="10. Quaerite Dominums" /></td>
                    <td><CanticleLink number="18" version="RiteII" label="18. Dignus es" /></td>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <em>
                            {t("canticle-table-lent")}
                            ": "
                        </em>
                        <br/>
                        <CanticleLink number="14" version="RiteII" label="14. Kyrie Pantokrator" />
                    </td>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-saturday_abbrev")}</td>
                    <AorB a=("1", "RiteI", "1.") b=("12", "RiteII", "12. Benedicite")/>
                    <td><CanticleLink number="19" version="RiteII" label="19. Magna et mirabilia" /></td>
                </tr>
                <tr>
                    <td colspan="3">
                        <p>
                            <em class="rubric">{t("canticle-table-on_feasts")}</em>
                        </p>
                    </td>
                </tr>
                    <tr>
                    <td></td>
                    <AorB a=("4", "RiteI", "4.") b=("16", "RiteII", "16. Benedictus Dominus")/>
                    <AorB a=("7", "RiteI", "7.") b=("21", "RiteII", "21. Te Deum laudamus")/>
                </tr>
            </tbody>
        </table>

        <h3>{t("canticle-table-canticles_ep")}</h3>
        <table class="CanticleTable-table">
            <thead>
                <tr>
                    <th></th>
                    <th><em>{t("canticle-table-after_ot")}</em></th>
                    <th><em>{t("canticle-table-after_nt")}</em></th>
                </tr>
            </thead>
            <tbody>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-sunday_abbrev")}</td>
                    <AorB a=("3", "RiteI", "3.") b=("15", "RiteII", "15. Magnificat")/>
                    <AorB a=("5", "RiteI", "5.") b=("17", "RiteII", "17. Nunc dimittis")/>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-monday_abbrev")}</td>
                    <td><CanticleLink number="8" version="RiteII" label="8. Cantemus, Domino" /></td>
                    <AorB a=("5", "RiteI", "5.") b=("17", "RiteII", "17. Nunc dimittis")/>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <em>
                            {t("canticle-table-lent")}
                            ": "
                        </em>
                        <br/>
                        <CanticleLink number="14" version="RiteII" label="14. Kyrie Pantrokrator" />
                    </td>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-tuesday_abbrev")}</td>
                    <td><CanticleLink number="10" version="RiteII" label="10. Quaerite Dominum" /></td>
                    <AorB a=("3", "RiteI", "3.") b=("15", "RiteII", "15. Magnificat")/>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-wednesday_abbrev")}</td>
                    <AorB a=("1", "RiteI", "1.") b=("12", "RiteII", "12. Benedicite")/>
                    <AorB a=("5", "RiteI", "5.") b=("17", "RiteII", "17. Nunc dimittis")/>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-thursday_abbrev")}</td>
                    <td><CanticleLink number="11" version="RiteII" label="11. Surge, illuminare" /></td>
                    <AorB a=("3", "RiteI", "3.") b=("15", "RiteII", "15. Magnificat")/>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-friday_abbrev")}</td>
                    <AorB a=("2", "RiteI", "2.") b=("13", "RiteII", "13. Benedictus es")/>
                    <AorB a=("5", "RiteI", "5.") b=("17", "RiteII", "17. Nunc dimittis")/>
                </tr>
                <tr class="day">
                    <td class="day-name">{t("canticle-table-saturday_abbrev")}</td>
                    <td><CanticleLink number="9" version="RiteII" label="9. Ecce, Deus" /></td>
                    <AorB a=("3", "RiteI", "3.") b=("15", "RiteII", "15. Magnificat")/>
                </tr>
                <tr>
                    <td colspan="3">
                        <p>
                            <em class="rubric">{t("canticle-table-on_feasts")}</em>
                        </p>
                    </td>
                </tr>
                <tr>
                    <td></td>
                    <AorB a=("3", "RiteI", "3.") b=("15", "RiteII", "15. Magnificat")/>
                    <AorB a=("5", "RiteI", "5.") b=("17", "RiteII", "17. Nunc dimittis")/>
                </tr>
            </tbody>
        </table>
    </section>
    }
}

#[component]
pub fn EOWTable(cx: Scope) -> Element {
    let (t, _, _) = use_i18n(cx);

    view! {
        // EOW 1 Canticle Table
        <section id="EOW1">
            <h2>{t("eow_1")}</h2>
             <Reference reference=liturgy::Reference {
                source: Source::EOW1,
                page: 44,
            }/>
            <details>
                <summary>{t("canticle-table-please_note")}</summary>
                <p>{t("canticle-table-eow_canticle_table_note")}</p>
            </details>
            <h3>{t("canticle-table-canticles_mp")}</h3>
            <h4>{t("canticle-table-supplemental_materials")}</h4>
            <table class="CanticleTable-table">
                <thead>
                    <tr>
                        <th></th>
                        <th><em>{t("canticle-table-after_ot")}</em></th>
                        <th><em>{t("canticle-table-after_nt")}</em></th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="day">
                        <td class="day-name">{t("canticle-table-sunday_abbrev")}</td>
                        <AorB a=("e", "EOW", "E. A Song of Jerusalem Our Mother") b=("16", "EOW", "16. The Song of Zechariah")/>
                        <AorB a=("k", "EOW", "K. A Song of Our Adoption") b=("21", "EOW", "21. We Praise You O Go")/>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-advent")}
                                ": "
                            </em>
                            <br/>
                            <CanticleLink number="d" version="EOW" label="D. A Song of the Wilderness" />
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-advent")}
                                ": "
                            </em>
                            <br/>
                            <CanticleLink number="p" version="EOW" label="P. A Song of the Spirit" />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-christmas")}
                                ": "
                            </em>
                            <br/>
                            <AorB a=("c", "EOW", "C. A Song of Hannah") b=("9", "RiteII", "9. The First Song of Isaiah")/>
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-christmas")}
                                ": "
                            </em>
                            <br/>
                            <AorB a=("n", "EOW", "N. A Song of God’s Love") b=("20", "RiteII", "20. Glory to God")/>
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-lent")}
                                ": "
                            </em>
                            <br/>
                            <CanticleLink number="h" version="EOW" label="H. A Song of Hosea" />
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-lent")}
                                ": "
                            </em>
                            <br/>
                            <CanticleLink number="l" version="EOW" label="L. A Song of Christ’s Humility" />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-easter")}
                                ": "
                            </em>
                            <br/>
                            <AorB a=("a", "EOW", "A. A Song of Wisdom") b=("8", "RiteII", "8. The Song of Moses")/>
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-easter")}
                                ": "
                            </em>
                            <br/>
                            <CanticleLink number="m" version="EOW" label="M. A Song of Faith" />
                        </td>
                    </tr>
                    <tr class="day">
                        <td class="day-name">{t("canticle-table-monday_abbrev")}</td>
                        <AorB a=("c", "EOW", "C. A Song of Hannah") b=("11", "RiteII", "11. The Third Song of Isaiah")/>
                        <AorB a=("l", "EOW", "L. A Song of Christ’s Humility") b=("q", "EOW", "Q. A Song of Christ’s Goodness")/>
                    </tr>
                    <tr class="day">
                        <td class="day-name">{t("canticle-table-tuesday_abbrev")}</td>
                        <AorB a=("b", "EOW", "B. A Song of Pilgrimage") b=("13", "EOW", "13. A Song of Praise")/>
                        <AorB a=("m", "EOW", "M. A Song of Faith") b=("n", "EOW", "N. A Song of God’s Love")/>
                    </tr>
                    <tr class="day">
                        <td class="day-name">{t("canticle-table-wednesday_abbrev")}</td>
                        <AorB a=("g", "EOW", "G. A Song of Ezekiel") b=("h", "EOW", "H. A Song of Hosea")/>
                        <AorB a=("p", "EOW", "P. A Song of the Spirit") b=("s", "EOW", "S. A Song of Our True Nature")/>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-lent")}
                                ": "
                            </em>
                            <br/>
                            <AorB a=("i", "EOW", "I. A Song of Jonah") b=("10", "RiteII", "10. The Second Song of Isaiah")/>
                        </td>
                        <td></td>
                    </tr>
                    <tr class="day">
                        <td class="day-name">{t("canticle-table-thursday_abbrev")}</td>
                        <AorB a=("a", "EOW", "A. A Song of Wishdom") b=("j", "EOW", "J. A Song of Judith")/>
                        <AorB a=("r", "EOW", "R. A Song of True Motherhood") b=("16", "RiteII", "16. A Song of Zechariah")/>
                    </tr>
                    <tr class="day">
                        <td class="day-name">{t("canticle-table-friday_abbrev")}</td>
                        <td><CanticleLink number="i" version="EOW" label="I. A Song of Jonah" /></td>
                        <td><CanticleLink number="18" version="EOW" label="18. Song to the Lamb" /></td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-christmas")}
                                ":* "
                            </em>
                            <br/>
                            <CanticleLink number="j" version="EOW" label="J. A Song of Judith" />
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-christmas")}
                                ":* "
                            </em>
                            <br/>
                            <CanticleLink number="r" version="EOW" label="R. A Song of True Motherhood" />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-lent")}
                                ": "
                            </em>
                            <br/>
                            <AorB a=("f", "EOW", "F. A Song of Lamentation") b=("14", "RiteII", "14. A Song of Penitence")/>
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-lent")}
                                ": "
                            </em>
                            <br/>
                            <CanticleLink number="s" version="EOW" label="S. A Song of Our True Nature" />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>
                            <em>
                                {t("canticle-table-easter")}
                                ":* "
                            </em>
                            <br/>
                            <CanticleLink number="g" version="EOW" label="G. A Song of Ezekiel" />
                        </td>
                        <td>
                            <em>
                                {t("canticle-table-easter")}
                                ":* "
                            </em>
                            <br/>
                            <CanticleLink number="k" version="EOW" label="K. A Song of Our Adoption" />
                        </td>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-saturday_abbrev")}</td>
                        <AorB a=("12", "EOW", "12. A Song of Creation") b=("d", "EOW", "D. A Song of the Wilderness")/>
                        <AorB a=("o", "EOW", "O. A Song of the Heavenly City") b=("19", "RiteII", "19. The Song of the Redeemed")/>
                    </tr>
                    <tr>
                        <td colspan="3">
                            <p>
                                <em class="rubric">{t("canticle-table-on_feasts")}</em>
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <AorB a=("16", "EOW", "16. A Song of Zechariah") b=("e", "EOW", "E. A Song of Jerusalem Our Mother")/>
                        <AorB a=("21", "EOW", "21. We Praise You O GOd") b=("K", "EOW", "K. A Song of Our Adoption")/>
                    </tr>
                    <tr>
                        <td colspan="3">
                            <p>
                                <em class="rubric">{t("canticle-table-canticles_appointed_for_christmas")}</em>
                            </p>
                        </td>
                    </tr>
                </tbody>
            </table>

            <h3>{t("canticle-table-canticles_ep")}</h3>
            <table class="CanticleTable-table">
                <thead>
                    <tr>
                        <th></th>
                        <th><em>{t("canticle-table-after_ot")}</em></th>
                        <th><em>{t("canticle-table-after_nt")}</em></th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="day-name">{t("canticle-table-sunday_abbrev")}</td>
                        <td><CanticleLink number="15" version="EOW" label="15. The Song of Mary" /></td>
                        <AorB a=("17", "RiteII", "The Song of Simeon**") b=("m", "EOW", "M. A Song of Faith")/>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-monday_abbrev")}</td>
                        <td><CanticleLink number="a" version="EOW" label="A. A Song of Wisdom" /></td>
                        <AorB a=("n", "EOW", "N. A Song of God’s Love") b=("17", "RiteII", "The Song of Simeon")/>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-tuesday_abbrev")}</td>
                        <td><CanticleLink number="d" version="EOW" label="D. A Song of the Wilderness" /></td>
                        <AorB a=("15", "EOW", "15. The Song of Mary") b=("p", "EOW", "P. A Song of the Spirit")/>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-wednesday_abbrev")}</td>
                        <td><CanticleLink number="c" version="EOW" label="C. The Song of Hannah" /></td>
                        <AorB a=("l", "EOW", "L. A Song of Christ’s Humility") b=("17", "RiteII", "The Song of Simeon")/>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-thursday_abbrev")}</td>
                        <td><CanticleLink number="j" version="EOW" label="J. A Song of Judith" /></td>
                        <AorB a=("15", "EOW", "15. The Song of Mary") b=("s", "EOW", "S. A Song of Our True Nature")/>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-friday_abbrev")}</td>
                        <td><CanticleLink number="g" version="EOW" label="G. A Song of Ezekiel" /></td>
                        <AorB a=("q", "EOW", "Q. A Song of Christ’s Goodness") b=("17", "RiteII", "The Song of Simeon")/>
                    </tr>
                    <tr>
                        <td class="day-name">{t("canticle-table-saturday_abbrev")}</td>
                        <td><CanticleLink number="b" version="EOW" label="B. A Song of Pilgrimage" /></td>
                        <AorB a=("15", "EOW", "15. The Song of Mary") b=("r", "EOW", "R. A Song of True Motherhood")/>
                    </tr>
                    <tr>
                        <td colspan="3">
                            <p>
                                <em class="rubric">{t("canticle-table-on_feasts")}</em>
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td><CanticleLink number="15" version="EOW" label="15. The Song of Mary" /></td>
                        <AorB a=("o", "EOW", "O. A Song of the Heavenly City**") b=("17", "RiteII", "The Song of Simeon**")/>
                    </tr>
                    <tr>
                        <td colspan="3">
                            <p>
                                <em class="rubric">{t("canticle-table-magnificat_note_eow")}</em>
                            </p>
                        </td>
                    </tr>
                </tbody>
            </table>
        </section>
    }
}

#[component]
fn CanticleLink(
    cx: Scope,
    number: &'static str,
    version: &'static str,
    label: &'static str,
) -> Element {
    let locale = use_language(cx);
    view! {
        <a href=move || format!("/{}/document/office/canticles/{}/{}", locale(), number, version)>
            {label}
        </a>
    }
}

#[component]
fn AorB(
    cx: Scope,
    a: (&'static str, &'static str, &'static str),
    b: (&'static str, &'static str, &'static str),
) -> Element {
    let (t, _, _) = use_i18n(cx);
    let (a_number, a_version, a_label) = a;
    let (b_number, b_version, b_label) = b;
    let separator = if a_label.len() < 5 {
        None
    } else {
        Some(view! { <br/> })
    };
    view! {
        <td>
            <CanticleLink number=a_number version=a_version label=a_label/>
            " "
            {separator}
            {t("canticle-table-or")}
            " "
            <CanticleLink number=b_number version=b_version label=b_label/>
        </td>
    }
}
