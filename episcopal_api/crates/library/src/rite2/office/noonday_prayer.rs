use crate::conditions::{NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::{GLORIA_PATRI, LORDS_PRAYER_ABBREV};
use liturgy::{
    Condition, Content, DisplayFormat, Document, Heading, HeadingLevel, Liturgy, Preces,
    PreferenceKey, PreferenceValue, PsalmCitation, Reference, ResponsivePrayer, Rubric, Sentence,
    Series, Source, Text,
};

lazy_static! {
    pub static ref NOONDAY_PRAYER: Document =
        Document::new()
            .label("Noonday Prayer")
            .source(Reference {
                source: Source::BCP1979,
                page: 103
            })
            .content(Content::Liturgy(Liturgy::from(Series::from([
                // Include the title, date, and day in any case
                Document::from(Heading::from((HeadingLevel::Heading1, "An Order of Service for Noonday")))
                    .condition(Condition::Not(Box::new(
                        Condition::Preference(
                            PreferenceKey::from("angelus"),
                            PreferenceValue::from("before")
                        )
                    ))),
                Document::from(Heading::InsertDate),
                Document::from(Heading::InsertDay),

                // Opening of Noonday Prayer proper
                Document::from(Preces::from([
                    ("Officiant", "O God, make speed to save us."),
                    ("People", "O Lord, make haste to help us.")
                ])),
                Document::from(Rubric::from("Officiant and People")),
                Document::from(GLORIA_PATRI.clone().display_format(DisplayFormat::Unison)),
                Document::from(Rubric::from("Except in Lent, add")).condition(NOT_LENT.clone()),
                Document::from("Alleluia.").condition(NOT_LENT.clone()),

                // Psalms
                Document::from(Rubric::from("One or more of the following Psalms is sung or said. Other suitable selections include Psalms 19,67, one or more sections of Psalm 119, or a selection from Psalms 120 through 133.")),
                Document::from(PsalmCitation::from("Psalm 119:105-112")),
                Document::from(PsalmCitation::from("Psalm 121")),
                Document::from(PsalmCitation::from("Psalm 126")),
                Document::from(Rubric::from("At the end of the Psalms is sung or said")).condition(NOT_INSERT_GLORIA.clone()),
                Document::from(GLORIA_PATRI.clone()).condition(NOT_INSERT_GLORIA.clone()),

                // Reading
                Document::from(Rubric::from("One of the following, or some other suitable passage of Scripture, is read")),
                Document::from(Sentence::from("The love of God has been poured into our hearts through the Holy Spirit that has been given to us.")
                    .citation("Romans 5:5")
                    .response(Preces::from([("People", "Thanks be to God.")]))
                ),
                Document::from(Sentence::from("From the rising of the sun to its setting my Name shall be great among the nations, and in every place incense shall be offered to my Name, and a pure offering; for my Name shall be great among the nations, says the Lord of Hosts.")
                    .citation("Malachi 1:11")
                    .response(Preces::from([("People", "Thanks be to God.")]))
                ),

                // Prayers
                Document::from(Rubric::from("A meditation, silent or spoken, may follow.")),
                Document::from(Rubric::from("The Officiant then begins the Prayers")),
                Document::from(ResponsivePrayer::from([
                    "Lord, have mercy.",
                    "Christ, have mercy.",
                    "Lord, have mercy."
                ])),
                Document::from(Rubric::from("Officiant and People")),
                Document::from(LORDS_PRAYER_ABBREV.clone()),
                Document::from(Preces::from([
                    ("Officiant", "Lord, hear our prayer."),
                    ("People", "And let our cry come to you."),
                    ("Officiant", "Let us pray.")
                ])),
                Document::from(Rubric::from("The Officiant then says one of the following Collects. If desired, the Collect of the Day may be used.")),

                // Collects
                Document::from(
                    Text::from("Heavenly Father, send your Holy Spirit into our hearts, to direct and rule us according to your will, to comfort us in all our afflictions, to defend us from all error, and to lead us into all truth; through Jesus Christ our Lord.")
                        .response("Amen.")
                ),
                Document::from(
                    Text::from("Blessed Savior, at this hour you hung upon the cross, stretching out your loving arms: Grant that all the peoples of the earth may look to you and be saved; for your tender mercies’ sake.")
                        .response("Amen.")
                ),
                Document::from(
                    Text::from("Lord Jesus Christ, you said to your apostles, “Peace I give to you; my peace I leave with you:” Regard not our sins, but the faith of your Church, and give to us the peace and unity of that heavenly city, where with the Father and the Holy Spirit you live and reign, now and for ever.")
                        .response("Amen.")
                ),
                Document::from(
                    Text::from("Almighty Savior, who at noonday called your servant Saint Paul to be an apostle to the Gentiles: We pray you to illumine the world with the radiance of your glory, that all nations may come and worship you; for you live and reign for ever and ever.")
                        .response("Amen.")
                ),
                Document::from(Content::CollectOfTheDay { allow_multiple: true }),

                // Closing
                Document::from(Rubric::from("Free intercessions may be offered.")),
                Document::from(Rubric::from("The service concludes as follows")),
                Document::from(Preces::from([
                    ("Officiant", "Let us bless the Lord."),
                    ("People", "Thanks be to God.")
                ]))
            ]))));
}
