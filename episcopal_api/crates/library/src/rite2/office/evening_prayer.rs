use crate::bcp1979::AN_ORDER_OF_WORSHIP_FOR_EVENING;
use crate::conditions::{EASTER_SEASON, NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::{
    APOSTLES_CREED, GLORIA_PATRI, LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL, WORD_OF_THE_LORD,
};
use calendar::Weekday;
use canticle_table::CanticleNumber;
use lectionary::ReadingType;
use liturgy::*;

lazy_static! {
    pub static ref PHOS_HILARON : Document = Document::from(Invitatory {
        local_name: String::from("O Gracious Light"),
        citation: None,
        latin_name: Some(String::from("Phos hilaron")),
        antiphon: SeasonalAntiphon::Omit,
        gloria_patri: None,
        sections: vec![
            InvitatorySection {
                verses: vec![
                    InvitatoryVerse {
                        a: String::from("O gracious Light,\npure brightness of the everliving Father in heaven,\nO Jesus Christ, holy and blessed!"),
                        b: String::from("")
                    }
                ]
            },
            InvitatorySection {
                verses: vec![
                    InvitatoryVerse {
                        a: String::from("Now as we come to the setting of the sun,\nand our eyes behold the vesper light,\nwe sing your praises, O God: Father, Son, and Holy Spirit."),
                        b: String::from("")
                    }
                    ]
            },
            InvitatorySection {
                verses: vec![
                    InvitatoryVerse {
                        a: String::from("You are worthy at all times to be praised by happy voices,\nO Son of God, O Giver of life,\nand to be glorified through all the worlds."),
                        b: String::from("")
                    }
                ]
            }
        ]
    });

    pub static ref EVENING_PRAYER_II : Document = Document::from(Liturgy::from(Series::from([
        Document::from(Heading::from((
            HeadingLevel::Heading1,
            "Daily Evening Prayer:\nRite Two",
        ))),
        Document::from(Heading::InsertDate),
        Document::from(Heading::InsertDay),

        Document::from(Rubric::from("The Officiant begins the service with one or more of the following sentences of Scripture;\n\nor with the Service of Light, and continuing with the appointed Psalmody;\n\nor with the versicle “O God, make speed to save us.”")).page(115),
        Document::from(Content::DocumentLink { label: "Opening Sentences".into(), path: SlugPath::from([Slug::Office, Slug::OpeningSentences, Slug::Version(Version::RiteII)]), rotate: false, link_only: false }).display(Show::TemplateOnly),
        Document::from(Content::DocumentLink {
          label: "Service of Light".into(),
          path: SlugPath::from([Slug::Office, Slug::ServiceOfLight]),
          rotate: false,
          link_only: true
        }).display(Show::TemplateOnly),

        // Service of Light
        AN_ORDER_OF_WORSHIP_FOR_EVENING.clone().condition(
            Condition::Preference(PreferenceKey::from("ServiceOfLight"), PreferenceValue::Bool(true))
        ).display(Show::CompiledOnly),

        // Neither Service of Light nor OmitForeOffice
        Document::from(Series::from([
            Document::from(Content::DocumentLink { label: "Opening Sentences".into(), path: SlugPath::from([Slug::Office, Slug::OpeningSentences, Slug::Version(Version::RiteII)]), rotate: true, link_only: false }).display(Show::CompiledOnly),
            Document::from(Rubric::from("The following Confession of Sin may then be said; or the Office may continue at once with “O God make speed to save us.”")).page(116),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confession of Sin"))),
            Document::from(Rubric::from("The Officiant says to the people")),
            Document::from(Choice::from([
                  Document::from("Dearly beloved, we have come together in the presence of Almighty God our heavenly Father, to set forth his praise, to hear his holy Word, and to ask, for ourselves and on behalf of others, those things that are necessary for our life and our salvation. And so that we may prepare ourselves in heart and mind to worship him, let us kneel in silence, and with penitent and obedient hearts confess our sins, that we may obtain forgiveness by his infinite goodness and mercy. ").version_label("Long"),
                  Document::from("Let us confess our sins against God and our neighbor.").version_label("Short")
                ]).selected(1)).page(116),
            Document::from(Rubric::from("Silence may be kept.\n\nOfficiant and People together, all kneeling")),
            Document::from(Text::from("Most merciful God,\nwe confess that we have sinned against you\nin thought, word, and deed,\nby what we have done,\nand by what we have left undone.\nWe have not loved you with our whole heart;\nwe have not loved our neighbors as ourselves.\nWe are truly sorry and we humbly repent.\nFor the sake of your Son Jesus Christ,\nhave mercy on us and forgive us;\nthat we may delight in your will,\nand walk in your ways,\nto the glory of your Name.")
                .response("Amen.")
                .display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Priest alone stands and says")),
            Document::from(Text::from("Almighty God have mercy on you, forgive you all your sins through our Lord Jesus Christ, strengthen you in all goodness, and by the power of the Holy Spirit keep you in eternal life.")
                .response("Amen."))
                .version_label("Priest"),
            Document::from(Rubric::from("A deacon or lay person using the preceding form remains kneeling, and substitutes “us” for “you” and “our” for “your.”"))
        ])).condition(
            Condition::None(vec![
                Condition::Preference(PreferenceKey::from(GlobalPref::OmitForeOffice), PreferenceValue::Bool(true)),
                Condition::Preference(PreferenceKey::from("ServiceOfLight"), PreferenceValue::Bool(true))
            ])
        ),

        // Invitatory and Psalter
        Document::from(Series::from([
            Document::from(Heading::from((HeadingLevel::Heading2, "The Invitatory and Psalter"))),
            Document::from(Rubric::from("All stand")),
            Document::from(Preces::from([
                ("Officiant", "O God, make speed to save us."),
                ("People",  "O Lord, make haste to help us.")
            ])).source(Reference::from(117)),
            Document::from(Rubric::from("Officiant and People")),
            Document::from(GLORIA_PATRI.clone()),
            Document::from(Rubric::from("Except in Lent, add")).display(Show::TemplateOnly),
            Document::from("Alleluia.").condition(NOT_LENT.clone()),
            Document::from(Rubric::from("The following, or some other suitable hymn, or an Invitatory Psalm, may be sung or said")),
            PHOS_HILARON.clone().page(118)
        ])).condition(Condition::Not(Box::new(
            Condition::Preference(PreferenceKey::from("ServiceOfLight"), PreferenceValue::Bool(true))
        ))),

        // Psalms
          Document::from(Rubric::from("Then follows")),
          Document::from(Heading::from((HeadingLevel::Heading3, "The Psalm or Psalms Appointed"))),
          Document::from(LectionaryReading {
            reading_type: ReadingTypeTable::Selected(ReadingType::EveningPsalm),
            reading_type_overridden_by: None,
            lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::PsalmCycle)),
            intro: None,
          }),
          Document::from(Rubric::from("At the end of the Psalms is sung or said")).condition(NOT_INSERT_GLORIA.clone()),
          Document::from(GLORIA_PATRI.clone()).condition(NOT_INSERT_GLORIA.clone()),

          // Lessons
          Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))),
          Document::from(Rubric::from("One or two lessons, as appointed, are read, the Reader first saying")).display(Show::TemplateOnly),
          Document::from("A Reading (Lesson) from _______________.").display(Show::TemplateOnly),
          Document::from(Rubric::from("After each Lesson the Reader may say")).display(Show::TemplateOnly),
          Document::from(WORD_OF_THE_LORD.clone()).display(Show::TemplateOnly),
          Document::from(Rubric::from("Or the Reader may say")).display(Show::TemplateOnly),
          Document::from("Here ends the Lesson (Reading).").display(Show::TemplateOnly),
          // A link to the daily readings page, TemplateOnly
          Document::from(LectionaryReading {
            reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingA)),
            reading_type_overridden_by: None,
            lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
            intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from {{long_name}}."))),
          }).display(Show::TemplateOnly),
          Document::from(Rubric::from("Silence may be kept after each Reading. One of the Canticles is sung or said after each Reading. If three Lessons are used, the Lesson from the Gospel is read after the second Canticle.")).display(Show::TemplateOnly),
          // A link to the canticle table, TemplateOnly
          Document::from(CanticleTableEntry {
            nth: CanticleNumber::One,
            table: CanticleTableChoice::Preference(PreferenceKey::from(GlobalPref::CanticleTable))
          }).display(Show::TemplateOnly),

          // First Lesson
          Document::from(LectionaryReading {
            reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingA)),
            reading_type_overridden_by: Some(ReadingType::Evening1),
            lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
            intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from {{long_name}}."))),
          }).label("The First Lesson").display(Show::CompiledOnly),
          Document::from(WORD_OF_THE_LORD.clone()).display(Show::CompiledOnly),

          // Canticle
          Document::from(CanticleTableEntry {
            nth: CanticleNumber::One,
            table: CanticleTableChoice::Preference(PreferenceKey::from(GlobalPref::CanticleTable))
          }).display(Show::CompiledOnly),

          // Second Lesson
          Document::from(Series::from([
            Document::from(LectionaryReading {
              reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingB)),
              reading_type_overridden_by: Some(ReadingType::Evening2),
              lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
              intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from {{long_name}}."))),
            }).label("The Second Lesson").display(Show::CompiledOnly),
            Document::from(WORD_OF_THE_LORD.clone()),
            // Canticle
            Document::from(CanticleTableEntry {
              nth: CanticleNumber::Two,
              table: CanticleTableChoice::Preference(PreferenceKey::from(GlobalPref::CanticleTable))
            }),
          ])).condition(  // include lesson and response unless the reading preference is set to "—"
            Condition::Not(Box::new(
              Condition::Preference(
                PreferenceKey::from(GlobalPref::ReadingB),
                PreferenceValue::from(ReadingType::Empty))
              )
            ))
            .display(Show::CompiledOnly),

          // Third Lesson
          Document::from(Series::from([
            Document::from(LectionaryReading {
              reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingC)),
              reading_type_overridden_by: None,
              lectionary: LectionaryTableChoice::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
              intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from {{longName}}."))),
            }).label("The Third Lesson"),
            Document::from(WORD_OF_THE_LORD.clone()),
          ])).condition( // include lesson and response unless the reading preference is set to "—"
            Condition::Not(Box::new(
              Condition::Preference(
                PreferenceKey::from(GlobalPref::ReadingC),
                PreferenceValue::from(ReadingType::Empty))
              )
            ))
            .display(Show::CompiledOnly),

        // Creed
        Document::from(Heading::from((HeadingLevel::Heading3, "The Apostles’ Creed"))),
        Document::from(Rubric::from("Officiant and People together, all standing")),
        Document::from(APOSTLES_CREED.clone()).page(120),

        // The Prayers
        Document::from(Heading::from((HeadingLevel::Heading2, "The Prayers"))).page(121),
        Document::from(Rubric::from("The People stand or kneel")),
        Document::from(Preces::from([
          ("Officiant", "The Lord be with you."),
          ("People", "And also with you."),
          ("Officiant", "Let us pray.")
        ])),
        Document::from(Rubric::from("Officiant and People")),
        Document::from(LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL.clone()),
        Document::from(Rubric::from("Then follows one of these sets of Suffrages")),
        Document::from(Choice::from([
          Document::from(Preces::from([
            ("V.", "Show us your mercy, O Lord;"),
            ("R.", "And grant us your salvation."),
            ("V.", "Clothe your ministers with righteousness;"),
            ("R.", "Let your people sing with joy."),
            ("V.", "Give peace, O Lord, in all the world;"),
            ("R.", "For only in you can we live in safety."),
            ("V.", "Lord, keep this nation under your care; "),
            ("R.", "And guide us in the way of justice and truth."),
            ("V.", "Let your way be known upon earth;"),
            ("R.", "Your saving health among all nations."),
            ("V.", "Let not the needy, O Lord, be forgotten;"),
            ("R.", "Nor the hope of the poor be taken away."),
            ("V.", "Create in us clean hearts, O God;"),
            ("R.", "And sustain us with your Holy Spirit.")
          ])).label("A"),
          Document::from(Litany::from((
              "We entreat you, O Lord.",
              [
                "That this evening may be holy, good, and peaceful,",
                "That your holy angels may lead us in paths of peace and goodwill,",
                "That we may be pardoned and forgiven for our sins and offenses,",
                "That there may be peace to your Church and to the whole world,",
                "That we may depart this life in your faith and fear, and not be condemned before the great judgment seat of Christ,",
                "That we may be bound together by your Holy Spirit in the communion of [___________ and] all your saints, entrusting one another and all our life to Christ,"
              ]
          ))).label("B")
        ]).should_rotate()),

        Document::from(Rubric::from("The Officiant then says one or more of the following Collects")),
        Document::from(Content::CollectOfTheDay { allow_multiple: true }),

        // Weekday collects: will rotate among the acceptable possibilities, by day
        Document::from(Text::from("Lord God, whose Son our Savior Jesus Christ triumphed over the powers of death and prepared for us our place in the new Jerusalem: Grant that we, who have this day given thanks for his resurrection, may praise you in that City of which he is the light, and where he lives and reigns for ever and ever.")
            .response("Amen."))
            .label("A Collect for Sundays")
            .condition(Condition::Weekday(Weekday::Sun)),
          Document::from(Text::from("Lord Jesus Christ, by your death you took away the sting of death: Grant to us your servants so to follow in faith where you have led the way, that we may at length fall asleep peacefully in you and wake up in your likeness; for your tender mercies’ sake.")
            .response("Amen."))
            .label("A Collect for Fridays")
            .condition(Condition::Weekday(Weekday::Fri)),
          Document::from(Text::from("O God, the source of eternal light: Shed forth your unending day upon us who watch for you, that our lips may praise you, our lives may bless you, and our worship on the morrow give you glory; through Jesus Christ our Lord.")
            .response("Amen."))
            .label("A Collect for Saturdays")
            .condition(Condition::Weekday(Weekday::Sat)),
        Document::from(Choice::from([
          Document::from(Text::from("Most holy God, the source of all good desires, all right judgments, and all just works: Give to us, your servants, that peace which the world cannot give, so that our minds may be fixed on the doing of your will, and that we, being delivered from the fear of all enemies, may live in peace and quietness; through the mercies of Christ Jesus our Savior.")
            .response("Amen."))
            .label("A Collect for Peace"),
          Document::from(Text::from("Be our light in the darkness, O Lord, and in your great mercy defend us from all perils and dangers of this night; for the love of your only Son, our Savior Jesus Christ.")
            .response("Amen."))
            .label("A Collect for Aid against Perils"),
          Document::from(Text::from("O God, the life of all who live, the light of the faithful, the strength of those who labor, and the repose of the dead: We thank you for the blessings of the day that is past, and humbly ask for your protection through the coming night. Bring us in safety to the morning hours; through him who died and rose again for us, your Son our Savior Jesus Christ.")
            .response("Amen."))
            .label("A Collect for Protection"),
          Document::from(Text::from("Lord Jesus, stay with us, for evening is at hand and the day is past; be our companion in the way, kindle our hearts, and awaken hope, that we may know you as you are revealed in Scripture and the breaking of bread. Grant this for the sake of your love.")
            .response("Amen."))
            .label("A Collect for the Presence of Christ")
        ]).should_rotate()).condition(Condition::None(vec![
          Condition::Weekday(Weekday::Fri),
          Condition::Weekday(Weekday::Sat),
          Condition::Weekday(Weekday::Sun)
        ])),

        Document::from(Rubric::from("Then, unless the Eucharist or a form of general intercession is to follow, one of these prayers for mission is added")),
        Document::from(Choice::from([
          Document::from(Text::from("O God and Father of all, whom the whole heavens adore: Let the whole earth also worship you, all nations obey you, all tongues confess and bless you, and men and women everywhere love you and serve you in peace; through Jesus Christ our Lord.")
            .response("Amen.")),
          Document::from(Text::from("Keep watch, dear Lord, with those who work, or watch, or weep this night, and give your angels charge over those who sleep. Tend the sick, Lord Christ; give rest to the weary, bless the dying, soothe the suffering, pity the afflicted, shield the joyous; and all for your love’s sake.")
            .response("Amen.")),
          Document::from(Text::from("O God, you manifest in your servants the signs of your presence: Send forth upon us the spirit of love, that in companionship with one another your abounding grace may increase among us; through Jesus Christ our Lord.")
            .response("Amen.")),
        ]).should_rotate()),

        // Closing Prayers
        Document::from(Rubric::from("Here may be sung a hymn or anthem.")),
        Document::from(HymnLink::Tag("Evening".into())),

        Document::from(Rubric::from("Authorized intercessions and thanksgivings may follow.")),

        Document::from(Content::DocumentLink { label: "Prayers and Thanksgivings".into(), path: SlugPath::from([Slug::PrayersAndThanksgivings]), rotate: false, link_only: true }),

        Document::from(Rubric::from("Before the close of the Office one or both of the following may be used")),
        Document::from(Text::from("Almighty God, Father of all mercies,\nwe your unworthy servants give you humble thanks\nfor all your goodness and loving-kindness\nto us and to all whom you have made.\nWe bless you for our creation, preservation,\nand all the blessings of this life;\nbut above all for your immeasurable love\nin the redemption of the world by our Lord Jesus Christ;\nfor the means of grace, and for the hope of glory.\n\nAnd, we pray, give us such an awareness of your mercies,\nthat with truly thankful hearts we may show forth your praise,\nnot only with our lips, but in our lives,\nby giving up our selves to your service,\nand by walking before you\nin holiness and righteousness all our days;\nthrough Jesus Christ our Lord,\nto whom, with you and the Holy Spirit,\nbe honor and glory throughout all ages. ")
          .response("Amen.")
          .display_format(DisplayFormat::Unison))
          .label("The General Thanksgiving")
          .source(Reference::from(125)),
        Document::from(Text::from("Almighty God, you have given us grace at this time with one accord to make our common supplication to you; and you have promised through your well-beloved Son that when two or three are gathered together in his Name you will be in the midst of them: Fulfill now, O Lord, our desires and petitions as may be best for us; granting us in this world knowledge of your truth, and in the age to come life everlasting.")
          .response("Amen."))
          .label("A Prayer of St. Chrysostom")
          .source(Reference::from(126)),
        Document::from(Rubric::from("Then may be said")),

        // Dismissal and Closing Sentence
        Document::from(ResponsivePrayer::from([
          "Let us bless the Lord.",
          "Thanks be to God."
        ])).condition(Condition::Not(Box::new(EASTER_SEASON.clone()))),
        Document::from(Rubric::from("From Easter Day through the Day of Pentecost “Alleluia, alleluia” may be added to the preceding versicle and response."))
          .display(Show::TemplateOnly),
        Document::from(ResponsivePrayer::from([
          "Let us bless the Lord. Alleluia, alleluia.",
          "Thanks be to God. Alleluia, alleluia."
        ]))
          .condition(EASTER_SEASON.clone())
          .display(Show::CompiledOnly),
        Document::from(Rubric::from("The Officiant may then conclude with one of the following")),

        Document::from(Content::DocumentLink { label: "Closing Sentences".into(), path: SlugPath::from([Slug::Office, Slug::ClosingSentences, Slug::Version(Version::RiteII)]), rotate: true, link_only: false }),
    ]))
        .evening(true)
        .preferences([
        // Supplemental Devotions
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::OmitForeOffice),
          "Omit Fore-Office",
          [
            LiturgyPreferenceOption::from(("No", PreferenceValue::Bool(false))),
            LiturgyPreferenceOption::from(("Yes", PreferenceValue::Bool(true))),
          ]
        )).category("Liturgy"),

        LiturgyPreference::from((
          PreferenceKey::from("ServiceOfLight"),
          "An Order of Worship for the Evening",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::Bool(false))),
            LiturgyPreferenceOption::from(("Before the Office", PreferenceValue::Bool(true))),
          ]
        )).category("Supplemental Devotions"),

                // Translations
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::BibleVersion),
          "Bible Version",
          [
            LiturgyPreferenceOption::from(Version::NRSV),
            LiturgyPreferenceOption::from(Version::CEB),
            LiturgyPreferenceOption::from(Version::ESV),
            LiturgyPreferenceOption::from(Version::KJV)
          ]
        )).category("Translations"),

        // Lectionaries
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::PsalmCycle),
          "Psalm Cycle",
          [
            LiturgyPreferenceOption::from(("Daily Office Lectionary", PreferenceValue::from(Lectionaries::BCP1979DailyOfficePsalms))),
            LiturgyPreferenceOption::from(("30-day Cycle", PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms))),
            LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
            LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
          ]
        )).category("Lectionaries and Cycles"),

        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::Lectionary),
          "Lectionary",
          [
            LiturgyPreferenceOption::from(("Daily Office Lectionary", PreferenceValue::from(Lectionaries::BCP1979DailyOffice))),
            LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
            LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
          ]
        )).category("Lectionaries and Cycles"),

        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::CanticleTable),
          "Canticle Table",
          [
            LiturgyPreferenceOption::from(("Book of Common Prayer (1979)", PreferenceValue::from(CanticleTables::BCP1979RiteII))),
            LiturgyPreferenceOption::from(("Enriching Our Worship 1", PreferenceValue::from(CanticleTables::EOW))),
            LiturgyPreferenceOption::from(("Magnificat/Nunc Dimittis", PreferenceValue::from(CanticleTables::Classical))),
          ]
        )).category("Lectionaries and Cycles"),

        // Readings
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingA),
          "First Lesson",
          [
            LiturgyPreferenceOption::from(("First Lesson (Alternate Year)", PreferenceValue::from(ReadingType::FirstReadingAlternateYear))),
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::FirstReadingAlternateYear)).category("Lessons"),
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingB),
          "Second Lesson",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::Gospel)).category("Lessons"),
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingC),
          "Third Lesson",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::Empty)).category("Lessons"),
    ]))
    .label("Evening Prayer")
    .version(Version::RiteII)
    .page(115);
}
