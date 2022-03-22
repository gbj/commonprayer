use calendar::{CommonOfSaints, Feast, LiturgicalWeek, Proper, VariousOccasions};
use liturgy::{Document, Heading, HeadingLevel, Series, Text};

use crate::{CollectData, CollectId};

lazy_static! {
    pub static ref COLLECTS_CONTEMPORARY: Vec<(CollectId, CollectData)> = vec![
        (
            CollectId::Week(LiturgicalWeek::Advent1),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, give us grace to cast away the works of darkness, and put on the armor of light, now in the time of this mortal life in which your Son Jesus Christ came to visit us in great humility; that in the last day, when he shall come again in his glorious majesty to judge both the living and the dead, we may rise to the life immortal; through him who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("First Sunday of Advent")
                .page(125)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent2),
            CollectData {
                document: Document::from(
                    Text::from("Merciful God, who sent your messengers the prophets to preach repentance and prepare the way for our salvation: Give us grace to heed their warnings and forsake our sins, that we may greet with joy the coming of Jesus Christ our Redeemer; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Second Sunday of Advent")
                .page(125)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent3),
            CollectData {
                document: Document::from(
                    Text::from("Stir up your power, O Lord, and with great might come among us; and, because we are sorely hindered by our sins, let your bountiful grace and mercy speedily help and deliver us; through Jesus Christ our Lord, to whom, with you and the Holy Spirit, be honor and glory, now and for ever.")
                        .response("Amen.")
                )
                .label("Third Sunday of Advent")
                .page(212)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: Some("Wednesday, Friday, and Saturday of this week are the traditional winter Ember Days.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Advent4),
            CollectData {
                document: Document::from(
                    Text::from("Purify our conscience, Almighty God, by your daily visitation, that your Son Jesus Christ, at his coming, may find in us a mansion prepared for himself; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday of Advent")
                .page(212)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("O God, you make us glad by the yearly festival of the birth of your only Son Jesus Christ: Grant that we, who joyfully receive him as our Redeemer, may with sure confidence behold him when he comes to be our Judge; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(212)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasEve),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have caused this holy night to shine with the brightness of the true Light: Grant that we, who have known the mystery of that Light on earth, may also enjoy him perfectly in heaven; where with you and the Holy Spirit he lives and reigns, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(213)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have caused this holy night to shine with the brightness of the true Light: Grant that we, who have known the mystery of that Light on earth, may also enjoy him perfectly in heaven; where with you and the Holy Spirit he lives and reigns, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(213)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::ChristmasDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have given your only-begotten Son to take our nature upon him, and to be born [this day] of a pure virgin: Grant that we, who have been born again and made your children by adoption and grace, may daily be renewed by your Holy Spirit; through our Lord Jesus Christ, to whom with you and the same Spirit be honor and glory, now and for ever.")
                        .response("Amen.")
                )
                .label("The Nativity of Our Lord: Christmas Day")
                    .subtitle("December 25")
                .page(213)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: Some("The Collect immediately preceding and any of the sets of Proper Lessons for Christmas Day serve for any weekdays between Holy Innocents’ Day and the First Sunday after Christmas Day.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Christmas1),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have poured upon us the new light of your incarnate Word: Grant that this light, enkindled in our hearts, may shine forth in our lives; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("First Sunday after Christmas Day")
                .page(213)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: Some("This Sunday takes precedence over the three Holy Days which follow Christmas Day. As necessary, the observance of one, two, or all three of them, is postponed one day.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyName),
            CollectData {
                document: Document::from(
                    Text::from("Eternal Father, you gave to your incarnate Son the holy name of Jesus to be the sign of our salvation: Plant in every heart, we pray, the love of him who is the Savior of the world, our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("The Holy Name")
                    .subtitle("January 1")
                .page(213)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Christmas2),
            CollectData {
                document: Document::from(
                    Text::from("O God, who wonderfully created, and yet more wonderfully restored, the dignity of human nature: Grant that we may share the divine life of him who humbled himself to share our humanity, your Son Jesus Christ; who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Second Sunday after Christmas Day")
                .page(214)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Epiphany),
            CollectData {
                document: Document::from(
                    Text::from("O God, by the leading of a star you manifested your only Son to the peoples of the earth: Lead us, who know you now by faith, to your presence, where we may see your glory face to face; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Epiphany")
                    .subtitle("January 6")
                .page(214)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: Some("The preceding Collect, with the Psalm and Lessons for the Epiphany, or those for the Second Sunday after Christmas, serves for weekdays between the Epiphany and the following Sunday. The Preface of the Epiphany is used.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany1),
            CollectData {
                document: Document::from(
                    Text::from("Father in heaven, who at the baptism of Jesus in the River Jordan proclaimed him your beloved Son and anointed him with the Holy Spirit: Grant that all who are baptized into his Name may keep the covenant they have made, and boldly confess him as Lord and Savior; who with you and the Holy Spirit lives and reigns, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("First Sunday after the Epiphany: The Baptism of our Lord")
                .page(214)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany2),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose Son our Savior Jesus Christ is the light of the world: Grant that your people, illumined by your Word and Sacraments, may shine with the radiance of Christ’s glory, that he may be known, worshiped, and obeyed to the ends of the earth; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Second Sunday after the Epiphany")
                .page(215)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany3),
            CollectData {
                document: Document::from(
                    Text::from("Give us grace, O Lord, to answer readily the call of our Savior Jesus Christ and proclaim to all people the Good News of his salvation, that we and the whole world may perceive the glory of his marvelous works; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Third Sunday after the Epiphany")
                .page(215)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany4),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, you govern all things both in heaven and on earth: Mercifully hear the supplications of your people, and in our time grant us your peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday after the Epiphany")
                .page(215)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany5),
            CollectData {
                document: Document::from(
                    Text::from("Set us free, O God, from the bondage of our sins, and give us the liberty of that abundant life which you have made known to us in your Son our Savior Jesus Christ; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fifth Sunday after the Epiphany")
                .page(216)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany6),
            CollectData {
                document: Document::from(
                    Text::from("O God, the strength of all who put their trust in you: Mercifully accept our prayers; and because in our weakness we can do nothing good without you, give us the help of your grace, that in keeping your commandments we may please you both in will and deed; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Sixth Sunday after the Epiphany")
                .page(216)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany7),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, you have taught us that without love whatever we do is worth nothing: Send your Holy Spirit and pour into our hearts your greatest gift, which is love, the true bond of peace and of all virtue, without which whoever lives is accounted dead before you. Grant this for the sake of your only Son Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Seventh Sunday after the Epiphany")
                .page(216)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Epiphany8),
            CollectData {
                document: Document::from(
                    Text::from("Most loving Father, whose will it is for us to give thanks for all things, to fear nothing but the loss of you, and to cast all our care on you who care for us: Preserve us from faithless fears and worldly anxieties, that no clouds of this mortal life may hide from us the light of that love which is immortal, and which you have manifested to us in your Son Jesus Christ our Lord; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Eighth Sunday after the Epiphany")
                .page(217)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany, or of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::LastEpiphany),
            CollectData {
                document: Document::from(
                    Text::from("O God, who before the passion of your only-begotten Son revealed his glory upon the holy mountain: Grant to us that we, beholding by faith the light of his countenance, may be strengthened to bear our cross, and be changed into his likeness from glory to glory; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Last Sunday after the Epiphany")
                .page(217)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: Some("This Proper is always used on the Sunday before Ash Wednesday".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::AshWednesday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, you hate nothing you have made and forgive the sins of all who are penitent: Create and make in us new and contrite hearts, that we, worthily lamenting our sins and acknowledging our wretchedness, may obtain of you, the God of all mercy, perfect remission and forgiveness; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Ash Wednesday")
                .page(217)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Lent".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 264.".into()),
                rubric_after: Some("This Collect, with the corresponding Psalm and Lessons, also serves for the weekdays which follow, except as otherwise appointed.".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent1),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed Son was led by the Spirit to be tempted by Satan: Come quickly to help us who are assaulted by many temptations; and, as you know the weaknesses of each of us, let each one find you mighty to save; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("First Sunday in Lent")
                .page(218)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: Some("Wednesday, Friday, and Saturday of this week are the traditional spring Ember Days.".into()),
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent2),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose glory it is always to have mercy: Be gracious to all who have gone astray from your ways, and bring them again with penitent hearts and steadfast faith to embrace and hold fast the unchangeable truth of your Word, Jesus Christ your Son; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Second Sunday in Lent")
                .page(218)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent3),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you know that we have no power in ourselves to help ourselves: Keep us both outwardly in our bodies and inwardly in our souls, that we may be defended from all adversities which may happen to the body, and from all evil thoughts which may assault and hurt the soul; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Third Sunday in Lent")
                .page(218)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent4),
            CollectData {
                document: Document::from(
                    Text::from("Gracious Father, whose blessed Son Jesus Christ came down from heaven to be the true bread which gives life to the world: Evermore give us this bread, that he may live in us, and we in him; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday in Lent")
                .page(219)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Lent5),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you alone can bring into order the unruly wills and affections of sinners: Grant your people grace to love what you command and desire what you promise; that, among the swift and varied changes of the world, our hearts may surely there be fixed where true joys are to be found; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Fifth Sunday in Lent")
                .page(219)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Lent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::HolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everliving God, in your tender love for the human race you sent your Son our Savior Jesus Christ to take upon him our nature, and to suffer death upon the cross, giving us the example of his great humility: Mercifully grant that we may walk in the way of his suffering, and also share in his resurrection; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Sunday of the Passion: Palm Sunday")
                .page(219)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 270.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MondayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose most dear Son went not up to joy but first he suffered pain, and entered not into glory before he was crucified: Mercifully grant that we, walking in the way of the cross, may find it none other than the way of life and peace; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Monday in Holy Week")
                .page(220)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TuesdayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("O God, by the passion of your blessed Son you made an instrument of shameful death to be for us the means of life: Grant us so to glory in the cross of Christ, that we may gladly suffer shame and loss for the sake of your Son our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Tuesday in Holy Week")
                .page(220)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::WednesdayInHolyWeek),
            CollectData {
                document: Document::from(
                    Text::from("Lord God, whose blessed Son our Savior gave his body to be whipped and his face to be spit upon: Give us grace to accept joyfully the sufferings of the present time, confident of the glory that shall be revealed; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Wednesday in Holy Week")
                .page(220)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MaundyThursday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, whose dear Son, on the night before he suffered, instituted the Sacrament of his Body and Blood: Mercifully grant that we may receive it thankfully in remembrance of Jesus Christ our Lord, who in these holy mysteries gives us a pledge of eternal life; and who now lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Maundy Thursday")
                .page(221)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 274.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::GoodFriday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, we pray you graciously to behold this your family, for whom our Lord Jesus Christ was willing to be betrayed, and given into the hands of sinners, and to suffer death upon the cross; who now lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Good Friday")
                .page(221)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 276.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolySaturday),
            CollectData {
                document: Document::from(
                    Text::from("O God, Creator of heaven and earth: Grant that, as the crucified body of your dear Son was laid in the tomb and rested on this holy Sabbath, so we may await with him the coming of the third day, and rise with him to newness of life; who now lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Holy Saturday")
                .page(221)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("The Proper Liturgy for this day is on page 283.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::EasterVigil),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who for our redemption gave your only-begotten Son to the death of the cross, and by his glorious resurrection delivered us from the power of our enemy: Grant us so to die daily to sin, that we may evermore live with him in the joy of his resurrection; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Vigil")
                .page(295),
                preface: "".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("O God, who for our redemption gave your only-begotten Son to the death of the cross, and by his glorious resurrection delivered us from the power of our enemy: Grant us so to die daily to sin, that we may evermore live with him in the joy of his resurrection; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(222)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: Some("The Liturgy of the Easter Vigil is on page 285.".into()),
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::EasterVigil),
            CollectData {
                document: Document::from(
                    Text::from("O God, who made this most holy night to shine with the glory of the Lord’s resurrection: Stir up in your Church that Spirit of adoption which is given to us in Baptism, that we, being renewed both in body and mind, may worship you in sincerity and truth; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(222)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("O God, who made this most holy night to shine with the glory of the Lord’s resurrection: Stir up in your Church that Spirit of adoption which is given to us in Baptism, that we, being renewed both in body and mind, may worship you in sincerity and truth; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(222)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who through your only-begotten Son Jesus Christ overcame death and opened to us the gate of everlasting life: Grant that we, who celebrate with joy the day of the Lord’s resurrection, may be raised from the death of sin by your life-giving Spirit; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Easter Day")
                .page(222)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::MondayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Grant, we pray, Almighty God, that we who celebrate with awe the Paschal feast may be found worthy to attain to everlasting joys; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Monday in Easter Week")
                .page(223)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TuesdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("O God, who by the glorious resurrection of your Son Jesus Christ destroyed death and brought life and immortality to light: Grant that we, who have been raised with him, may abide in his presence and rejoice in the hope of eternal glory; through Jesus Christ our Lord, to whom, with you and the Holy Spirit, be dominion and praise for ever and ever.")
                        .response("Amen.")
                )
                .label("Tuesday in Easter Week")
                .page(223)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::WednesdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son made himself known to his disciples in the breaking of bread: Open the eyes of our faith, that we may behold him in all his redeeming work; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Wednesday in Easter Week")
                .page(223)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThursdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who in the Paschal mystery established the new covenant of reconciliation: Grant that all who have been reborn into the fellowship of Christ’s Body may show forth in their lives what they profess by their faith; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Thursday in Easter Week")
                .page(223)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::FridayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, who gave your only Son to die for our sins and to rise for our justification: Give us grace so to put away the leaven of malice and wickedness, that we may always serve you in pureness of living and truth; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Friday in Easter Week")
                .page(224)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::SaturdayInEasterWeek),
            CollectData {
                document: Document::from(
                    Text::from("We thank you, heavenly Father, that you have delivered us from the dominion of sin and death and brought us into the kingdom of your Son; and we pray that, as by his death he has recalled us to life, so by his love he may raise us to eternal joys; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saturday in Easter Week")
                .page(224)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter2),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who in the Paschal mystery established the new covenant of reconciliation: Grant that all who have been reborn into the fellowship of Christ’s Body may show forth in their lives what they profess by their faith; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Second Sunday of Easter")
                .page(224)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter3),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son made himself known to his disciples in the breaking of bread: Open the eyes of our faith, that we may behold him in all his redeeming work; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Third Sunday of Easter")
                .page(225)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter4),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose Son Jesus is the good shepherd of your people: Grant that when we hear his voice we may know him who calls us each by name, and follow where he leads; who, with you and the Holy Spirit, lives and reigns, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Fourth Sunday of Easter")
                .page(225)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter5),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whom truly to know is everlasting life: Grant us so perfectly to know your Son Jesus Christ to be the way, the truth, and the life, that we may steadfastly follow his steps in the way that leads to eternal life; through Jesus Christ your Son our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Fifth Sunday of Easter")
                .page(225)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter6),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have prepared for those who love you such good things as surpass our understanding: Pour into our hearts such love towards you, that we, loving you in all things and above all things, may obtain your promises, which exceed all that we can desire; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Sixth Sunday of Easter")
                .page(225)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Easter".into(),
                rubric_before: None,
                rubric_after: Some("Monday, Tuesday, and Wednesday of this week are the traditional Rogation Days.".into())
            }
        ),
        (
            CollectId::Feast(Feast::AscensionDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed Son our Savior Jesus Christ ascended far above all heavens that he might fill all things: Mercifully give us faith to perceive that, according to his promise, he abides with his Church on earth, even to the end of the ages; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Ascension Day")
                .page(226)
                .tags(["Seasons of the Year"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::AscensionDay),
            CollectData {
                document: Document::from(
                    Text::from("Grant, we pray, Almighty God, that as we believe your only-begotten Son our Lord Jesus Christ to have ascended into heaven, so we may also in heart and mind there ascend, and with him continually dwell; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Ascension Day")
                .page(226)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Ascension".into(),
                rubric_before: None,
                rubric_after: Some("Either of the preceding Collects, with the proper Psalm and Lessons for Ascension Day, serves for the following weekdays, except as otherwise appointed .".into())
            }
        ),
        (
            CollectId::Week(LiturgicalWeek::Easter7),
            CollectData {
                document: Document::from(
                    Text::from("O God, the King of glory, you have exalted your only Son Jesus Christ with great triumph to your kingdom in heaven: Do not leave us comfortless, but send us your Holy Spirit to strengthen us, and exalt us to that place where our Savior Christ has gone before; who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Seventh Sunday of Easter: The Sunday after Ascension Day")
                .page(226)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Ascension".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Pentecost),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, on this day you opened the way of eternal life to every race and nation by the promised gift of your Holy Spirit: Shed abroad this gift throughout the world by the preaching of the Gospel, that it may reach to the ends of the earth; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Day of Pentecost: Whitsunday")
                .page(227)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Pentecost".into(),
                rubric_before: Some("When a Vigil of Pentecost is observed, it begins with the Service of Light, page 109 (substituting, if desired, the *Gloria in excelsis* for the *Phos hilaron*), and continues with the Salutation and Collect of the Day. Three or more of the appointed Lessons are read before the Gospel, each followed by a Psalm, Canticle, or hymn. Holy Baptism or Confirmation (beginning with the Presentation of the Candidates), or the Renewal of Baptismal Vows, page 292, follows the Sermon.".into()),
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::Feast(Feast::Pentecost),
            CollectData {
                document: Document::from(
                    Text::from("O God, who on this day taught the hearts of your faithful people by sending to them the light of your Holy Spirit: Grant us by the same Spirit to have a right judgment in all things, and evermore to rejoice in his holy comfort; through Jesus Christ your Son our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Day of Pentecost: Whitsunday")
                .page(227)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Pentecost".into(),
                rubric_before: None,
                rubric_after: Some("On the weekdays which follow, the numbered Proper which corresponds most closely to the date of Pentecost in that year is used. See page 158.\n\nWednesday, Friday, and Saturday of this week are the traditional summer Ember Days.".into())
            }
        ),
        (
            CollectId::Feast(Feast::TrinitySunday),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, you have given to us your servants grace, by the confession of a true faith, to acknowledge the glory of the eternal Trinity, and in the power of your divine Majesty to worship the Unity: Keep us steadfast in this faith and worship, and bring us at last to see you in your one and eternal glory, O Father; who with the Son and the Holy Spirit live and reign, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("First Sunday after Pentecost: Trinity Sunday")
                .page(228)
                .tags(["Seasons of the Year"]),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("On the weekdays which follow, the numbered Proper which corresponds most closely to the date of Trinity Sunday in that year is used.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper1),
            CollectData {
                document: Document::from(
                    Text::from("Remember, O Lord, what you have wrought in us and not what we deserve; and, as you have called us to your service, make us worthy of our calling; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 1")
                .subtitle("Week of the Sunday closest to May 11")
                .page(228)
                .tags(["Seasons of the Year", "The Season after Pentecost"]),
                preface: "No Proper Preface is used.".into(),
                rubric_before: Some("Directions for the use of the Propers which follow are on page 158.".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper2),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and merciful God, in your goodness keep us, we pray, from all things that may hurt us, that we, being ready both in mind and body, may accomplish with free hearts those things which belong to your purpose; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever; Amen.No Proper Preface is used.")
                        .response("Amen.")
                )
                .label("Proper 2")
                .subtitle("Week of the Sunday closest to May 18")
                .page(229)
                .tags(["Seasons of the Year"]),
                preface: "No Proper Preface is used.".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper3),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O Lord, that the course of this world may be peaceably governed by your providence; and that your Church may joyfully serve you in confidence and serenity; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 3")
                    .subtitle("The Sunday closest to May 25")
                .page(229)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper4),
            CollectData {
                document: Document::from(
                    Text::from("O God, your never-failing providence sets in order all things both in heaven and earth: Put away from us, we entreat you, all hurtful things, and give us those things which are profitable for us; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 4")
                    .subtitle("The Sunday closest to June 1")
                .page(229)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper5),
            CollectData {
                document: Document::from(
                    Text::from("O God, from whom all good proceeds: Grant that by your inspiration we may think those things that are right, and by your merciful guiding may do them; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 5")
                    .subtitle("The Sunday closest to June 8")
                .page(229)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper6),
            CollectData {
                document: Document::from(
                    Text::from("Keep, O Lord, your household the Church in your steadfast faith and love, that through your grace we may proclaim your truth with boldness, and minister your justice with compassion; for the sake of our Savior Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 6")
                    .subtitle("The Sunday closest to June 15")
                .page(230)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper7),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, make us have perpetual love and reverence for your holy Name, for you never fail to help and govern those whom you have set upon the sure foundation of your loving-kindness; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 7")
                    .subtitle("The Sunday closest to June 22")
                .page(230)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper8),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have built your Church upon the foundation of the apostles and prophets, Jesus Christ himself being the chief cornerstone: Grant us so to be joined together in unity of spirit by their teaching, that we may be made a holy temple acceptable to you; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 8")
                    .subtitle("The Sunday closest to June 29")
                .page(230)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper9),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have taught us to keep all your commandments by loving you and our neighbor: Grant us the grace of your Holy Spirit, that we may be devoted to you with our whole heart, and united to one another with pure affection; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 9")
                    .subtitle("The Sunday closest to July 6")
                .page(231)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper10),
            CollectData {
                document: Document::from(
                    Text::from("O Lord, mercifully receive the prayers of your people who call upon you, and grant that they may know and understand what things they ought to do, and also may have grace and power faithfully to accomplish them; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 10")
                    .subtitle("The Sunday closest to July 13")
                .page(231)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper11),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, the fountain of all wisdom, you know our necessities before we ask and our ignorance in asking: Have compassion on our weakness, and mercifully give us those things which for our unworthiness we dare not, and for our blindness we cannot ask; through the worthiness of your Son Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 11")
                    .subtitle("The Sunday closest to July 20")
                .page(231)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper12),
            CollectData {
                document: Document::from(
                    Text::from("O God, the protector of all who trust in you, without whom nothing is strong, nothing is holy: Increase and multiply upon us your mercy; that, with you as our ruler and guide, we may so pass through things temporal, that we lose not the things eternal; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 12")
                    .subtitle("The Sunday closest to July 27")
                .page(231)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper13),
            CollectData {
                document: Document::from(
                    Text::from("Let your continual mercy, O Lord, cleanse and defend your Church; and, because it cannot continue in safety without your help, protect and govern it always by your goodness; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 13")
                    .subtitle("The Sunday closest to August 3")
                .page(232)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper14),
            CollectData {
                document: Document::from(
                    Text::from("Grant to us, Lord, we pray, the spirit to think and do always those things that are right, that we, who cannot exist without you, may by you be enabled to live according to your will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 14")
                    .subtitle("The Sunday closest to August 10")
                .page(232)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper15),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have given your only Son to be for us a sacrifice for sin, and also an example of godly life: Give us grace to receive thankfully the fruits of his redeeming work, and to follow daily in the blessed steps of his most holy life; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 15")
                    .subtitle("The Sunday closest to August 17")
                .page(232)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper16),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O merciful God, that your Church, being gathered together in unity by your Holy Spirit, may show forth your power among all peoples, to the glory of your Name;through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 16")
                    .subtitle("The Sunday closest to August 24")
                .page(233)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper17),
            CollectData {
                document: Document::from(
                    Text::from("Lord of all power and might, the author and giver of all good things: Graft in our hearts the love of your Name; increase in us true religion; nourish us with all goodness; and bring forth in us the fruit of good works; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 17")
                    .subtitle("The Sunday closest to August 31")
                .page(233)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper18),
            CollectData {
                document: Document::from(
                    Text::from("Grant us, O Lord, to trust in you with all our hearts; for, as you always resist the proud who confide in their own strength, so you never forsake those who make their boast of your mercy; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 18")
                    .subtitle("The Sunday closest to September 7")
                .page(233)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper19),
            CollectData {
                document: Document::from(
                    Text::from("O God, because without you we are not able to please you, mercifully grant that your Holy Spirit may in all things direct and rule our hearts; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 19")
                    .subtitle("The Sunday closest to September 14")
                .page(233)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: Some("The Wednesday, Friday, and Saturday after September 14 are the traditional autumnal Ember Days.".into())
            }
        ),
        (
            CollectId::Proper(Proper::Proper20),
            CollectData {
                document: Document::from(
                    Text::from("Grant us, Lord, not to be anxious about earthly things, but to love things heavenly; and even now, while we are placed among things that are passing away, to hold fast to those that shall endure; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 20")
                    .subtitle("The Sunday closest to September 21")
                .page(234)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper21),
            CollectData {
                document: Document::from(
                    Text::from("O God, you declare your almighty power chiefly in showing mercy and pity: Grant us the fullness of your grace, that we, running to obtain your promises, may become partakers of your heavenly treasure; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 21")
                    .subtitle("The Sunday closest to September 28")
                .page(234)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper22),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, you are always more ready to hear than we to pray, and to give more than we either desire or deserve: Pour upon us the abundance of your mercy, forgiving us those things of which our conscience is afraid, and giving us those good things for which we are not worthy to ask, except through the merits and mediation of Jesus Christ our Savior; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 22")
                    .subtitle("The Sunday closest to October 5")
                .page(234)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper23),
            CollectData {
                document: Document::from(
                    Text::from("Lord, we pray that your grace may always precede and follow us, that we may continually be given to good works;through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 23")
                    .subtitle("The Sunday closest to October 12")
                .page(235)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper24),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, in Christ you have revealed your glory among the nations: Preserve the works of your mercy, that your Church throughout the world may persevere with steadfast faith in the confession of your Name; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 24")
                    .subtitle("The Sunday closest to October 19")
                .page(235)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper25),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, increase in us the gifts of faith, hope, and charity; and, that we may obtain what you promise, make us love what you command; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 25")
                    .subtitle("The Sunday closest to October 26")
                .page(235)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper26),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and merciful God, it is only by your gift that your faithful people offer you true and laudable service: Grant that we may run without stumbling to obtain your heavenly promises; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 26")
                    .subtitle("The Sunday closest to November 2")
                .page(235)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper27),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son came into the world that he might destroy the works of the devil and make us children of God and heirs of eternal life: Grant that, having this hope, we may purify ourselves as he is pure; that, when he comes again with power and great glory, we may be made like him in his eternal and glorious kingdom; where he lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 27")
                    .subtitle("The Sunday closest to November 9")
                .page(236)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper28),
            CollectData {
                document: Document::from(
                    Text::from("Blessed Lord, who caused all holy Scriptures to be written for our learning: Grant us so to hear them, read, mark, learn, and inwardly digest them, that we may embrace and ever hold fast the blessed hope of everlasting life, which you have given us in our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Proper 28")
                    .subtitle("The Sunday closest to November 16")
                .page(236)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Proper(Proper::Proper29),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, whose will it is to restore all things in your well-beloved Son, the King of kings and Lord of lords: Mercifully grant that the peoples of the earth, divided and enslaved by sin, may be freed and brought together under his most gracious rule; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Proper 29")
                    .subtitle("The Sunday closest to November 23")
                .page(236)
                .tags(["Seasons of the Year"]),
                preface: "Preface of the Lord’s Day, or of Baptism".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Andrew),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who gave such grace to your apostle Andrew that he readily obeyed the call of your Son Jesus Christ, and brought his brother with him: Give us, who are called by your Holy Word, grace to follow him without delay, and to bring those near to us into his gracious presence; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Andrew")
                    .subtitle("November 30")
                .page(237)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Thomas),
            CollectData {
                document: Document::from(
                    Text::from("Everliving God, who strengthened your apostle Thomas with firm and certain faith in your Son’s resurrection: Grant us so perfectly and without doubt to believe in Jesus Christ, our Lord and our God, that our faith may never be found wanting in your sight; through him who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Thomas")
                    .subtitle("December 21")
                .page(237)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Stephen),
            CollectData {
                document: Document::from(
                    Text::from("We give you thanks, O Lord of glory, for the example of the first martyr Stephen, who looked up to heaven and prayed for his persecutors to your Son Jesus Christ, who stands at your right hand; where he lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Saint Stephen")
                    .subtitle("December 26")
                .page(237)
                .tags(["Holy Days"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::John),
            CollectData {
                document: Document::from(
                    Text::from("Shed upon your Church, O Lord, the brightness of your light, that we, being illumined by the teaching of your apostle and evangelist John, may so walk in the light of your truth, that at length we may attain to the fullness of eternal life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint John")
                    .subtitle("December 27")
                .page(238)
                .tags(["Holy Days"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyInnocents),
            CollectData {
                document: Document::from(
                    Text::from("We remember today, O God, the slaughter of the holy innocents of Bethlehem by King Herod. Receive, we pray, into the arms of your mercy all innocent victims; and by your great might frustrate the designs of evil tyrants and establish your rule of justice, love, and peace; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Holy Innocents")
                    .subtitle("December 28")
                .page(238)
                .tags(["Holy Days"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ConfessionOfStPeter),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, who inspired Simon Peter, first among the apostles, to confess Jesus as Messiah and Son of the living God: Keep your Church steadfast upon the rock of this faith, so that in unity and peace we may proclaim the one truth and follow the one Lord, our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Confession of Saint Peter")
                    .subtitle("January 18")
                .page(238)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ConversionOfStPaul),
            CollectData {
                document: Document::from(
                    Text::from("O God, by the preaching of your apostle Paul you have caused the light of the Gospel to shine throughout the world:Grant, we pray, that we, having his wonderful conversion in remembrance, may show ourselves thankful to you by following his holy teaching; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Conversion of Saint Paul")
                    .subtitle("January 25")
                .page(239)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThePresentation),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everliving God, we humbly pray that, as your only-begotten Son was this day presented in the temple, so we may be presented to you with pure and clean hearts by Jesus Christ our Lord; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Presentation")
                    .subtitle("February 2")
                .page(239)
                .tags(["Holy Days"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Matthias),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who in the place of Judas chose your faithful servant Matthias to be numbered among the Twelve: Grant that your Church, being delivered from false apostles, may always be guided and governed by faithful and true pastors; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Matthias")
                    .subtitle("February 24")
                .page(239)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Joseph),
            CollectData {
                document: Document::from(
                    Text::from("O God, who from the family of your servant David raised up Joseph to be the guardian of your incarnate Son and the spouse of his virgin mother: Give us grace to imitate his uprightness of life and his obedience to your commands; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Joseph")
                    .subtitle("March 19")
                .page(239)
                .tags(["Holy Days"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Annunciation),
            CollectData {
                document: Document::from(
                    Text::from("Pour your grace into our hearts, O Lord, that we who have known the incarnation of your Son Jesus Christ, announced by an angel to the Virgin Mary, may by his cross and passion be brought to the glory of his resurrection; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("The Annunciation")
                    .subtitle("March 25")
                .page(240)
                .tags(["Holy Days"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Mark),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, by the hand of Mark the evangelist you have given to your Church the Gospel of Jesus Christ the Son of God: We thank you for this witness, and pray that we may be firmly grounded in its truth; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Mark")
                    .subtitle("April 25")
                .page(240)
                .tags(["Holy Days"]),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::PhilipAndJames),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who gave to your apostles Philip and James grace and strength to bear witness to the truth: Grant that we, being mindful of their victory of faith, may glorify in life and death the Name of our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Philip and Saint James")
                    .subtitle("May 1")
                .page(240)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TheVisitation),
            CollectData {
                document: Document::from(
                    Text::from("Father in heaven, by your grace the virgin mother of your incarnate Son was blessed in bearing him, but still more blessed in keeping your word: Grant us who honor the exaltation of her lowliness to follow the example of her devotion to your will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Visitation")
                    .subtitle("May 31")
                .page(241)
                .tags(["Holy Days"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Barnabas),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O God, that we may follow the example of your faithful servant Barnabas, who, seeking not his own renown but the well-being of your Church, gave generously of his life and substance for the relief of the poor and the spread of the Gospel; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Barnabas")
                    .subtitle("June 11")
                .page(241)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::NativityOfStJohnTheBaptist),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, by whose providence your servant John the Baptist was wonderfully born, and sent to prepare the way of your Son our Savior by preaching repentance: Make us so to follow his teaching and holy life, that we may truly repent according to his preaching; and, following his example, constantly speak the truth, boldly rebuke vice, and patiently suffer for the truth’s sake; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Nativity of Saint John the Baptist")
                    .subtitle("June 24")
                .page(241)
                .tags(["Holy Days"]),
                preface: "Preface of Advent".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::PeterAndPaul),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed apostles Peter and Paul glorified you by their martyrdom: Grant that your Church, instructed by their teaching and example, and knit together in unity by your Spirit, may ever stand firm upon the one foundation, which is Jesus Christ our Lord; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Peter and Saint Paul")
                    .subtitle("June 29")
                .page(241)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::IndependenceDay),
            CollectData {
                document: Document::from(
                    Text::from("Lord God Almighty, in whose Name the founders of this country won liberty for themselves and for us, and lit the torch of freedom for nations then unborn: Grant that we and all the people of this land may have grace to maintain our liberties in righteousness and peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Independence Day")
                    .subtitle("July 4")
                .page(242)
                .tags(["Holy Days"]),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("The Collect “For the Nation,” page 258, may be used instead.".into())
            }
        ),
        (
            CollectId::Feast(Feast::MaryMagdalene),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose blessed Son restored Mary Magdalene to health of body and of mind, and called her to be a witness of his resurrection: Mercifully grant that by your grace we may be healed from all our infirmities and know you in the power of his unending life; who with you and the Holy Spirit lives and reigns, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Mary Magdalene")
                    .subtitle("July 22")
                .page(242)
                .tags(["Holy Days"]),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::James),
            CollectData {
                document: Document::from(
                    Text::from("O gracious God, we remember before you today your servant and apostle James, first among the Twelve to suffer martyrdom for the Name of Jesus Christ; and we pray that you will pour out upon the leaders of your Church that spirit of self-denying service by which alone they may have true authority among your people; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint James")
                    .subtitle("July 25")
                .page(242)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::TheTransfiguration),
            CollectData {
                document: Document::from(
                    Text::from("O God, who on the holy mount revealed to chosen witnesses your well-beloved Son, wonderfully transfigured, in raiment white and glistening: Mercifully grant that we, being delivered from the disquietude of this world, may by faith behold the King in his beauty; who with you, O Father, and you, O Holy Spirit, lives and reigns, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("The Transfiguration")
                    .subtitle("August 6")
                .page(243)
                .tags(["Holy Days"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Mary),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have taken to yourself the blessed Virgin Mary, mother of your incarnate Son: Grant that we, who have been redeemed by his blood, may share with her the glory of your eternal kingdom; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Mary the Virgin")
                    .subtitle("August 15")
                .page(243)
                .tags(["Holy Days"]),
                preface: "Preface of the Incarnation".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Bartholomew),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who gave to your apostle Bartholomew grace truly to believe and to preach your Word: Grant that your Church may love what he believed and preach what he taught; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Bartholomew")
                    .subtitle("August 24")
                .page(243)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::HolyCross),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose Son our Savior Jesus Christ was lifted high upon the cross that he might draw the whole world to himself: Mercifully grant that we, who glory in the mystery of our redemption, may have grace to take up our cross and follow him; who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("Holy Cross Day")
                    .subtitle("September 14")
                .page(244)
                .tags(["Holy Days"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Matthew),
            CollectData {
                document: Document::from(
                    Text::from("We thank you, heavenly Father, for the witness of your apostle and evangelist Matthew to the Gospel of your Son our Savior; and we pray that, after his example, we may with ready wills and hearts obey the calling of our Lord to follow him;through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Matthew")
                    .subtitle("September 21")
                .page(244)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Michael),
            CollectData {
                document: Document::from(
                    Text::from("Everlasting God, you have ordained and constituted in a wonderful order the ministries of angels and mortals: Mercifully grant that, as your holy angels always serve and worship you in heaven, so by your appointment they may help and defend us here on earth; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Michael and All Angels")
                    .subtitle("September 29")
                .page(244)
                .tags(["Holy Days"]),
                preface: "Preface of Trinity Sunday Saint Luke October 18".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::Luke),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who inspired your servant Luke the physician to set forth in the Gospel the love and healing power of your Son: Graciously continue in your Church this love and power to heal, to the praise and glory of your Name; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint Luke")
                .subtitle("October 18")
                .page(245)
                .tags(["Holy Days"]),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::JamesOfJerusalem),
            CollectData {
                document: Document::from(
                    Text::from("Grant, O God, that, following the example of your servant James the Just, brother of our Lord, your Church may give itself continually to prayer and to the reconciliation of all who are at variance and enmity; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Saint James of Jerusalem")
                    .subtitle("October 23")
                .page(245)
                .tags(["Holy Days"]),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::SimonAndJude),
            CollectData {
                document: Document::from(
                    Text::from("O God, we thank you for the glorious company of the apostles, and especially on this day for Simon and Jude; and we pray that, as they were faithful and zealous in their mission, so we may with ardent devotion make known the love and mercy of our Lord and Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Saint Simon and Saint Jude")
                    .subtitle("October 28")
                .page(245)
                .tags(["Holy Days"]),
                preface: "Preface of Apostles".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::AllSaintsDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have knit together your elect in one communion and fellowship in the mystical body of your Son Christ our Lord: Give us grace so to follow your blessed saints in all virtuous and godly living, that we may come to those ineffable joys that you have prepared for those who truly love you; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, in glory everlasting.")
                        .response("Amen.")
                )
                .label("All Saints’ Day")
                    .subtitle("November 1")
                .page(245)
                .tags(["Holy Days"]),
                preface: "Preface of All Saints".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::Feast(Feast::ThanksgivingDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and gracious Father, we give you thanks for the fruits of the earth in their season and for the labors of those who harvest them. Make us, we pray, faithful stewards of your great bounty, for the provision of our necessities and the relief of all who are in need, to the glory of your Name; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Thanksgiving Day")
                .page(246)
                .tags(["Holy Days"]),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("For the Prayers of the People, the Litany of Thanksgiving on page 836 may be used.".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who gave to your servant N. boldness to confess the Name of our Savior Jesus Christ before the rulers of this world, and courage to die for this faith: Grant that we may always be ready to give a reason for the hope that is in us, and to suffer gladly for the sake of our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Martyr")
                .page(247)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, by whose grace and power your holy martyr N. triumphed over suffering and was faithful even to death: Grant us, who now remember him in thanksgiving, to be so faithful in our witness to you in this world, that we may receive with him the crown of life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Martyr")
                .page(247)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Martyr),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, who kindled the flame of your love in the heart of your holy martyr N.: Grant to us, your humble servants, a like faith and power of love, that we who rejoice in her triumph may profit by her example; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Martyr")
                .page(247)
                .tags(["The Common of Saints"]),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Missionary),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, we thank you for your servant N., whom you called to preach the Gospel to the people of__________ (or to the_________ people). Raise up in this and every land evangelists and heralds of your kingdom, that your Church may proclaim the unsearchable riches of our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Missionary")
                .page(247)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or the following".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Missionary),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose will it is to be glorified in your saints, and who raised up your servant N. to be a light in the world: Shine, we pray, in our hearts, that we also in our generation may show forth your praise, who called us out of darkness into your marvelous light; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Missionary")
                .page(248)
                .tags(["The Common of Saints"]),
                preface: "Preface of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Pastor),
            CollectData {
                document: Document::from(
                    Text::from("Heavenly Father, Shepherd of your people, we thank you for your servant N., who was faithful in the care and nurture of your flock; and we pray that, following his example and the teaching of his holy life, we may by your grace grow into the stature of the fullness of our Lord and Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Pastor")
                .page(248)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Pastor),
            CollectData {
                document: Document::from(
                    Text::from("O God, our heavenly Father, who raised up your faithful servant N., to be a [bishop and] pastor in your Church and to feed your flock: Give abundantly to all pastors the gifts of your Holy Spirit, that they may minister in your household as true servants of Christ and stewards of your divine mysteries; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Pastor")
                .page(248)
                .tags(["The Common of Saints"]),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Theologian),
            CollectData {
                document: Document::from(
                    Text::from("O God, by your Holy Spirit you give to some the word of wisdom, to others the word of knowledge, and to others the word of faith: We praise your Name for the gifts of grace manifested in your servant N., and we pray that your Church may never be destitute of such gifts; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Theologian and Teacher")
                .page(249)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Theologian),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you gave to your servant N. special gifts of grace to understand and teach the truth as it is in Christ Jesus: Grant that by this teaching we may know you, the one true God, and Jesus Christ whom you have sent; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Theologian and Teacher")
                .page(249)
                .tags(["The Common of Saints"]),
                preface: "Preface of a Saint, or of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Monastic),
            CollectData {
                document: Document::from(
                    Text::from("O God, whose blessed Son became poor that we through his poverty might be rich: Deliver us from an inordinate love of this world, that we, inspired by the devotion of your servant N., may serve you with singleness of heart, and attain to the riches of the age to come; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Monastic")
                .page(249)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Monastic),
            CollectData {
                document: Document::from(
                    Text::from("O God, by whose grace your servant N., kindled with the flame of your love, became a burning and a shining light in your Church: Grant that we also may be aflame with the spirit of love and discipline, and walk before you as children of light; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Monastic")
                .page(249)
                .tags(["The Common of Saints"]),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have surrounded us with a great cloud of witnesses: Grant that we, encouraged by the good example of your servant N., may persevere in running the race that is set before us, until at last we may with him attain to your eternal joy; through Jesus Christ, the pioneer and perfecter of our faith, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Saint")
                .page(250)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have brought us near to an innumerable company of angels, and to the spirits of just men made perfect: Grant us during our earthly pilgrimage to abide in their fellowship, and in our heavenly country to become partakers of their joy; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("Of a Saint")
                .page(250)
                .tags(["The Common of Saints"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::CommonOfSaints(CommonOfSaints::Saint),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, by your Holy Spirit you have made us one with your saints in heaven and on earth: Grant that in our earthly pilgrimage we may always be supported by this fellowship of love and prayer, and know ourselves to be surrounded by their witness to your power and mercy. We ask this for the sake of Jesus Christ, in whom all our intercessions are acceptable through the Spirit, and who lives and reigns for ever and ever.")
                        .response("Amen.")
                )
                .label("Of a Saint")
                .page(250)
                .tags(["The Common of Saints"]),
                preface: "Preface of a Saint".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyTrinity),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have revealed to your Church your eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace to continue steadfast in the confession of this faith, and constant in our worship of you, Father, Son, and Holy Spirit; for you live and reign, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("1. Of the Holy Trinity")
                .page(251)
                .tags(["Various Occasions"]),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolySpirit),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and most merciful God, grant that by the indwelling of your Holy Spirit we may be enlightened and strengthened for your service; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("2. Of the Holy Spirit")
                .page(251)
                .tags(["Various Occasions"]),
                preface: "Preface of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyAngels),
            CollectData {
                document: Document::from(
                    Text::from("Everlasting God, you have ordained and constituted in a wonderful order the ministries of angels and mortals: Mercifully grant that, as your holy angels always serve and worship you in heaven, so by your appointment they may help and defend us here on earth; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("3. Of the Holy Angels")
                .page(251)
                .tags(["Various Occasions"]),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Incarnation),
            CollectData {
                document: Document::from(
                    Text::from("O God, who wonderfully created, and yet more wonderfully restored, the dignity of human nature: Grant that we may share the divine life of him who humbled himself to share our humanity, your Son Jesus Christ; who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("4. Of the Incarnation")
                .page(252)
                .tags(["Various Occasions"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyEucharist),
            CollectData {
                document: Document::from(
                    Text::from("God our Father, whose Son our Lord Jesus Christ in a wonderful Sacrament has left us a memorial of his passion: Grant us so to venerate the sacred mysteries of his Body and Blood, that we may ever perceive within ourselves the fruit of his redemption; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("5. Of the Holy Eucharist")
                .page(252)
                .tags(["Various Occasions"]),
                preface: "Preface of the Epiphany".into(),
                rubric_before: Some("Especially suitable for Thursdays".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::HolyCross),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, whose beloved Son willingly endured the agony and shame of the cross for our redemption: Give us courage to take up our cross and follow him; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("6. Of the Holy Cross")
                .page(252)
                .tags(["Various Occasions"]),
                preface: "Preface of Holy Week".into(),
                rubric_before: Some("Especially suitable for Fridays".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::AllBaptizedChristians),
            CollectData {
                document: Document::from(
                    Text::from("Grant, Lord God, to all who have been baptized into the death and resurrection of your Son Jesus Christ, that, as we have put away the old life of sin, so we may be renewed in the spirit of our minds, and live in righteousness and true holiness; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("7. For all Baptized Christians")
                .page(253)
                .tags(["Various Occasions"]),
                preface: "Preface of Baptism".into(),
                rubric_before: Some("Especially suitable for Saturdays".into()),
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::TheDeparted),
            CollectData {
                document: Document::from(
                    Text::from("Eternal Lord God, you hold all souls in life: Give to your whole Church in paradise and on earth your light and your peace; and grant that we, following the good examples of those who have served you here and are now at rest, may at the last enter with them into your unending joy; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("8. For the Departed")
                .page(253)
                .tags(["Various Occasions"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::TheDeparted),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, we remember before you today your faithful servant N.; and we pray that, having opened to him the gates of larger life, you will receive him more and more into your joyful service, that, with all who have faithfully served you in the past, he may share in the eternal victory of Jesus Christ our Lord; who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.Any of the Collects appointed for use at the Burial of the Dead may be used instead.For the Prayers of the People, one of the forms appointed for the Burial of the Dead may be used.")
                        .response("Amen.")
                )
                .label("8. For the Departed")
                .page(253)
                .tags(["Various Occasions"]),
                preface: "Preface of the Commemoration of the Dead".into(),
                rubric_before: None,
                rubric_after: Some("The postcommunion prayer on page 498 may be used.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::ReignOfChrist),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting God, whose will it is to restore all things in your well-beloved Son, the King of kings and Lord of lords: Mercifully grant that the peoples of the earth,divided and enslaved by sin, may be freed and brought together under his most gracious rule; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("9. Of the Reign of Christ")
                .page(254)
                .tags(["Various Occasions"]),
                preface: "Preface of the Ascension, or of Baptism".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Baptism),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, by our baptism into the death and resurrection of your Son Jesus Christ, you turn us from the old life of sin: Grant that we, being reborn to new life in him, may live in righteousness and holiness all our days; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("10. At Baptism")
                .page(254)
                .tags(["Various Occasions"]),
                preface: "Preface of Baptism".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Confirmation),
            CollectData {
                document: Document::from(
                    Text::from("Grant, Almighty God, that we, who have been redeemed from the old life of sin by our baptism into the death and resurrection of your Son Jesus Christ, may be renewed in your Holy Spirit, and live in righteousness and true holiness; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("11. At Confirmation")
                .page(254)
                .tags(["Various Occasions"]),
                preface: "Preface of Baptism, or of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Dedication),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, to whose glory we celebrate the dedication of this house of prayer: We give you thanks for the fellowship of those who have worshiped in this place, and we pray that all who seek you here may find you, and be filled with your joy and peace; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.The Litany of Thanksgiving for a Church, page 578, may be used for the Prayers of the People.")
                        .response("Amen.")
                )
                .label("12. On the Anniversary of the Dedication of a Church")
                .page(255)
                .tags(["Various Occasions"]),
                preface: "Preface of the Dedication of a Church".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::ChurchConvention),
            CollectData {
                document: Document::from(
                    Text::from("Almighty and everlasting Father, you have given the Holy Spirit to abide with us for ever: Bless, we pray, with his grace and presence, the bishops and the other clergy and the laity here (or now, or soon to be) assembled in your Name, that your Church, being preserved in true faith and godly discipline, may fulfill all the mind of him who loved it and gave himself for it, your Son Jesus Christ our Savior; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("13. For a Church Convention")
                .page(255)
                .tags(["Various Occasions"]),
                preface: "Preface of Pentecost, or of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::UnityOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("Almighty Father, whose blessed Son before his passion prayed for his disciples that they might be one, as you and he are one: Grant that your Church, being bound together in love and obedience to you, may be united in one body by the one Spirit, that the world may believe in him whom you have sent, your Son Jesus Christ our Lord; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("14. For the Unity of the Church")
                .page(255)
                .tags(["Various Occasions"]),
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
                        Document::from(Text::from("Almighty God, the giver of all good gifts, in your divine providence you have appointed various orders in your Church: Give your grace, we humbly pray, to all who are [now] called to any office and ministry for your people; and so fill them with the truth of your doctrine and clothe them with holiness of life, that they may faithfully serve before you, to the glory of your great Name and for the benefit of your holy Church; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                            .response("Amen.")
                        )
                    ])
                )
                .label("15. For the Ministry (Ember Days)")
                .page(256)
                .tags(["Various Occasions"]),
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
                        Document::from(Text::from("O God, you led your holy apostles to ordain ministers in every place: Grant that your Church, under the guidance of the Holy Spirit, may choose suitable persons for the ministry of Word and Sacrament, and may uphold them in their work for the extension of your kingdom; through him who is the Shepherd and Bishop of our souls, Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                            .response("Amen.")
                        )
                    ])
                )
                .label("15. For the Ministry (Ember Days)")
                .page(256)
                .tags(["Various Occasions"]),
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
                        Document::from(Text::from("Almighty and everlasting God, by whose Spirit the whole body of your faithful people is governed and sanctified: Receive our supplications and prayers, which we offer before you for all members of your holy Church, that in their vocation and ministry they may truly and devoutly serve you; through our Lord and Savior Jesus Christ, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                            .response("Amen.")
                        )
                    ])
                )
                .label("15. For the Ministry (Ember Days)")
                .page(257)
                .tags(["Various Occasions"]),
                preface: "Preface of Baptism, or of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::MissionOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("O God, you have made of one blood all the peoples of the earth, and sent your blessed Son to preach peace to those who are far off and to those who are near: Grant that people everywhere may seek after you and find you, bring the nations into your fold, pour out your Spirit upon all flesh, and hasten the coming of your kingdom; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("16. For the Mission of the Church")
                .page(257)
                .tags(["Various Occasions"]),
                preface: "".into(),
                rubric_before: None,
                rubric_after: Some("or this".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::MissionOfTheChurch),
            CollectData {
                document: Document::from(
                    Text::from("O God of all the nations of the earth: Remember the multitudes who have been created in your image but have not known the redeeming work of our Savior Jesus Christ; and grant that, by the prayers and labors of your holy Church, they may be brought to know and worship you as you have been revealed in your Son; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("16. For the Mission of the Church")
                .page(257)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season, or of Pentecost".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Nation),
            CollectData {
                document: Document::from(
                    Text::from("Lord God Almighty, you have made all the peoples of the earth for your glory, to serve you in freedom and in peace: Give to the people of our country a zeal for justice and the strength of forbearance, that we may use our liberty in accordance with your gracious will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("17. For the Nation")
                .page(258)
                .tags(["Various Occasions"]),
                preface: "Preface of Trinity Sunday".into(),
                rubric_before: None,
                rubric_after: Some("The Collect for Independence Day may be used instead.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Peace),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, kindle, we pray, in every heart the true love of peace, and guide with your wisdom those who take counsel for the nations of the earth, that in tranquillity your dominion may increase until the earth is filled with the knowledge of your love; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and forever.")
                        .response("Amen.")
                )
                .label("18. For Peace")
                .page(258)
                .tags(["Various Occasions"]),
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
                            Text::from("Almighty God, Lord of heaven and earth: We humbly pray that your gracious providence may give and preserve to our use the harvests of the land and of the seas, and may prosper all who labor to gather them, that we, who are constantly receiving good things from your hand, may always give you thanks; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("19. For Rogation Days")
                .page(259)
                .tags(["Various Occasions"]),
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
                            Text::from("Almighty God, whose Son Jesus Christ in his earthly life shared our toil and hallowed our labor: Be present with your people where they work; make those who carry on the industries and commerce of this land responsive to your will; and give to us all a pride in what we do, and a just return for our labor; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and forever.")
                                .response("Amen.")
                        )
                    ])

                )
                .label("19. For Rogation Days")
                .page(259)
                .tags(["Various Occasions"]),
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
                            Text::from("O merciful Creator, your hand is open wide to satisfy the needs of every living creature: Make us always thankful for your loving providence; and grant that we, remembering the account that we must one day give, may be faithful stewards of your good gifts; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                                .response("Amen.")
                        )
                    ])
                )
                .label("19. For Rogation Days")
                .page(259)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Sick),
            CollectData {
                document: Document::from(
                    Text::from("Heavenly Father, giver of life and health: Comfort and relieve your sick servants, and give your power of healing to those who minister to their needs, that those (or N., or NN.) for whom our prayers are offered may be strengthened in their weakness and have confidence in your loving care; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("20. For the Sick")
                .page(260)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: Some("The postcommunion prayer on page 457 may be used.".into())
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::SocialJustice),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, who created us in your own image: Grant us grace fearlessly to contend against evil and to make no peace with oppression; and, that we may reverently use our freedom, help us to employ it in the maintenance of justice in our communities and among the nations, to the glory of your holy Name; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                        .response("Amen.")
                )
                .label("21. For Social Justice")
                .page(260)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::SocialService),
            CollectData {
                document: Document::from(
                    Text::from("Heavenly Father, whose blessed Son came not to be served but to serve: Bless all who, following in his steps, give themselves to the service of others; that with wisdom, patience, and courage, they may minister in his Name to the suffering, the friendless, and the needy- for the love of him who laid down his life for us, your Son our Savior Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("22. For Social Service")
                .page(260)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Education),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, the fountain of all wisdom: Enlighten by your Holy Spirit those who teach and those who learn, that, rejoicing in the knowledge of your truth, they may worship you and serve you from generation to generation; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("23. For Education")
                .page(261)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::Vocation),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God our heavenly Father, you declare your glory and show forth your handiwork in the heavens and in the earth: Deliver us in our various occupations from the service of self alone, that we may do the work you give us to do in truth and beauty and for the common good; for the sake of him who came among us as one who serves, your Son Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("24. For Vocation in Daily Work")
                .page(261)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),
        (
            CollectId::VariousOccasions(VariousOccasions::LaborDay),
            CollectData {
                document: Document::from(
                    Text::from("Almighty God, you have so linked our lives one with another that all we do affects, for good or ill, all other lives: So guide us in the work we do, that we may do it not for self alone, but for the common good; and, as we seek a proper return for our own labor, make us mindful of the rightful aspirations of other workers, and arouse our concern for those who are out of work; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                        .response("Amen.")
                )
                .label("25. For Labor Day")
                .page(261)
                .tags(["Various Occasions"]),
                preface: "Preface of the Season".into(),
                rubric_before: None,
                rubric_after: None
            }
        ),

    ];
}
