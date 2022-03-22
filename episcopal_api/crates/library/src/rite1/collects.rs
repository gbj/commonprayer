use calendar::{feasts::CommonOfSaints, Feast, LiturgicalWeek, Proper, VariousOccasions};
use liturgy::{Document, Heading, HeadingLevel, Series, Text, Version};

use crate::{CollectData, CollectId};

lazy_static! {
    pub static ref COLLECTS_TRADITIONAL: Vec<(CollectId, CollectData)> = vec![
        (
            CollectId::Week(LiturgicalWeek::Advent1),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, give us grace that we may cast away the works of darkness, and put upon us the armor of light, now in the time of this mortal life in which thy Son Jesus Christ came to visit us in great humility; that in the last day, when he shall come again in his glorious majesty to judge both the quick and the dead, we may rise to the life immortal; through him who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("First Sunday of Advent")
                .page(125)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent2),
            CollectData {
                document: Document::from(
                    Text::from("Merciful God, who sent thy messengers the prophets to preach repentance and prepare the way for our salvation: Give us grace to heed their warnings and forsake our sins, that we may greet with joy the coming of Jesus Christ our Redeemer; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Second Sunday of Advent")
                .page(125)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent3),
            CollectData {
                document: Document::from(
                    Text::from("Stir up thy power, O Lord, and with great might come among us; and, because we are sorely hindered by our sins, let thy bountiful grace and mercy speedily help and deliver us; through Jesus Christ our Lord, to whom, with thee and the Holy Ghost, be honor and glory, world without end.")
                        .response("Amen.")
                )
                .label("Third Sunday of Advent")
                .page(160)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: Some("Wednesday, Friday, and Saturday of this week are the traditional winter Ember Days.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent4),
            CollectData {
                document: Document::from(
                    Text::from("We beseech thee, Almighty God, to purify our consciences by thy daily visitation, that when thy Son our Lord cometh he may find in us a mansion prepared for himself; through the same Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday of Advent")
                .page(160)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("O God, who makest us glad with the yearly remembrance of the birth of thy only Son Jesus Christ: Grant that as we joyfully receive him for our Redeemer, so we may with sure confidence behold him when he shall come to be our Judge; who liveth and reigneth with thee and the Holy Ghost, one God, world without end.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(160)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or the following".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasEve),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast caused this holy night to shine with the illumination of the true Light: Grant us, we beseech thee, that as we have known the mystery of that Light upon earth, so may we also perfectly enjoy him in heaven; where with thee and the Holy Spirit he liveth and reigneth, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(161)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast caused this holy night to shine with the illumination of the true Light: Grant us, we beseech thee, that as we have known the mystery of that Light upon earth, so may we also perfectly enjoy him in heaven; where with thee and the Holy Spirit he liveth and reigneth, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(161)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who hast given us thy only-begotten Son to take our nature upon him and as at this time to be born of a pure virgin: Grant that we, being regenerate and made thy children by adoption and grace, may daily be renewed by thy Holy Spirit; through the same our Lord Jesus Christ, who liveth and reigneth with thee and the same Spirit ever, one God, world without end.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(161)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: Some("The Collect immediately preceding and any of the sets of Proper Lessons for Christmas Day serve for any weekdays between Holy Innocents’ Day and the First Sunday after Christmas Day.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Christmas1),
            CollectData {
                document: Document::from(
                    Text::from("This Sunday takes precedence over the three Holy Days which follow Christmas Day. As necessary, the observance of one, two, or all three of them, is postponed one day.Almighty God, who hast poured upon us the new light of thine incarnate Word: Grant that the same light, enkindled in our hearts, may shine forth in our lives; through the same Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("First Sunday after Christmas Day")
                .page(161)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyName),
            CollectData {
                document: Document::from(
                    Text::from("Eternal Father, who didst give to thine incarnate Son the holy name of Jesus to be the sign of our salvation: Plant in every heart, we beseech thee, the love of him who is the Savior of the world, even our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("The Holy Name")
                    .subtitle("January 1")
                .page(162)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Christmas2),
            CollectData {
                document: Document::from(
                    Text::from("O God, who didst wonderfully create, and yet more wonderfully restore, the dignity of human nature: Grant that we may share the divine life of him who humbled himself to share our humanity, thy Son Jesus Christ; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Second Sunday after Christmas Day")
                .page(162)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation The Epiphany January 6".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Epiphany),
            CollectData {
                document: Document::from(
                    Text::from("O God, who by the leading of a star didst manifest thy only- begotten Son to the peoples of the earth: Lead us, who know thee now by faith, to thy presence, where we may behold thy glory face to face; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Epiphany")
                .subtitle("January 6")
                .page(162)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: Some("The preceding Collect, with the Psalm and Lessons for the Epiphany, or those for the Second Sunday after Christmas Day, serves for weekdays between the Epiphany and the following Sunday. The Preface of the Epiphany is used.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany1),
            CollectData {
                document: Document::from(
                    Text::from("Father in heaven, who at the baptism of Jesus in the River Jordan didst proclaim him thy beloved Son and anoint him with the Holy Spirit: Grant that all who are baptized into his Name may keep the covenant they have made, and boldly confess him as Lord and Savior; who with thee and the same Spirit liveth and reigneth, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("First Sunday after the Epiphany: The Baptism of our Lord")
                .page(163)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany2),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose Son our Savior Jesus Christ is the light of the world: Grant that thy people, illumined by thy Word and Sacraments, may shine with the radiance of Christ’s glory, that he may be known, worshiped, and obeyed to the ends of the earth; through the same Jesus Christ our Lord, who with thee and the Holy Spirit liveth and reigneth, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Second Sunday after the Epiphany")
                .page(163)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany3),
            CollectData {
                document: Document::from(
                    Text::from("Give us grace, O Lord, to answer readily the call of our Savior Jesus Christ and proclaim to all people the Good News of his salvation, that we and all the whole world may perceive the glory of his marvelous works; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Third Sunday after the Epiphany")
                .page(163)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany4),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who dost govern all things in heaven and earth: Mercifully hear the supplications of thy people, and in our time grant us thy peace; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday after the Epiphany")
                .page(164)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany5),
            CollectData {
                document: Document::from(
                    Text::from("Set us free, O God, from the bondage of our sins and give us, we beseech thee, the liberty of that abundant life which thou hast manifested to us in thy Son our Savior Jesus Christ; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fifth Sunday after the Epiphany")
                .page(164)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany6),
            CollectData {
                document: Document::from(
                    Text::from("O God, the strength of all those who put their trust in thee: Mercifully accept our prayers; and because, through the weakness of our mortal nature, we can do no good thing without thee, grant us the help of thy grace, that in keeping thy commandments we may please thee both in will and deed; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Sixth Sunday after the Epiphany")
                .page(164)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany7),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, who hast taught us that all our doings without charity are nothing worth: Send thy Holy Ghost and pour into our hearts that most excellent gift of charity, the very bond of peace and of all virtues, without which whosoever liveth is counted dead before thee. Grant this for thine only Son Jesus Christ’s sake, who liveth and reigneth with thee and the same Holy Ghost, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Seventh Sunday after the Epiphany")
                .page(165)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany8),
            CollectData {
                document: Document::from(
                    Text::from("O most loving Father, who willest us to give thanks for all things, to dread nothing but the loss of thee, and to cast all our care on thee who carest for us: Preserve us from faithless fears and worldly anxieties, and grant that no clouds of this mortal life may hide from us the light of that love which is immortal, and which thou hast manifested unto us in thy Son Jesus Christ our Lord; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Eighth Sunday after the Epiphany")
                .page(165)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::LastEpiphany),
            CollectData {
                document: Document::from(
                    Text::from("O God, who before the passion of thy only-begotten Son didst reveal his glory upon the holy mount: Grant unto us that we, beholding by faith the light of his countenance, may be strengthened to bear our cross, and be changed into his likeness from glory to glory; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Last Sunday after the Epiphany")
                .page(165)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: Some("This Proper is always used on the Sunday before Ash Wednesday.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::AshWednesday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who hatest nothing that thou hast made and dost forgive the sins of all those who are penitent: Create and make in us new and contrite hearts, that we, worthily lamenting our sins and acknowledging our wretchedness, may obtain of thee, the God of all mercy, perfect remission and forgiveness; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Ash Wednesday")
                .page(166)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Lent".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 264.".into()),
                rubric_after: Some("This Collect, with the corresponding Psalm and Lessons, serves for the weekdays which follow, except as otherwise appointed.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent1),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed Son was led by the Spirit to be tempted of Satan: Make speed to help thy servants who are assaulted by manifold temptations; and, as thou knowest their several infirmities, let each one find thee mighty to save; through Jesus Christ thy Son our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("First Sunday in Lent")
                .page(166)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: Some("Wednesday, Friday, and Saturday of this week are the traditional spring Ember Days.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent2),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose glory it is always to have mercy: Be gracious to all who have gone astray from thy ways, and bring them again with penitent hearts and steadfast faith to embrace and hold fast the unchangeable truth of thy Word, Jesus Christ thy Son; who with thee and the Holy Spirit liveth and reigneth, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Second Sunday in Lent")
                .page(167)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent3),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who seest that we have no power of ourselves to help ourselves: Keep us both outwardly in our bodies and inwardly in our souls, that we may be defended from all adversities which may happen to the body, and from all evil thoughts which may assault and hurt the soul; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Third Sunday in Lent")
                .page(167)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent4),
            CollectData {
                document: Document::from(
                    Text::from("Gracious Father, whose blessed Son Jesus Christ came down from heaven to be the true bread which giveth life to the world: Evermore give us this bread, that he may live in us, and we in him; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday in Lent")
                .page(167)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent5),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who alone canst order the unruly wills and affections of sinful men: Grant unto thy people that they may love the thing which thou commandest, and desire that which thou dost promise; that so, among the sundry and manifold changes of the world, our hearts may surely there be fixed where true joys are to be found; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fifth Sunday in Lent")
                .page(167)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::HolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who, of thy tender love towards mankind, hast sent thy Son our Savior Jesus Christ to take upon him our flesh, and to suffer death upon the cross, that all mankind should follow the example of his great humility: Mercifully grant that we may both follow the example of his patience, and also be made partakers of his resurrection; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Sunday of the Passion: Palm Sunday")
                .page(168)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 270.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MondayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose most dear Son went not up to joy but first he suffered pain, and entered not into glory before he was crucified: Mercifully grant that we, walking in the way of the cross, may find it none other than the way of life and peace; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Monday in Holy Week")
                .page(168)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TuesdayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("O God, who by the passion of thy blessed Son didst make an instrument of shameful death to be unto us the means of life: Grant us so to glory in the cross of Christ, that we may gladly suffer shame and loss for the sake of thy Son our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Tuesday in Holy Week")
                .page(168)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::WednesdayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("O Lord God, whose blessed Son our Savior gave his back to the smiters and hid not his face from shame: Grant us grace to take joyfully the sufferings of the present time, in full assurance of the glory that shall be revealed; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Wednesday in Holy Week")
                .page(169)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MaundyThursday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, whose dear Son, on the night before he suffered, did institute the Sacrament of his Body and Blood: Mercifully grant that we may thankfully receive the same in remembrance of him who in these holy mysteries giveth us a pledge of life eternal, the same thy Son Jesus Christ our Lord; who now liveth and reigneth with thee and the Holy Spirit ever, one God, world without end.")
                        .response("Amen.")
                )
                .label("Maundy Thursday")
                .page(169)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 274.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::GoodFriday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, we beseech thee graciously to behold this thy family, for which our Lord Jesus Christ was contented to be betrayed, and given up into the hands of sinners, and to suffer death upon the cross; who now liveth and reigneth with thee and the Holy Ghost ever, one God, world without end.")
                        .response("Amen.")
                )
                .label("Good Friday")
                .page(169)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 276.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolySaturday),
            CollectData {
                document: Document::from(
                    Text::from("O God, Creator of heaven and earth: Grant that, as the crucified body of thy dear Son was laid in the tomb and rested on this holy Sabbath, so we may await with him the coming of the third day, and rise with him to newness of life; who now liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Holy Saturday")
                .page(170)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 283.".into()),
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("O God, who for our redemption didst give thine only-begotten Son to the death of the cross, and by his glorious resurrection hast delivered us from the power of our enemy: Grant us so to die daily to sin, that we may evermore live with him in the joy of his resurrection; through the same thy Son Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(170)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: Some("The Liturgy of the Easter Vigil is on page 285.".into()),
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::EasterVigil),
            CollectData {
                document: Document::from(
                    Text::from("O God, who didst make this most holy night to shine with the glory of the Lord’s resurrection: Stir up in thy Church that Spirit of adoption which is given to us in Baptism, that we, being renewed both in body and mind, may worship thee in sincerity and truth; through the same Jesus Christ our Lord, who liveth and reigneth with thee in the unity of the same Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(170)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("O God, who didst make this most holy night to shine with the glory of the Lord’s resurrection: Stir up in thy Church that Spirit of adoption which is given to us in Baptism, that we, being renewed both in body and mind, may worship thee in sincerity and truth; through the same Jesus Christ our Lord, who liveth and reigneth with thee in the unity of the same Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(170)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who through thine only-begotten Son Jesus Christ hast overcome death and opened unto us the gate of everlasting life: Grant that we, who celebrate with joy the day of the Lord’s resurrection, may be raised from the death of sin by thy life-giving Spirit; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit ever, one God, world without end.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(171)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MondayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Grant, we beseech thee, Almighty God, that we who celebrate with reverence the Paschal feast may be found worthy to attain to everlasting joys; through Jesus Christ our Lord,who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Monday in Easter Week")
                .page(171)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TuesdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("O God, who by the glorious resurrection of thy Son Jesus Christ destroyed death and brought life and immortality to light: Grant that we, who have been raised with him, may abide in his presence and rejoice in the hope of eternal glory; through the same Jesus Christ our Lord, to whom, with thee and the Holy Spirit, be dominion and praise for ever and ever.")
                        .response("Amen.")
                )
                .label("Tuesday in Easter Week")
                .page(171)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::WednesdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son did manifest himself to his disciples in the breaking of bread: Open, we pray thee, the eyes of our faith, that we may behold him in all his redeeming work; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Wednesday in Easter Week")
                .page(171)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThursdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who in the Paschal mystery hast established the new covenant of reconciliation: Grant that all who have been reborn into the fellowship of Christ’s Body may show forth in their lives what they profess by their faith; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Thursday in Easter Week")
                .page(172)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::FridayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, who hast given thine only Son to die for our sins and to rise again for our justification: Grant us so to put away the leaven of malice and wickedness, that we may always serve thee in pureness of living and truth; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Friday in Easter Week")
                .page(172)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::SaturdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("We thank thee, heavenly Father, for that thou hast delivered us from the dominion of sin and death and hast brought us into the kingdom of thy Son; and we pray thee that, as by his death he hath recalled us to life, so by his love he may raise us to joys eternal; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saturday in Easter Week")
                .page(172)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter2),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who in the Paschal mystery hast established the new covenant of reconciliation: Grant that all who have been reborn into the fellowship of Christ’sBody may show forth in their lives what they profess by their faith; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Second Sunday of Easter")
                .page(173)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter3),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son did manifest himself to his disciples in the breaking of bread: Open, we pray thee, the eyes of our faith, that we may behold him in all his redeeming work; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Third Sunday of Easter")
                .page(173)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter4),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose Son Jesus is the good shepherd of thy people: Grant that when we hear his voice we may know him who calleth us each by name, and follow where he doth lead; who, with thee and the Holy Spirit, liveth and reigneth, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday of Easter")
                .page(173)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter5),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, whom truly to know is everlasting life: Grant us so perfectly to know thy Son Jesus Christ to be the way, the truth, and the life, that we may steadfastly follow his steps in the way that leadeth to eternal life; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Fifth Sunday of Easter")
                .page(173)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter6),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast prepared for those who love thee such good things as pass man’s understanding: Pour into our hearts such love toward thee, that we, loving thee in all things and above all things, may obtain thy promises, which exceed all that we can desire; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Sixth Sunday of Easter")
                .page(174)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: Some("Monday, Tuesday, and Wednesday of this week are the traditional Rogation Days.".into())
            }
        ),
        (
            CollectId::Feast(Feast::AscensionDay),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, whose blessed Son our Savior Jesus Christ ascended far above all heavens that he might fill all things: Mercifully give us faith to perceive that, according to his promise, he abideth with his Church on earth, even unto the end of the ages; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Ascension Day")
                .page(174)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::AscensionDay),
            CollectData {
                document: Document::from(
                    Text::from("Grant, we beseech thee, Almighty God, that like as we do believe thy only-begotten Son our Lord Jesus Christ to have ascended into the heavens, so we may also in heart and mind thither ascend, and with him continually dwell; who liveth and reigneth with thee and the Holy Ghost, one God, world without end.")
                        .response("Amen.")
                )
                .label("Ascension Day")
                .page(174)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Ascension".into(),
                rubric_before: None,
                rubric_after: Some("Either of the preceding Collects, with the proper Psalm and Lessons for Ascension Day, serves for the following weekdays, except as otherwise appointed.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter7),
            CollectData {
                document: Document::from(
                    Text::from("O God, the King of glory, who hast exalted thine only Son Jesus Christ with great triumph unto thy kingdom in heaven: We beseech thee, leave us not comfortless, but send to us thine Holy Ghost to comfort us, and exalt us unto the same place whither our Savior Christ is gone before; who liveth and reigneth with thee and the same Holy Ghost, one God, world without end.")
                        .response("Amen.")
                )
                .label("Seventh Sunday of Easter: The Sunday after Ascension Day")
                .page(175)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Ascension".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Pentecost),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who on this day didst open the way of eternal life to every race and nation by the promised gift of thy Holy Spirit: Shed abroad this gift throughout the world by the preaching of the Gospel, that it may reach to the ends of the earth; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the same Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Day of Pentecost: Whitsunday")
                .page(175)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: Some("When a Vigil of Pentecost is observed, it begins with the Service of Light, page 109 (substituting, if desired, the Gloria in excelsis for the Phos hilaron), and continues with the Salutation and Collect of the Day. Three or more of the appointed Lessons are read before the Gospel, each followed by a Psalm, Canticle, or hymn. Holy Baptism or Confirmation (beginning with the Presentation of the Candidates), or the Renewal of Baptismal Vows, page 292, follows the Sermon.".into()),
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::Pentecost),
            CollectData {
                document: Document::from(
                    Text::from("O God, who on this day didst teach the hearts of thy faithful people by sending to them the light of thy Holy Spirit: Grant us by the same Spirit to have a right judgment in all things, and evermore to rejoice in his holy comfort; through the merits of Christ Jesus our Savior, who liveth and reigneth with thee, in the unity of the same Spirit, one God, world without end.")
                        .response("Amen.")
                )
                .label("The Day of Pentecost: Whitsunday")
                .page(175)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Pentecost".into(),
                rubric_before: None,
                rubric_after: Some("On the weekdays which follow, the numbered Proper which corresponds most closely to the date of Pentecost in that year is used. See page 158.\n\nWednesday, Friday, and Saturday of this week are the traditional summer Ember Days.".into())
            }
        ),
        (
            CollectId::Feast(Feast::TrinitySunday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who hast given unto us thy servants grace, by the confession of a true faith, to acknowledge the glory of the eternal Trinity, and in the power of the Divine Majesty to worship the Unity: We beseech thee that thou wouldest keep us steadfast in this faith and worship, and bring us at last to see thee in thy one and eternal glory, O Father; who with the Son and the Holy Spirit livest and reignest, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("First Sunday after Pentecost: Trinity Sunday")
                .page(176)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("On the weekdays which follow, the numbered Proper which corresponds most closely to the date of Trinity Sunday in that year is used.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper1),
            CollectData {
                document: Document::from(
                    Text::from("Remember, O Lord, what thou hast wrought in us and not what we deserve; and, as thou hast called us to thy service, make us worthy of our calling; through Jesus Christ ourLord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 1")
                    .subtitle("Week of the Sunday closest to May 11")
                .page(177)
                .tags(["Seasons of the Year", "The Season after Pentecost"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: Some("Directions for the use of the Propers which follow are on page 158.".into()),
                rubric_after: Some("No Proper Preface is used.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper2),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty and most merciful God, of thy bountiful goodness keep us, we beseech thee, from all things that may hurt us, that we, being ready both in body and soul, may with free hearts accomplish those things which belong to thy purpose; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever. Amen.No Proper Preface is used.")
                        .response("Amen.")
                )
                .label("Proper 2")
                    .subtitle("Week of the Sunday closest to May 18")
                .page(177)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper3),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O Lord, we beseech thee, that the course of this world may be peaceably governed by thy providence, and that thy Church may joyfully serve thee in confidence and serenity; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 3")
                    .subtitle("The Sunday closest to May 25")
                .page(177)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper4),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose never-failing providence ordereth all things both in heaven and earth: We humbly beseech thee to put away from us all hurtful things, and to give us those things which are profitable for us; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 4")
                    .subtitle("The Sunday closest to June 1")
                .page(177)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper5),
            CollectData {
                document: Document::from(
                    Text::from("O God, from whom all good doth come: Grant that by thy inspiration we may think those things that are right, and by thy merciful guiding may perform the same; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 5")
                    .subtitle("The Sunday closest to June 8")
                .page(178)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper6),
            CollectData {
                document: Document::from(
                    Text::from("Keep, O Lord, we beseech thee, thy household the Church in thy steadfast faith and love, that by the help of thy grace we may proclaim thy truth with boldness, and minister thy justice with compassion; for the sake of our Savior Jesus Christ, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 6")
                    .subtitle("The Sunday closest to June 15")
                .page(178)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper7),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, we beseech thee, make us to have a perpetual fear and love of thy holy Name, for thou never failest to help and govern those whom thou hast set upon the sure foundation of thy loving-kindness; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 7")
                    .subtitle("The Sunday closest to June 22")
                .page(178)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper8),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who hast built thy Church upon the foundation of the apostles and prophets, Jesus Christ himself being the chief cornerstone: Grant us so to be joined together in unity of spirit by their doctrine, that we may be made an holy temple acceptable unto thee; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 8")
                    .subtitle("The Sunday closest to June 29")
                .page(179)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper9),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast taught us to keep all thy commandments by loving thee and our neighbor: Grant us the grace of thy Holy Spirit, that we may be devoted to thee with our whole heart, and united to one another with pure affection; through Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 9")
                    .subtitle("The Sunday closest to July 6")
                .page(179)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper10),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, we beseech thee mercifully to receive the prayers of thy people who call upon thee, and grant that they may both perceive and know what things they ought to do, and also may have grace and power faithfully to fulfill the same; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 10")
                    .subtitle("The Sunday closest to July 13")
                .page(179)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper11),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, the fountain of all wisdom, who knowest our necessities before we ask and our ignorance in asking: Have compassion, we beseech thee, upon our infirmities, and those things which for our unworthiness we dare not, and for our blindness we cannot ask, mercifully give us for the worthiness of thy Son Jesus Christ our Lord; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 11")
                    .subtitle("The Sunday closest to July 20")
                .page(179)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper12),
            CollectData {
                document: Document::from(
                    Text::from("O God, the protector of all that trust in thee, without whom nothing is strong, nothing is holy: Increase and multiply upon us thy mercy, that, thou being our ruler and guide, we may so pass through things temporal, that we finally lose not the things eternal; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 12")
                    .subtitle("The Sunday closest to July 27")
                .page(180)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper13),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, we beseech thee, let thy continual pity cleanse and defend thy Church, and, because it cannot continue in safety without thy succor, preserve it evermore by thy help and goodness; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 13")
                    .subtitle("The Sunday closest to August 3")
                .page(180)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper14),
            CollectData {
                document: Document::from(
                    Text::from("Grant to us, Lord, we beseech thee, the spirit to think and do always such things as are right, that we, who cannot exist without thee, may by thee be enabled to live according to thy will; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 14")
                    .subtitle("The Sunday closest to August 10")
                .page(180)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper15),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who hast given thy only Son to be unto us both a sacrifice for sin and also an example of godly life: Give us grace that we may always most thankfully receive that his inestimable benefit, and also daily endeavor ourselves to follow the blessed steps of his most holy life; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 15")
                    .subtitle("The Sunday closest to August 17")
                .page(181)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper16),
            CollectData {
                document: Document::from(
                    Text::from("Grant, we beseech thee, merciful God, that thy Church, being gathered together in unity by thy Holy Spirit, may manifest thy power among all peoples, to the glory of thy Name; through Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, world without end.")
                        .response("Amen.")
                )
                .label("Proper 16")
                    .subtitle("The Sunday closest to August 24")
                .page(181)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper17),
            CollectData {
                document: Document::from(
                    Text::from("Lord of all power and might, who art the author and giver of all good things: Graft in our hearts the love of thy Name, increase in us true religion, nourish us with all goodness,and bring forth in us the fruit of good works; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 17")
                    .subtitle("The Sunday closest to August 31")
                .page(181)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper18),
            CollectData {
                document: Document::from(
                    Text::from("Grant us, O Lord, we pray thee, to trust in thee with all our heart; seeing that, as thou dost alway resist the proud who confide in their own strength, so thou dost not forsake those who make their boast of thy mercy; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 18")
                    .subtitle("The Sunday closest to September 7")
                .page(181)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper19),
            CollectData {
                document: Document::from(
                    Text::from("O God, forasmuch as without thee we are not able to please thee, mercifully grant that thy Holy Spirit may in all things direct and rule our hearts; through Jesus Christ our Lord, who with thee and the same Spirit liveth and reigneth, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 19")
                    .subtitle("The Sunday closest to September 14")
                .page(182)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: Some("The Wednesday, Friday, and Saturday after September 14 are the traditional autumnal Ember Days.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper20),
            CollectData {
                document: Document::from(
                    Text::from("Grant us, O Lord, not to mind earthly things, but to love things heavenly; and even now, while we are placed among things that are passing away, to cleave to those that shall abide; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 20")
                    .subtitle("The Sunday closest to September 21")
                .page(182)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper21),
            CollectData {
                document: Document::from(
                    Text::from("O God, who declarest thy almighty power chiefly in showing mercy and pity: Mercifully grant unto us such a measure of thy grace, that we, running to obtain thy promises, may be made partakers of thy heavenly treasure; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 21")
                    .subtitle("The Sunday closest to September 28")
                .page(182)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper22),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who art always more ready to hear than we to pray, and art wont to give more than either we desire or deserve: Pour down upon us the abundance of thy mercy, forgiving us those things whereof our conscience is afraid, and giving us those good things which we are not worthy to ask, but through the merits and mediation of Jesus Christ thy Son our Lord; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 22")
                    .subtitle("The Sunday closest to October 5")
                .page(183)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper23),
            CollectData {
                document: Document::from(
                    Text::from("Lord, we pray thee that thy grace may always precede and follow us, and make us continually to be given to all good works; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 23")
                    .subtitle("The Sunday closest to October 12")
                .page(183)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper24),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who in Christ hast revealed thy glory among the nations: Preserve the works of thy mercy, that thy Church throughout the world may persevere with steadfast faith in the confession of thy Name; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 24")
                    .subtitle("The Sunday closest to October 19")
                .page(183)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper25),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, give unto us the increase of faith, hope, and charity; and, that we may obtain that which thou dost promise, make us to love that which thou dost command; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 25")
                    .subtitle("The Sunday closest to October 26")
                .page(183)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper26),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and merciful God, of whose only gift it cometh that thy faithful people do unto thee true and laudable service: Grant, we beseech thee, that we may run without stumbling to obtain thy heavenly promises; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 26")
                    .subtitle("The Sunday closest to November 2")
                .page(184)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper27),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son was manifested that he might destroy the works of the devil and make us the children of God and heirs of eternal life: Grant us, we beseech thee, that, having this hope, we may purify ourselves even as he is pure; that, when he shall appear again with power and great glory, we may be made like unto him in his eternal and glorious kingdom; where with thee, O Father, and thee, O Holy Ghost, he liveth and reigneth ever, one God, world without end.")
                        .response("Amen.")
                )
                .label("Proper 27")
                    .subtitle("The Sunday closest to November 9")
                .page(184)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper28),
            CollectData {
                document: Document::from(
                    Text::from("Blessed Lord, who hast caused all holy Scriptures to be written for our learning: Grant that we may in such wise hear them, read, mark, learn, and inwardly digest them; that, by patience and comfort of thy holy Word, we may embrace and ever hold fast the blessed hope of everlasting life, which thou hast given us in our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 28")
                    .subtitle("The Sunday closest to November 16")
                .page(184)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper29),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, whose will it is to restore all things in thy well-beloved Son, the King of kings and Lord of lords: Mercifully grant that the peoples of the earth, divided and enslaved by sin, may be freed and brought together under his most gracious rule; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 29")
                    .subtitle("The Sunday closest to November 23")
                .page(185)
                .tags(["Seasons of the Year"])
                .version(Version::RiteI),
                preface: "Preface of the Lord’s Day, or of Baptism".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Andrew),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who didst give such grace to thine apostle Andrew that he readily obeyed the call of thy Son Jesus Christ, and brought his brother with him: Give unto us, who are called by thy Word, grace to follow him without delay, and to bring those near to us into his gracious presence; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Andrew")
                    .subtitle("November 30")
                .page(185)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Thomas),
            CollectData {
                document: Document::from(
                    Text::from("Everliving God, who didst strengthen thine apostle Thomas with sure and certain faith in thy Son’s resurrection: Grant us so perfectly and without doubt to believe in Jesus Christ, our Lord and our God, that our faith may never be found wanting in thy sight; through him who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Thomas")
                    .subtitle("December 21")
                .page(185)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Stephen),
            CollectData {
                document: Document::from(
                    Text::from("We give thee thanks, O Lord of glory, for the example of the first martyr Stephen, who looked up to heaven and prayed for his persecutors to thy Son Jesus Christ, who standeth at thy right hand; where he liveth and reigneth with thee and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Saint Stephen")
                    .subtitle("December 26")
                .page(186)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::John),
            CollectData {
                document: Document::from(
                    Text::from("Shed upon thy Church, we beseech thee, O Lord, the brightness of thy light; that we, being illumined by the teaching of thine apostle and evangelist John, may so walk in the light of thy truth, that we may at length attain to the fullness of life everlasting; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint John")
                .subtitle("December 27")
                .page(186)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyInnocents),
            CollectData {
                document: Document::from(
                    Text::from("We remember this day, O God, the slaughter of the holy innocents of Bethlehem by the order of King Herod. Receive, we beseech thee, into the arms of thy mercy all innocent victims; and by thy great might frustrate the designs of evil tyrants and establish thy rule of justice, love, and peace; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Holy Innocents")
                    .subtitle("December 28")
                .page(186)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ConfessionOfStPeter),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, who didst inspire Simon Peter, first among the apostles, to confess Jesus as Messiah and Son of the living God: Keep thy Church steadfast upon the rock of this faith, that in unity and peace we may proclaim the one truth and follow the one Lord, our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Confession of Saint Peter")
                    .subtitle("January 18")
                .page(187)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ConversionOfStPaul),
            CollectData {
                document: Document::from(
                    Text::from("O God, who, by the preaching of thine apostle Paul, hast caused the light of the Gospel to shine throughout the world: Grant, we beseech thee, that we, having his wonderful conversion in remembrance, may show forth our thankfulness unto thee for the same by following the holy doctrine which he taught; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Conversion of Saint Paul")
                    .subtitle("January 25")
                .page(187)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThePresentation),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everliving God, we humbly beseech thee that, as thy only-begotten Son was this day presented in the temple, so we may be presented unto thee with pure and clean hearts by the same thy Son Jesus Christ our Lord; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Presentation")
                    .subtitle("February 2")
                .page(187)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Matthias),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who into the place of Judas didst choose thy faithful servant Matthias to be of the number of the Twelve: Grant that thy Church, being delivered from false apostles, may always be ordered and guided by faithful and true pastors; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Matthias")
                    .subtitle("February 24")
                .page(188)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Joseph),
            CollectData {
                document: Document::from(
                    Text::from("O God, who from the family of thy servant David didst raise up Joseph to be the guardian of thy incarnate Son and the spouse of his virgin mother: Give us grace to imitate his uprightness of life and his obedience to thy commands; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Joseph")
                    .subtitle("March 19")
                .page(188)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Annunciation),
            CollectData {
                document: Document::from(
                    Text::from("We beseech thee, O Lord, pour thy grace into our hearts, that we who have known the incarnation of thy Son Jesus Christ, announced by an angel to the Virgin Mary, may by his cross and passion be brought unto the glory of his resurrection; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Annunciation")
                    .subtitle("March 25")
                .page(188)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Mark),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who by the hand of Mark the evangelist hast given to thy Church the Gospel of Jesus Christ the Son ofGod: We thank thee for this witness, and pray that we may be firmly grounded in its truth; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Mark")
                    .subtitle("April 25")
                .page(189)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::PhilipAndJames),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who didst give to thine apostles Philip and James grace and strength to bear witness to the truth: Grant that we, being mindful of their victory of faith, may glorify in life and death the Name of our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Philip and Saint James")
                    .subtitle("May 1")
                .page(189)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TheVisitation),
            CollectData {
                document: Document::from(
                    Text::from("Father in heaven, by whose grace the virgin mother of thy incarnate Son was blessed in bearing him, but still more blessed in keeping thy word: Grant us who honor the exaltation of her lowliness to follow the example of her devotion to thy will; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Visitation")
                    .subtitle("May 31")
                .page(189)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Barnabas),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O God, that we may follow the example of thy faithful servant Barnabas, who, seeking not his own renown but the well-being of thy Church, gave generously of his life and substance for the relief of the poor and the spread of the Gospel; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Barnabas")
                    .subtitle("June 11")
                .page(189)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::NativityOfStJohnTheBaptist),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, by whose providence thy servant John the Baptist was wonderfully born, and sent to prepare the way of thy Son our Savior by preaching repentance: Make us so to follow his doctrine and holy life, that we may truly repent according to his preaching; and after his example constantly speak the truth, boldly rebuke vice, and patiently suffer for the truth’s sake; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Nativity of Saint John the Baptist")
                    .subtitle("June 24")
                .page(190)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::PeterAndPaul),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed apostles Peter and Paul glorified thee by their martyrdom: Grant that thy Church, instructed by their teaching and example, and knit together in unity by thy Spirit, may ever stand firm upon the one foundation, which is Jesus Christ our Lord; who liveth and reigneth with thee, in the unity of the same Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Peter and Saint Paul")
                    .subtitle("June 29")
                .page(190)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles Independence Day July 4".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::IndependenceDay),
            CollectData {
                document: Document::from(
                    Text::from("Lord God Almighty, in whose Name the founders of this country won liberty for themselves and for us, and lit the torch of freedom for nations then unborn: Grant, we beseech thee, that we and all the people of this land may have grace to maintain these liberties in righteousness and peace; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Independence Day")
                .subtitle("July 4")
                .page(190)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("The Collect “For the Nation,” page 207, may be used instead.".into())
            }
        ),
        (
            CollectId::Feast(Feast::MaryMagdalene),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed Son restored Mary Magdalene to health of body and mind, and called her to be a witness of his resurrection: Mercifully grant that by thy grace we may be healed of all our infirmities and know thee in the power of his endless life; who with thee and the Holy Spirit liveth and reigneth, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Mary Magdalene")
                    .subtitle("July 22")
                .page(191)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::James),
            CollectData {
                document: Document::from(
                    Text::from("O gracious God, we remember before thee this day thy servant and apostle James, first among the Twelve to suffer martyrdom for the Name of Jesus Christ; and we pray that thou wilt pour out upon the leaders of thy Church that spirit of self-denying service by which alone they may have true authority among thy people; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint James")
                .subtitle("July 25")
                .page(191)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TheTransfiguration),
            CollectData {
                document: Document::from(
                    Text::from("O God, who on the holy mount didst reveal to chosen witnesses thy well-beloved Son, wonderfully transfigured, in raiment white and glistening: Mercifully grant that we, being delivered from the disquietude of this world, may by faith behold the King in his beauty; who with thee, O Father, and thee, O Holy Ghost, liveth and reigneth, one God, world without end.")
                        .response("Amen.")
                )
                .label("The Transfiguration")
                    .subtitle("August 6")
                .page(191)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Mary),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast taken to thyself the blessed Virgin Mary, mother of thy incarnate Son: Grant that we, who have been redeemed by his blood, may share with her the glory of thine eternal kingdom; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Mary the Virgin")
                    .subtitle("August 15")
                .page(192)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Bartholomew),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty and everlasting God, who didst give to thine apostle Bartholomew grace truly to believe and to preach thy Word: Grant, we beseech thee, unto thy Church to love what he believed and to preach what he taught; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Bartholomew")
                    .subtitle("August 24")
                .page(192)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyCross),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose Son our Savior Jesus Christ was lifted high upon the cross that he might draw the whole world unto himself: Mercifully grant that we, who glory in the mystery of our redemption, may have grace to take up our cross and follow him; who liveth and reigneth with thee and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Holy Cross Day")
                    .subtitle("September 14")
                .page(192)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Matthew),
            CollectData {
                document: Document::from(
                    Text::from("We thank thee, heavenly Father, for the witness of thine apostle and evangelist Matthew to the Gospel of thy Son our Savior; and we pray that, after his example, we may with ready wills and hearts obey the calling of our Lord to follow him; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Matthew")
                    .subtitle("September 21")
                .page(193)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Michael),
            CollectData {
                document: Document::from(
                    Text::from("O everlasting God, who hast ordained and constituted the ministries of angels and men in a wonderful order: Mercifully grant that, as thy holy angels always serve and worship thee in heaven, so by thy appointment they may help and defend us on earth; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Michael and All Angels")
                    .subtitle("September 29")
                .page(193)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Luke),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who didst inspire thy servant Luke the physician to set forth in the Gospel the love and healing power of thy Son: Graciously continue in thy Church the like love and power to heal, to the praise and glory of thy Name; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Luke")
                    .subtitle("October 18")
                .page(193)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::JamesOfJerusalem),
            CollectData {
                document: Document::from(
                    Text::from("Grant, we beseech thee, O God, that after the example of thy servant James the Just, brother of our Lord, thy Church may give itself continually to prayer and to the reconciliation of all who are at variance and enmity; through the same our Lord Jesus Christ, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint James of Jerusalem")
                    .subtitle("October 23")
                .page(193)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::SimonAndJude),
            CollectData {
                document: Document::from(
                    Text::from("O God, we thank thee for the glorious company of the apostles, and especially on this day for Simon and Jude; and we pray that, as they were faithful and zealous in their mission, so we may with ardent devotion make known the love and mercy of our Lord and Savior Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Simon and Saint Jude")
                    .subtitle("October 28")
                .page(194)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::AllSaintsDay),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who hast knit together thine elect in one communion and fellowship in the mystical body of thy Son Christ our Lord: Grant us grace so to follow thy blessed saints in all virtuous and godly living, that we may come to those ineffable joys which thou hast prepared for those who unfeignedly love thee; through the same Jesus Christ our Lord, who with thee and the Holy Spirit liveth and reigneth, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("All Saints’ Day")
                    .subtitle("November 1")
                .page(194)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThanksgivingDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and gracious Father, we give thee thanks for the fruits of the earth in their season and for the labors of those who harvest them. Make us, we beseech thee, faithful stewards of thy great bounty, for the provision of our necessities and the relief of all who are in need, to the glory of thy Name; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Thanksgiving Day")
                .page(194)
                .tags(["Holy Days"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("For the Prayers of the People, the Litany of Thanksgiving on page 836 may be used.".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who didst give to thy servant N. boldness to confess the Name of our Savior Jesus Christ before the rulers of this world, and courage to die for this faith: Grant that we may always be ready to give a reason for the hope that is in us, and to suffer gladly for the sake of the same our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Martyr")
                .page(195)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, by whose grace and power thy holy martyr N. triumphed over suffering and was faithful even unto death: Grant us, who now remember him with thanksgiving, to be so faithful in our witness to thee in this world, that we may receive with him the crown of life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Martyr")
                .page(195)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or the following".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who didst enkindle the flame of thy love in the heart of thy holy martyr N.: Grant to us, thy humble servants, a like faith and power of love, that we who rejoice in her triumph may profit by her example; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Martyr")
                .page(196)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Missionary),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, we thank thee for thy servant N., whom thou didst call to preach the Gospel to the people of __________ (or to the ___________ people). Raise up, we beseech thee, in this and every land evangelists and heralds of thy kingdom, that thy Church may proclaim the unsearchable riches of our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Missionary")
                .page(196)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Missionary),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who willest to be glorified in thy saints, and didst raise up thy servant N. to be a light in the world: Shine, we pray thee, in our hearts, that we also in our generation may show forth thy praise, who hast called us out of darkness into thy marvelous light; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Missionary")
                .page(196)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "Preface of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Pastor),
            CollectData {
                document: Document::from(
                    Text::from("O heavenly Father, Shepherd of thy people, we give thee thanks for thy servant N., who was faithful in the care and nurture of thy flock; and we pray that, following his example and the teaching of his holy life, we may by thy grace grow into the stature of the fullness of our Lord and Savior Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Pastor")
                .page(197)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Pastor),
            CollectData {
                document: Document::from(
                    Text::from("O God, our heavenly Father, who didst raise up thy faithful servant N. to be a [bishop and] pastor in thy Church and to feed thy flock: Give abundantly to all pastors the gifts of thy Holy Spirit, that they may minister in thy household as true servants of Christ and stewards of thy divine mysteries; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Pastor")
                .page(197)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Theologian),
            CollectData {
                document: Document::from(
                    Text::from("O God, who by thy Holy Spirit dost give to some the word of wisdom, to others the word of knowledge, and to others the word of faith: We praise thy Name for the gifts of grace manifested in thy servant N., and we pray that thy Church may never be destitute of such gifts; through Jesus Christ our Lord, who with thee and the same Spirit liveth and reigneth, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Theologian and Teacher")
                .page(197)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Theologian),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who didst give to thy servant N. special gifts of grace to understand and teach the truth as it is in Christ Jesus: Grant, we beseech thee, that by this teaching we may know thee, the one true God, and Jesus Christ whom thou hast sent; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Theologian and Teacher")
                .page(197)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "Preface of a Saint, or of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Monastic),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son became poor that we through his poverty might be rich: Deliver us, we pray thee, from an inordinate love of this world, that, inspired by the devotion of thy servant N., we may serve thee with singleness of heart, and attain to the riches of the age to come; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Monastic")
                .page(198)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Monastic),
            CollectData {
                document: Document::from(
                    Text::from("O God, by whose grace thy servant N., enkindled with the fire of thy love, became a burning and a shining light in thy Church: Grant that we also may be aflame with the spirit of love and discipline, and may ever walk before thee as children of light; through Jesus Christ our Lord, who with thee, in the unity of the Holy Spirit, liveth and reigneth, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Monastic")
                .page(198)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who hast compassed us about with so great a cloud of witnesses: Grant that we, encouraged by the good example of thy servant N., may persevere in running the race that is set before us, until at length, through thy mercy, we may with him attain to thine eternal joy; through Jesus Christ, the author and perfecter of our faith, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Saint")
                .page(198)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast brought us near to an innumerable company of angels and to the spirits of just men made perfect: Grant us during our earthly pilgrimage to abide in their fellowship, and in our heavenly country to become partakers of their joy; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Saint")
                .page(199)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, who by thy Holy Spirit hast made us one with thy saints in heaven and on earth: Grant that in our earthly pilgrimage we may ever be supported by this fellowship of love and prayer, and may know ourselves to be surrounded by their witness to thy power and mercy. We ask this for the sake of Jesus Christ, in whom all our intercessions are acceptable through the Spirit, and who liveth and reigneth for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Saint")
                .page(199)
                .tags(["The Common of Saints"])
                .version(Version::RiteI),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyTrinity),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who hast revealed to thy Church thine eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace to continue steadfast in the confession of this faith, and constant in our worship of thee, Father, Son, and Holy Spirit; who livest and reignest, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("1. Of the Holy Trinity")
                .page(199)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: Some("For optional use, when desired, subject to the rules set forth in the Calendar of the Church Year.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolySpirit),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and most merciful God, grant, we beseech thee, that by the indwelling of thy Holy Spirit we may be enlightened and strengthened for thy service; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the same Spirit ever, one God, world without end.")
                        .response("Amen.")
                )
                .label("2. Of the Holy Spirit")
                .page(200)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyAngels),
            CollectData {
                document: Document::from(
                    Text::from("O everlasting God, who hast ordained and constituted the ministries of angels and men in a wonderful order: Mercifully grant that, as thy holy angels always serve and worship thee in heaven, so by thy appointment they may help and defend us on earth; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("3. Of the Holy Angels")
                .page(200)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Incarnation),
            CollectData {
                document: Document::from(
                    Text::from("O God, who didst wonderfully create, and yet more wonderfully restore, the dignity of human nature: Grant that we may share the divine life of him who humbled himself to share our humanity, thy Son Jesus Christ; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("4. Of the Incarnation")
                .page(200)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyEucharist),
            CollectData {
                document: Document::from(
                    Text::from("God our Father, whose Son our Lord Jesus Christ in a wonderful Sacrament hath left unto us a memorial of his passion: Grant us so to venerate the sacred mysteries of his Body and Blood, that we may ever perceive within ourselves the fruit of his redemption; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("5. Of the Holy Eucharist")
                .page(201)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Epiphany".into(),
                rubric_before: Some("Especially suitable for Thursdays".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyCross),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose beloved Son willingly endured the agony and shame of the cross for our redemption: Give us courage, we beseech thee, to take up our cross and follow him; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("6. Of the Holy Cross")
                .page(201)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("Especially suitable for Fridays".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::AllBaptizedChristians),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O Lord God, to all who have been baptized into the death and resurrection of thy Son Jesus Christ, that, as we have put away the old life of sin, so we may be renewed in the spirit of our minds, and live in righteousness and true holiness; through the same Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("7. For All Baptized Christians")
                .page(201)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Baptism".into(),
                rubric_before: Some("Especially suitable for Saturdays".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::TheDeparted),
            CollectData {
                document: Document::from(
                    Text::from("O eternal Lord God, who holdest all souls in life: Give, we beseech thee, to thy whole Church in paradise and on earth thy light and thy peace; and grant that we, following the good examples of those who have served thee here and are now at rest, may at the last enter with them into thine unending joy; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("8. For the Departed")
                .page(202)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::TheDeparted),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, we remember this day before thee thy faithful servant N.; and we pray that, having opened to him the gates of larger life, thou wilt receive him more and more into thy joyful service, that, with all who have faithfully served thee in the past, he may share in the eternal victory of Jesus Christ our Lord; who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("8. For the Departed")
                .page(202)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Commemoration of the Dead".into(),
                rubric_before: None,
                rubric_after: Some("Any of the Collects appointed for use at the Burial of the Dead may be used instead.\n\nFor the Prayers of the People, one of the forms appointed for the Burial of the Dead may be used.\n\nThe postcommunion prayer on page 482 may be used.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::ReignOfChrist),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, whose will it is to restore all things in thy well-beloved Son, the King of kings and Lord of lords: Mercifully grant that the peoples of the earth, divided and enslaved by sin, may be freed and brought together under his most gracious rule; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("9. Of the Reign of Christ")
                .page(203)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Ascension, or of Baptism".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Baptism),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who by our baptism into the death and resurrection of thy Son Jesus Christ dost turn us from the old life of sin: Grant that we, being reborn to new life in him, may live in righteousness and holiness all our days; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("10. At Baptism")
                .page(203)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Baptism".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Confirmation),
            CollectData {
                document: Document::from(
                    Text::from("Grant, Almighty God, that we, who have been redeemed from the old life of sin by our baptism into the death and resurrection of thy Son Jesus Christ, may be renewed in thy Holy Spirit, and live in righteousness and true holiness; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("11. At Confirmation")
                .page(203)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Baptism, or of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Dedication),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, to whose glory we celebrate the dedication of this house of prayer: We give thee thanks for the fellowship of those who have worshiped in this place; and we pray that all who seek thee here may find thee, and be filled with thy joy and peace; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("12. On the Anniversary of the Dedication of a Church")
                .page(204)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Dedication of a Church".into(),
                rubric_before: None,
                rubric_after: Some("The Litany of Thanksgiving for a Church, page 578, may be used for the Prayers of the People.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::ChurchConvention),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting Father, who hast given the Holy Spirit to abide with us for ever: Bless, we beseech thee, with his grace and presence, the bishops and the other clergy and the laity here (or now, or soon to be) assembled in thy Name, that thy Church, being preserved in true faith and godly discipline, may fulfill all the mind of him who loved it and gave himself for it, thy Son Jesus Christ our Savior; who liveth and reigneth with thee, in the unity of the same Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("13. For a Church Convention")
                .page(204)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Pentecost, or of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::UnityOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, whose blessed Son before his passion prayed for his disciples that they might be one, even as thou and he are one: Grant that thy Church, being bound together in love and obedience to thee, may be united in one body by the one Spirit, that the world may believe in him whom thou didst send, the same thy Son Jesus Christ our Lord; who liveth and reigneth with thee, in the unity of the same Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("14. For the Unity of the Church")
                .page(205)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Baptism, or of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::EmberDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "I. For those to be ordained"))),
                        Document::from(
                            Text::from("Almighty God, the giver of all good gifts, who of thy divine providence hast appointed various orders in thy Church: Give thy grace, we humbly beseech thee, to all who are [now] called to any office and ministry for thy people; and so fill them with the truth of thy doctrine and clothe them with holiness of life, that they may faithfully serve before thee, to the glory of thy great Name and for the benefit of thy holy Church; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("15. For the Ministry (Ember Days)")
                .page(205)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Apostles".into(),
                rubric_before: Some("For use on the traditional days or at other times".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::EmberDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "II. For the choice of fit persons for the ministry"))),
                        Document::from(
                            Text::from("O God, who didst lead thy holy apostles to ordain ministers in every place: Grant that thy Church, under the guidance of the Holy Spirit, may choose suitable persons for the ministry of Word and Sacrament, and may uphold them in their work for the extension of thy kingdom; through him who is the Shepherd and Bishop of our souls, Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, for ever and ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("15. For the Ministry (Ember Days)")
                .page(205)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::EmberDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "III. For all Christians in their vocation"))),
                        Document::from(
                            Text::from("Almighty and everlasting God, by whose Spirit the whole body of thy faithful people is governed and sanctified: Receive our supplications and prayers, which we offer before thee for all members of thy holy Church, that in their vocation and ministry they may truly and godly serve thee; through our Lord and Savior Jesus Christ, who liveth and reigneth with thee, in the unity of the same Spirit, one God, now and for ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("15. For the Ministry (Ember Days)")
                .page(206)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Baptism, or of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::MissionOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("O God, who hast made of one blood all the peoples of the earth, and didst send thy blessed Son to preach peace to those who are far off and to those who are near: Grant that people everywhere may seek after thee and find thee, bring the nations into thy fold, pour out thy Spirit upon all flesh, and hasten the coming of thy kingdom; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("16. For the Mission of the Church")
                .page(206)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::MissionOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("O God of all the nations of the earth: Remember the multitudes who have been created in thine image but have not known the redeeming work of our Savior Jesus Christ and grant that, by the prayers and labors of thy holy Church, they may be brought to know and worship thee as thou hast been revealed in thy Son; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("16. For the Mission of the Church")
                .page(206)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season, or of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Nation),
            CollectData {
                document: Document::from(
                    Text::from("Lord God Almighty, who hast made all peoples of the earth for thy glory, to serve thee in freedom and peace: Grant to the people of our country a zeal for justice and the strength of forbearance, that we may use our liberty in accordance with thy gracious will; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("17. For the Nation")
                .page(207)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("The Collect for Independence Day may be used instead.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Peace),
            CollectData {
                document: Document::from(
                    Text::from("O Almighty God, kindle, we beseech thee, in every heart the true love of peace, and guide with thy wisdom those who take counsel for the nations of the earth, that in tranquillity thy dominion may increase till the earth is filled with the knowledge of thy love; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("18. For Peace")
                .page(207)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::RogationDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "I. For fruitful seasons"))),
                        Document::from(
                            Text::from("Almighty God, Lord of heaven and earth: We humbly pray that thy gracious providence may give and preserve to our use the harvests of the land and of the seas, and may prosper all who labor to gather them, that we, who constantly receive good things from thy hand, may always give thee thanks; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("19. For Rogation Days")
                .page(207)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: Some("For use on the traditional days or at other times".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::RogationDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "II. For commerce and industry"))),
                        Document::from(
                            Text::from("Almighty God, whose Son Jesus Christ in his earthly life shared our toil and hallowed our labor: Be present with thy people where they work; make those who carry on the industries and commerce of this land responsive to thy will; and give to us all a pride in what we do, and a just return for our labor; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("19. For Rogation Days")
                .page(208)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::RogationDays),
            CollectData {
                document: Document::from(
                    Series::from(vec![
                        Document::from(Heading::from((HeadingLevel::Heading4, "III. For stewardship of creation"))),
                        Document::from(
                            Text::from("O merciful Creator, whose hand is open wide to satisfy the needs of every living creature: Make us, we beseech thee, ever thankful for thy loving providence; and grant that we, remembering the account that we must one day give, may be faithful stewards of thy bounty; through Jesus Christ our Lord, who with thee and the Holy Spirit liveth and reigneth, one God, for ever and ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("19. For Rogation Days")
                .label("III. For stewardship of creation")
                .page(208)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Sick),
            CollectData {
                document: Document::from(
                    Text::from("Heavenly Father, giver of life and health: Comfort and relieve thy sick servants, and give thy power of healing to those who minister to their needs, that those (or N., or NN.) for whom our prayers are offered may be strengthened in their weakness and have confidence in thy loving care; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("20. For the Sick")
                .page(208)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: Some("The postcommunion prayer on page 457 may be used.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::SocialJustice),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who hast created us in thine own image: Grant us grace fearlessly to contend against evil and to make no peace with oppression; and, that we may reverently use our freedom, help us to employ it in the maintenance of justice in our communities and among the nations, to the glory of thy holy Name; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("21. For Social Justice")
                .page(209)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::SocialService),
            CollectData {
                document: Document::from(
                    Text::from("O Lord our heavenly Father, whose blessed Son came not to be ministered unto but to minister: Bless, we beseech thee, all who, following in his steps, give themselves to the service of others; that with wisdom, patience, and courage, they may minister in his name to the suffering, the friendless, and the needy; for the love of him who laid down his life for us, the same thy Son our Savior Jesus Christ, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("22. For Social Service")
                .page(209)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Education),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, the fountain of all wisdom: Enlighten by thy Holy Spirit those who teach and those who learn, that, rejoicing in the knowledge of thy truth, they may worship thee and serve thee from generation to generation; through Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("23. For Education")
                .page(209)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Vocation),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God our heavenly Father, who declarest thy glory and showest forth thy handiwork in the heavens and in the earth: Deliver us, we beseech thee, in our several occupations from the service of self alone, that we may do the work which thou givest us to do, in truth and beauty and for the common good; for the sake of him who came among us as one that serveth, thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("24. For Vocation in Daily Work")
                .page(210)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::LaborDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who hast so linked our lives one with another that all we do affects, for good or ill, all other lives: So guide us in the work we do, that we may do it not for self alone, but for the common good; and, as we seek a proper return for our own labor, make us mindful of the rightful aspirations of other workers, and arouse our concern for those who are out of work; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("25. For Labor Day")
                .page(210)
                .tags(["Various Occasions"])
                .version(Version::RiteI),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),

    ];
}
