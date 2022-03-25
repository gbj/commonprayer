use crate::bcp1979::burial::parallels::*;
use lazy_static::lazy_static;
use liturgy::*;
use psalter::bcp1979::{PSALM_121, PSALM_130, PSALM_23, PSALM_90};

lazy_static! {
    pub static ref BURIAL_OF_A_NON_CHRISTIAN: Document = Document::new()
        .version(Version::BOS)
        .label("Burial of a Non-Christian")
        .source(Reference {
            source: Source::BookOfOccasionalServices2018,
            page: 188
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "Burial of One Who Does Not Profess the Christian Faith"))).tags([TITLE]),
            Document::from(Rubric::from("This anthem; and any of the following Psalms, Lessons, and Prayers; and the form of Committal given below may be used with the Order for Burial on page 506 of the Prayer Book.")),
            Document::from(Content::DocumentLink(Version::BCP1979, "An Order for Burial".into(), "burial".into(), "an-order-for-burial".into())),
            Document::from(Heading::from((HeadingLevel::Heading2, "Opening Anthem"))),
            Document::from(Text::from("The steadfast love of the Lord never ceases, \nhis mercies never come to an end;\nthey are new every morning; \ngreat is his faithfulness.\nThe Lord will not cast off forever.\nThough he cause grief, he will have compassion \naccording to the abundance of his steadfast love;\nThe Lord does not willingly afflict or grieve his children.")).tags([ANTHEMS]),
            Document::from(Heading::from((HeadingLevel::Heading2, "Lessons and Psalms"))),
            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Ecclesiastes 3:1-11").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Ecclesiastes.")))).label("The First Lesson").version_label("Ecclesiastes 3:1-11 (For everything there is a season)"),
                Document::from(BiblicalCitation::from("Ecclesiastes 12:1-7").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Ecclesiastes.")))).label("The First Lesson").version_label("Ecclesiastes 12:1-7 (Remember your Creator in the days of your youth)"),
            ])).tags([FIRST_LESSON]),
            Document::from(Choice::from(vec![
                Document::from(PSALM_23.clone()).version_label("Psalm 23 (The Lord is my shepherd)"),
                Document::from(PSALM_90.clone()).version_label("Psalm 90 (Lord, you have been our refuge)"),
                Document::from(PSALM_121.clone()).version_label("Psalm 121 (I lift up my eyes to the hills)"),
                Document::from(PSALM_130.clone()).version_label("Psalm 130 (Out of the depths have I called to you, O Lord) ")
            ])).tags([PSALM]),
            Document::from(BiblicalCitation::from("Romans 8:35-39").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Romans.")))).label("The Second Lesson").version_label("Romans 8:35-39 (Who shall separate us from the love of Christ?)").tags([SECOND_LESSON]),
            Document::from(BiblicalCitation::from("John 10:11-16")
                .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                    ("", "The Holy Gospel of our Lord Jesus Christ according to Matthew."),
                    ("People", "Glory to you, Lord Christ.")
                ]))))
            ).label("The Gospel").version_label("John 10:11-16 (I am the good shepherd)").tags([GOSPEL]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "Prayers"))),
                Document::from(Heading::from((HeadingLevel::Heading3, "For the Deceased"))),
                Document::from(Text::from("Almighty God, we entrust all who are dear to us to your never-failing care and love, for this life and the life to come, knowing that you are doing for them better things than we can desire or pray for; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Into your hands, O God, we commend our *brother, N.*, as into the hands of a faithful Creator and most loving Savior. In your infinite goodness, wisdom, and power, work in *him* the merciful purpose of your perfect will, through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Heading::from((HeadingLevel::Heading3, "For those who mourn"))),
                Document::from(Text::from("O God of grace and glory, we remember before you this day our brother (sister), *N.* We thank you for giving *him* to us, *his* family and friends, to know and to love as a companion on our earthly pilgrimage. In your boundless compassion, console us who mourn. Give us quiet confidence that we may continue our course in faith; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("O merciful Father, you have taught us in your holy Word that you do not willingly afflict or grieve your children: Look with pity upon the sorrows of your servants for whom our prayers are offered. Remember them, O Lord, in mercy, nourish their souls with patience, comfort them with a sense of your goodness, lift up your countenance upon them, and give them peace; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Almighty God, Father of mercies and giver of comfort: Deal graciously, we pray, with all who mourn; that, casting all their care on you, they may know the consolation of your love; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Most merciful God, whose wisdom is beyond our understanding, deal graciously with *N.N.* in *their* grief. Surround *them* with your love, that *they* may not be overwhelmed by *their* loss, but have confidence in your goodness, and strength to meet the days to come; through Jesus Christ our Lord.").response("Amen.")),
                Document::from(Heading::from((HeadingLevel::Heading3, "For the Christian community"))),
                Document::from(Text::from("Most loving Father, whose will it is for us to give thanks for all things, to fear nothing but the loss of you, and to cast all our care on you who care for us: Preserve us from faithless fears and worldly anxieties, that no clouds of this mortal life may hide from us the light of that love which is immortal, and which you have manifested to us in your Son Jesus Christ our Lord.").response("Amen.")),
                Document::from(Text::from("Almighty God, give us grace to cast away the works of darkness, and put on the armor of light, now in the time of this mortal life in which your Son Jesus Christ came to visit us in great humility; that in the last day, when he shall come again in his glorious majesty to judge both the living and the dead, we may rise to the life immortal; through him who lives and reigns for ever and ever.").response("Amen.")),
            ])).tags([PRAYERS]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Committal"))),
                Document::from(Antiphon::from("Holy God, Holy and Mighty, Holy Immortal One, have mercy upon us.")),
                Document::from(Text::from("You only are immortal, the creator and maker of mankind; and we are mortal, formed of the earth, and to earth shall we return. For so did you ordain when you created me, saying, “You are dust, and to dust you shall return.” All of us go down to the dust; yet even at the grave we make our song: Alleluia, alleluia, alleluia.")),
                Document::from(Antiphon::from("Holy God, Holy and Mighty, Holy Immortal One, have mercy upon us."))
            ])).tags([COMMITTAL])
        ]))));
}
