use crate::conditions::{EASTER_SEASON, NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::{
    APOSTLES_CREED, GLORIA_PATRI, LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL, WORD_OF_THE_LORD,
};
use calendar::Weekday;
use canticle_table::CanticleNumber;
use lectionary::ReadingType;
use liturgy::*;

lazy_static! {
    pub static ref MORNING_PRAYER_II: Document = Document::from(
      Liturgy::from(Series::from([
          Document::from(Heading::from((HeadingLevel::Heading1, "Daily Morning Prayer: Rite Two"))),
          Document::from(Heading::InsertDate),
          Document::from(Heading::InsertDay),

          // Fore-Office -- optionally omitted with "omitForeOffice" preference
          Document::from(Series::from([
            Document::from(Rubric::from("The Officiant begins the service with one or more of these sentences of Scripture, or with the versicle “Lord, open our lips.”")),
            Document::from(Categories::OpeningSentences),
            Document::from(Rubric::from("The following Confession of Sin may then be said; or the Office may continue at once with “Lord, open our lips.”")),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confession of Sin"))),
            Document::from(Rubric::from("The Officiant says to the people")),
            Document::from(Choice::from([
                  Document::from("Dearly beloved, we have come together in the presence of Almighty God our heavenly Father, to set forth his praise, to hear his holy Word, and to ask, for ourselves and on behalf of others, those things that are necessary for our life and our salvation. And so that we may prepare ourselves in heart and mind to worship him, let us kneel in silence, and with penitent and obedient hearts confess our sins, that we may obtain forgiveness by his infinite goodness and mercy. ").version_label("Long"),
                  Document::from("Let us confess our sins against God and our neighbor.").version_label("Short")
                ])).source(Reference::from(79)),
            Document::from(Rubric::from("Silence may be kept.\n\nOfficiant and People together, all kneeling")),
            Document::from(Text::from("Most merciful God,\nwe confess that we have sinned against you\nin thought, word, and deed,\nby what we have done,\nand by what we have left undone.\nWe have not loved you with our whole heart;\nwe have not loved our neighbors as ourselves.\nWe are truly sorry and we humbly repent.\nFor the sake of your Son Jesus Christ,\nhave mercy on us and forgive us;\nthat we may delight in your will,\nand walk in your ways,\nto the glory of your Name.")
.response("Amen.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Priest alone stands and says")),
            Document::from(Text::from("Almighty God have mercy on you, forgive you all your sins through our Lord Jesus Christ, strengthen you in all goodness, and by the power of the Holy Spirit keep you in eternal life.")
.response("Amen.")).version_label("Priest"),
            Document::from(Rubric::from("A deacon or lay person using the preceding form remains kneeling, and substitutes “us” for “you” and “our” for “your.”")),
          ])).condition(Condition::Not(
            Box::new(Condition::Preference(PreferenceKey::from("omitForeOffice"), PreferenceValue::from("yes"))))
          ),

          // Invitatory and Psalter
          Document::from(Heading::from((HeadingLevel::Heading2, "The Invitatory and Psalter"))),
          Document::from(Rubric::from("All stand")),
          Document::from(Preces::from([
            ("Officiant", "Lord, open our lips."),
            ("People", "And our mouth shall proclaim your praise.")
          ])).source(Reference::from(80)),
          Document::from(Rubric::from("Officiant and People")),
          Document::from(GLORIA_PATRI.clone()),
          Document::from(Rubric::from("Except in Lent,")).display(Show::TemplateOnly),
          Document::from("Alleluia.").condition(NOT_LENT.clone()),
          Document::from(Rubric::from("may be added.")).display(Show::TemplateOnly),
          Document::from(Rubric::from("One of the following Antiphons may be sung or said with the Invitatory Psalm")).display(Show::TemplateOnly),

          Document::from(Categories::InvitatoryAntiphons)
            .version(Version::RiteII)
            .display(Show::TemplateOnly),
          Document::from(Rubric::from("Then follows one of the Invitatory Psalms, Venite or Jubilate.")).display(Show::TemplateOnly),
          Document::from(Rubric::from("In Easter Week, in place of an Invitatory Psalm, the Pascha Nostrum is sung or said. It may also be used daily until the Day of Pentecost."))
            .condition(EASTER_SEASON.clone())
            .display(Show::TemplateOnly),

          // TODO: insert invitatories as printed
          // TODO add antiphons during compilation
          Document::from(Choice::from([
            // TODO Document::from(undefined).version(Version::RiteII).label("Venite").source(Reference::from(82)).condition(/* {"conditions":[{"preference":{"key":"psalmCycle","value":"30day-psalter","is":false}},{"day_of_month":{"neq":"19"}}],"mode":"or"} */),
            // TODO Document::from(undefined).version(Version::RiteII).label("Jubilate").source(Reference::from(82)),
            // TODO Document::from(undefined).version(Version::RiteII).label("Christ our Passover").version_label("Pascha Nostrum").source(Reference::from(83)).condition(/* {"mode":"and","conditions":[{"season":{"except":[],"only":["Easter","Ascension","Pentecost"]}}]} */)
          ])),

          // Psalms
          Document::from(Rubric::from("Then follows")),
          Document::from(Heading::from((HeadingLevel::Heading3, "The Psalm or Psalms Appointed"))).display(Show::TemplateOnly),
          Document::from(LectionaryReading {
            reading_type: ReadingTypeTable::Selected(ReadingType::MorningPsalm),
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

        // Possible location #1 for sermon
        Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon")))
          .condition(Condition::Preference(PreferenceKey::from("sermon"), PreferenceValue::from("after_readings"))),

        // Creed
        Document::from(Heading::from((HeadingLevel::Heading3, "The Apostles’ Creed"))),
        Document::from(Rubric::from("Officiant and People together, all standing")),
        Document::from(APOSTLES_CREED.clone()).source(Reference::from(53)),

        // Prayers
        Document::from(Heading::from((HeadingLevel::Heading2, "The Prayers"))).source(Reference::from(54)),
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
          Document::from(Preces::from([
            ("V.", "Save your people, Lord, and bless your inheritance;"),
            ("R.", "Govern and uphold them, now and always."),
            ("V.", "Day by day we bless you;"),
            ("R.", "We praise your name for ever."),
            ("V.", "Lord, keep us from all sin today;"),
            ("R.", "Have mercy on us, Lord, have mercy."),
            ("V.", "Lord, show us your love and mercy;"),
            ("R.", "For we put our trust in you."),
            ("V.", "In you, Lord, is our hope;"),
            ("R.", "And we shall never hope in vain.")
          ])).label("B")
        ])),
        Document::from(Rubric::from("The Officiant then says one or more of the following Collects")),

        Document::from(Content::CollectOfTheDay { allow_multiple: true }),

        Document::from(Text::from("O God, you make us glad with the weekly remembrance of the glorious resurrection of your Son our Lord: Give us this day such blessing through our worship of you, that the week to come may be spent in your favor; through Jesus Christ our Lord.")
          .response("Amen."))
          .label("A Collect for Sundays")
          .condition(Condition::Weekday(Weekday::Sun)),
        Document::from(Text::from("Almighty God, whose most dear Son went not up to joy but first he suffered pain, and entered not into glory before he was crucified: Mercifully grant that we, walking in the way of the cross, may find it none other than the way of life and peace; through Jesus Christ our Lord.")
          .response("Amen."))
          .label("A Collect for Fridays")
          .condition(Condition::Weekday(Weekday::Fri)),
        Document::from(Text::from("Almighty God, who after the creation of the world rested from all your works and sanctified a day of rest for all your creatures: Grant that we, putting away all earthly anxieties, may be duly prepared for the service of your sanctuary, and that our rest here upon earth may be a preparation for the eternal rest promised to your people in heaven; through Jesus Christ our Lord.")
          .response("Amen."))
          .label("A Collect for Saturdays")
          .condition(Condition::Weekday(Weekday::Sat)),
        Document::from(Text::from("O God, the King eternal, whose light divides the day from the night and turns the shadow of death into the morning: Drive far from us all wrong desires, incline our hearts to keep your law, and guide our feet into the way of peace; that, having done your will with cheerfulness during the day, we may, when night comes, rejoice to give you thanks; through Jesus Christ our Lord.")
          .response("Amen."))
          .label("A Collect for the Renewal of Life"),
        Document::from(Text::from("O God, the author of peace and lover of concord, to know you is eternal life and to serve you is perfect freedom: Defend us, your humble servants, in all assaults of our enemies; that we, surely trusting in your defense, may not fear the power of any adversaries; through the might of Jesus Christ our Lord. ")
          .response("Amen."))
          .label("A Collect for Peace"),
        Document::from(Text::from("Lord God, almighty and everlasting Father, you have brought us in safety to this new day: Preserve us with your mighty power, that we may not fall into sin, nor be overcome by adversity; and in all we do, direct us to the fulfilling of your purpose; through Jesus Christ our Lord.")
          .response("Amen."))
          .label("A Collect for Grace"),
        Document::from(Text::from("Heavenly Father, in you we live and move and have our being: We humbly pray you so to guide and govern us by your Holy Spirit, that in all the cares and occupations of our life we may not forget you, but may remember that we are ever walking in your sight; through Jesus Christ our Lord. ")
          .response("Amen."))
          .label("A Collect for Guidance"),
        Document::from(Rubric::from("Then, unless the Eucharist or a form of general intercession is to follow, one of these prayers for mission is added")),
        Document::from(Choice::from([
          Document::from(Text::from("O God, you have made of one blood all the peoples of the earth, and sent your blessed Son to preach peace to those who are far off and to those who are near: Grant that people everywhere may seek after you and find you; bring the nations into your fold; pour out your Spirit upon all flesh; and hasten the coming of your kingdom; through Jesus Christ our Lord. ")
            .response("Amen.")),
          Document::from(Text::from("Almighty and everlasting God, by whose Spirit the whole body of your faithful people is governed and sanctified: Receive our supplications and prayers which we offer before you for all members of your holy Church, that in their vocation and ministry they may truly and devoutly serve you; through our Lord and Savior Jesus Christ.")
            .response("Amen.")),
          Document::from(Text::from("Lord Jesus Christ, you stretched out your arms of love on the hard wood of the cross that everyone might come within the reach of your saving embrace: So clothe us in your Spirit that we, reaching forth our hands in love, may bring those who do not know you to the knowledge and love of you; for the honor of your Name.")
            .response("Amen.")),
        ])),

        // Possible location #2 for sermon
        Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon")))
          .condition(Condition::Preference(PreferenceKey::from("sermon"), PreferenceValue::from("after_collects"))),

        // Closing Prayers
        Document::from(Rubric::from("Here may be sung a hymn or anthem.\n\nAuthorized intercessions and thanksgivings may follow.")),

        Document::from(Categories::PrayersAndThanksgivings),

        Document::from(Rubric::from("Before the close of the Office one or both of the following may be used")),
        Document::from(Text::from("Almighty God, Father of all mercies,\nwe your unworthy servants give you humble thanks\nfor all your goodness and loving-kindness\nto us and to all whom you have made.\nWe bless you for our creation, preservation,\nand all the blessings of this life;\nbut above all for your immeasurable love\nin the redemption of the world by our Lord Jesus Christ;\nfor the means of grace, and for the hope of glory.\n\nAnd, we pray, give us such an awareness of your mercies,\nthat with truly thankful hearts we may show forth your praise,\nnot only with our lips, but in our lives,\nby giving up our selves to your service,\nand by walking before you\nin holiness and righteousness all our days;\nthrough Jesus Christ our Lord,\nto whom, with you and the Holy Spirit,\nbe honor and glory throughout all ages. ")
          .response("Amen.")
          .display_format(DisplayFormat::Unison))
          .label("The General Thanksgiving")
          .source(Reference::from(58)),
        Document::from(Text::from("Almighty God, you have given us grace at this time with one accord to make our common supplication to you; and you have promised through your well-beloved Son that when two or three are gathered together in his Name you will be in the midst of them: Fulfill now, O Lord, our desires and petitions as may be best for us; granting us in this world knowledge of your truth, and in the age to come life everlasting.")
          .response("Amen."))
          .label("A Prayer of St. Chrysostom")
          .source(Reference::from(59)),
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

        Document::from(Categories::ClosingSentences),

        // Possible location #3 for sermon
        Document::from(Heading::from((HeadingLevel::Heading3, "The Sermon")))
          .condition(Condition::Preference(PreferenceKey::from("sermon"), PreferenceValue::from("after_office"))),
        ]))
      .preferences([
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
          PreferenceKey::from(GlobalPref::PsalmCycle),
          "Lectionary",
          [
            LiturgyPreferenceOption::from(("Daily Office Lectionary", PreferenceValue::from(Lectionaries::BCP1979DailyOffice))),
            LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
            LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
          ]
        )).category("Lectionaries and Cycles"),

        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::PsalmCycle),
          "Canticle Table",
          [
            LiturgyPreferenceOption::from(("Book of Common Prayer (1979)", PreferenceValue::from(CanticleTables::BCP1979RiteII))),
            LiturgyPreferenceOption::from(("Enriching Our Worship 1", PreferenceValue::from(CanticleTables::EOW))),
            LiturgyPreferenceOption::from(("Te Deum/Benedictus", PreferenceValue::from(CanticleTables::Classical))),
          ]
        )).category("Lectionaries and Cycles"),

        // Readings
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingA),
          "First Lesson",
          [
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::FirstReading)).category("Lessons"),
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingB),
          "Second Lesson",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::SecondReading)).category("Lessons"),
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

        // Advanced Settings
        LiturgyPreference::from((
          PreferenceKey::from("sermon"),
          "Sermon",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::from("none"))),
            LiturgyPreferenceOption::from(("After the Readings", PreferenceValue::from("after_readings"))),
            LiturgyPreferenceOption::from(("After the Collects", PreferenceValue::from("after_collects"))),
            LiturgyPreferenceOption::from(("After the Office", PreferenceValue::from("after_office"))),
          ]
        )).default_value(PreferenceValue::from("none")).category("Advanced Settings")
      ]),
    )
    .version(Version::RiteII)
    .label("Morning Prayer");
}
