use crate::conditions::{EASTER_SEASON, NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::{CANTICLE_17, GLORIA_PATRI, LORDS_PRAYER_ABBREV};
use calendar::Weekday;
use liturgy::{
    Antiphon, Choice, Condition, DisplayFormat, Document, Heading, HeadingLevel, Liturgy, Preces,
    PsalmCitation, Reference, Rubric, Sentence, Series, Text, Version,
};

lazy_static! {
    pub static ref COMPLINE: Document = Document::from(Liturgy::from(Series::from([
      Document::from(Heading::from((HeadingLevel::Heading1, "An Order for Compline"))),
      Document::from(Heading::InsertDate),
      Document::from(Heading::InsertDay),
      Document::from(Rubric::from("The Officiant begins")),
      Document::from(Text::from("The Lord Almighty grant us a peaceful night and a perfect end.").response("Amen")).source(Reference::from(127)),
      Document::from(Preces::from([
          ("Officiant", "Our help is in the name of the Lord."),
          ("People", "The maker of heaven and earth.")
      ])),
      Document::from(Rubric::from("The Officiant may then say")),
Document::from("Let us confess our sins to God."),
Document::from(Rubric::from("Silence may be kept.")),
Document::from(Rubric::from("Officiant and People")),
Document::from(Text::from("Almighty God, our heavenly Father:\nWe have sinned against you,\nthrough our own fault,\nin thought, and word, and deed,\nand in what we have left undone.\nFor the sake of your Son our Lord Jesus Christ,\nforgive us all our offenses;\nand grant that we may serve you\nin newness of life,\nto the glory of your Name.").response("Amen").display_format(DisplayFormat::Unison)),
Document::from(Rubric::from("Officiant")),
Document::from(Text::from("May the Almighty God grant us forgiveness of all our sins, and the grace and comfort of the Holy Spirit.").response("Amen")),
Document::from(Rubric::from("The Officiant then says")),
Document::from(Preces::from([
          ("", "O God, make speed to save us."),
("People", "O Lord, make haste to help us.")
        ])),
Document::from(Rubric::from("Officiant and People")),
Document::from(GLORIA_PATRI.clone()),
        Document::from(Rubric::from("Except in Lent, add")).condition(NOT_LENT.clone()),
        Document::from("Alleluia.").condition(NOT_LENT.clone()),
Document::from(Rubric::from("\nOne or more of the following Psalms are sung or said. Other suitable selections may be substituted.")),
Document::from(PsalmCitation::from("Psalm 4")),
                Document::from(PsalmCitation::from("Psalm 31:1-5")),
                Document::from(PsalmCitation::from("Psalm 91")),
                Document::from(PsalmCitation::from("Psalm 134")),
                Document::from(Rubric::from("At the end of the Psalms is sung or said")).condition(NOT_INSERT_GLORIA.clone()),
                Document::from(GLORIA_PATRI.clone()).condition(NOT_INSERT_GLORIA.clone()),
Document::from(Rubric::from("One of the following, or some other suitable passage of Scripture, is read")),
Document::from(Choice::from([
      Document::from(Sentence::from("Lord, you are in the midst of us, and we are called by your Name: Do not forsake us, O Lord our God.").citation("Jeremiah 14:9,22").response(Text::from("Thanks be to God.").display_format(DisplayFormat::Unison))),
Document::from(Sentence::from("Be sober, be watchful. Your adversary the devil prowls around like a roaring lion, seeking someone to devour. Resist him, firm in your faith.").citation("1 Peter 5:8-9a").response(Text::from("Thanks be to God.").display_format(DisplayFormat::Unison))),
Document::from(Sentence::from("May the God of peace, who brought again from the dead our Lord Jesus, the great shepherd of the sheep, by the blood of the eternal covenant, equip you with everything good that you may do his will, working in you that which is pleasing in his sight; through Jesus Christ, to whom be glory for ever and ever. ").citation("Hebrews 13:20-21").response(Text::from("Thanks be to God.").display_format(DisplayFormat::Unison))),
Document::from(Sentence::from("Come to me, all who labor and are heavy-laden, and I will give you rest. Take my yoke upon you, and learn from me; for I am gentle and lowly in heart, and you will find rest for your souls. For my yoke is easy, and my burden is light.").citation("Matthew 11:28-30").response(Text::from("Thanks be to God.").display_format(DisplayFormat::Unison)))
    ])),
Document::from(Rubric::from("A hymn suitable for the evening may be sung. ")),
Document::from(Rubric::from("Then follows")),
Document::from(Preces::from([
          ("V.", "Into your hands, O Lord, I commend my spirit;"),
("R.", "For you have redeemed me, O Lord, O God of truth."),
("V.", "Keep us, O Lord, as the apple of your eye;"),
("R.", "Hide us under the shadow of your wings.")
        ])).source(Reference::from(132)),
Document::from(Preces::from([
          ("Officiant", "Lord, have mercy."),
("People", "Christ, have mercy."),
("Officiant", "Lord, have mercy.")
        ])),
Document::from(Rubric::from("Officiant and People")),
Document::from(LORDS_PRAYER_ABBREV.clone()),
Document::from(Preces::from([
          ("Officiant", "Lord, hear our prayer."),
("People", "And let our cry come to you."),
("Officiant", "Let us pray.")
        ])),
Document::from(Choice::from([
Document::from(Text::from("Be our light in the darkness, O Lord, and in your great mercy defend us from all perils and dangers of this night; for the love of your only Son, our Savior Jesus Christ.").response("Amen")),
Document::from(Text::from("Be present, O merciful God, and protect us through the hours of this night, so that we who are wearied by the changes and chances of this life may rest in your eternal changelessness; through Jesus Christ our Lord.").response("Amen")),
Document::from(Text::from("Look down, O Lord, from your heavenly throne, and illumine this night with your celestial brightness; that by night as by day your people may glorify your holy Name; through Jesus Christ our Lord.").response("Amen")),
Document::from(Text::from("Visit this place, O Lord, and drive far from it all snares of the enemy; let your holy angels dwell with us to preserve us in peace; and let your blessing be upon us always; through Jesus Christ our Lord.").response("Amen")),
])),
Document::from(Text::from("We give you thanks, O God, for revealing your Son Jesus Christ to us by the light of his resurrection: Grant that as we sing your glory at the close of this day, our joy may abound in the morning as we celebrate the Paschal mystery; through Jesus Christ our Lord.").response("Amen")).label("A Collect for Saturdays").condition(Condition::Weekday(Weekday::Sat)),
Document::from(Rubric::from("One of the following prayers may be added")),
Document::from(Choice::from([
      Document::from(Text::from("Keep watch, dear Lord, with those who work, or watch, or weep this night, and give your angels charge over those who sleep. Tend the sick, Lord Christ; give rest to the weary, bless the dying, soothe the suffering, pity the afflicted, shield the joyous; and all for your love’s sake.").response("Amen")),
Document::from(Text::from("O God, your unfailing providence sustains the world we live in and the life we live: Watch over those, both night and day, who work while others sleep, and grant that we may never forget that our common life depends upon each other’s toil; through Jesus Christ our Lord.").response("Amen"))
    ])),
Document::from(Rubric::from("Silence may be kept, and free intercessions and thanksgivings may be offered.\n")),
Document::from(Rubric::from("The service concludes with the Song of Simeon with this Antiphon, which is sung or said by all")),
Document::from(Antiphon::from("Guide us waking, O Lord, and guard us sleeping; that awake we may watch with Christ, and asleep we may rest in peace.")),
Document::from(Rubric::from("In Easter Season, add")).condition(EASTER_SEASON.clone()),
Document::from(Antiphon::from("Alleluia, alleluia, alleuia.")).condition(EASTER_SEASON.clone()),
CANTICLE_17.clone(),
Document::from(Rubric::from("All repeat the Antiphon")),
Document::from(Antiphon::from("Guide us waking, O Lord, and guard us sleeping; that awake we may watch with Christ, and asleep we may rest in peace.")),
Document::from(Rubric::from("In Easter Season, add")).condition(EASTER_SEASON.clone()),
Document::from(Antiphon::from("Alleluia, alleluia, alleuia.")).condition(EASTER_SEASON.clone()),
Document::from(Preces::from([
          ("Officiant", "Let us bless the Lord."),
("People", "Thanks be to God.")
        ])),
Document::from(Rubric::from("The Officiant concludes")),
Document::from(Text::from("The almighty and merciful Lord, Father, Son, and Holy Spirit, bless us and keep us.").response("Amen"))
    ])).evening(true)).version(Version::RiteII).label("Compline")
;
}
