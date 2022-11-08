use calendar::Feast;
use liturgy::{Document, Reference, Source, Text, Version};

use crate::{CollectData, CollectId};

lazy_static! {
    pub static ref LFF_COLLECTS_CONTEMPORARY: [(CollectId, CollectData); 265] = [(
        CollectId::Feast(Feast::RichardOfChichester,),
        CollectData::from(
            Document::from(
                Text::from("Almighty and most merciful God, who calls your people to yourself, we pray that, following the example of your bishop Richard of Chichester, we may see your Son Jesus Christ more clearly, love him more dearly, and follow him more nearly; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Richard of Chichester")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
        )
    ),
        (
        CollectId::Feast(Feast::RichardOfChichester),
        CollectData::from(
            Document::from(
                Text::from("Almighty and most merciful God, who calls your people to yourself, we pray that, following the example of your bishop Richard of Chichester, we may see your Son Jesus Christ more clearly, love him more dearly, and follow him more nearly; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Richard of Chichester")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::DavidOfWales),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called your servant David to be a faithful and wise steward of your mysteries for the people of Wales: Mercifully grant that, following his purity of life and zeal for the Gospel of Christ, we may, with him, praise you both here on earth and also in your everlasting kingdom; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("David of Wales")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfJapan),
        CollectData::from(
            Document::from(
                Text::from("O God our Father, who brought the holy martyrs of Japan through the suffering of the cross to the joys of eternal life: Grant that we, encouraged by their example, may hold fast to the faith we profess, even unto death itself; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of Japan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Stephen),
        CollectData::from(
            Document::from(
                Text::from("We give you thanks, O Lord of glory, for the example of the first martyr Stephen, who looked up to heaven and prayed for his persecutors to your Son Jesus Christ, who stands at your right hand; where he lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnAndCharlesWesley),
        CollectData::from(
            Document::from(
                Text::from("Lord God, you inspired your servants Johnand Charles Wesley with burning zeal for the sanctification of souls and endowed them with eloquence in speech and song: Kindle such fervor in your church, we entreat you, that those whose faith has cooled may be warmed, and those who have not known Christ may turn to him and be saved; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("John and Charles Wesley")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PhillipsBrooks),
        CollectData::from(
            Document::from(
                Text::from("Everlasting God, who implants your living Word in the minds and on the lips of all who proclaim your truth: Grant that we, like your pastor and preacher Phillips Brooks, might proclaim your Gospel in our own generation with grace and power.  Through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, ever one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Phillips Brooks")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JananiLuwum),
        CollectData::from(
            Document::from(
                Text::from("O God, whose Son the Good Shepherd laid down his life for his sheep: We give you thanks for your faithful shepherd, Janani Luwum, who after his Savior’s example gave up his life for the sake of his flock. Grant us to be so inspired by his witness that we make no peace with oppression, but live as those who are sealed with the cross of Christ, who died and rose again, and now lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Janani Luwum")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnMasonNeale),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that in all time of our testing we may know your presence and obey your will; that, following the example of your servant John Mason Neale, we may with integrity and courage accomplish what you give us to do, and endure what you give us to bear; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Mason Neale")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HughOfLincoln),
        CollectData::from(
            Document::from(
                Text::from("Holy God, who endowed your servant Hugh of Lincoln with wise and cheerful boldness, and taught him to commend the discipline of holy life to kings and princes: Grant that we also, rejoicing in the Good News of your mercy, and fearing nothing but the loss of you, may be bold to speak the truth in love, in the name of Jesus Christ our Redeemer; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Hugh of Lincoln")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PhilanderChase),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose Son Jesus Christ is the pioneer and perfecter of our faith: Grant that like your servant Philander Chase we might have the grace to minister in Christ’s name in every place, led by bold witnesses to the Gospel of the Prince of Peace, Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Philander Chase")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PeterAndPaul),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose blessed apostles Peter and Paul glorified you by their martyrdom: Grant that your Church, instructed by their teaching and example, and knit together in unity by your Spirit, may ever stand firm upon the one foundation, which is Jesus Christ our Lord; who lives and reigns with you, in the unity of the same Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Saint Peter and Saint Paul")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Dunstan),
        CollectData::from(
            Document::from(
                Text::from("Direct your Church, O Lord, into the beauty of holiness, that, following the good example of your servant Dunstan, we may honor your Son Jesus Christ with our lips and in our lives; to the glory of his Name, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Dunstan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfUganda),
        CollectData::from(
            Document::from(
                Text::from("O God, by whose providence the blood of the martyrs is the seed of the church: Grant that we who remember before you the blessed martyrs of Uganda, may, like them, be steadfast in our faith in Jesus Christ, to whom they gave obedience even to death, and by their sacrifice brought forth a plentiful harvest; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of Uganda")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FrancisDeSalesJaneDeChantal),
        CollectData::from(
            Document::from(
                Text::from("Most Gracious God, who has bidden us to act justly, love mercy, and walk humbly before you; Grant that we, like your servants Francis and Jane, may see and serve Christ in all people, and know him as the giver of all good things; Through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Francis de Sales")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AugustineOfCanterbury),
        CollectData::from(
            Document::from(
                Text::from("O Lord our God, who by your Son Jesus Christ called your servant Augustine to preach the Gospel to the English people: We pray that all whom you call and send may do your will, bide your time, and see your glory; through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Augustine")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThePresentation),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everliving God, we humbly pray that, as your only-begotten Son was this day presented in the temple, so we may be presented to you with pure and clean hearts by Jesus Christ our Lord; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AnnaEllisonButlerAlexander),
        CollectData::from(
            Document::from(
                Text::from("Loving God, who called Anna Alexander as a deaconess in your church: Grant us the wisdom to teach the gospel of Christ to whomever we meet, by word and by example, that all may come to the enlightenment that you intend for your people; through Jesus Christ, our Teacher and Savior.")
                    .response("Amen.")
                )
                .label("Anna Ellison Butler Alexander")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryOfNyssa),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who has revealed to your Church your eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace that, like your bishop Gregory of Nyssa, we may continue steadfast in the confession of this faith, and constant in our worship of you, Father, Son and Holy Spirit; who live and reign for ever and ever.")
                    .response("Amen.")
                )
                .label("Gregory of Nyssa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Boniface),
        CollectData::from(
            Document::from(
                Text::from("Pour out your Holy Spirit, O God, upon your church in every land, that like your servant Boniface we might proclaim the Gospel to all nations, that your kingdom might be enlarged and that your holy Name might be glorified in all the world; through Jesus Christ our Lord, who lives and reigns with you and the same Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Boniface")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AmbroseOfMilan),
        CollectData::from(
            Document::from(
                Text::from("O God, who gave your servant Ambrose grace eloquently to proclaim your righteousness in the great congregation and fearlessly to bear reproach for the honor of your Name: Mercifully grant to all bishops and pastors such excellence in preaching and faithfulness in ministering your Word, that your people may be partakers with them of the glory that shall be revealed; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Ambrose of Milan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Bede),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who has enriched your church with the learning and holiness of your servant Bede: Grant us to find in Scripture and disciplined prayer the image of your Son our Savior Jesus Christ, and to fashion our lives according to his likeness, to the glory of your great Name and to the benefit of your holy church; through the same Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Bede")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::LancelotAndrewes),
        CollectData::from(
            Document::from(
                Text::from("Perfect in us, Almighty God, whatever is lacking of your gifts: of faith, to increase it; of hope, to establish it; of love, to kindle it; that like your servant Lancelot Andrewes we may live in the life of your grace and glory; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Lancelot Andrewes")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::KamehamehaAndEmma),
        CollectData::from(
            Document::from(
                Text::from("O God, who called your servants Kamehameha and Emma toan earthly throne that they might advance your heavenly kingdom, and gave them zeal for your church and love for your people: Mercifully grant that we also may be fruitful in good works, and attain to the glorious crown of your saints; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Kamehameha and Emma")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ElisabethCruciger),
        CollectData::from(
            Document::from(
                Text::from("Pour out your Spirit upon all of your sons and daughters, Almighty God, that like your servant Elisabeth Cruciger our lips may praise you, our lives may bless you, and our worship may give you glory; through Jesus Christ our Lord. ")
                    .response("Amen.")
                )
                .label("Elisabeth Cruciger")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HildegardOfBingen),
        CollectData::from(
            Document::from(
                Text::from("God of all times and seasons: Give us grace that we, after the example of your servant Hildegard, may both know and make known the joy and jubilation of being part of your creation, and show forth your glory in the world; through Jesus Christ our Savior, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Hildegard of Bingen")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ZenaidaPhilonellaHermione),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, whose most dear Son came to heal the sick, raise the dead, cast out demons, and preach good news to the poor: Lead us by the example of your servants, Zenaida, Philonella, and Hermione, to freely give even as we have freely received; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Zenaida")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PauliMurray),
        CollectData::from(
            Document::from(
                Text::from("Liberating God, we thank you for the steadfast courage of your servant Pauli Murray, who fought long and well: Unshackle us from the chains of prejudice and fear, that we may show forth the reconciling love and true freedom which you revealed in your Son our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Pauli Murray")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Michael),
        CollectData::from(
            Document::from(
                Text::from("Everlasting God, who has ordained and constituted ina wonderful order the ministries of angels and mortals: Mercifully grant that, as your holy angels always serve and worship you in heaven, so by your appointment they may help and defend us here on earth; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BenedictOfNursia),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, whose service is perfect freedom and in whose commandments there is nothing harsh nor burdensome: Grant that we, with your servant Benedict, may listen with attentive minds, pray with fervent hearts, and serve you with willing hands, so that we might live at peace with one another and in obedience to your Word, Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, now and for ever. ")
                    .response("Amen.")
                )
                .label("Benedict of Nursia")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::NicholasFerrar),
        CollectData::from(
            Document::from(
                Text::from("Lord God, make us worthy of your perfect love; that, with your deacon Nicholas Ferrar and his household, we may rule ourselves according to your Word, and serve you with our whole heart; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Nicholas Ferrar")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::VincentOfSaragossa),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose deacon Vincent, upheld by you, was neither terrified by threats nor overcome by torments: Strengthen us to endure all adversity with invincible and steadfast faith; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Vincent of Saragossa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ScholasticaOfNursia),
        CollectData::from(
            Document::from(
                Text::from("Assist us, O God, to love one other as sisters and brothers, and to balance discipline with love and rules with compassion, according to the example shown by your servant Scholastica; for the sake of your Son Jesus Christ our Lord, to whom with you and the Holy Spirit be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Scholastica of Nursia")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::LatimerRidleyCranmer),
        CollectData::from(
            Document::from(
                Text::from("Keep us, O Lord, constant in faith and zealous in witness, that, like your servants Hugh Latimer, Nicholas Ridley, and Thomas Cranmer we may live in your fear, die in your favor, and rest in your peace; for the sake of Jesus Christ, your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Hugh Latimer and Nicholas Ridley")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MacrinaOfCaesarea),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who called your servant Macrina to reveal in her life and teaching the riches of your grace and truth: Grant that we, following her example, may seek after your wisdom and live according to the way of your Son our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Macrina of Caesarea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::GeorgeHerbert),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called your servant George Herbert from the pursuit of worldly honors to be a poet and a pastor of souls: Give us grace, we pray, joyfully to dedicate all our powers to your service; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("George Herbert")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TabithaDorcasOfJoppa),
        CollectData::from(
            Document::from(
                Text::from("Most Holy God, who did raise from the dead your servant Tabitha to display your power and confirm that your Son is Lord; Grant unto us your grace, that, aided by her prayers and example, we may be given a new life in you, to do works pleasing in your sight; through Jesus Christ your Son our Lord; who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Tabitha (Dorcas) of Joppa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FlorenceLiTimOi),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who pours out your Spirit upon your sons and daughters: Grant that we, following the example of your servant Florence Li Tim-Oi, chosen priest in your church, may with faithfulness, patience, and tenacity proclaim your holy gospel to all the nations, through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Florence Li Tim-Oi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnChrysostom),
        CollectData::from(
            Document::from(
                Text::from("O God, who gave your servant John Chrysostom grace eloquently to proclaim your righteousness in the great congregation, and fearlessly to bear reproach for the honor of your Name: Mercifully grant to all who proclaim your word such excellence in preaching, that all your people may be made partakers of the glory that shall be revealed; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("John Chrysostom")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AnnaJuliaHaywoodCooper),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who inspired your servant Anna Julia Haywood Cooper with the love of learning and the skill of teaching: Enlighten us more and more through the discipline of learning, and deepen our commitment to the education of all your children; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Anna Julia Haywood Cooper")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::DorothyLSayers),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who strengthened your servant Dorothy Sayers with eloquence to defend Christian teaching: Keep us, we pray, steadfast in your true religion, that in constancy and peace we may always teach right doctrine, and teach doctrine rightly; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Dorothy L Sayers")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryTheGreat),
        CollectData::from(
            Document::from(
                Text::from("Almighty and merciful God, you raised up Gregory of Rome to be a servant of the servants of God, and inspired him to send missionaries to preach the Gospel to the English people: Preserve your church in the catholic and apostolic faith, that your people, being fruitful in every good work, may receive the crown of glory that never fades away; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Gregory the Great")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamPorcherDubose),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, you gave to your servant William Porcher DuBose special gifts of grace to understand the Scriptures and to teach the truth as it is in Christ Jesus: Grant that by this teaching we may know you, the one true God, and Jesus Christ whom you have sent; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("William Porcher Dubose")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ChanningMooreWilliams),
        CollectData::from(
            Document::from(
                Text::from("O God, who in your providence called Channing Moore Williams to the ministry of this church and gave him the gifts and the perseverance to preach the Gospel in new lands: Inspire us, by his example and prayers, to commit our talents to your service, confident that you uphold those whom you call; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Channing Moore Williams")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ChadOfLichfield),
        CollectData::from(
            Document::from(
                Text::from("Heavenly Father, whose son our Lord Jesus Christ took the form of a servant for the sake of his brothers and sisters: Strengthen us with the prayers and example of your servant Chad, who became the least of all to minister to all; through the same Christ our Lord, who lives and reigns, with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Chad of Lichfield")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EphremOfNisibis),
        CollectData::from(
            Document::from(
                Text::from("Pour out upon us, O Lord, that same Spirit by which your deacon Ephrem declared the mysteries of faith in sacred song; that, with gladdened hearts, we too might proclaim the riches of your glory; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Ephrem of Nisibis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheclaOfIconium),
        CollectData::from(
            Document::from(
                Text::from("God of liberating power, who called Thecla to proclaim the gospel and did not permit any obstacle or peril to inhibit her: Empower courageous evangelists among us, that men and women everywhere may know the freedom that you offer us in Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thecla of Iconium")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MartinLutherKing),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by the hand of Moses your servant you led your people out of slavery, and made them free at last: Grant that your church, following the example of your prophet Martin Luther King, may resist oppression in the name of your love, and may strive to secure for all your children the blessed liberty of the Gospel of Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Martin Luther King")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CatherineOfGenoa),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, reveal to your church the depths of your love; that, like your servant Catherine of Genoa, we might give ourselves in loving service, knowing that we have been perfectly loved by you; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Catherine of Genoa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PhilipAndJames),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave to your apostles Philip and James grace and strength to bear witness to the truth: Grant that we, being mindful of their victory of faith, may glorify in life and death the Name of our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnOfDamascus),
        CollectData::from(
            Document::from(
                Text::from("Confirm our minds, O Lord, in the mysteries ofthe true faith, set forth with power by your servant John of Damascus; that we, with him, confessing Jesus to be true God and true Man and singing the praises of the risen Lord, may, by the power of the resurrection, attain to eternal joy; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("John of Damascus")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnRaleighMott),
        CollectData::from(
            Document::from(
                Text::from("Everlasting God, who leads your people's feet into the ways of peace; Raise up heralds and evangelists of your kingdom like your servant John Mott, that your church may make known to all the world the unsearcheable riches and unsurpassed peace of your Son, Jesus Christ our Lord; to whom with you and the Holy Spirit be all honor and glory, now and for ever. ")
                    .response("Amen.")
                )
                .label("John Raleigh Mott")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TitusAndTimothy),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called Timothy and Titus to be evangelists and teachers, and made them strong to endure hardship: Strengthen us to stand fast in adversity, and to live godly and righteous lives in this present time, that with sure confidence we may look for our blessed hope, the glorious appearing of our great God and Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Titus and Timothy")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::KatharinaVonBora),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called servant Katharina von Bora from a cloister to work for the reform of your church: Grant that, for the sake of your glory and the welfare of your church, we may go wherever you should call, and serve however you should will; through Jesus Christ, our only mediator and advocate.")
                    .response("Amen.")
                )
                .label("Katharina Von Bora")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Alban),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose grace and power your holy martyr Alban triumphed over suffering and was faithful even unto death: Grant us, who now remember him in thanksgiving, to be so faithfulin our witness to you in this world that we may receive with him the crown of life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Alban")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Matthias),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who in the place of Judas chose your faithful servant Matthias to be numbered among the Twelve: Grant that your Church, being delivered from false apostles, may always be guided and governed by faithful and true pastors; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamAugustusMuhlenberg),
        CollectData::from(
            Document::from(
                Text::from("Open the eyes of your church, O Lord, to the plight of the poor and neglected, the homeless and destitute,the old and the sick, the lonely and those who have none to care for them. Give to us the vision and compassion with which you so richly endowed your servant William Augustus Muhlenberg, that we may labor tirelessly to heal those who are broken in body or spirit, and to turn their sorrow into joy; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Augustus Muhlenberg")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MechthildeAndGertude),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave to your servants Mechthilde and Gertrude special gifts of grace to understand and teach the truth in Christ Jesus: Grant that by their teachings we may know you, the one true God, and Jesus Christ your Son; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Mechthilde of Hackeborn and Gertrude the Great")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ColumbaOfIona),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the preaching of your servant Columba caused the light of the Gospel to shine in Scotland: Grant, we pray, that, remembering his life and labors, we may follow the example of his zeal and patience; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Columba of Iona")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheVisitation),
        CollectData::from(
            Document::from(
                Text::from("Father in heaven, by your grace the virgin mother of your incarnate Son was blessed in bearing him, but still more blessed in keeping your word: Grant us who honor the exaltation of her lowliness to follow the example of her devotion to your will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnDonne),
        CollectData::from(
            Document::from(
                Text::from("O God of eternal glory, whom no one living can see and yet whom to see is to live; grant that with your servant John Donne, we may see your glory in the face of your Son, Jesus Christ, and then, with all our skill and wit, offer you our crown of prayer and praise, until by his grace we stand in that last and everlasting day, when death itself will die, and all will live in you, who with the Holy Spirit and the same Lord Jesus Christ are one God in everlasting light and glory. ")
                    .response("Amen.")
                )
                .label("John Donne")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FrancesPerkins),
        CollectData::from(
            Document::from(
                Text::from("Loving God, we bless your Name for Frances Perkins, who in faithfulness to her baptism envisioned a society in which all might live in health and decency: Help us, following her example andin union with her prayers, to contend tirelessly for justice and for the protection of all, that we may be faithful followers of Jesus Christ; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Frances Perkins")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FrancisOfAssisi),
        CollectData::from(
            Document::from(
                Text::from("Most high, omnipotent, good Lord, grant your people grace to renounce gladly the vanities of this world; that, following the way of blessed Francis, we may, for love of you, delight in your whole creation with perfectness of joy; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Francis of Assisi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::SimonAndJude),
        CollectData::from(
            Document::from(
                Text::from("O God, we thank you for the glorious company of the apostles, and especially on this day for Simon and Jude; and we pray that, as they were faithful and zealous in their mission, so we may with ardent devotion make known the love and mercy of our Lord and Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HolyInnocents),
        CollectData::from(
            Document::from(
                Text::from("We remember today, O God, the slaughter of the holy innocents of Bethlehem by King Herod. Receive, we pray, into the arms of your mercy all innocent victims; and by your great might frustrate the designs of evil tyrants and establish your rule of justice, love, and peace; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Holy Innocents")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ClementOfAlexandria),
        CollectData::from(
            Document::from(
                Text::from("O God of unsearchable wisdom, you gave your servant Clement grace to understand and teach the truth as it is in Jesus Christ, the source of all truth: Grant to your church the same grace to discern your Word wherever truth is found; through Jesus Christ our unfailing light, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Clement of Alexandria")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::July4),
        CollectData::from(
            Document::from(
                Text::from("Lord God Almighty, in whose Name the founders of this country won liberty for themselves and for us, and lit the torch of freedom for nations then unborn: Grant that we and all the people of this land may have grace to maintain our liberties in righteousness and peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Independence Day (United States)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnXxiiiAngeloGiuseppeRoncalli),
        CollectData::from(
            Document::from(
                Text::from("God of all truth and peace, who raised up your bishop John to be servant of the servants of God and bestowed on him wisdom to call for the work of renewing your church: Grant that, following his example, we may reach out to other Christians in the love of your Son, and labor throughout the nations of the world to kindle a desire for justice and peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("John XXIII (Angelo Giuseppe Roncalli)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Alphege),
        CollectData::from(
            Document::from(
                Text::from("Lord Jesus Christ, who willingly walked the way of the cross: Strengthen your church through the example and prayers of your servant Alphege to hold fast to the path of discipleship; for with the Father and Holy Spirit you live and reign, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Alphege")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FrederickDenisonMaurice),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who has restored our human nature to heavenly glory through the perfect obedience of our Savior Jesus Christ: Enliven in your Church, we pray, the passion for justice and truth, that, like your servant Frederick Denison Maurice, we may work and pray for the triumph of the kingdom of Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever,")
                    .response("Amen.")
                )
                .label("Frederick Denison Maurice")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Theodora),
        CollectData::from(
            Document::from(
                Text::from("O God, who called your servant Theodora to an earthly throne that she might advance your heavenly kingdom and who gave her the wisdom to establish unity where there had been division; Create in your church such godly union and concord that we might proclaim the Gospel of the Prince of Peace, not only in correct theology but in right actions; through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Theodora")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AlcuinOfYork),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who raised up your servant Alcuin as a beacon of learning: Shine in our hearts, we pray that we may also show forth your praise in our own generation, for you have called us out of darkness and into your marvelous light; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Alcuin of York")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MechthildOfMagdeburg),
        CollectData::from(
            Document::from(
                Text::from("Draw the souls of your people into your love, O God, that like your servant Mechthild, we may yearn to be fully yours, for you know us better than we can know ourselves; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God now and for ever.")
                    .response("Amen.")
                )
                .label("Mechthild of Magdeburg")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ClementOfRome),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who chose your servant Clement of Rome to recall the church in Corinth to obedience and stability: Grant that your church may be grounded and settled in your truth by the indwelling of the Holy Spirit; reveal to it what is not yet known; fill up what is lacking; confirm what has already been revealed; and keep it blameless in your service; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Clement of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HolyCross),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose Son our Savior Jesus Christ was lifted high upon the cross that he might draw the whole world to himself: Mercifully grant that we, who glory in the mystery of our redemption, may have grace to take up our cross and follow him; who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Bakhita),
        CollectData::from(
            Document::from(
                Text::from("O God of Love, who delivered your servant Josephine Margaret Bakhita from the bondage of slavery to the true freedom of your service; Grant to the wounded your healing grace in mind, body, and spirit and to your church the zeal to combat exploitation and slavery in all its forms; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Bakhita (Josephine Margaret Bakhita)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::RobertGrosseteste),
        CollectData::from(
            Document::from(
                Text::from("O God, our heavenly Father, who raised up your faithful servant Robert Grosseteste to be a bishop and pastor in your church and to feed your flock: Give abundantly to all pastors the gifts of your Holy Spirit, that they may minister in your household as true servants of Christ and stewards of your divine mysteries; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Robert Grosseteste")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HarrietStarrCannon),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, who called Harriet Starr Cannon and her companions to revive the monastic vocation in the Episcopal Church and to dedicate their lives to you: Grant that we, after their example, may ever surrender ourselves to the revelation of your holy will; through our Lord and Savior Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Harriet Starr Cannon")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BrigidOfKildare),
        CollectData::from(
            Document::from(
                Text::from("O God, whose servant Brigid, kindled with the flame of your love, became a shining light in your church: Grant that we also may be aflame with the spirit of love and discipline, and walk before you as children of light; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Brigid of Kildare")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::RichardHooker),
        CollectData::from(
            Document::from(
                Text::from("O God of truth and peace, you raised up your servant Richard Hooker in a day of bitter controversy to defend with sound reasoning and great charity the catholic and reformed religion: Grant that we may maintain that middle way, not as a compromise for the sake of peace, but as a comprehension for the sake of truth; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Richard Hooker")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AgnesAndCeciliaOfRome),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who chooses those whom the world deems powerless to put the powerful to shame: Grant us so to cherish the memory of your youthful martyrs Agnes and Cecilia, that we might share their pure and steadfast faith in you; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("Agnes and Cecilia of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EvaLeeMatthews),
        CollectData::from(
            Document::from(
                Text::from("O God, whose blessed Son became poor that we through his poverty might be rich: Deliver us from an inordinate love of this world, that, inspired by the devotion of your servant Eva Lee Matthews, we may serve you with singleness of heart, and attain to the riches of the age to come; through the same Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Eva Lee Matthews")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EdwardBouveriePusey),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that in all time of our testing we may know your presence and obey your will; that, following the example of your servant Edward Bouverie Pusey, we may with integrity and courage accomplish what you give us to do, and endure what you give us to bear; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Edward Bouverie Pusey")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::VincentDePaul),
        CollectData::from(
            Document::from(
                Text::from("Most Gracious God, who has bidden us to act justly, love mercy, and walk humbly before you; Teach us, like your servants Vincent de Paul and Louise de Marillac, to see and to serve Christ by feeding the hungry, welcoming the stranger, clothing the naked, and caring for the sick; that we may know him to be the giver of all good things, through the same Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Vincent de Paul")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Justin),
        CollectData::from(
            Document::from(
                Text::from("O God, who has given your church wisdom and revealed to it deep and secret things: Grant that we, like your servant Justin and in union with his prayers, may find your Word an abiding refuge all the days of our lives; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Justin")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CyprianOfCarthage),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave to your servant Cyprian boldness to confess the Name of our Savior Jesus Christ before the rulers of this world and courage to die for this faith: Grant that we may always be ready to give a reason for the hope that is in us and to suffer gladly for the sake of our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Cyprian of Carthage")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AugustineOfHippo),
        CollectData::from(
            Document::from(
                Text::from("Lord God, the light of the minds that know you, the life of the souls that love you, and the strength of the hearts that serve you: Help us, following the example of your servant, Augustine of Hippo, so to know you that we may truly love you, and soto love you that we may fully serve you, whose service is perfect freedom; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Augustine of Hippo")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ArgulaVonGrumbach),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave your servant Argula von Grumbach a spirit of wisdom and power to love your Word and to boldly draw others to its truth: Pour out that same spirit upon us, that we, knowing and loving your Holy Word, may be unashamed of Christ and may not sin against the Holy Spirit that is within us.")
                    .response("Amen.")
                )
                .label("Argula Von Grumbach")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::GeorgeAugustusSelwyn),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, whose servant George Augustus Selwyn laid a firm foundation for the growth of your church in many nations: Raise up in this and every land evangelists and heralds of your kingdom, that your church may proclaim the unsearchable riches of our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("George Augustus Selwyn")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasGallaudetAndHenryWinterSyle),
        CollectData::from(
            Document::from(
                Text::from("O Loving God, whose will it is that everyone should come to you and be saved: We bless your holy Name for your servants Thomas Gallaudet and Henry Winter Syle, and we pray that you will continually move your church to respond in love to the needs of all people; through Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Thomas Gallaudet and Henry Winter Syle")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Fabian),
        CollectData::from(
            Document::from(
                Text::from("Grant, Almighty God, that in all times of trial and persecution, we might remain steadfast in faith and endurance, according to the example of your servant Fabian, who was faithful even unto death. We ask this for the sake of him who laid down his life for us all, Jesus Christ our Savior; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Fabian")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::SamuelIsaacJosephScherechewsky),
        CollectData::from(
            Document::from(
                Text::from("O God, who in your providence called Joseph Schereschewsky to the ministry of this church and gave him the gifts and the perseverance to translate the Holy Scriptures: Inspire us, by his example and prayers, to commit our talents to your service, confident that you uphold those whom you call; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Samuel Isaac Joseph Scherechewsky")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::StantonBloomerTruthTubman),
        CollectData::from(
            Document::from(
                Text::from("O God, whose Spirit guides us into all truth and makes us free: Strengthen and sustain us as you did your servants Elizabeth, Amelia, Sojourner, and Harriet. Give us vision and courage to stand against oppression and injustice and all that works against the glorious liberty to which you call all your children; through Jesus Christ our Savior, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Elizabeth Cady Stanton")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesSolomonRussell),
        CollectData::from(
            Document::from(
                Text::from("O God, the font of resurrected life, draw us into the wilderness and speak tenderly to us, so that we might love and worship you as your servant James Solomon Russell did, in assurance of the saving grace of Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Solomon Russell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamLaud),
        CollectData::from(
            Document::from(
                Text::from("Keep us, O Lord, constant in faith and zealous in witness; that, like your servant William Laud, we may live in your fear, die in your favor, and rest in your peace; for the sake of Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Laud")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamWilberforce),
        CollectData::from(
            Document::from(
                Text::from("Let your continual mercy, O Lord, kindle in your Church the never-failing gift of love; that, following the example of your servant William Wilberforce, we may have grace to defend the poor, and maintain the cause of those who have no helper; for the sake of him who gave his life for us, your Son our Savior Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("William Wilberforce")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MaryAndMarthaOfBethany),
        CollectData::from(
            Document::from(
                Text::from("O God, heavenly Father, your Son Jesus Christ enjoyed rest and refreshment in the home of Mary and Martha of Bethany: Give us the will to love you, open our hearts to hear you, and strengthen our hands to serve you in others for his sake; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Mary and Martha of Bethany")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasBray),
        CollectData::from(
            Document::from(
                Text::from("O God of compassion, who opened the heart of your servant Thomas Bray to answer the needs of the church in the New World: Make your church diligent at all times to propagate the Gospel, and to promote the spread of Christian knowledge; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Bray")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JuliaChesterEmery),
        CollectData::from(
            Document::from(
                Text::from("God of all creation, who calls us to make disciples of all nations and to proclaim your mercy and love: Grant that we, after the example of your servant Julia Chester Emery, might have vision and courage in proclaiming the Gospel to the ends of the earth; through Jesus Christ, our light and our salvation, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Julia Chester Emery")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MaryMagdalene),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose blessed Son restored Mary Magdalene to health of body and of mind, and called her to be a witness of his resurrection: Mercifully grant that by your grace we may be healed from all our infirmities and know you in the power of his unending life; Through the same Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AbsalomJones),
        CollectData::from(
            Document::from(
                Text::from("Set us free, heavenly Father, from every bond of prejudice and fear; that, honoring the steadfast courage of your servant Absalom Jones, we may show forth in our lives the reconciling love and true freedom of the children of God, which you have given us in your Son our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Absalom Jones")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AidanOfLindisfarne),
        CollectData::from(
            Document::from(
                Text::from("O loving God, you called your servant Aidan from the cloister to re-establish the Christian mission in northern England: Grant that we, following his example, may use what you have given us for the relief of human need, and may persevere in commending the saving Gospel of our Redeemer Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Aidan of Lindisfarne")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MaryOfEgypt),
        CollectData::from(
            Document::from(
                Text::from("Merciful Lord, who raises up sinners by your boundless compassion and mercy: Cause the desert sun to burn away our coarseness and to melt our hardness of heart, that, like your servant Mary of Egypt, we may not depart from this life until we understand the ways of repentance and the benefits of prayer; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Mary of Egypt")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ToyohikoKagawa),
        CollectData::from(
            Document::from(
                Text::from("Strengthen and protect, O God, all those who suffer for their fidelity to Jesus Christ; that, like your servant Toyohiko Kawaga, they might persevere in seeking and serving Christ in all persons, and work tirelessly for the advancement of your kingdom; through the same Jesus Christ our Lord, to whom with you and the Holy Spirit be all honor and glory now and for ever.")
                    .response("Amen.")
                )
                .label("Toyohiko Kagawa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::James),
        CollectData::from(
            Document::from(
                Text::from("O Gracious God, we remember before you today your servant and apostle James, first among the Twelve to suffer martyrdom for the Name of Jesus Christ; and we pray that you will pour out upon the leaders of your church that spirit of self-denying service by which alone they may have true authority among your people; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamLaw),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose servant William Law taught us to hear and follow your call to a devout and holy life: Grant that we, loving you above all things and in all things, may seek your purpose and shape our actions to your will, that we may grow in all virtue and be diligent in prayer all the days of our lives, through Jesus Christ our Lord, to whom with you and the Holy Spirit be all honor and glory now and for ever. ")
                    .response("Amen.")
                )
                .label("William Law")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BasilOfCaesarea),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who has revealed to your church your eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace that, like your bishop Basil of Caesarea, we may continue steadfastly in the confession of this faith and remain constant in our worship of you, Father, Son, and Holy Spirit; every one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Basil of Caesarea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::LucyOfSyracuse),
        CollectData::from(
            Document::from(
                Text::from("Loving God, for the salvation of all you gave Jesus Christ as light to a world in darkness: Illumine us, as you did your daughter Lucy, with the light of Christ, that by the merits of his passion, we may be led to eternal life; through the same Jesus Christ, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Lucy of Syracuse")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Mary),
        CollectData::from(
            Document::from(
                Text::from("O God, you have taken to yourself the blessed Virgin Mary, mother of your incarnate Son: Grant that we, who have been redeemed by his blood, may share with her the glory of your eternal kingdom; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::VidaDuttonScudder),
        CollectData::from(
            Document::from(
                Text::from("Most gracious God, you sent your beloved Son to preach peace to those who are far off and to those who are near: Raise up in your church witnesses who, after the example of your servant Vida Dutton Scudder, stand firm in proclaiming the power of the gospel of Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Vida Dutton Scudder")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CatherineBarbaraMargaret),
        CollectData::from(
            Document::from(
                Text::from("Embolden your church, O God, with the stories of your saints Catherine, Barbara, and Margaret, that we might face all trials and adversities with a fearless mind and an unbroken spirit, knowing that we are more than conquerors through Jesus Christ who strengthens us.  Through the same Jesus Christ our Lord,")
                    .response("Amen.")
                )
                .label("Catherine of Alexandria")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Mark),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by the hand of Mark the evangelist you have given to your church the Gospel of Jesus Christ the Son of God: We thank you for this witness, and pray that we may be firmly grounded in its truth; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Saint Mark")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfMemphis),
        CollectData::from(
            Document::from(
                Text::from("We give you thanks and praise, O God of compassion, for the heroic witness of the Martyrs of Memphis, who, in a time of plague and pestilence, were steadfast in their care for the sick and dying, and loved not their own lives, even unto death; Inspire in us a like love and commitment to those in need, following the example of our Savior Jesus Christ; who with you and the Holy Spirit lives and reigns, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of Memphis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamTemple),
        CollectData::from(
            Document::from(
                Text::from("O God of light and love, you illumined your church through the witness of your servant William Temple: Inspire us, we pray, by his teaching and example, that we may rejoice with courage, confidence, and faith in the Word made flesh, and may be led to establish that city which has justice for its foundation and love for its law; through Jesus Christ, the light of the world, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("William Temple")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesLloydBreck),
        CollectData::from(
            Document::from(
                Text::from("O God, who sent your Son to preach peace to those who are far off and to those who are near: call us from comfortable complacency to preach, teach, and plant your church on new frontiers, after the example of your servant James Lloyd Breck; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Lloyd Breck")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnColeridgePatteson),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called your faithful servant John Coleridge Patteson and his companions to witness to the gospel, and by their labors and sufferings raised up a people for your own possession: Pour out your Holy Spirit upon your church in every land, that, by the service and sacrifice of many, your holy Name may be glorified and your kingdom enlarged; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Coleridge Patteson")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PaulaAndEustochium),
        CollectData::from(
            Document::from(
                Text::from("Compel us, O God, to attend diligently to your Word, as did your faithful servants Paula and Eustochium; that, by the inspiration of the Holy Spirit, we may find it profitable for doctrine, for reproof, for correction, and for instruction in righteousness; and that thereby we may be made wise unto salvation through faith in Christ Jesus our Lord.")
                    .response("Amen.")
                )
                .label("Paula and Eustochium of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Matthew),
        CollectData::from(
            Document::from(
                Text::from("We thank you, heavenly Father, for the witness of your apostle and evangelist Matthew to the Gospel of your Son our Savior; and we pray that, after his example, we may with ready wills and hearts obey the calling of our Lord to follow him; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Kassiani),
        CollectData::from(
            Document::from(
                Text::from("O God of boundless mercy, whose handmaiden Kassiani brought forth poetry and song: Inspire in your church a new song, that following her most excellent example, we may boldly proclaim the truth of your Word; even Jesus Christ, our Savior and Deliverer.")
                    .response("Amen.")
                )
                .label("Kassiani")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohannArndtAndJacobBoehme),
        CollectData::from(
            Document::from(
                Text::from("Holy God, who dwells with those have a contrite and humble spirit: Revive our spirits; purify us from deceitful lusts; and cloth us in righteousness and true holiness; though Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God now and for ever.")
                    .response("Amen.")
                )
                .label("Johann Arndt and Jacob Boehme")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::IsabelFlorenceHapgood),
        CollectData::from(
            Document::from(
                Text::from("Teach your divided church, O God, so to follow the example of your servant Isabel Florence Hapgood that we might look upon one another with a holy envy, to honor whatever is good and right in our separate traditions, and to continually seek the unity that you desire for all your people. We ask this in the name of Jesus Christ our Lord, who prayed that his church might be one. ")
                    .response("Amen.")
                )
                .label("Isabel Florence Hapgood")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MarinaTheMonk),
        CollectData::from(
            Document::from(
                Text::from("Give us grace, Lord God, to refrain from judgments about the sins of others; that, like your servant Marina the Monk, we may hold fast to the path of discipleship in the midst of unjust judgments; through Jesus Christ our Lord who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Marina the Monk")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HermanOfAlaska),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who raised up your servant Herman to be a light in the world, and to preach the Gospel to the people of Alaska: Illuminate our hearts, that we also in our own generation may show forth your praise, who called us out of darkness and into your marvelous light; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Herman of Alaska")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::IrenaeusOfLyons),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who strengthened your servant Irenaeus to defend thy truth against every blast of vain doctrine: Keep us, we pray, steadfast in your true religion, that in constancy and peace we may walk in the way that leads to eternal life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Irenaeus of Lyons")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Louis),
        CollectData::from(
            Document::from(
                Text::from("O God, you called your servant Louis of France to an earthly throne that he might advance your heavenly kingdom, and gave him zeal for your church and love for your people: Mercifully grant that we who commemorate him this day may be fruitful in good works and attain to the glorious crown of your saints; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Louis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::NinoOfGeorgia),
        CollectData::from(
            Document::from(
                Text::from("O Almighty God, who called your servant Nino to by your apostle to the people of Georgia, to bring those wandering in darkness to the true light and knowledge of you; Grant us so to walk in that light, that we may come at last to the light of your everlasting day; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Nino of Georgia")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FrederickDouglass),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, we bless your Name for the witness of Frederick Douglass, whose impassioned and reasonable speech moved the hearts of peopleto a deeper obedience to Christ: Strengthen us also to speak on behalf of those in captivity and tribulation, continuing in the way of Jesus Christ our Liberator; who with you and the Holy Spirit dwells in glory everlasting.")
                    .response("Amen.")
                )
                .label("Frederick Douglass")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WardClitherowLine),
        CollectData::from(
            Document::from(
                Text::from("Most Merciful God, who despises not a broken and contrite heart and has promised to fill those who hunger and thirst after righteousness; We humbly beseech you, remember not the sins and offenses of our ancestors, but grant that, like your servants Margaret Ward, Margaret Clitherow, and Anne Line, we may sanctify you in our hearts and be always ready to answer for our faith with meekness and fear; through our only Mediator and Advocate, Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Margaret Ward")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Enmegahbowh),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who led your pilgrim people of old by fire and cloud: Grant that the ministers of your church, following the example of your servant Enmegahbowh, may lead your people with fiery zeal and gentle humility; through Jesus Christ, who lives and reigns with you in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Enmegahbowh")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Ninian),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the preaching of your blessed servant and bishop Ninian caused the light of the Gospel to shine in the land of Britain: Grant, we pray, that having his life and labors in remembrance we may show our thankfulness by following the example of his zeal and patience; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Ninian")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Willibrord),
        CollectData::from(
            Document::from(
                Text::from("Pour out your Holy Spirit, O God, upon your church in every land, that like your servant Willibrord we might proclaim the Gospel to all nations, that your kingdom might be enlarged and that your holy Name might be glorified in all the world; through Jesus Christ our Lord, who lives and reigns with you and the same Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Willibrord")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheBeheadingOfSaintJohnTheBaptist),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called your servant John the Baptist to go before your Son our Lord both in life and death; Grant that we who remember his witness may with boldness speak your truth and in humility hear it when it is spoken to us, through Jesus Christ, the firstborn from the dead, who with you and the Holy Spirit lives and reigns one God for ever and ever.")
                    .response("Amen.")
                )
                .label("the Beheading of Saint John the Baptist")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::DavidPendletonOakerhater),
        CollectData::from(
            Document::from(
                Text::from("O God of unsearchable wisdom and mercy; Liberate us from bondage to self, and empower us to serve you and our neighbors, that like your servant David Oakerhater, we might bring those who do not know you to the knowledge and love of you; through Jesus Christ, the captain of our salvation, who lives and reigns with you and the Holy Spirit, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("David Pendleton Oakerhater")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MaryamOfQidun),
        CollectData::from(
            Document::from(
                Text::from("O God, whose glory it is always to have mercy: Be gracious to all who have gone astray from your ways, and restore them again like your servant Maryam of Qidun, that with penitent hearts and steadfast faith they might embrace and hold fast the unchangeable truth of your Word, Jesus Christ; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Maryam of Qidun")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasAquinas),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who has enriched your church with the singular learning and holiness of your servant Thomas Aquinas: Enlighten us more and more, we pray, by the disciplined thinking and teaching of Christian scholars, and deepen our devotion by the example of saintly lives; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Aquinas")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JeremyTaylor),
        CollectData::from(
            Document::from(
                Text::from("O God, whose days are without end, and whose mercies cannot be numbered: Make us, like your servant Jeremy Taylor, deeply aware of the shortness and uncertainty of human life; and let your Holy Spirit lead us in holiness and righteousness all our days; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Jeremy Taylor")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MartinLuther),
        CollectData::from(
            Document::from(
                Text::from("O God, our refuge and our strength, who raised up your servant Martin Luther to reform and renew your church in the light of your word: Defend and purify the church in our own day and grant that, through faith, we may boldly proclaim the richesof your grace, which you have made known in Jesus Christ our Savior, who with you and the Holy Spirit, lives and reigns, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Martin Luther")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThurgoodMarshall),
        CollectData::from(
            Document::from(
                Text::from("Eternal and ever-gracious God, who blessed your servant Thurgood Marshall with grace and courage to discern and speak the truth: Grant that, following his example, we may know you and recognize that we are all your children, brothers and sisters of Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thurgood Marshall")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HolyName),
        CollectData::from(
            Document::from(
                Text::from("Eternal Father, who gave to your incarnate Son the holy name of Jesus to be the sign of our salvation: Plant in every heart, we pray, the love of him who is the Savior of the world, our Lord Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::SamuelSeabury),
        CollectData::from(
            Document::from(
                Text::from("We give you thanks, O Lord our God, for your goodness in bestowing upon this church the gift of the episcopate; and we pray that, joined together in unity with our bishops and nourished by your holy sacraments, we may proclaim the Gospel of redemption with apostolic zeal; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Jerome),
        CollectData::from(
            Document::from(
                Text::from("O God, who gave us the holy Scriptures as a light to shine upon our path: Grant us, after the example of your servant Jerome, so to learn of you according to your holy Word, that we may find the Light that shines more and more to the perfect day; even Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and ever.")
                    .response("Amen.")
                )
                .label("Jerome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HildaOfWhitby),
        CollectData::from(
            Document::from(
                Text::from("O God of peace, by whose grace the abbess Hilda was endowed with gifts of justice, prudence, and strength to rule as a wise mother over the nuns and monks of her household: Raise up these gifts in us, that we, following her example and prayers, may build up one another in love to the benefit of your church; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Hilda of Whitby")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasAKempis),
        CollectData::from(
            Document::from(
                Text::from("Holy Father, you have nourished and strengthened your church by the writings of your servant Thomas à Kempis: Grant that we may learn from him to know what is necessary to be known, to love what is to be loved, to praise what highly pleases you, and always to seek to know and to follow your will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas à Kempis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Joseph),
        CollectData::from(
            Document::from(
                Text::from("O God, who from the family of your servant Davidraised up Joseph to be the guardian of your incarnateSon and the spouse of his virgin mother: Give us grace to imitate his uprightness of life and his obedience to your commands; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::DietrichBonhoeffer),
        CollectData::from(
            Document::from(
                Text::from("Embolden our lives, O Lord, and inspire our faiths, that we, following the example of your servant Dietrich Bonhoeffer, might embrace your call with undivided hearts; through Jesus Christ our Savior, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Dietrich Bonhoeffer")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ConfessionOfStPeter),
        CollectData::from(
            Document::from(
                Text::from("Almighty Father, who inspired Simon Peter, first among the apostles, to confess Jesus as Messiah and Son of the living God: Keep your Church steadfast upon the rock of this faith, so that in unity and peace we may proclaim the one truth and follow the one Lord, our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::RichardMeuxBensonAndCharlesGore),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, who kindled in your servants Richard Meux Benson and Charles Gore the grace to lead a revival of monastic life:  Grant us also the resolve to serve you faithfully in contemplation and prayer, ministering to the world that you have made, through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, ever one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Richard Meux Benson and Charles Gore]")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EuphrosynesmaragdusOfAlexandria),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who looks not with outward eyes but discerns the heart of each: we confess that those whom we love the most are often strangers to us.  Give to all parents and children, we pray, the grace to see one another as they truly are and as you have called them to be.  All this we ask in the name of Jesus Christ, our only mediator and advocate.")
                    .response("Amen.")
                )
                .label("Euphrosyne/Smaragdus of Alexandria]")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MariaSkobtsova),
        CollectData::from(
            Document::from(
                Text::from("O Creator and Giver of Life, who crowned your martyr Maria Skobtsova with glory and gave her as an example of service to the suffering and poor even unto death: Teach us to love Christ in our neighbors, and thereby battle injustice and evil with the light of the Resurrection; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God in glory everlasting.")
                    .response("Amen.")
                )
                .label("Maria Skobtsova")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CliveStaplesLewis),
        CollectData::from(
            Document::from(
                Text::from("O God of searing truth and surpassing beauty, we give you thanks for Clive Staples Lewis, whose sanctified imagination lit fires of faith in young and old alike; Surprise us also with your joy and draw us into that new and abundant life which is ours in Christ Jesus, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Clive Staples Lewis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesTheodoreHolly),
        CollectData::from(
            Document::from(
                Text::from("Most gracious God, whose servant James Theodore Holly labored to build a church in which all might be free: Grant that we might overcome our prejudice, and honor those whom you call from every family, language, people, and nation; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("James Theodore Holly")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasBecket),
        CollectData::from(
            Document::from(
                Text::from("O God, our strength and our salvation, you called your servant Thomas Becket to be a shepherd of your people and a defender of your church; Keep your household from all evil and raise up faithful pastors and leaders who are wise in the ways of the gospel; through Jesus Christ, the shepherd of our souls, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Becket")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HilaryOfPoitiers),
        CollectData::from(
            Document::from(
                Text::from("Keep us steadfast, Lord God, in that true faith that we professed at our baptism; that, like your servant Hilary of Poitiers, we may rejoice in having you for our Father, and may abide in your Son, in the fellowship of the Holy Spirit; for you live and reign for ever and ever as one God in Trinity of Persons.")
                    .response("Amen.")
                )
                .label("Hilary of Poitiers")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CharlesDeFoucauld),
        CollectData::from(
            Document::from(
                Text::from("Loving God, help us to know you wherever we find you and seek to serve you in all people, that with your servant Charles de Foucauld, we may be faithful even unto death; through Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Charles de Foucauld")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PriscillaAndAquila),
        CollectData::from(
            Document::from(
                Text::from("God of grace and might, who gave to your servants Aquila and Priscilla gifts of zeal and eloquence to make known the truth of the Gospel: Raise up, we pray, in every country, heralds and evangelists of your kingdom, so that the world may know the immeasurable riches of our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Priscilla and Aquila")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HarrietMonsell),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, who led your servant Harriet Monsell through grief to a new vocation; grant that we, inspired by her example, may grow in the life of prayer and the work of service so that in sorrow or joy, your presence may increase among us and our lives reveal the mind of Jesus Christ, to whom, with you and the Holy Spirit be honor and glory, now and for ever. ")
                    .response("Amen.")
                )
                .label("Harriet Monsell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Philip),
        CollectData::from(
            Document::from(
                Text::from("O God, who has made of one blood all the peoples of the earth and sent your Son to preach peace to those who are far off and to those who are near: Grant that we, following the example of your servant Philip, may bring your Word to those who seek you, for the glory of your Name; through Jesus Christ our Lord, who lives and reigns with you in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Philip")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::OscarRomero),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, you called your servant Óscar Romero to be a voice for the voiceless poor, and to give his life as a seed of freedom and a sign of hope: Grant that we, inspired by his sacrifice and the example of the martyrs of El Salvador, may without fear or favor witness to your Word who abides, your Word who is Life, even Jesus Christ our Lord, to whom, with you and the Holy Spirit, be praise and glory now and for ever.")
                    .response("Amen.")
                )
                .label("Oscar Romero")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThereseOfLisieux),
        CollectData::from(
            Document::from(
                Text::from("Gracious Father, who called your servant Thérèse to a life of fervent prayer: Give to us that spirit of prayer and zeal for the ministry of the Gospel, that the love of Christ may be known throughout all the world; through the same Jesus Christ, our Lord.")
                    .response("Amen.")
                )
                .label("Therese of Lisieux")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AllSoulsDay),
        CollectData::from(
            Document::from(
                Text::from("O God, the Maker and Redeemer of all believers: Grant to the faithful departed the unsearchable benefits of the passion of your Son; that on the day of his appearing they may be manifested as your children; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::LeoOfRome),
        CollectData::from(
            Document::from(
                Text::from("O Lord our God, grant that your church, following the teaching of your servant Leo of Rome, may hold fast the great mystery of our redemption and adore the one Christ, true God and true Man, neither divided from our human nature nor separate from your divine Being; through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Leo of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::RolleHiltonKempe),
        CollectData::from(
            Document::from(
                Text::from("Direct our hearts, O Gracious God, and inspire our minds; that like your servants Richard Rolle, Walter Hilton, and Margery Kempe, we might pass through the cloud of unknowing until we behold your glory face to face; in the Name of Jesus Christ our Lord; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Richard Rolle 1349")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PatrickOfIreland),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, in your providence you chose your servant Patrick to be the apostle to the Irish people, to bring those who were wandering in darkness and error to the true light and knowledge of you: Grant us so to walk in that way that we may come at last to the light of everlasting life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Patrick of Ireland")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JacksonKemper),
        CollectData::from(
            Document::from(
                Text::from("O God, who sent your son Jesus Christ to preach peace to those who are far off and to those who are near: Grant that we, like your servant Jackson Kemper, may proclaim the Gospel in our own day, with courage, vision, and perseverance; through the same Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, now and for ever.")
                    .response("Amen.")
                )
                .label("Jackson Kemper")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BernardMizeki),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who kindled the flame of your love in the heart of your holy martyr Bernard Mizeki: Grant unto us your servants a like faith and power of love, that we, who rejoice in his triumph, may profit by his example; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Bernard Mizeki")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::SarahTheodoraSyncletica),
        CollectData::from(
            Document::from(
                Text::from("Fix our hearts on You, O God, in pure devotion, that aided by the example of your servants Sarah, Theodora, and Syncletica, the vain pursuits of this world may have no hold upon us, and that by the consuming fire of your Spirit, we may be changed into the image and likeness of your Son, Jesus Christ our Lord; to whom with you and the same Spirit be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Sarah")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::RemigiusOfRheims),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who by your servant Remigius spread the truth of the gospel and the fullness of the catholic faith: Grant that we who glory in the name of Christian may show forth our faith in worthy deeds; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Remigius of Rheims")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BirgittaOfSweden),
        CollectData::from(
            Document::from(
                Text::from("O God, who beholds all things and whose judgment is always mercy; by the example of your servant Birgitta of Sweden, give to us in this life the vision of your kingdom, where Jesus Christ is all and in all, that we may pattern our earthly lives on things heavenly, where our lives are hidden with Christ in you; who with him and the Holy Spirit live and reign for ever and ever.")
                    .response("Amen.")
                )
                .label("Birgitta of Sweden")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CatherineOfSiena),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who kindled the flame of thy love in the heart of your servant Catherine of Siena: Grant unto us the same strength of conviction and power of love that, as we rejoice in her triumph, we may profit by her example; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Catherine of Siena")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::NativityOfStJohnTheBaptist),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose providence your servant John the Baptist was wonderfully born, and sent to prepare the way of your Son our Savior by preaching repentance: Makeus so to follow his teaching and holy life, that we may truly repent according to his preaching; and, following his example, constantly speak the truth, boldly rebuke vice, and patiently suffer for the truth’s sake; through Jesus Christ your Son our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HelenaOfConstantinople),
        CollectData::from(
            Document::from(
                Text::from("Most Merciful God, who blessed your servant Helena with such grace and devotion to you that she venerated the very footsteps of our Savior; Grant unto us the same grace that, aided by her prayers and example, we also may always behold your glory in the cross of your Son. Through the same Jesus Christ our Lord; who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Helena of Constantinople")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ElizabethAnnSeton),
        CollectData::from(
            Document::from(
                Text::from("Give us grace, O God, to love you in all things and above all things; that, following the example of your servant Elizabeth Ann Seton, we might express our love for you in the service of others. Through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Elizabeth Ann Seton")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Dominic),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, grant unto your people a hunger for your Word and an urgent longing to share your Gospel; that, like your servant Dominic, we might labor to bring the whole world to the knowledge and love of you as you are revealed in your Son Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Dominic")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JulianOfNorwich),
        CollectData::from(
            Document::from(
                Text::from("Triune God, Father and Mother to us all, who showed your servant Julian revelations of your nurturing and sustaining love: Move our hearts, like hers, to seek you above all things, for in giving us yourself you give us all.")
                    .response("Amen.")
                )
                .label("Julian of Norwich")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EmilyMalboneMorgan),
        CollectData::from(
            Document::from(
                Text::from("Inspire us, Gracious God, with that same spirit of devotion that animated your servant Emily Malbone Morgan; that, like her, we might dedicate our lives to your service and to the welfare of others; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Emily Malbone Morgan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Alfred),
        CollectData::from(
            Document::from(
                Text::from("O God, who called your servant Alfred to an earthly throne that he might advance your heavenly kingdom and gave him zeal for your church and love for your people: Grant that we, inspired by his example and prayers, may remain steadfast in the work you have given us to do for the building up of your reign of love; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Alfred")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MosesTheBlack),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose blessed Son guides our footsteps in the way of peace: Deliver us from the paths of hatred and violence, that we, following the example of your servant Moses, may serve you with singleness of heart and attain to the tranquility of the world to come; through Jesus Christ our Lord, who lives and reigns with you in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Moses the Black")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CorneliusTheCenturion),
        CollectData::from(
            Document::from(
                Text::from("O God, who by your Spirit called Cornelius the Centurion to be the first Christian among the Gentiles: Grant to your church such a ready will to go where you send and to do what you command that the prejudices that blind us might cease, and that we might welcome all who turn to you in love; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Cornelius the Centurion")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnKeble),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that in all time of our testing we may know your presence and obey your will, that, following the example of your servant John Keble, we may accomplish with integrity and courage what you give us to do and endure what you give us to bear; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Keble")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AgathaOfSicily),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who strengthened your martyr Agatha with constancy and courage: Grant us for the love of you to make no peace with oppression, to fear no adversity, and to have no tolerance for those who would use their power to abuse or exploit; Through Jesus Christ our Lord, to whom with you and the Holy Spirit be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Agatha of Sicily")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BlandinaAndHerCompanions),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave such courage and endurance to Blandina and her companions that by their deaths many hearts were turned to you; Grant that we, in accordance with their example, may also gladly endure all that is required of as we witness to you in our own day; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Blandina and Her Companions")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Tikhon),
        CollectData::from(
            Document::from(
                Text::from("Holy God, holy and mighty, you call us together into one communion and fellowship: Open our eyes, we pray, as you opened the eyes of your servant Tikhon, that we may see the faithfulnessof others as we strive to be steadfast in the faith delivered unto us, that the world may see and know you; through Jesus Christ our Lord, to whom, with you and the Holy Spirit, be glory and praise unto ages of ages.")
                    .response("Amen.")
                )
                .label("Tikhon")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Thomas),
        CollectData::from(
            Document::from(
                Text::from("Everliving God, who strengthened your apostle Thomas with firm and certain faith in your Son’s resurrection: Grant us so perfectly and without doubt to believe in Jesus Christ, our Lord and our God, that our faith may never be found wanting in your sight; through him who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasKen),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, you gave your servant Thomas Ken grace and courage to bear witness to the truth before rulers and kings: Give us strength also that we may constantly defend what is right, boldly reprove what is evil, and patiently suffer for the truth’s sake; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Ken")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FrancisXavier),
        CollectData::from(
            Document::from(
                Text::from("God of all nations; Raise up in this and every land, evangelists and heralds of your kingdom, that like your servant Francis Xavier we may proclaim the unsearchable riches of our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Francis Xavier")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesOfJerusalem),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that, following the example of your servant James the Just, brother of our Lord, your church may give itself continually to prayer and to the reconciliation of all who are at variance and enmity; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MartinOfTours),
        CollectData::from(
            Document::from(
                Text::from("Lord God of hosts, you clothed your servant Martin the soldier with the spirit of sacrifice and set him as a bishop in your church to be a defender of the catholic faith: Give us grace to follow in his holy steps, that, at the last, we may be found clothed with righteousness in the dwellings of peace; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Martin of Tours")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Barnabas),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that we may follow the example ofyour faithful servant Barnabas, who, seeking not his own renown but the well-being of your church, gave generously of his life and substance for the relief of the poor and the spread of the Gospel; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesOtisSargentHuntington),
        CollectData::from(
            Document::from(
                Text::from("Preserve your people, O God, from discouragement in the face of adversity, as you did your servant James Huntington, knowing that when you have begun a good work you will bring it to completion. Through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Otis Sargent Huntington")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::NicholasOfMyra),
        CollectData::from(
            Document::from(
                Text::from("Grant, Almighty God, that your church may be so inspired by the example of your servant Nicholas of Myra, that it may never cease to work for the welfare of children, the safety of sailors, the relief of the poor, and the help of those tossed by tempests of doubt or grief; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Nicholas of Myra")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TeresaOfAvila),
        CollectData::from(
            Document::from(
                Text::from("O God, who by your Holy Spirit moved Teresa of Avila to manifest to your church the way of perfection: Grant us, we pray, to be nourished by her teaching, and enkindle within us a keen and unquenchable longing for true holiness; through Jesus Christ, the joy of loving hearts, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Teresa of Avila")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PerpetuaAndFelicity),
        CollectData::from(
            Document::from(
                Text::from("O God, the King of Saints, who strengthened your servants Perpetua, Felicity, and their companions to make a good confession and to encourage one another in the time of trial: Grant that we who cherish their blessed memory may share their pure and steadfast faith, and win with them the palm of victory; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Perpetua and Felicity")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MarcellaOfRome),
        CollectData::from(
            Document::from(
                Text::from("O God, who satisfies the longing soul and fills the hungry with good things: Grant that we, like your servant Marcella, may hunger and thirst after you above the vain pomp and glory of the world, and delight in your word above all manner of riches; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Marcella of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EvelynUnderhill),
        CollectData::from(
            Document::from(
                Text::from("O God, Origin, Sustainer, and End of all creatures: Grant that your church, taught by your servant Evelyn Underhill, may continually offer to you all glory and thanksgiving, and attain with your saints to the blessed hope of everlasting life, which you have promised us by our Savior Jesus Christ; who with you and the Holy Spirit lives and reigns, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Evelyn Underhill")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MargaretOfScotland),
        CollectData::from(
            Document::from(
                Text::from("O God, who called your servant Margaret toan earthly throne that she might advance your heavenly kingdom, and gave her zeal for your church and love for your people: Mercifully grant that we also may be fruitful in good works, and attain to the glorious crown of your saints; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Margaret of Scotland")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PaulJones),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, you sent your beloved Son to preach peace to those who are far off and to those who are near: Raise up in this and every land witnesses who, after the example of your servant Paul Jones, will stand firm in proclaiming the Gospel of the Prince of Peace, our Savior Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Paul Jones")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HenryMartyn),
        CollectData::from(
            Document::from(
                Text::from("O God of the nations, who gave to your servant Henry Martyn a longing to share your Gospel with all peoples; Inspire the church in our own day with that said desire, that we may be eager to commit both life and talents to you who gave them; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Henry Martyn")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::LaurenceOfRome),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose grace and power your servant Laurence triumphed over suffering and despised death: Grant that we may be steadfast in service to the poor and outcast, and may share with him in the joys of your everlasting kingdom; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Laurence of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::BernardOfClairvaux),
        CollectData::from(
            Document::from(
                Text::from("O God, by whose grace your servant Bernard of Clairvaux, kindled with the flame of your love, became a burning and a shining light in your church: Grant that we also may be aflame with the spirit of love and discipline and walk before you as children of light; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Bernard of Clairvaux")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::DorothyLSayers),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who strengthened your servant Dorothy Sayers with eloquence to defend Christian teaching: Keep us, we pray, steadfast in your true religion, that in constancy and peace we may always teach right doctrine, and teach doctrine rightly; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Dorothy L Sayers")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AthanasiusOfAlexandria),
        CollectData::from(
            Document::from(
                Text::from("O Lord, who established your servant Athanasius, through wisdom, in your truth: Grant that we, perceiving the humanity and divinity of your Son Jesus Christ, may follow in his footsteps and ascend the way to eternal life, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Athanasius of Alexandria")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohannSebasatianBach),
        CollectData::from(
            Document::from(
                Text::from("Sound out your majesty, O God, and call us to your work; that, like thy servant Johann Sebastian Bach, we might present our lives and our works to your glory alone; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Johann Sebasatian Bach")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Cuthbert),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who called Cuthbert from following the flock to be a shepherd of your people: Mercifully grant that we also may go without fear to dangerous and remote places, to seek the indifferent and the lost; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Cuthbert")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ElizabethOfHungary),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by your grace your servant Elizabeth of Hungary recognized and honored Jesus in the poor of this world: Grant that we, following her example, may with love and gladness serve those in any need or trouble, in the name and for the sake of Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Elizabeth of Hungary")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ClareOfAssisi),
        CollectData::from(
            Document::from(
                Text::from("O God, whose blessed Son became poor that we, through his poverty, might become rich: Deliver us from an inordinate love of this world, that we, inspired by the devotion of your servant Clare, might serve you with singleness of heart and attain to the riches of the age to come; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Clare of Assisi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheNativityOfTheBlessedVirginMary),
        CollectData::from(
            Document::from(
                Text::from("Father in heaven, by your grace the virgin mother of your incarnate Son was blessed in bearing him, but still more blessed in keeping your word: Grant us who honor the exaltation of her lowliness to follow the example of her devotion to your will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Nativity of the Blessed Virgin Mary")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnHenryHobart),
        CollectData::from(
            Document::from(
                Text::from("Revive your church, Lord God of hosts, whenever it falls into complacency and sloth, by raising up devoted leaders like your servant John Henry Hobart; and grant that their faith and vigor of mind may awaken your people to your message and their mission; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Henry Hobart")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheParentsOfTheBlessedVirginMary),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, heavenly Father, we remember in thanksgiving this day the parents of the Blessed Virgin Mary; and we pray that we all may be made one in the heavenly family of your Son Jesus Christ our Lord; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Parents of the Blessed Virgin Mary")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryTheIlluminator),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who raised up your servant Gregory to be a light in the world, and to preach the Gospel to the people of Armenia: Illuminate our hearts, that we also in our own generation may show forth your praise, who called us out of darkness and into your marvelous light; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Gregory the Illuminator")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Ammonius),
        CollectData::from(
            Document::from(
                Text::from("Drive far from your church, O God, every vain spirit of clerical ambition, that, like your servant Ammonius, we may refuse to conflate ordination and leadership, and may never confuse rank with holiness; in the name of your son Jesus Christ our Lord, who alone is our great High Priest.")
                    .response("Amen.")
                )
                .label("Ammonius")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Luke),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who inspired your servant Luke the physician to set forth in the Gospel the love and healing power of your Son: Graciously continue in your church this love and power to heal, to the praise and glory of your Name; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamReedHuntington),
        CollectData::from(
            Document::from(
                Text::from("O Lord our God, we thank you for instilling in the heart of your servant William Reed Huntington a fervent love for your church and its mission in the world; and we pray that, with unflagging faith in your promises, we may make known to all people your blessed gift of eternal life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Reed Huntington")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AntonyOfEgypt),
        CollectData::from(
            Document::from(
                Text::from("O God, as you by your Holy Spirit enabled your servant Antony to withstand the temptations of the world, the flesh, and the devil; so give us grace to follow you with pure hearts and minds, through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Antony of Egypt")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Annunciation),
        CollectData::from(
            Document::from(
                Text::from("Pour your grace into our hearts, O Lord, that we who have known the incarnation of your Son Jesus Christ, announced by an angel to the Virgin Mary, may byhis cross and passion be brought to the glory of his resurrection; who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::SergiusOfRadonezh),
        CollectData::from(
            Document::from(
                Text::from("O God, whose blessed Son became poor that we, through his poverty, might be rich: Deliver us from an inordinate love of this world, that we, inspired by the devotion of your servant Sergius, may serve you with singleness of heart and attain to the riches of the age to come; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Sergius of Radonezh")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Photini),
        CollectData::from(
            Document::from(
                Text::from("O Almighty God, whose most blessed Son revealed to the Samaritan woman that He is indeed the Christ, the Savior of the World; Grant us to drink of the well that springs up to everlasting life that we may worship you in spirit and in truth through your Son, Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Photini")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::FlorenceNightingale),
        CollectData::from(
            Document::from(
                Text::from("O God, who gave grace to your servant Florence Nightingale to bear your healing love into the shadow of death: Grant to all who heal the same virtues of patience, mercy, and steadfast love, that your saving health may be revealed to all; through Jesus Christ, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Florence Nightingale")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfNewGuinea),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, we remember before you this day the blessed martyrs of New Guinea, who, following the example of their Savior, laid down their lives for their friends, and we pray that we who honor their memory may imitate their loyalty and faith; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of New Guinea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesHannington),
        CollectData::from(
            Document::from(
                Text::from("O God, by whose providence the blood of the martyrs is the seed of the church: Grant that we who remember before you James Hannington and his companions, may, like them, be steadfast in our faith in Jesus Christ, to whom they gave obedience even to death, and by their sacrifice brought forth a plentiful harvest; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Hannington")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::KatharinaZell),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose servant Katharina Zell toiled for the reform of your church both in word and in deed: Fill us with the wisdom to speak out in defense of your truth, with love for you and for our neighbor, that we may serve you and welcome all your people with a mother’s heart; through Christ our Lord.")
                    .response("Amen.")
                )
                .label("Katharina Zell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EdithCavell),
        CollectData::from(
            Document::from(
                Text::from("Living God, the source of all healing and wholeness: we bless you for the compassionate witness of your servant Edith Cavell. Inspire us to be agents of peace and reconciliation in a world beset by injustice, poverty, and war. We ask this through Jesus Christ, the Prince of Peace, who lives and reigns with you and the Holy Spirit, one God, to the ages of ages.")
                    .response("Amen.")
                )
                .label("Edith Cavell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JosephOfArimathea),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, whose servant Joseph of Arimathea with reverence and godly fear prepared the bodyof our Lord and Savior for burial and laid it in his own tomb: Grant to us, your faithful people, grace and courage to love and serve Jesus with sincere devotion all the days of our life; through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Joseph of Arimathea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamWhite),
        CollectData::from(
            Document::from(
                Text::from("O Lord, who in a time of turmoil and confusion raised up your servant William White to lead your church into ways of stability and peace; Hear our prayer, and give us wise and faithful leaders, that, through their ministry, your people may be blessed and your will be done; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William White")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheodoreOfTarsus),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave your servant Theodore of Tarsus gifts of grace and wisdom to establish unity where there had been division and order where there had been chaos: Create in your church, by the operation of the Holy Spirit, such godly union and concord that it may proclaim, both by word and example, the Gospel of the Prince of Peace; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Theodore of Tarsus")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AdelaideTeagueCase),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who raises up educators and teachers of the faith in every generation of your church: Grant that following the example of your servant Adelaide Teague Case, we might be bold to proclaim the reconciling power of Christ’s love in our own generation.  Through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, in glory everlasting. ")
                    .response("Amen.")
                )
                .label("Adelaide Teague Case")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::EdithSteinTeresaBenedictaOfTheCross),
        CollectData::from(
            Document::from(
                Text::from("Pour out your grace upon thy church, O God; that, like your servant Edith Stein, we may always seek what is true, defend what is right, reprove what is evil, and forgive those who sin against us, even as your Son commanded; through the same Jesus Christ our Lord, to whom with you and the Holy Spirit be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Edith Stein (Teresa Benedicta of the Cross)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Phoebe),
        CollectData::from(
            Document::from(
                Text::from("Eternal God, who raised up Phoebe as a deacon in your church and minister of your Gospel; Grant us that same grace that, assisted by her prayers and example, we too may take the Gospel to the ends of the earth; through Jesus Christ your Son our Lord who lives and reigns with you, in the unity of the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Phoebe")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AlexanderCrummell),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, we thank you for your servant Alexander Crummell, whom you called to preach the gospel to those who were far off and to those who were near: Raise up, in this and every land, evangelists and heralds of your kingdom, that your church may proclaim the unsearchable riches of our Savior Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Alexander Crummell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MelaniaTheElder),
        CollectData::from(
            Document::from(
                Text::from("Most High and Merciful God, who called your servant Melania to forsake earthly comforts in order to devote herself to studying the scriptures and to welcoming the poor: Instruct us in the ways of poverty and the grace of hospitality, that we might comfort those who have no place to rest and teach the way of your love; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Melania the Elder")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::IgnatiusOfLoyola),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who called Ignatius of Loyola to the service of your Divine Majesty and to seek you in all things; Give us also the grace to labor without counting the cost and to seek no reward other than knowing that we do your will; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, now and for ever.")
                    .response("Amen.")
                )
                .label("Ignatius of Loyola")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryOfNazianzus),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who has revealed to your Church your eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace that, like your bishop Gregory of Nazianzus, we may continue steadfast in the confession of this faith, and constant in our worship of you, Father, Son, and Holy Spirit; who live and reign for ever and ever.")
                    .response("Amen.")
                )
                .label("Gregory of Nazianzus")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PachomiusOfTabenissi),
        CollectData::from(
            Document::from(
                Text::from("Set us free, O God, from all false desires, vain ambitious, and everything that would separate us from your love; that, like your servant Pachomius, we might give ourselves fully to a life of discipleship, seeking you alone and serving those whom you have given us to serve; through Jesus Christ, our only mediator and advocate.")
                    .response("Amen.")
                )
                .label("Pachomius of Tabenissi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MartyrsOfTheReformationEra),
        CollectData::from(
            Document::from(
                Text::from("Almighty and Most Merciful God, give to your church that peace which the world cannot give, and grant that those who have been divided on earth may be reconciled in heaven, and share together in the vision of your glory; through Jesus Christ your Son our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever. ")
                    .response("Amen.")
                )
                .label("Martyrs of the Reformation Era")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Epiphany),
        CollectData::from(
            Document::from(
                Text::from("O God, by the leading of a star you manifested your only Son to the peoples of the earth: Lead us, who know you now by faith, to your presence, where we may see your glory face to face; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::PolycarpOfSmyrna),
        CollectData::from(
            Document::from(
                Text::from("O God, the maker of heaven and earth, yougave your venerable servant, the holy and gentle Polycarp, the boldness to confess Jesus Christ as King and Savior and the steadfastness to die for his faith: Give us grace, following his example, to share the cup of Christ and to rise to eternal life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Polycarp of Smyrna")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnCassian),
        CollectData::from(
            Document::from(
                Text::from("Holy God, whose beloved Son Jesus Christ blessed the pure in heart: Grant that we, together with your servant John Cassian and in union with his prayers, may ever seek the purity with which to behold you as you are; one God in Trinity of persons now and for ever.")
                    .response("Amen.")
                )
                .label("John Cassian")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Monica),
        CollectData::from(
            Document::from(
                Text::from("Deepen our devotion, O Lord, and use us in accordance with your will; that inspired by the example of your servant Monica, we may bring others to acknowledge Jesus Christ as Savior and Lord; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Monica")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MancheMasemola),
        CollectData::from(
            Document::from(
                Text::from("Almighty and Everlasting God, who kindled the flame of your love in the heart of your faithful martyr Manche Masemola: Grant to us your servants, a like faith and power of love, that we who rejoice in her triumph may profit by her example; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Manche Masemola")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AelredOfRievaulx),
        CollectData::from(
            Document::from(
                Text::from("Grant to your people, Almighty God, a spirit of mutual affection; that, following the example of your servant Aelred of Rivaulx, we might know the love of Christ in loving one another; through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Aelred of Rievaulx")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WulfstanOfWorcester),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose only-begotten Son led captivity captive and gave gifts to your people: Multiply among us faithful pastors, who, like your holy bishop Wulfstan, will give courage to those who are oppressed and held in bondage; and bring us all, we pray, into the true freedom of your kingdom; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Wulfstan of Worcester")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::LydiaOfThyatira),
        CollectData::from(
            Document::from(
                Text::from("Eternal God, who gives good gifts to all people, and who grants the spirit of generosity: Give us, we pray you, hearts always open to hear your word, that, following the example of your servant Lydia, we may show hospitality to those who are in any need or trouble; through Jesus Christ our Lord who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Lydia of Thyatira")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JohnOfTheCross),
        CollectData::from(
            Document::from(
                Text::from("Judge eternal, throned in splendor, who gave John of the Cross strength of purpose and faith that sustained him even through the dark night of the soul:  Shed your light on all who love you, in unity with Jesus Christ our Savior; who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John of the Cross")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Andrew),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave such grace to your apostle Andrew that he readily obeyed the call of your SonJesus Christ, and brought his brother with him: Give untous, who are called by your Word, grace to follow him without delay, and to bring those near to us into his gracious presence; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::ConversionOfStPaul),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the preaching of your apostle Paul has caused the light of the Gospel to shine throughout the world: Grant, we pray, that we, having his wonderful conversion in remembrance, may show ourselves thankful to you by following his holy teaching; through Jesus Christ our Lord, who lives and reigns with you, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CyrilOfJerusalem),
        CollectData::from(
            Document::from(
                Text::from("Strengthen, O God, your church in the sacraments of your grace, that we, in union with the teaching and prayers of your servant Cyril of Jerusalem, may enter more fully into your Paschal mystery; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Cyril of Jerusalem")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AllSaintsDay),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, you have knit together your elect in one communion and fellowship in the mystical body of your Son Christ our Lord: Give us grace so to follow your blessed saints in all virtuous and godly living, that we may come to those ineffable joys that you have prepared for those who truly love you; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JosephButler),
        CollectData::from(
            Document::from(
                Text::from("O God, who raises up scholars for your church in every generation; we praise you for the wisdom and insight granted to your bishop and theologian Joseph Butler, and pray that your church may never be destitute of such gifts; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Joseph Butler")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AgnesAgathaLucy),
        CollectData::from(
            Document::from(
                Text::from("Lord Jesus Christ, who willingly walked the wayof the cross: Strengthen your church through the witness of your servants Agnes Tsao Kou Ying, Agatha Lin Zhao, and Lucy Yi Zhenmei to hold fast to the path of discipleship even unto death; for with the Father and Holy Spirit you live and reign, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Agnes Tsao Kou Ying")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JoannaMarySalome),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who revealed the resurrection of your Son to Joanna, Mary and Salome as they faithfully came bearing myrrh to his tomb: Grant that we too may perceive the presence of the risen Lord in the midst of pain and fear, and go forth proclaiming his resurrection; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Joanna")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamTyndale),
        CollectData::from(
            Document::from(
                Text::from("Reveal to us your saving word, O God, that like your servant William Tyndale we might hear its call to repentance and new life.  Plant in our hearts that same consuming passion to bring the scriptures to all people in their native tongue, and the strength to endure amidst all obstacles; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Tyndale")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::DamienAndMarianne),
        CollectData::from(
            Document::from(
                Text::from("Bind up the wounds of your children, O God, and help us to be bold and loving in service to all who are shunned for the diseases they suffer, following the example of your servants Damien and Marianne, that your grace may be poured forth upon all; through Jesus Christ our Lord, who with you and the Holy Spirit lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Damien")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JamesDeKoven),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who led your servant James De Koven to honor your presence at the altar, and constantly to point to Christ: Grant that all ministers and stewards of your mysteries may impart to your faithful people the knowledge of your presence and the truth of your grace; through the same Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James de Koven")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Anskar),
        CollectData::from(
            Document::from(
                Text::from("Keep your church from discouragement in the day of small things, O God, in the knowledge that when you have begun a good work, you will bring it to a fruitful conclusion, just as you did for your servant Anskar; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Anskar")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CyrilAndMethodius),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who by the power of the Holy Spirit moved your servants Cyril and Methodius to bring the light of the Gospel to a hostile and divided people: Overcome all bitterness and strife among us by the love of Christ, and make us one united family under the banner of the Prince of Peace; who lives and reigns with you and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Cyril and Methodius")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HarrietBedell),
        CollectData::from(
            Document::from(
                Text::from("Holy God, fill us with compassion and respect for all people, and empower us for the work of ministry whether near or far away; that like thy servant Harriet Bedell, we may how forth your praise, not only with our lips, but in our lives, and by giving up ourselves to your service.  Through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Harriet Bedell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::CharlesSimeon),
        CollectData::from(
            Document::from(
                Text::from("Loving God, whose unerring wisdom and unbounded love order all things: Grant us in all things to see your hand; that, following the example and teaching of your servant Charles Simeon, we may walk with Christ in all simplicity and serve you with a quiet and contented mind; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Charles Simeon")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::AnselmOfCanterbury),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose servant Anselm helped your church to understand its faith in your eternal Being, perfect justice, and saving mercy: Provide your church in all ages with devout and learned scholars and teachers, that we may be able to give a reason for the hope that is in us; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Anselm of Canterbury")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HannahMore),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose only-begotten Son led captivity captive: Multiply among us faithful witnesses like your servant Hannah More, who will fight for all who are oppressed or held in bondage, and bring us all, we pray, into the glorious liberty that you have promised to all your children; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Hannah More")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::John),
        CollectData::from(
            Document::from(
                Text::from("Shed upon your church, O Lord, the brightness of your light, that we, being illumined by the teaching of your apostle and evangelist John, may so walk in the light of your truth, that at length we may attain to the fullness of eternal life; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::HadewijchOfBrabant),
        CollectData::from(
            Document::from(
                Text::from("Triune God of Love, overwhelming and all-encompassing: Visit us in our solitude and in our companionship, and draw us ever more deeply into union with you, who are ever present and ever mysterious; that we, like your servant Hadewijch, might know you ever more fully, even as we have been fully known.")
                    .response("Amen.")
                )
                .label("Hadewijch of Brabant")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::IgnatiusOfAntioch),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, we praise your Name for your bishop and martyr Ignatius of Antioch, who offered himself as grain to be ground by the teeth of wild beasts that he might present to you the pure bread of sacrifice. Accept, we pray, the willing tributeof our lives and give us a share in the pure and spotless offering of your Son Jesus Christ; who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Ignatius of Antioch")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::TheTransfiguration),
        CollectData::from(
            Document::from(
                Text::from("O God, who on the holy mount revealed to chosen witnesses your well-beloved Son, wonderfully transfigured, in raiment white and glistening: Mercifully grant that we, being delivered from the disquietude of this world, mayby faith behold the King in his beauty; who with you, O Father, and you, O Holy Spirit, lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::Bartholomew),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who gave to your apostle Bartholomew grace truly to believe and to preach your Word: Grant that your church may love what he believed and preach what he taught; through Jesus Christ our Lord, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::JonathanMyrickDaniels),
        CollectData::from(
            Document::from(
                Text::from("O God of justice and compassion, who puts down the proud and mighty from their place, and lifts up the poor and the afflicted: We give you thanks for your faithful witness Jonathan Myrick Daniels, who, in the midst of injustice and violence, risked and gave his life for another; and we pray that we, following his example, may make no peace with oppression; through Jesus Christ our Savior, who lives and reigns with you and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Jonathan Myrick Daniels")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
        (
        CollectId::Feast(Feast::MargaretOfCortona),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, to all your people, as to your servant Margaret of Cortona, the spirit of repentance and supplication, that we might seek and desire nothing in this transitory life above you; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Margaret of Cortona")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteII)
        )
    ),
    ];

    pub static ref LFF_COLLECTS_TRADITIONAL: [(CollectId, CollectData); 272] = [
                (
        CollectId::Feast(Feast::JohnMasonNeale),
        CollectData::from(
            Document::from(
                Text::from("Grant unto us, O God, that in all time of our testing we may know thy presence and obey thy will; that, following the example of thy servant John Mason Neale, we may with integrity and courage accomplish what thou givest us to do, and endure what thou givest us to bear; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Mason Neale")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PaulJones),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who didst send thy beloved Son to preach peace to those who are far off and to those who are near: Raise up in this and every land witnesses, who, after the example of thy servant Paul Jones, will stand firm in proclaiming the Gospel of the Prince of Peace, our Savior Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Paul Jones")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::KatharinaVonBora),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant Katharina von Bora from a cloister to work for the reform of thy church: Grant that, for the sake of thy glory and the welfare of thy church, we may go wherever thou dost call, and serve however thou dost will; through Jesus Christ, our only mediator and advocate.")
                    .response("Amen.")
                )
                .label("Katharina Von Bora")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Justin),
        CollectData::from(
            Document::from(
                Text::from("O God, who hast given thy church wisdom and revealed to it deep and secret things: Grant that we, like thy servant Justin and in union with his prayers, may find thy Word an abiding refuge all the days of our lives; through Jesus Christ of Lord, who with thee and the Holy Ghost liveth and reigneth, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Justin")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamWhite),
        CollectData::from(
            Document::from(
                Text::from("O Lord, who in a time of turmoil and confusion didst raise up thy servant William White to lead thy church into ways of stability and peace; Hear our prayer, and give us wise and faithful leaders, that, through their ministry, thy people may be blessed and thy will be done; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William White")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::SamuelSeabury),
        CollectData::from(
            Document::from(
                Text::from("We give thanks to thee, O Lord our God, forthy goodness in bestowing upon this churchthe gift of the episcopate; and we pray that, joined together in unity with our bishops and nourished by thy holy sacraments, we may proclaim the Gospelof redemption with apostolic zeal; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Dunstan),
        CollectData::from(
            Document::from(
                Text::from("Direct thy Church, O Lord, into the beauty of holiness, that, following the good example of thy servant Dunstan, we may honor thy Son Jesus Christ with our lips and in our lives; to the glory of his Name, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Dunstan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Edmund),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who didst give grace and fortitude to Edmund to die nobly for thy Name: Bestow on us thy servants, we beseech thee, the shield of faith, wherewith we may withstand the assaults of our ancient enemy; through Jesus Christ our Redeemer, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Edmund")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesOtisSargentHuntington),
        CollectData::from(
            Document::from(
                Text::from("Preserve thy people, O God, from discouragement in the face of adversity, as thou didst thy servant James Huntington, knowing that when thou hast begun a good work thou wilt bring it to completion. Through Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Otis Sargent Huntington")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Andrew),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give such grace to thine apostle Andrew that he readily obeyed the call of thy Son Jesus Christ, and brought his brother with him: Give unto us, who are called by thy Word, grace to follow him without delay, and to bring those near to us into his gracious presence; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ToyohikoKagawa),
        CollectData::from(
            Document::from(
                Text::from("Strengthen and protect, O God, all those who suffer for their fidelity to Jesus Christ; that, like thy servant Toyohiko Kawaga, they might persevere in seeking and serving Christ in all persons, and work tirelessly for the advancement of thy kingdom; through the same Jesus Christ our Lord, to whom with thee and the Holy Ghost be all honor and glory now and for ever.")
                    .response("Amen.")
                )
                .label("Toyohiko Kagawa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Louis),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst call thy servant Louis of France to an earthly throne that he might advance thy heavenly kingdom, and gave him zeal for thy church and love for thy people: Mercifully grant that we who commemorate him this day may be fruitful in good works and attain to the glorious crown of thy saints; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Louis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EdithSteinTeresaBenedictaOfTheCross),
        CollectData::from(
            Document::from(
                Text::from("Pour out thy grace upon thy church, O God; that, like thy servant Edith Stein, we may always seek what is true, defend what is right, reprove what is evil, and forgive those who sin against us, even as thy Son hath commanded; through the same Jesus Christ our Lord, to whom with thee and the Holy Ghost be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Edith Stein (Teresa Benedicta of the Cross)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnOfDamascus),
        CollectData::from(
            Document::from(
                Text::from("Confirm our minds, O Lord, in the mysteries of the true faith, set forth with power by thy servant John of Damascus; that we, with him, confessing Jesus to be true God and true Man and singing the praises of the risen Lord, may, by the power of the resurrection, attain to eternal joy; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("John of Damascus")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BlandinaAndHerCompanions),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give such courage and endurance to Blandina and her companions, that by their deaths many hearts were turned to thee; Grant that we, in accordance with their example, may also gladly endure all that is required of as we witness to thee in our own day; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Blandina and Her Companions")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheodoreOfTarsus),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give thy servant Theodore of Tarsus gifts of grace and wisdom to establish unity where there had been division and order where there had been chaos: Create in thy church, we pray thee, by the operation of the Holy Ghost, such godly union and concord that it may proclaim, both by word and example, the Gospel of the Prince of Peace; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Theodore of Tarsus")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Joseph),
        CollectData::from(
            Document::from(
                Text::from("O God, who from the family of thy servant David didst raise up Joseph to be the guardian of thy incarnate Son and the spouse of his virgin mother: Give us grace to imitate his uprightness of life and his obedience to thy commands; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::GeorgeAugustusSelwyn),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, whose servant George Augustus Selwyn didst lay a firm foundation for the growth of thy church in many nations: Raise up in this and every land evangelists and heralds of thy kingdom, that thy church may proclaim the unsearchable riches of our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("George Augustus Selwyn")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnHenryHobart),
        CollectData::from(
            Document::from(
                Text::from("Revive thy church, Lord God of hosts, wheneverit falls into complacency and sloth, by raisingup devoted leaders like thy servant John Henry Hobart; and grantthat their faith and vigor of mind may awaken thy people to thy message and their mission; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Henry Hobart")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JuanaInesDeLaCruz),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, Source of all knowledge, we give thee thanks for the witness of thy servant Juana Inés de la Cruz in her fierce passion for learning and creativity. Teach us, we beseech thee, so to be faithful stewards of our minds and hearts, that, following her example, we might forever proclaim the riches of thine unending love in Christ Jesus our Lord. Through the same Jesus Christ who, with thee and the Holy Ghost, livest and reignest for ever and ever.")
                    .response("Amen.")
                )
                .label("Juana Ines de La Cruz")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::James),
        CollectData::from(
            Document::from(
                Text::from("O Gracious God, we remember before thee this day thy servant and apostle James, first among the Twelve to suffer martyrdom for the Name of Jesus Christ; and we pray that thou wilt pour out upon the leaders of thy church that spirit of self-denying service by which alone they may have true authority among thy people; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheVisitation),
        CollectData::from(
            Document::from(
                Text::from("Father in heaven, by whose grace the virgin mother of thy incarnate Son was blessed in bearing him, but still more blessed in keeping thy word: Grant us who honor the exaltation of her lowliness to follow the example of her devotion to thy will; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MarinaTheMonk),
        CollectData::from(
            Document::from(
                Text::from("Give us grace, Lord God, to refrain from judgments about the sins of others; that, like thy servant Marina the Monk, we may hold fast to the path of discipleship in the midst of unjust judgments; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Marina the Monk")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TitusAndTimothy),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call Timothy and Titus to do the work of evangelists and teachers, and didst make them strong to endure hardship: Strengthen us to stand fastin adversity, and to live godly and righteous lives in this present time, that with sure confidence we may look for our blessed hope, the glorious appearing of our great God and Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Titus and Timothy")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FrancesPerkins),
        CollectData::from(
            Document::from(
                Text::from("Loving God, we bless thy Name for Frances Perkins, who in faithfulness to her baptism envisioned a society in which all might live in health and decency: Help us, following her example and in union with her prayers, to contend tirelessly for justice and for the protection of all, that we may be faithful followers of Jesus Christ; who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Frances Perkins")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MarcellaOfRome),
        CollectData::from(
            Document::from(
                Text::from("O God, who dost satisfy the longing soul and fillest the hungry with good things: Grant that we, like thy servant Marcella, may hunger and thirst after thee above the vain pomp and glory of the world, and delight in thy word above all manner of riches; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God now and for ever.")
                    .response("Amen.")
                )
                .label("Marcella of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::John),
        CollectData::from(
            Document::from(
                Text::from("Shed upon thy church, we beseech thee, O Lord, the brightness of thy light; that we, being illumined by the teaching of thine apostle and evangelist John, may so walk in the light of thy truth, that we may at length attain to the fullness of life everlasting; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HelenaOfConstantinople),
        CollectData::from(
            Document::from(
                Text::from("Most Merciful God, who didst vouchsafe to bless thy servant Helena with such grace and devotion to thee that she didst venerate the very footsteps of our Savior; Grant unto us the same grace that, aided by her prayers and example, we too may evermore behold thy glory in the cross of thy Son. Through the same Jesus Christ our Lord; who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Helena of Constantinople")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasKen),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give thy servant Thomas Ken grace and courage to bear witness to the truth before rulers and kings: Give us strength also that we may constantly defend what is right, boldly reprove what is evil, and patiently suffer for the truth’s sake; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Ken")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AlcuinOfYork),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst raise up thy servant Alcuin as a beacon of learning: Shine in our hearts, we pray, that we may also show forth thy praise in our own generation, for thou hast called us out of darkness and into thy marvelous light; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Alcuin of York")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::RobertGrosseteste),
        CollectData::from(
            Document::from(
                Text::from("O God, our heavenly Father, who didst raise up thy faithful servant Robert Grosseteste to be a bishopand pastor in thy church and to feed thy flock: Give abundantly to all pastors the gifts of thy Holy Ghost,that they may minister in thy household as true servants of Christ and stewards of thy divine mysteries; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Robert Grosseteste")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::KamehamehaAndEmma),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst call thy servants Kamehameha and Emma to an earthly throne that they might advance thy heavenly kingdom, and gave them zeal for thy church and love for thy people: Mercifully grant that we also may be fruitful in good works and attain to the glorious crown of thy saints; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Kamehameha and Emma")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HannahMore),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose only-begotten Son led captivity captive: Multiply among us faithful witnesses like thy servant Hannah More, who will fight for all who are oppressed or held in bondage, and bring us all, we pray, into the glorious liberty that thou hast promised to all thy children; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Hannah More")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HildegardOfBingen),
        CollectData::from(
            Document::from(
                Text::from("God of all times and seasons: Give us grace that we, after the example of thy servant Hildegard, may both know and make known the joy and jubilation of being part of thy creation, and show forth thy glory in the world; through Jesus Christ our Savior, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Hildegard of Bingen")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Bede),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who hast enriched thy church with the learning and holiness of thy servant Bede: Grant us to find in Scripture and disciplined prayer the image of thy Son our Savior Jesus Christ, and to fashion our lives according to his likeness, to the glory of thy great Name and to the benefit of thy holy church; through the same Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Bede")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AugustineOfHippo),
        CollectData::from(
            Document::from(
                Text::from("Lord God, the light of the minds that know thee, the life of the souls that love thee, and the strength of the hearts that serve thee: Help us, following the example of thy servant, Augustine of Hippo, so to know thee that we may truly love thee, and so to love thee that we may fully serve thee, whose service is perfect freedom; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Augustine of Hippo")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Ninian),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the preaching of thy blessedservant and bishop Ninian didst cause the light of the Gospel to shine in the land of Britain: Grant,we beseech thee, that, having his life and labors in remembrance, we may show forth our thankfulness by following the example of his zeal and patience; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Ninian")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesHannington),
        CollectData::from(
            Document::from(
                Text::from("O God, by whose providence the blood of the martyrs is the seed of the church: Grant that we who remember before thee James Hannington and his companions, may, like them, be steadfast in our faith in Jesus Christ, to whom they gave obedience unto death, and by their sacrifice brought forth a plentiful harvest; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Hannington")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::NinoOfGeorgia),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant Nino to be thine apostle to the people of Georgia, to bring those wandering in darkness to the true light and knowledge of thee; Grant us so to walk in that light, that we may come at last to the light of thine everlasting day; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Nino of Georgia")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MaryOfEgypt),
        CollectData::from(
            Document::from(
                Text::from("Merciful Lord, who dost raise up sinners by thy boundless compassion and mercy: Cause the desert sun to burn away our coarseness and to melt our hardness of heart, that, like thy servant Mary of Egypt, we may not depart from this life until we understand the ways of repentance and the benefits of prayer; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy God, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Mary of Egypt")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FrancisOfAssisi),
        CollectData::from(
            Document::from(
                Text::from("Most high, omnipotent, good Lord, grant thy people grace to renounce gladly the vanities of this world; that, following the way of blessed Francis, we may, for love of thee, delight in thy whole creation with perfectness of joy; through Jesus Christ our Lord, who lives and reigns with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Francis of Assisi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Tikhon),
        CollectData::from(
            Document::from(
                Text::from("Holy God, holy and mighty, who hast called us together into one communion and fellowship: Open our eyes, we pray thee, as thou didst open the eyes of thy servant Tikhon, that we may see the faithfulness of others as we strive to be steadfast in the faith delivered unto us, that the world may see and know thee; through Jesus Christ our Lord, to whom, with thee and the Holy Ghost, be glory and praise unto ages of ages.")
                    .response("Amen.")
                )
                .label("Tikhon")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Photini),
        CollectData::from(
            Document::from(
                Text::from("O Almighty God, whose most blessed Son didst reveal to the Samaritan woman that He is indeed the Christ, the Savior of the World; Grant us to drink of the well that springeth up to everlasting life that we may worship Thee in spirit and in truth through thy Son, Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Photini")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HenryMartyn),
        CollectData::from(
            Document::from(
                Text::from("O God of the nations, who gave to thy servant Henry Martyn a longing to share thy Gospel with all peoples; Inspire the church in our own day, we beseech thee, with that same desire, that we may be eager to commit both life and talents to thee who didst bestow them; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Henry Martyn")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JonathanMyrickDaniels),
        CollectData::from(
            Document::from(
                Text::from("O God of justice and compassion, who didst put down the proud and the mighty from their place, and dost lift up the poor and the afflicted: Wegive thee thanks for thy faithful witness Jonathan Myrick Daniels, who, in the midst of injustice and violence, risked and gave his life for another; and we pray that we, following his example, may make no peace with oppression; through Jesus Christ our Savior, who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Jonathan Myrick Daniels")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FrederickDouglass),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, we bless thy Name for the witness of Frederick Douglass, whose impassioned and reasonable speech moved the hearts of peopleto a deeper obedience to Christ: Strengthen us also to speak on behalf of those in captivity and tribulation, continuing in the way of Jesus Christ our Liberator; who with thee and the Holy Ghost dwelleth in glory everlasting.")
                    .response("Amen.")
                )
                .label("Frederick Douglass")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BasilOfCaesarea),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who hast revealed to thy church thine eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace that, like thy bishop Basil of Caesarea, we may continue steadfastly in the confession of this faith and remain constant in our worship of thee, Father, Son, and Holy Ghost; ever one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Basil of Caesarea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfJapan),
        CollectData::from(
            Document::from(
                Text::from("O God our Father, who didst bring the holy martyrs of Japan through the suffering of the cross to the joys of eternal life: Grant that we, encouraged by their example, may hold fast to the faith we profess, even unto death itself; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of Japan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Bakhita),
        CollectData::from(
            Document::from(
                Text::from("O God of Love, who didst deliver thy servant Josephine Margaret Bakhita from the bondage of slavery to the true freedom of thy service; Grant to the wounded thy healing grace in mind, body, and spirit and to thy church the zeal to combat exploitation and slavery in all its forms; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Bakhita (Josephine Margaret Bakhita)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JosephButler),
        CollectData::from(
            Document::from(
                Text::from("O God, who dost raise up scholars for thy church in every generation; we praise thee for the wisdom and insight granted to thy bishop and theologian Joseph Butler, and pray that thy church may never be destitute of such gifts; through Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Joseph Butler")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AgnesAgathaLucy),
        CollectData::from(
            Document::from(
                Text::from("Lord Jesus Christ, who willingly walked the wayof the cross: Strengthen thy church through the witness of thy servants Agnes Tsao Kou Ying, Agatha Lin Zhao, and Lucy Yi Zhenmei to hold fast to the path of discipleship even unto death; for with the Father and Holy Ghost thou livest and reignest, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Agnes Tsao Kou Ying")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MartinOfTours),
        CollectData::from(
            Document::from(
                Text::from("Lord God of hosts, who didst clothe thy servant Martin the soldier with the spirit of sacrifice and set him as a bishop in thy church to be a defender of the catholic faith: Give us grace to follow in his holy steps, that, at the last, we may be found clothed with righteousness in the dwellings of peace; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Martin of Tours")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Epiphany),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the leading of a star didst manifest thy only-begotten Son to the peoples of the earth: Lead us, who know thee now by faith, to thy presence, where we may behold thy glory face to face; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Matthew),
        CollectData::from(
            Document::from(
                Text::from("We thank thee, heavenly Father, for the witness of thine apostle and evangelist Matthew to the Gospel of thy Son our Savior; and we pray that, after his example, we may with ready wills and hearts obey the calling of our Lord to follow him; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FrancisDeSalesJaneDeChantal),
        CollectData::from(
            Document::from(
                Text::from("Most Gracious God, who hast bidden us to act justly, love mercy, and walk humbly before thee; Grant that we, like thy servants Francis and Jane, may see and to serve Christ in all people, and know him as the giver of all good things; Through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Francis de Sales")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FlorenceNightingale),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst give grace to thy servant Florence Nightingale to bear thy healing love into the shadow of death: Grant unto all who healthe same virtues of patience, mercy, and steadfast love, that thy saving health may be revealed to all; through Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Florence Nightingale")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PaulaAndEustochium),
        CollectData::from(
            Document::from(
                Text::from("Compel us, O God, to attend diligently to thy Word, as didst thy faithful servants Paula and Eustochium; that, by the inspiration of the Holy Ghost, we may find it profitable for doctrine, for reproof, for correction, and for instruction in righteousness; and that thereby we may be made wise unto salvation through faith in Christ Jesus our Lord.")
                    .response("Amen.")
                )
                .label("Paula and Eustochium of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CatherineBarbaraMargaret),
        CollectData::from(
            Document::from(
                Text::from("Embolden thy church, O God, with the stories of thy saints Catherine, Barbara, and Margaret, that we might face all trials and adversities with a fearless mind and an unbroken spirit, knowing that we are more than conquerors through Jesus Christ who strengthens us.  Through the same Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Catherine of Alexandria")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PeterWilliamsCassey),
        CollectData::from(
            Document::from(
                Text::from("O God of justice and mercy, we remember before thee thy servants Peter Williams Cassey and Anna Besant Cassey, who, in the face of slavery and discrimination, gave the blessings of education and spiritual haven to the marginalized; Grant us to be fearless in the face of injustice and to work for blessings that will touch those whom the world does not count of value; through Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth for ever and ever.")
                    .response("Amen.")
                )
                .label("Peter Williams Cassey")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryOfNazianzus),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who hast revealed to thy Church thine eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace that, like thy bishop Gregory of Nazianzus, we may continue steadfast in the confession of this faith, and constant in our worship of thee, Father, Son, and Holy Ghost; who livest and reignest for ever and ever.")
                    .response("Amen.")
                )
                .label("Gregory of Nazianzus")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamTyndale),
        CollectData::from(
            Document::from(
                Text::from("Reveal to us thy saving word, O God, that like thy servant William Tyndale we might hear its call to repentance and new life.  Plant in our hearts that same consuming passion to bring the scriptures to all people in their native tongue, and the strength to endure amidst all obstacles; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Tyndale")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AthanasiusOfAlexandria),
        CollectData::from(
            Document::from(
                Text::from("O Lord, who didst establish thy servant Athanasius, through wisdom, in thy truth: Grant that we, perceiving the humanity and divinity of thy Son Jesus Christ, may follow in his footsteps and ascend the way to eternal life; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Athanasius of Alexandria")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AlexanderCrummell),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, we give thanks to thee for thy servant Alexander Crummell, whom thou didst call to preach the gospel to those who were far off and to those who were near: Raise up, we beseech thee, in this and every land, evangelists and heralds of thy kingdom, that thy church may proclaim the unsearchable riches of our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Alexander Crummell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesLloydBreck),
        CollectData::from(
            Document::from(
                Text::from("O God, who sent thy Son to preach peace to those who are far off and to those who are near: call us from comfortable complacency to preach, teach, and plant thy church on new frontiers, after the example of thy servant James Lloyd Breck; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Lloyd Breck")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Barnabas),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that we may follow the example of thy faithful servant Barnabas, who, seeking not his own renown but the well-being of thy church, gave generously of his life and substance for the relief of the poor and the spread of the Gospel; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AbsalomJones),
        CollectData::from(
            Document::from(
                Text::from("Set us free, O heavenly Father, from every bondof prejudice and fear; that, honoring the steadfast courage of thy servant Absalom Jones, we may show forth in our lives the reconciling love and true freedom of the children of God, which thou hast given us in thy Son our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Absalom Jones")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AdelaideTeagueCase),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who dost raise up educators and teachers of the faith in every generation of thy church: Grant that following the example of thy servant Adelaide Teague Case, we might be bold to proclaim the reconciling power of Christ’s love in our own generation.  Through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Adelaide Teague Case")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::VincentDePaul),
        CollectData::from(
            Document::from(
                Text::from("Most Gracious God, who hast bidden us to act justly, love mercy, and walk humbly before thee; Teach us, like thy servants Vincent de Paul and Louise de Marillac, to see and to serve Christ by feeding the hungry, welcoming the stranger, clothing the naked, and caring for the sick; that we may know him to be the giver of all good things, through the same, Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Vincent de Paul")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PriscillaAndAquila),
        CollectData::from(
            Document::from(
                Text::from("God of grace and might, who didst plenteously endow thy servants Priscilla and Aquila with gifts of zeal and eloquence to make known the truth of the Gospel: Raise up, we pray thee, in every country, heralds and evangelists of thy kingdom, that the world may know the immeasurable riches of our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Priscilla and Aquila")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JoannaMarySalome),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst reveal the resurrection of thy Son to Joanna, Mary and Salome as they faithfully came bearing myrrh to his tomb: Grant that we too may perceive the presence of the risen Lord in the midst of pain and fear, and go forth proclaiming his resurrection; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Joanna")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HilaryOfPoitiers),
        CollectData::from(
            Document::from(
                Text::from("Keep us steadfast, Lord God, in that true faith that we professed at our baptism; that, like thy servant Hilary of Poitiers, we may rejoice in having thee for our Father, and may abide in thy Son, in the fellowship of the Holy Ghost; for thou livest and reignest for ever and ever as one God in Trinity of Persons.")
                    .response("Amen.")
                )
                .label("Hilary of Poitiers")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesDeKoven),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who led thy servant James De Koven to honor thy presence at the altar, and constantly to point to Christ: Grant that all ministers and stewards of thy mysteries may impart to thy faithful people the knowledge of thy presence and the truth of thy grace; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James de Koven")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::LucyOfSyracuse),
        CollectData::from(
            Document::from(
                Text::from("Loving God, who for the salvation of all didst give Jesus Christ as light to a world in darkness: Illumine us, as thou didst thy daughter Lucy, with the light of Christ, that by the merits of his passion, we may be led to eternal life; through the same Jesus Christ, who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Lucy of Syracuse")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AmbroseOfMilan),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst give thy servant Ambrose grace eloquently to proclaim thy righteousness in the great congregation and fearlessly to bear reproach for the honor of thy Name: Mercifully grant to all bishops and pastors such excellence in preaching and faithfulness in ministering thy Word, that thy people may be partakers with them of the glory that shall be revealed; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Ambrose of Milan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EuphrosynesmaragdusOfAlexandria),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who lookest not with outward eyes but dost discern the heart of each: we confess that those whom we love the most are often strangers to us. Give to all parents and children, we pray, the grace to see one another as they truly are and as thou hast called them to be.  All this we ask in the name of Jesus Christ, our only mediator and advocate.")
                    .response("Amen.")
                )
                .label("Euphrosyne/Smaragdus of Alexandria Monastic")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ClementOfAlexandria),
        CollectData::from(
            Document::from(
                Text::from("O God of unsearchable wisdom, who didst give thy servant Clement grace to understand and teach the truth as it is in Jesus Christ, the source of all truth: Grant to thy church the same grace to discern thy Word wherever truth is found; through Jesus Christ our unfailing light, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Clement of Alexandria")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MaryAndMarthaOfBethany),
        CollectData::from(
            Document::from(
                Text::from("O God, heavenly Father, whose Son Jesus Christ enjoyed rest and refreshment in the home of Mary and Martha of Bethany: Give us the will to love thee, open our hearts to hear thee, and strengthen our hands to serve thee in others for his sake; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Mary and Martha of Bethany")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CharlesHenryBrent),
        CollectData::from(
            Document::from(
                Text::from("Heavenly Father, whose Son prayed that we all might be one: Deliver us from arrogance and prejudice, and give us wisdom and forbearance, that, following thy servant Charles Henry Brent, we may be united in one family with all who confess the Name of thy Son Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Charles Henry Brent")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::IsabelFlorenceHapgood),
        CollectData::from(
            Document::from(
                Text::from("Teach thy divided church, O God, so to follow the example of thy servant Isabel Florence Hapgood that we might look upon one another with a holy envy, to honor whatever is good and right in our separate traditions, and to continually seek the unity that thou desirest for all thy people. We ask this in the name of Jesus Christ our Lord, who didst pray that his church might be one. ")
                    .response("Amen.")
                )
                .label("Isabel Florence Hapgood")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FrederickDenisonMaurice),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who hast restored our human nature to heavenly glory through the perfect obedience of our Savior Jesus Christ: Enliven in thy Church, we beseech thee, the passion for justice and truth, that, like thy servant Frederick Denison Maurice, we may work and pray for the triumph of the kingdom of Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Frederick Denison Maurice")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnOfTheCross),
        CollectData::from(
            Document::from(
                Text::from("Judge eternal, throned in splendor, who didst give John of the Cross strength of purpose and faith that sustained him even through the dark night of the soul:  Shed thy light on all who love thee, in unity with Jesus Christ our Savior; who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John of the Cross")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PolycarpOfSmyrna),
        CollectData::from(
            Document::from(
                Text::from("O God, the maker of heaven and earth, who didst give thy venerable servant, the holy and gentle Polycarp, the boldness to confess Jesus Christ as King and Savior and the steadfastness to die for that faith: Give us grace, following his example, to share the cup of Christ and to rise to eternal life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Polycarp of Smyrna")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CyprianOfCarthage),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who gave to thy servant Cyprian boldness to confess the Name of our Savior Jesus Christ before the rulers of this world and courage to die for this faith: Grant that we may always be ready to give a reason for the hope that is in us and to suffer gladly for the sake of our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Cyprian of Carthage")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::DorothyLSayers),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant Nino to be thine apostle to the people of Georgia, to bring those wandering in darkness to the true light and knowledge of thee; Grant us so to walk in that light, that we may come at last to the light of thine everlasting day; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Dorothy L Sayers")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThereseOfLisieux),
        CollectData::from(
            Document::from(
                Text::from("O Gracious Father, who didst call thy servant Thérèse to a life of fervent prayer: Give unto us that spirit of prayer and zeal for the ministry of the Gospel, that the love of Christ may be known throughout all the world; through the same, Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Therese of Lisieux")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CharlesDeFoucauld),
        CollectData::from(
            Document::from(
                Text::from("Loving God, help us to know thee wherever we find thee and seek to serve thee in all people, that with thy servant Charles de Foucauld, we may be faithful even unto death; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Charles de Foucauld")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Enmegahbowh),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst lead thy pilgrim people of old by fire and cloud: Grant that the ministers of thy church, following the example of thy servant Enmegahbowh, may lead thy people with fiery zeal and gentle humility; through Jesus Christ, who liveth and reigneth with thee in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Enmegahbowh")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AgathaOfSicily),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who didst strengthen thy martyr Agatha with constancy and courage: Grant us for the love of thee to make no peace with oppression, to fear no adversity, and to have no tolerance for those who wouldst use their power to abuse or exploit; Through Jesus Christ our Lord, to whom with thee and the Holy Ghost be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Agatha of Sicily")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheBeheadingOfSaintJohnTheBaptist),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant John to be the forerunner of thy Son our Lord both in life and death; Grant, we beseech thee, that as we remember his faithfulness unto death, we may with boldness speak thy truth and with humility be ready to hear it; through Jesus Christ, the firstborn from the dead, who with Thee and the Holy Ghost, livest and reignest one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Beheading of Saint John the Baptist")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::RichardHooker),
        CollectData::from(
            Document::from(
                Text::from("O God of truth and peace, who didst raise up thy servant Richard Hooker in a day of bitter controversy to defend with sound reasoning and great charity the catholic and reformed religion: Grant that we may maintain that middle way, not as a compromise for the sake of peace, but as a comprehension for the sake of truth; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Richard Hooker")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HildaOfWhitby),
        CollectData::from(
            Document::from(
                Text::from("O God of peace, by whose grace the abbess Hilda was endowed with gifts of justice, prudence, and strength to rule as a wise mother over the nuns and monks of her household: Raise up these gifts in us, that we, following her example and prayers, might build up one another in love to the benefit of thy church; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Hilda of Whitby")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HolyCross),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose Son our Savior Jesus Christ was lifted high upon the cross that he might draw the whole world unto himself: Mercifully grant that we, who glory in the mystery of our redemption, may have grace to take up our cross and follow him; who liveth and reigneth with thee and the Holy Ghost, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::DavidOfWales),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant David to be a faithful and wise steward of thy mysteries for the people of Wales: Mercifully grant that, following his purity of life and zeal for the Gospel of Christ, we may, with him, praise thee both here on earth and also in thy everlasting kingdom; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("David of Wales")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::DavidPendletonOakerhater),
        CollectData::from(
            Document::from(
                Text::from("O God of unsearchable wisdom and mercy; Liberate us from bondage to self, and empower us to serve thee and our neighbors that like thy servant David Oakerhater, we might bring those who do not know thee to the knowledge and love of thee; through Jesus Christ, the captain of our salvation, who liveth and reigneth with thee and the Holy Ghost, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("David Pendleton Oakerhater")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryOfNyssa),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who hast revealed to thy Church thine eternal Being of glorious majesty and perfect love as one God in Trinity of Persons: Give us grace that, like thy bishop Gregory of Nyssa, we may continue steadfast in the confession of this faith, and constant in our worship of thee, Father, Son, and Holy Ghost; who livest and reignest now and for ever.")
                    .response("Amen.")
                )
                .label("Gregory of Nyssa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::LatimerRidleyCranmer),
        CollectData::from(
            Document::from(
                Text::from("Keep us, O Lord, constant in faith and zealous in witness, that, like thy servants Hugh Latimer, Nicholas Ridley, and Thomas Cranmer we may live in thy fear, die in thy favor, and rest in thy peace; for the sake of Jesus Christ, thy Son our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Hugh Latimer and Nicholas Ridley")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamAugustusMuhlenberg),
        CollectData::from(
            Document::from(
                Text::from("Open the eyes of thy church, O Lord, to the plight of the poor and neglected, the homeless and destitute, the old and the sick, the lonely and those who have none to care for them. Give unto us the vision and compassion with which thou didst so richly endow thy servant William Augustus Muhlenberg, that we may labor tirelessly to heal those who are broken in body or spirit, and to turn their sorrow into joy; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Augustus Muhlenberg")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Mary),
        CollectData::from(
            Document::from(
                Text::from("O God, who hast taken to thyself the blessed Virgin Mary, mother of thy incarnate Son: Grant that we, who have been redeemed by his blood, may share with her the glory of thine eternal kingdom; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AllSaintsDay),
        CollectData::from(
            Document::from(
                Text::from("O Almighty God, who hast knit together thine elect in one communion and fellowship in the mystical body of thy Son Christ our Lord: Grant us grace so to follow thy blessed saints in all virtuous and godly living, that we may come to those ineffable joys which thou hast prepared for those who unfeignedly love thee; through the same Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::LydiaOfThyatira),
        CollectData::from(
            Document::from(
                Text::from("Eternal God, who givest good gifts to all people and dost grant the spirit of generosity: Give unto us, we pray thee, hearts always open to hear thy word, that, following the example of thy servant Lydia, we may show hospitality to those who are in any need or trouble; through Jesus Christ our Lord who lives and reigns with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Lydia of Thyatira")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThePresentation),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everliving God, we humbly beseech thee that, as thy only-begotten Son was this day presented in the temple, so we may be presented unto thee with pure and clean hearts by the same thy Son Jesus Christ our Lord; who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HarrietMonsell),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, who didst lead thy servant Harriet Monsell through grief to a new vocation; grant that we, inspired by her example, may grow in the life of prayer and the work of service, so that in all our sorrows and in all our joys, thy presence may evermore increase among us, and that our lives may be so ordered as to reveal the mind of Christ, to whom with thee and the Holy Ghost, be honor and glory, now and for ever. ")
                    .response("Amen.")
                )
                .label("Harriet Monsell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesTheodoreHolly),
        CollectData::from(
            Document::from(
                Text::from("Most gracious God, whose servant James Theodore Holly labored to build a church in which all might be free: Grant that we might overcome our prejudice, and honor those whom thou dost call from every family, language, people, and nation; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("James Theodore Holly")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AidanOfLindisfarne),
        CollectData::from(
            Document::from(
                Text::from("O loving God, who didst call thy servant Aidan from the cloister to re-establish the Christian mission in northern England: Grant, we beseech thee, that we, following his example, may use what thou hast given us for the relief of human need, and may persevere in commending the saving Gospel of our Redeemer Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Aidan of Lindisfarne")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MartinLutherKing),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who by the hand of Moses thy servant didst lead thy people out of slavery, and didst make them free at last: Grant that thy church, following the example of thy prophet Martin Luther King, may resist oppression in the nameof thy love, and may strive to secure for all thy children the blessed liberty of the Gospel of Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Martin Luther King")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnKeble),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, that in all time of our testing we may know thy presence and obey thy will, that, following the example of thy servant John Keble, we may accomplish with integrity and courage that which thou givest us to do and endure that which thou givest us to bear; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Keble")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MartyrsOfTheReformationEra),
        CollectData::from(
            Document::from(
                Text::from("Almighty and Most Merciful God, give to thy church that peace which the world cannot give, and grant that those who have been divided on earth may be reconciled in heaven, and share together in the vision of thy glory; through Jesus Christ thy Son our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever. ")
                    .response("Amen.")
                )
                .label("Martyrs of the Reformation Era")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamLaw),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose servant William Law taught us to hear and follow your call to a devout and holy life: Grant that we, loving thee above all things and in all things, may seek thy purpose and shape our actions to thy will, that we may grow in all virtue and be diligent in prayer all the days of our lives, through Jesus Christ our Lord, to whom with thee and the Holy Ghost be all honor and glory now and for ever. ")
                    .response("Amen.")
                )
                .label("William Law")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AugustineOfCanterbury),
        CollectData::from(
            Document::from(
                Text::from("O Lord our God, who by thy Son Jesus Christdidst call thy servant Augustine to preach the Gospel to the English people: We pray that all whom thou dost call and send may do thy will, bide thy time, and see thy glory; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Augustine")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Alfred),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst call thy servant Alfred to an earthly throne that he might advance thy heavenly kingdom and didst give him zeal for thy church and love for thy people: Grant that we, inspired by his example and prayers, may remain steadfast in the work thou hast given us to do for the building up of thy reign of love; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Alfred")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PatrickOfIreland),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who in thy providence didst choose thy servant Patrick to be the apostle to the Irish people, to bring those who were wandering in darkness and error to the true light and knowledge of thee: Grant us so to walk in that way that we may come at last to the light of everlasting life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Patrick of Ireland")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfUganda),
        CollectData::from(
            Document::from(
                Text::from("O God, by whose providence the blood of the martyrs is the seed of the church: Grant that we who remember before thee the blessed martyrs of Uganda, may, like them, be steadfast in our faith in Jesus Christ, to whom they gave obedience unto death, and by their sacrifice brought forth a plentiful harvest; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of Uganda")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasBecket),
        CollectData::from(
            Document::from(
                Text::from("O God, our strength and our salvation, who didst call thy servant Thomas Becket to be a shepherd of thy people and a defender of thy church; Keep thy household from all evil and raise up faithful pastors and leaders who are wise in the ways of the gospel; through Jesus Christ, the shepherd of our souls, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Becket")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::IrenaeusOfLyons),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst strengthen thy servant Irenaeus to defend thy truth against every blast of vain doctrine: Keep us, we pray, steadfast in thy true religion, that in constancy and peace we may walk in the way that leads to eternal life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Irenaeus of Lyons")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PhilipAndJames),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give to thine apostles Philip and James grace and strength to bear witness to the truth: Grant that we, being mindful of their victory of faith, may glorify in life and death the Name of our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::DietrichBonhoeffer),
        CollectData::from(
            Document::from(
                Text::from("Embolden our lives, O Lord, and inspire our faiths, that we, following the example of thy servant Dietrich Bonhoeffer, might embrace thy call with undivided hearts; through Jesus Christ our Savior, with liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Dietrich Bonhoeffer")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnChrysostom),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst give thy servant John Chrysostom grace eloquently to proclaim thy righteousness in the great congregation, and fearlessly to bear reproach for the honor of thy Name: Mercifully grant to all who proclaim thy word such excellence in preaching that all thy people may be made partakers of the glory that shall be revealed; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("John Chrysostom")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PerpetuaAndFelicity),
        CollectData::from(
            Document::from(
                Text::from("O God, the King of Saints, who didst strengthen thy servants Perpetua, Felicity, and their companions to make a good confession and to encourage one another in the time of trial: Grant that we who cherish their blessed memory may share their pure and steadfast faith, and win with them the palm of victory; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Perpetua and Felicity")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Michael),
        CollectData::from(
            Document::from(
                Text::from("O Everlasting God, who hast ordained and constituted the ministries of angels and men in a wonderful order: Mercifully grant that, as thy holy angels always serve and worship thee in heaven, so by thy appointment they may help and defend us on earth; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohannArndtAndJacobBoehme),
        CollectData::from(
            Document::from(
                Text::from("Holy God, who dwellest with them that are of a contrite and humble spirit: Revive our spirits; purify us from deceitful lusts; and cloth us in righteousness and true holiness; though Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God now and for ever.")
                    .response("Amen.")
                )
                .label("Johann Arndt and Jacob Boehme")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnXxiiiAngeloGiuseppeRoncalli),
        CollectData::from(
            Document::from(
                Text::from("God of all truth and peace, who didst raise up thy bishop John to be servant of the servants of God and bestowed on him wisdom to call for the work of renewing thy church: Grant that, following his example, we may reach out to other Christians in the love of thy Son, and labor throughout the nations of the world to kindle a desire for justice and peace; through Jesus Christ our Lord, who livest and reignest with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("John XXIII (Angelo Giuseppe Roncalli)")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::KateriTekakwitha),
        CollectData::from(
            Document::from(
                Text::from("O God of justice and mercy, we remember before thee thy servants Peter Williams Cassey and Anna Besant Cassey, who, in the face of slavery and discrimination, gave the blessings of education and spiritual haven to the marginalized; Grant us to be fearless in the face of injustice and to work for blessings that will touch those whom the world does not count of value; through Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth for ever and ever.")
                    .response("Amen.")
                )
                .label("Kateri Tekakwitha")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamTemple),
        CollectData::from(
            Document::from(
                Text::from("O God of light and love, who illumined thy church through the witness of thy servant William Temple: Inspire us, we pray, by his teaching and example, that we may rejoice with courage, confidence, and faith in the Word made flesh, and may be led to establish that city which has justice for its foundation and love for its law; through Jesus Christ, the light of the world, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("William Temple")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AllSoulsDay),
        CollectData::from(
            Document::from(
                Text::from("O God, the Maker and Redeemer of all believers: Grant to the faithful departed the unsearchable benefits of the passion of thy Son; that on the day of his appearing they may be manifested as thy children; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AntonyOfEgypt),
        CollectData::from(
            Document::from(
                Text::from("O God, as thou by thy Holy Ghost didst enable thy servant Antony to withstand the temptations of the world, the flesh, and the devil; so give us grace to follow thee with pure hearts and minds; through Jesus Christ our Lord, who liveth and reigneth with thee and the same Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Antony of Egypt")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ElisabethCruciger),
        CollectData::from(
            Document::from(
                Text::from("Pour out thy Spirit upon all of thy sons and daughters, Almighty God, that like thy servant Elisabeth Cruciger our lips may praise thee, our lives may bless thee, and our worship may give thee glory; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Elisabeth Cruciger")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ClareOfAssisi),
        CollectData::from(
            Document::from(
                Text::from("O God, whose blessed Son became poor that we, through his poverty, might become rich: Deliver us, we pray thee, from an inordinate love of this world, that we, inspired by the devotion of thy servant Clare, might serve thee with singleness of heart and attain to the riches of the age to come; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Clare of Assisi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HarrietStarrCannon),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, who didst call Harriet Starr Cannon and her companions to revive the monastic vocation in the Episcopal Church and to dedicate their lives to thee: Grant that we, after their example, may ever surrender ourselves to the revelation of thy holy will; through our Lord and Savior Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("Harriet Starr Cannon")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::RichardOfChichester),
        CollectData::from(
            Document::from(
                Text::from("Almighty and most merciful God, who callest thy people to thyself, we pray that, following the example of thy bishop Richard of Chichester, we may see thy Son Jesus Christ more clearly, love him more dearly, and follow him more nearly; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Richard of Chichester")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MargaretOfScotland),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst call thy servant Margaret to an earthly throne that she might advance thy heavenly kingdom, and gave her zeal for thy church and love for thy people: Mercifully grant that we also may be fruitful in good works and attain to the glorious crown of thy saints; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Margaret of Scotland")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AnselmOfCanterbury),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose servant Anselm helped thy church to understand its faith in thine eternal Being, perfect justice, and saving mercy: Provide thy church in all ages with devout and learned scholars and teachers, that we may be able to give a reason for the hope that is in us; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Anselm of Canterbury")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::RemigiusOfRheims),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who by thy servant Remigius spread the truth of the gospel and the fullness of the catholic faith: Grant that we who glory in the name of Christian may show forth our faith in worthy deeds; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Remigius of Rheims")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CyrilAndMethodius),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who by the power of the Holy Spirit didst move thy servants Cyril and Methodius to bring the light of the Gospel to a hostile and divided people: Overcome, we pray thee, by the love of Christ, all bitterness and contention among us, and make us one united family under the banner of the Prince of Peace; who liveth and reigneth with thee and the same Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Cyril and Methodius")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamPorcherDubose),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give to thy servant William Porcher DuBose special gifts of grace to understand the Scriptures and to teach the truth as it is in Christ Jesus: Grant that by this teaching we may know thee, the one true God, and Jesus Christ whom who hast sent; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("William Porcher Dubose")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BrigidOfKildare),
        CollectData::from(
            Document::from(
                Text::from("O God, whose servant Brigid, kindled with the flame of thy love, became a shining light in thy church: Grant that we also may be aflame with the spirit of love and discipline, and walk before thee as children of light; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Brigid of Kildare")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheParentsOfTheBlessedVirginMary),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, heavenly Father, we remember in thanksgiving this day the parents of the Blessed Virgin Mary; and we pray that we all may be made one in the heavenly family of thy Son Jesus Christ our Lord; who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Parents of the Blessed Virgin Mary")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Kassiani),
        CollectData::from(
            Document::from(
                Text::from("O God of boundless mercy, whose handmaiden Kassiani brought forth poetry and song: Inspire in thy church a new song, that following her most excellent example, we may boldly proclaim the truth of thy Word; even Jesus Christ, our Savior and Deliverer.")
                    .response("Amen.")
                )
                .label("Kassiani")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasBray),
        CollectData::from(
            Document::from(
                Text::from("O God of compassion, who didst open the heartof thy servant Thomas Bray to answer the needs of the church in the New World: Make thy church diligent at all times to propagate the Gospel and to promote the spread of Christian knowledge; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Bray")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HolyName),
        CollectData::from(
            Document::from(
                Text::from("Eternal Father, who didst give to thine incarnate Son the holy name of Jesus to be the sign of our salvation: Plant in every heart, we beseech thee, the love of him who is the Savior of the world, even our Lord Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HughOfLincoln),
        CollectData::from(
            Document::from(
                Text::from("Holy God, who didst endow thy servant Hugh of Lincoln with wise and cheerful boldness, and taught him to commend the discipline of holy life to kings and princes: Grant that we also, rejoicing in the Good News of thy mercy, and fearing nothing but the loss of thee, may be bold to speak the truth in love, in the name of Jesus Christ our Redeemer; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Hugh of Lincoln")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EvelynUnderhill),
        CollectData::from(
            Document::from(
                Text::from("O God, Origin, Sustainer, and End of all creatures: Grant that thy church, taught by thy servant Evelyn Underhill, may continually offer to thee all glory and thanksgiving, and attain with thy saints to the blessed hope of everlasting life, which thou hast promised us by our Savior Jesus Christ; who with thee and the Holy Ghost liveth and reigneth, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Evelyn Underhill")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::RichardMeuxBensonAndCharlesGore),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, who didst kindle in thy servants Richard Meux Benson and Charles Gore the grace to lead a revival of monastic life:  Grant us also the resolve to serve thee faithfully in contemplation and prayer, ministering to the world that thou hast made, through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, ever one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Richard Meux Benson and Charles Gore]")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WulfstanOfWorcester),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose only-begotten Son led captivity captive and gave gifts to thy people: Multiply among us faithful pastors, who, like thy holy bishop Wulfstan, will give courage to those who are oppressed and held in bondage; and bring us all, we pray, into the true freedom of thy kingdom; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Wulfstan of Worcester")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AelredOfRievaulx),
        CollectData::from(
            Document::from(
                Text::from("Grant to thy people, Almighty God, a spirit of mutual affection; that, following the example of thy servant Aelred of Rievaulx, we might know the love of Christ in loving one another; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Aelred of Rievaulx")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Alban),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose grace and power thy holy martyr Alban triumphed over suffering and was faithful even unto death: Grant us, who now remember him in thanksgiving, to be so faithfulin our witness to thee in this world that we may receive with him the crown of life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Alban")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::IgnatiusOfAntioch),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, we praise thy Name for thy bishop and martyr Ignatius of Antioch, who offered himself as grain to be ground by the teeth of wild beasts that he might present to thee the pure bread of sacrifice. Accept, we pray, the willing tribute of our lives and give us a share in the pure and spotless offering of thy Son Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Ignatius of Antioch")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AgnesAndCeciliaOfRome),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who dost choose those whom the world deemeth powerless to put the powerful to shame: Grant us so to cherish the memory of thy youthful martyrs Agnes and Cecilia, that we might share their pure and steadfast faith in thee; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God for ever and ever.")
                    .response("Amen.")
                )
                .label("Agnes and Cecilia of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Phoebe),
        CollectData::from(
            Document::from(
                Text::from("Eternal God, who didst raise up Phoebe as a deacon in thy church and minister of thy Gospel; Grant unto us the same grace, that aided by her prayers and example, we too may take the Gospel unto the ends of the earth; through Jesus Christ thy Son our Lord who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Phoebe")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesSolomonRussell),
        CollectData::from(
            Document::from(
                Text::from("O God, the font of resurrected life, draw us into the wilderness and speak tenderly to us, so that we might love and worship thee as did thy servant James Solomon Russell, in assurance of the saving grace of Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("James Solomon Russell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Jerome),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst give us the holy Scriptures as a light to shine upon our path: Grant us, after the example of thy servant Jerome, so to learn of thee according to thy holy Word, that we may find the Light that shines more and more to the perfect day; even Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and ever.")
                    .response("Amen.")
                )
                .label("Jerome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ScholasticaOfNursia),
        CollectData::from(
            Document::from(
                Text::from("Assist us, O God, to love one other as sisters and brothers, and to balance discipline with love and rules with compassion, according to the example shown by thy servant Scholastica; for the sake of thy Son Jesus Christ our Lord, to whom with thee and the Holy Ghost be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Scholastica of Nursia")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MacrinaOfCaesarea),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who didst call thy servant Macrina to reveal in her life and teaching the richesof thy grace and truth: Grant that we, following her example, may seek after thy wisdom and live according to the way of thy Son our Savior Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Macrina of Caesarea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfNewGuinea),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, we remember before thee this day the blessed martyrs of New Guinea, who, following the example of their Savior, laid down their lives for their friends, and we pray that we who honor their memory may imitate their loyalty and faith; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of New Guinea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThurgoodMarshall),
        CollectData::from(
            Document::from(
                Text::from("Eternal and ever-gracious God, who didst bless thy servant Thurgood Marshall with grace and courage to discern and speak the truth: Grant that, following his example, we may know thee and recognize that we are all thy children, brothers and sisters of Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thurgood Marshall")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Theodora),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst call thy servant Theodora to an earthly throne that she might advance thy heavenly kingdom and who didst give her the wisdom to establish unity where there had been division; Create in thy church such godly union and concord that we might proclaim the Gospel of the Prince of Peace, not only in correct theology but in right actions; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Theodora")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::NicholasFerrar),
        CollectData::from(
            Document::from(
                Text::from("Lord God, make us worthy of thy perfect love; that, with thy deacon Nicholas Ferrar and his household, we may rule ourselves according to thy Word, and serve thee with our whole heart; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Nicholas Ferrar")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnRaleighMott),
        CollectData::from(
            Document::from(
                Text::from("Everlasting God, who dost lead thy people's feet into the ways of peace; Raise up, we beseech thee, heralds and evangelists of thy kingdom like thy servant John Mott, that thy church may make known to all the world the unsearcheable riches and unsurpassed peace of thy Son, Jesus Christ our Lord; to whom with thee and the Holy Ghost be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("John Raleigh Mott")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::OscarRomero),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant Óscar Romero to be a voice for the voiceless poor, and to give his life as a seed of freedom and a sign of hope: Grant that we, inspired by his sacrifice and the example of the martyrs of El Salvador, may without fear or favor witness to thy Word who abides, thy Word who is Life, even Jesus Christ our Lord, to whom, with thee and the Holy Ghost, be praise and glory now and for ever.")
                    .response("Amen.")
                )
                .label("Oscar Romero")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JulianOfNorwich),
        CollectData::from(
            Document::from(
                Text::from("Triune God, Father and Mother to us all, who in thy compassion didst grant to your servant Julian many revelations of thy nurturing and sustaining love: Move our hearts, like hers, to seek thee above all things, for in giving us thyself thou givest us all.")
                    .response("Amen.")
                )
                .label("Julian of Norwich")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryTheIlluminator),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst raise up thy servant Gregory to be a light in the world, and to preach the Gospel to the people of Armenia: Illuminate our hearts, that we also in our own generation may show forth thy praise, who hast called us out of darkness and into thy marvelous light; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Gregory the Illuminator")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PachomiusOfTabenissi),
        CollectData::from(
            Document::from(
                Text::from("Set us free, O God, from all false desires, vain ambitious, and everything that would separate us from thy love; that, like thy servant Pachomius, we might give ourselves fully to a life of discipleship, seeking thee alone and serving those whom thou hast given us to serve; through Jesus Christ, our only mediator and advocate.")
                    .response("Amen.")
                )
                .label("Pachomius of Tabenissi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FlorenceLiTimOi),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who dost pour out thy Spirit upon thy sons and daughters: Grant that we, following the example of thy servant Florence Li Tim-Oi, chosen priest in thy church, may with faithfulness, patience, and tenacity proclaim thy holy gospel to all the nations, through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Florence Li Tim-Oi")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EvaLeeMatthews),
        CollectData::from(
            Document::from(
                Text::from("O God, whose blessed Son became poor that we through his poverty might be rich: Deliver us, we pray thee from an inordinate love of this world, that, inspired by the devotion of thy servant Eva Lee Matthews, we may serve thee with singleness of heart, and attain to the riches of the age to come; through the same Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Spirit, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Eva Lee Matthews")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamReedHuntington),
        CollectData::from(
            Document::from(
                Text::from("O Lord our God, we thank thee for instilling in the heart of thy servant William Reed Huntington a fervent love for thy church and its mission in the world; and we pray that, with unflagging faith in thy promises, we may make known to all people thy blessed gift of eternal life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Reed Huntington")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::SimonAndJude),
        CollectData::from(
            Document::from(
                Text::from("O God, we thank thee for the glorious company of the apostles, and especially on this day for Simon and Jude; and we pray that, as they were faithful and zealous in their mission, so we may with ardent devotion make known the love and mercy of our Lord and Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Cuthbert),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who didst call Cuthbert from following the flock to be a shepherd of thy people: Mercifully grant that we also may go without fear to dangerous and remote places, to seek the indifferent and the lost; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Cuthbert")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HadewijchOfBrabant),
        CollectData::from(
            Document::from(
                Text::from("Triune God of Love, overwhelming and all-encompassing: Visit us in our solitude and in our companionship, and draw us ever more deeply into union with thee, who art ever present and ever mysterious; that we, like thy servant Hadewijch, might know thee ever more fully, even as we have been fully known.")
                    .response("Amen.")
                )
                .label("Hadewijch of Brabant")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TabithaDorcasOfJoppa),
        CollectData::from(
            Document::from(
                Text::from("Most Holy God, who didst raise from the dead thy servant Tabitha to display thy power and confirm that thy Son is Lord; Grant unto us thy grace, that, aided by her prayers and example, we may be given a new life in thee, to do works pleasing in thy sight; through Jesus Christ thy Son our Lord; who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Tabitha (Dorcas) of Joppa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EphremOfNisibis),
        CollectData::from(
            Document::from(
                Text::from("Pour out upon us, O Lord, that same Spirit by which thy deacon Ephrem declared the mysteries of faith in sacred song; that, with gladdened hearts, we too might proclaim the riches of thy glory; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Ephrem of Nisibis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ConfessionOfStPeter),
        CollectData::from(
            Document::from(
                Text::from("Almighty Father, who didst inspire Simon Peter, first among the apostles, to confess Jesus as Messiah and Son of the living God: Keep thy Church steadfast upon the rock of this faith, that in unity and peace we may proclaim the one truth and follow the one Lord, our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AnnaJuliaHaywoodCooper),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst inspire thy servant Anna Julia Haywood Cooper with the love of learning and the skill of teaching: Enlighten us more and more through the discipline of learning, and deepen our commitment to the education of all thy children; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Anna Julia Haywood Cooper")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MechthildOfMagdeburg),
        CollectData::from(
            Document::from(
                Text::from("Draw the souls of thy people into thy love, O God; that, like thy servant Mechthild, we may yearn to be fully thine, for thou dost know us better than we can know ourselves; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God now and for ever.")
                    .response("Amen.")
                )
                .label("Mechthild of Magdeburg")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Fabian),
        CollectData::from(
            Document::from(
                Text::from("Grant, Almighty God, that in all times of trial and persecution, we might remain steadfast in faith and endurance, according to the example of thy servant Fabian, who was faithful even unto death. We ask this for the sake of him who laid down his life for us all, Jesus Christ our Savior; who livest and reignest with thee and the Holy Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Fabian")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::VidaDuttonScudder),
        CollectData::from(
            Document::from(
                Text::from("Most gracious God, who didst send thy beloved Son to preach peace to those who are far off and to those who are near: Raise up in thy church witnesses who, after the example of thy servant Vida Dutton Scudder, stand firm in proclaiming the power of the gospel of Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Vida Dutton Scudder")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BernardOfClairvaux),
        CollectData::from(
            Document::from(
                Text::from("O God, by whose grace thy servant Bernard of Clairvaux, kindled with the flame of thy love, became a burning and a shining light in thy church: Grant that we also may be aflame with the spirit of love and discipline and walk before thee as children of light; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Bernard of Clairvaux")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ColumbaOfIona),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the preaching of thy servant Columba didst cause the light of the Gospel to shine in Scotland: Grant, we beseech thee that, having his life and labors in remembrance, we may following the example of his zeal and patience; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Columba of Iona")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::GeorgeHerbert),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy servant George Herbert from the pursuit of worldly honors to be a poet and a pastor of souls: Give us grace, we pray, joyfully to dedicate all our powers to thy service; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("George Herbert")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MelaniaTheElder),
        CollectData::from(
            Document::from(
                Text::from("Most High and Merciful God, who didst call thy servant Melania to forsake earthly comforts that she might devote herself to studying the scriptures and to welcoming the poor: Instruct us in the ways of poverty and the grace of hospitality, that we might comfort those who have no place to rest and teach the way of thy love; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Melania the Elder")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MancheMasemola),
        CollectData::from(
            Document::from(
                Text::from("Almighty and Everlasting God, who didst kindle the flame of thy love in the heart of thy faithful martyr Manche Masemola: Grant unto us thy servants, a like faith and power of love, that we who rejoice in her triumph may profit by her example; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Manche Masemola")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::FrancisXavier),
        CollectData::from(
            Document::from(
                Text::from("God of all nations; Raise up, we beseech thee, in this and every land, evangelists and heralds of thy kingdom, that like thy servant Francis Xavier we may proclaim the unsearchable riches of our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Francis Xavier")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Willibrord),
        CollectData::from(
            Document::from(
                Text::from("Pour out thy Holy Spirit, O God, upon thy church in every land, that like thy servant Willibrord we might proclaim the Gospel unto all nations, that thy kingdom might be enlarged and that thy holy Name might be glorified in all the world; through Jesus Christ our Lord, who liveth and reigneth with thee and the same Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Willibrord")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::RolleHiltonKempe),
        CollectData::from(
            Document::from(
                Text::from("Direct our hearts, O Gracious God, and inspire our minds; that, like thy servants Richard Rolle, Walter Hilton, and Margery Kempe, we might pass through the cloud of unknowing until we behold thy glory face to face; in the Name of Jesus Christ our Lord who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Richard Rolle 1349")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BernardMizeki),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who kindled the flame of thy love in the heart of thy holy martyr Bernard Mizeki: Grant unto us thy servants a like faith and power of love, that we, who rejoice in his triumph, may profit by his example; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Bernard Mizeki")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::SarahTheodoraSyncletica),
        CollectData::from(
            Document::from(
                Text::from("Fix our hearts on thee, O God, in pure devotion, that aided by the example of thy servants Sarah, Theodora, and Syncletica, the vain pursuits of this world may have no hold upon us, and that by the consuming fire of thy Spirit, we may be changed into the image and likeness of thy Son, Jesus Christ our Lord; to whom with thee and the same Spirit be all honor and glory, now and for ever.")
                    .response("Amen.")
                )
                .label("Sarah")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasAquinas),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who hast enriched thy church with the singular learning and holiness of thy servant Thomas Aquinas: Enlighten us more and more, we pray, by the disciplined thinking and teaching of Christian scholars, and deepen our devotion by the example of saintly lives; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas Aquinas")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Dominic),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, grant unto thy people a hunger for thy Word and an urgent longing to share thy Gospel; that, like thy servant Dominic, we might labor to bring the whole world to the knowledge and love of thee as thou art revealed in thy Son Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Dominic")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::NativityOfStJohnTheBaptist),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose providence thy servant Johnthe Baptist was wonderfully born, and sent to preparethe way of thy Son our Savior by preaching repentance: Make us so to follow his doctrine and holy life, that we may truly repent according to his preaching; and after his example constantly speak the truth, boldly rebuke vice, and patiently suffer for the truth’s sake; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EmilyMalboneMorgan),
        CollectData::from(
            Document::from(
                Text::from("Inspire us, Gracious God, with that same spirit of devotion that animated thy servant Emily Malbone Morgan; that, like her, we might dedicate our lives to thy service and to the welfare of others; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Emily Malbone Morgan")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Stephen),
        CollectData::from(
            Document::from(
                Text::from("We give thee thanks, O Lord of glory, for the example of the first martyr Stephen, who looked up to heaven and prayed for his persecutors to thy Son Jesus Christ, who standeth at thy right hand; where he liveth and reigneth with thee and the Holy Ghost, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BenedictOfNursia),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, whose service is perfect freedom and in whose commandments there is nothing harsh nor burdensome: Grant that we, with thy servant Benedict, may listen with attentive minds, pray with fervent hearts, and serve thee with willing hands, so that we might live at peace with one another and in obedience to thy Word, Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth, one God, now and for ever. ")
                    .response("Amen.")
                )
                .label("Benedict of Nursia")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MaryamOfQidun),
        CollectData::from(
            Document::from(
                Text::from("O God, whose glory it is always to have mercy: Be gracious to all who have gone astray from your ways, and restore them again like thy servant Maryam of Qidun, that with penitent hearts and steadfast faith they might embrace and hold fast the unchangeable truth of thy Word, Jesus Christ; who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Maryam of Qidun")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EdwardBouveriePusey),
        CollectData::from(
            Document::from(
                Text::from("Grant unto us, O God, that in all time of our testing we may know thy presence and obey thy will; that, following the example of thy servant Edward Bouverie Pusey, we may with integrity and courage accomplish what thou givest us to do, and endure what thou givest us to bear; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Edward Bouverie Pusey")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MartinLuther),
        CollectData::from(
            Document::from(
                Text::from("O God, our refuge and our strength, who didst raise up thy servant Martin Luther to reform and renew thy church in the light of thy word: Defend and purify the church in our own day and grant that, through faith, we may boldly proclaim the riches of thy grace, which thou hast made known in Jesus Christ our Savior, who with thee and the Holy Ghost, liveth and reigneth, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Martin Luther")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JacksonKemper),
        CollectData::from(
            Document::from(
                Text::from("O God, who didst send thy son Jesus Christ to preach peace to those who are far off and to those who are near: Grant that we, like thy servant Jackson Kemper, may proclaim the Gospel in our own day, with courage, vision, and perseverance; through the same Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth, now and for ever.")
                    .response("Amen.")
                )
                .label("Jackson Kemper")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Mark),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who by the hand of Mark the evangelist hast given to thy church the Gospel of Jesus Christ the Son of God: We thank thee for this witness, and pray that we may be firmly grounded in its truth; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Saint Mark")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheMartyrsOfMemphis),
        CollectData::from(
            Document::from(
                Text::from("We give thee thanks and praise, O God of compassion, for the heroic witness of the Martyrs of Memphis, who, in a time of plague and pestilence, were steadfast in their care for the sick and dying, and loved not their own lives, even unto death; Inspire in us a like love and commitment to those in need, following the example of our Savior Jesus Christ; who with thee and the Holy Ghost liveth and reigneth, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("the Martyrs of Memphis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CliveStaplesLewis),
        CollectData::from(
            Document::from(
                Text::from("O God of searing truth and surpassing beauty, we give thanks to thee for Clive Staples Lewis, whose sanctified imagination lit fires of faith in young and old alike; Surprise us also with thy joy and draw us into that new and abundant life which is ours in Christ Jesus, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Clive Staples Lewis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnCassian),
        CollectData::from(
            Document::from(
                Text::from("Holy God, whose beloved Son Jesus Christ didst bless the pure in heart: Grant that we, together with thy servant John Cassian and in union with his prayers, may ever seek the purity with which to behold thee as thou art; one God in Trinity of persons now and for ever.")
                    .response("Amen.")
                )
                .label("John Cassian")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JananiLuwum),
        CollectData::from(
            Document::from(
                Text::from("O God, whose Son the Good Shepherd laid down his life for his sheep: We give thee thanks for thy faithful shepherd, Janani Luwum, who after his Savior’s example gave up his life for the sake of his flock. Grant us to be so inspired by his witness that we make no peace with oppression, but live as those who are sealed with the cross of Christ, who died and rose again, and now liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Janani Luwum")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::LancelotAndrewes),
        CollectData::from(
            Document::from(
                Text::from("Perfect in us, Almighty God, whatever is lacking of thy gifts: of faith, to increase it; of hope, to establish it; of love, to kindle it; that like thy servant Lancelot Andrewes we may live in the life of thy grace and glory; through Jesus Christ thy Son our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Lancelot Andrewes")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Philip),
        CollectData::from(
            Document::from(
                Text::from("O God, who hast made of one blood all the peoples of the earth and sent thy Son to preach peace to those who are far off and to those who are near: Grant that we, following the example of thy servant Philip, may bring thy Word to those who seek thee, for the glory of thy Name; through Jesus Christ our Lord, who liveth and reigneth with thee in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Philip")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HolyInnocents),
        CollectData::from(
            Document::from(
                Text::from("We remember this day, O God, the slaughter of the holy innocents of Bethlehem by the order of King Herod. Receive, we beseech thee, into the arms of thy mercy all innocent victims; and by thy great might frustrate the designs of evil tyrants and establish thy rule of justice, love, and peace; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Holy Innocents")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::NicholasOfMyra),
        CollectData::from(
            Document::from(
                Text::from("Grant, Almighty God, that thy church may be so inspired by the example of thy servant Nicholas of Myra, that it may never cease to work for the welfare of children, the safety of sailors, the relief of the poor, and the help of those tossed by tempests of doubt or grief; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Nicholas of Myra")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Monica),
        CollectData::from(
            Document::from(
                Text::from("Deepen our devotion, O Lord, and use us in accordance with thy will; that inspired by the example of your servant Monica, we may bring others to acknowledge Jesus Christ as Savior and Lord; who with thee and the Holy Ghost lives and reigns, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Monica")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ElizabethAnnSeton),
        CollectData::from(
            Document::from(
                Text::from("Give us grace, O God, to love thee in all things and above all things; that, following the example of thy servant Elizabeth Ann Seton, we might express our love for thee in the service of others. Through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, in glory everlasting.")
                    .response("Amen.")
                )
                .label("Elizabeth Ann Seton")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CorneliusTheCenturion),
        CollectData::from(
            Document::from(
                Text::from("O God, who by thy Spirit thou didst call Cornelius the Centurion to be the first Christian among the Gentiles: Grant to thy church, we beseech thee, such a ready will to go where thou dost send and to do what thou dost command that the prejudices that blind us might cease, and that we might welcome all who turn to thee in love through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Cornelius the Centurion")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasGallaudetAndHenryWinterSyle),
        CollectData::from(
            Document::from(
                Text::from("O Loving God, whose will it is that everyone shouldst come to thee and be saved: We bless thy holy Name for thy servants Thomas Gallaudet and Henry Winter Syle, and we pray that thou wilt continually move thy church to respond in love to the needs of all people; through Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Thomas Gallaudet and Henry Winter Syle")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HarrietBedell),
        CollectData::from(
            Document::from(
                Text::from("Holy God, fill us with compassion and respect for all people, and empower us for the work of ministry whether near or far away; that like thy servant Harriet Bedell, we may show forth thy praise, not only with our lips, but in our lives, and by giving up our selves to thy service.  Through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Harriet Bedell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Boniface),
        CollectData::from(
            Document::from(
                Text::from("Pour out thy Holy Ghost, O God, upon thy church in every land, that like thy servant Boniface we might proclaim the Gospel unto all nations, that thy kingdom might be enlarged and that thy holy Name might be glorified in all the world; through Jesus Christ our Lord, who liveth and reigneth with thee and the same Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Boniface")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AnnaEllisonButlerAlexander),
        CollectData::from(
            Document::from(
                Text::from("Loving God, who didst call Anna Alexander as a deaconess in thy church: Grant unto us the wisdom to teach the gospel of Christ to whomever we meet, by word and by example, that all may come to the enlightenment thou dost intend for thy people; through Jesus Christ, our Teacher and Savior.")
                    .response("Amen.")
                )
                .label("Anna Ellison Butler Alexander")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ZitaOfTuscany),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, who hast given unto us all things that pertain unto life and godliness; Grant that we, like thy servant Zita, may be faithful in the exercise of our duties and that, whatsoever thou givest us to do, we may do it heartily unto thee for the honor and glory of thy Name; through him who hast called us to virtue, Jesus Christ, thy Son, our Lord. Amen.")
                    .response("Amen.")
                )
                .label("Zita of Tuscany")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnAndCharlesWesley),
        CollectData::from(
            Document::from(
                Text::from("Lord God, who didst inspire thy servants John and Charles Wesley with burning zeal for the sanctification of souls and didst endow themwith eloquence in speech and song: Kindle such fervor in thy church, we beseech thee, that those whose faith has cooled may be warmed, and those who have not known thy Christ may turn to him and be saved; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("John and Charles Wesley")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnColeridgePatteson),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call thy faithful servant John Coleridge Patteson and his companions to witness to the gospel, and by their labors and sufferings didst raise up a people for thine own possession: Pour out thy Holy Ghost upon thy church in every land, that, by the service and sacrifice of many, thy holy Name may be glorified and thy kingdom enlarged; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("John Coleridge Patteson")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ArgulaVonGrumbach),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give to thy servant Argula von Grumbach a spirit of wisdom and power to love thy Word and to boldly draw others unto its truth: Pour out that same spirit upon us, that we, knowing and loving thy Holy Word, may be unashamed of Christ and may not sin against the Holy Spirit that is within us.")
                    .response("Amen.")
                )
                .label("Argula Von Grumbach")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ChanningMooreWilliams),
        CollectData::from(
            Document::from(
                Text::from("O God, who in thy providence didst call Channing Moore Williams to the ministry of this church and gave him the gifts and the perseverance to preach the Gospel in new lands: Inspire us, by his example and prayers, to commit our talents to thy service, confident that thou dost uphold those whom thou dost call; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Channing Moore Williams")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::LaurenceOfRome),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose grace and power thy servant Laurence didst triumph over sufferingand didst despise death: Grant that we may be steadfast in service to the poor and outcast, and may share with him in the joys of thine everlasting kingdom; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Laurence of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::GregoryTheGreat),
        CollectData::from(
            Document::from(
                Text::from("Almighty and merciful God, who didst raise up Gregory of Rome to be a servant of the servants of God, and didst inspire him to send missionaries to preach the Gospel to the English people: Preserve thy church in the catholic and apostolic faith, that thy people, being fruitful in every good work, may receive the crown of glory that never fades away; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Gregory the Great")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JeremyTaylor),
        CollectData::from(
            Document::from(
                Text::from("O God, whose days are without end, and whose mercies cannot be numbered: Make us, we beseech thee, like thy servant Jeremy Taylor, deeply sensible of the shortness and uncertainty of human life; and let thy Holy Ghost lead us in holiness and righteousness all our days; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Jeremy Taylor")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamLaud),
        CollectData::from(
            Document::from(
                Text::from("Keep us, O Lord, constant in faith and zealous in witness; that, like thy servant William Laud, we may live in thy fear, die in thy favor, and rest in thy peace; for the sake of Jesus Christ thy Son our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("William Laud")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::LeoOfRome),
        CollectData::from(
            Document::from(
                Text::from("O Lord our God, grant that thy church, following the teaching of thy servant Leo of Rome, may hold fast the great mystery of our redemption and adore the one Christ, true God and true Man, neither divided from our human nature nor separate from thy divine Being; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Leo of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::HermanOfAlaska),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst raise up thy servant Herman to be a light in the world, and to preach the Gospel to the people of Alaska: Illuminate our hearts, that we also in our own generation may show forth thy praise, who hast called us out of darkness and into thy marvelous light; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Herman of Alaska")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ElizabethOfHungary),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, by whose grace thy servant Elizabeth of Hungary recognized and honored Jesus in the poor of this world: Grant that we, following her example, may with love and gladness serve those in any need or trouble, in the name and for the sake of Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Elizabeth of Hungary")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ClementOfRome),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst choose thy servant Clement of Rome to recall the church in Corinth to obedience and stability: Grant that thy church may be grounded and settled in thy truth by the indwelling of the Holy Ghost; reveal to it what is not yet known; fill up what is lacking; confirm what has already been revealed; and keep it blameless in thy service; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Clement of Rome")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PhillipsBrooks),
        CollectData::from(
            Document::from(
                Text::from("Everlasting God, who dost implant thy living Word in the minds and on the lips of all who proclaim thy truth: Grant that we, like thy pastor and preacher Phillips Brooks, might proclaim thy Gospel in our own generation with grace and power.  Through Jesus Christ our Lord, who livest and reignest with thee and the Holy Ghost, ever one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Phillips Brooks")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MariaSkobtsova),
        CollectData::from(
            Document::from(
                Text::from("O Creator and Giver of Life, who didst crown thy martyr Maria Skobtsova with glory and didst give her as an example of service to the suffering and poor even unto death: Teach us to love Christ in our neighbors, and thereby battle injustice and evil with the light of the Resurrection; through Jesus Christ our Lord, who livest and reignest with thee and the Holy Ghost, one God in glory everlasting.")
                    .response("Amen.")
                )
                .label("Maria Skobtsova")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JosephOfArimathea),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, whose servant Joseph of Arimathea with reverence and godly fear prepared the body of our Lord and Savior for burial and laid it in his own tomb: Grant to us, thy faithful people, grace and courage to love and serve Jesus with sincere devotion all the days of our life; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Joseph of Arimathea")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CyrilOfJerusalem),
        CollectData::from(
            Document::from(
                Text::from("Strengthen, O God, thy church in the sacraments of thy grace, that we, in union with the teaching and prayers of thy servant Cyril of Jerusalem, may enter more fully into thy Paschal mystery; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Cyril of Jerusalem")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PhilanderChase),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose Son Jesus Christ is the pioneer and perfecter of our faith: Grant that like thy servant Philander Chase, we might have the grace to minister in Christ’s name in every place, led by bold witnesses to the Gospel of the Prince of Peace, even Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Philander Chase")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Alphege),
        CollectData::from(
            Document::from(
                Text::from("Lord Jesus Christ, who didst willingly walk the way of the cross: Strengthen thy church through the example and prayers of thy servant Alphege to hold fast to the path of discipleship; for with the Father and the Holy Ghost thou livest and reignest, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Alphege")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::KatharinaZell),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose servant Katharina Zell toiled for the reform of thy church both in word and in deed: Fill us with the wisdom to speak out in defense of thy truth, with love for thee and for our neighbor, that we may serve thee and welcome all thy people with a mother’s heart; through Christ our Lord.")
                    .response("Amen.")
                )
                .label("Katharina Zell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MaryMagdalene),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose blessed Son restored Mary Magdalene to health of body and mind, and called her to be a witness of his resurrection: Mercifully grant that by thy grace we may be healed of all our infirmities and know thee in the power of his endless life; Through the same Jesus Christ our Lord, who with thee andthe Ghost liveth and reigneth, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CharlesSimeon),
        CollectData::from(
            Document::from(
                Text::from("Loving God, whose unerring wisdom and unbounded love doth order all things: Grant us in all things to see thy hand; that, following the example and teaching of thy servant Charles Simeon, we may walk with Christ in all simplicity and serve thee with a quiet and contented mind; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Charles Simeon")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::SamuelIsaacJosephScherechewsky),
        CollectData::from(
            Document::from(
                Text::from("O God, who in thy providence didst call Joseph Schereschewsky to the ministry of this church and gave him the gifts and the perseverance to translate the Holy Scriptures: Inspire us, by his example and prayers, to commit our talents to thy service, confident that thou dost uphold those whom thou dost call; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Samuel Isaac Joseph Scherechewsky")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ConversionOfStPaul),
        CollectData::from(
            Document::from(
                Text::from("O God, who, by the preaching of thine apostle Paul, hast caused the light of the Gospel to shine throughoutthe world: Grant, we beseech thee, that we, having his wonderful conversion in remembrance, may show forth our thankfulness unto thee for the same by following the holy doctrine which he taught; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ThomasAKempis),
        CollectData::from(
            Document::from(
                Text::from("Holy Father, who hast nourished and strengthened thy church by the writings of thy servant Thomas à Kempis: Grant that we may learn from him to know what is necessary to be known, to love what is to be loved, to praise what highly pleases thee, and always to seek to know and to follow thy will; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thomas à Kempis")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::IrenaeusOfLyons),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst strengthen thy servant Irenaeus to defend thy truth against every blast of vain doctrine: Keep us, we pray, steadfast in thy true religion, that in constancy and peace we may walk in the way that leads to eternal life; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Irenaeus of Lyons")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PeterAndPaul),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose blessed apostles Peter and Paul glorified thee by their martyrdom: Grant that thy church, instructed by their teaching and example, and knit together in unity by thy Spirit, may ever stand firm upon the one foundation, which is Jesus Christ our Lord; who liveth and reigneth with thee, in the unity of the same Spirit, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Saint Peter and Saint Paul")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohannSebasatianBach),
        CollectData::from(
            Document::from(
                Text::from("Sound out thy majesty, O God, and call us to thy work; that, like thy servant Johann Sebastian Bach, we might present our lives and our works to thy glory alone; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Johann Sebasatian Bach")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::DamienAndMarianne),
        CollectData::from(
            Document::from(
                Text::from("Bind up the wounds of thy children O God, and help us to be bold and loving in service to all who are shunned for the diseases they suffer, following the example of thy servants Damien and Marianne, that thy grace may be poured forth upon all; through Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Damien")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CatherineOfGenoa),
        CollectData::from(
            Document::from(
                Text::from("Gracious God, reveal to thy church the depths of thy love; that, like thy servant Catherine of Genoa, we might give ourselves in loving service, knowing that we have been perfectly loved by thee; through Jesus Christ our Lord. ")
                    .response("Amen.")
                )
                .label("Catherine of Genoa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MargaretOfCortona),
        CollectData::from(
            Document::from(
                Text::from("Grant, O God, to all thy people, as to thy servant Margaret of Cortona, the spirit of repentance and supplication, that we might seek and desire nothing in this transitory life above thee; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Margaret of Cortona")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WilliamWilberforce),
        CollectData::from(
            Document::from(
                Text::from("Let thy continual mercy, O Lord, enkindle in thy Church the never-failing gift of love; that, following the example of thy servant William Wilberforce, we may have grace to defend the poor, and maintain the cause of those who have no helper; for the sake of him who gave his life for us, thy Son our Savior Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("William Wilberforce")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheNativityOfTheBlessedVirginMary),
        CollectData::from(
            Document::from(
                Text::from("Father in heaven, by whose grace the virgin mother of thy incarnate Son was blessed in bearing him, but still more blessed in keeping thy word: Grant us who honor the exaltation of her lowliness to follow the example of her devotion to thy will; through the same Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("the Nativity of the Blessed Virgin Mary")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheclaOfIconium),
        CollectData::from(
            Document::from(
                Text::from("God of liberating power, who didst call Thecla to proclaim the gospel and didst not permit any obstacle or peril to inhibit her: Empower courageous evangelists among us, that men and women everywhere may know the freedom that thou dost offer us in Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Thecla of Iconium")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::PauliMurray),
        CollectData::from(
            Document::from(
                Text::from("Liberating God, we give thee thanks for the steadfast courage of thy servant Pauli Murray, who didst fight long and well: Unshackle us from the chains of prejudice and fear, that we may show forth the reconciling love and true freedom which thou didst reveal in thy Son our Savior Jesus Christ; who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Pauli Murray")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::BirgittaOfSweden),
        CollectData::from(
            Document::from(
                Text::from("O God, who beholdest all things and whose judgment is always mercy; by the example of thy servant Birgitta of Sweden, give to us in this life the vision of thy kingdom, where Jesus Christ is all and in all, that we might pattern our earthly lives on things heavenly, where our lives art hid with the same Christ in thee; who with him and the Holy Ghost livest and reignest for ever and ever.")
                    .response("Amen.")
                )
                .label("Birgitta of Sweden")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::StantonBloomerTruthTubman),
        CollectData::from(
            Document::from(
                Text::from("O God, whose Spirit guideth us into all truth and maketh us free: Strengthen and sustain us as thou didst thy servants Elizabeth, Amelia, Sojourner, and Harriet. Give us vision and courage to stand against oppression and injustice and all that worketh against the glorious liberty to which thou dost call all thy children; through Jesus Christ our Savior, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Elizabeth Cady Stanton")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JohnDonne),
        CollectData::from(
            Document::from(
                Text::from("O God of eternal glory, whom no one living can see and yet whom to see is to live; grant that with thy servant John Donne, we may see thy glory in the face of thy Son, Jesus Christ, and then, with all our skill and wit, offer thee our crown of prayer and praise, until by his grace we stand in that last and everlasting day, when death itself will die, and all will live in thee, who with the Holy Ghost and the same Lord Jesus Christ art one God in everlasting light and glory. ")
                    .response("Amen.")
                )
                .label("John Donne")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::CatherineOfSiena),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who didst kindle the flame of thy love in the heart of thy servant Catherine of Siena: Grant unto us the same strength of conviction and power of love that, as we rejoice in her triumph, we may profit by her example; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Catherine of Siena")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Anskar),
        CollectData::from(
            Document::from(
                Text::from("Keep thy church from discouragement in the day of small things, O God, in the knowledge that when thou hast begun a good work, thou shalt bring it to a fruitful conclusion, just as thou didst for thy servant Anskar; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Anskar")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ZenaidaPhilonellaHermione),
        CollectData::from(
            Document::from(
                Text::from("Merciful God, whose most dear Son came to heal the sick, raise the dead, cast out demons, and preach good news to the poor: Lead us by the example of thy servants, Zenaida, Philonella, and Hermione, to freely give even as we have freely received; through Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Zenaida")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MosesTheBlack),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose blessed Son dost guide our footsteps into the way of peace: Deliver us from the paths of hatred and violence, that we, following the example of thy servant Moses, may serve thee with singleness of heart and attain to the tranquility of the world to come; through Jesus Christ our Lord, who liveth and reigneth with thee in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Moses the Black")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JamesOfJerusalem),
        CollectData::from(
            Document::from(
                Text::from("Grant, we beseech thee, O God, that after the example of thy servant James the Just, brother of our Lord, thy church may give itself continually to prayer and to the reconciliation of all who are at variance and enmity; through the same our Lord Jesus Christ, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Bartholomew),
        CollectData::from(
            Document::from(
                Text::from("Almighty and everlasting God, who didst give to thine apostle Bartholomew grace truly to believe and to preach thy Word: Grant, we beseech thee, that thy church may love what he believed and to preach what he taught; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Annunciation),
        CollectData::from(
            Document::from(
                Text::from("We beseech thee, O Lord, pour thy grace into our hearts, that we who have known the incarnation of thy Son Jesus Christ, announced by an angel to the Virgin Mary, may by his cross and passion be brought unto the glory of his resurrection; who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Ammonius),
        CollectData::from(
            Document::from(
                Text::from("Drive far from thy church, O God, every vain spirit of clerical ambition, that, like thy servant Ammonius, we may refuse to conflate ordination and leadership, and may never confuse rank with holiness; in the name of thy son Jesus Christ our Lord, who alone is our great High Priest.")
                    .response("Amen.")
                )
                .label("Ammonius")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::ChadOfLichfield),
        CollectData::from(
            Document::from(
                Text::from("Heavenly Father, whose son our Lord Jesus Christ didst take the form of a servant for the sake of his brothers and sisters: Strengthen us with the prayers and example of thy servant Chad, who became the least of all to minister to all; through the same Christ our Lord, who liveth and reigneth with three and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Chad of Lichfield")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::JuliaChesterEmery),
        CollectData::from(
            Document::from(
                Text::from("God of all creation, who dost call us to make disciples of all nations and to proclaim thy mercy and love: Grant that we, after the exampleof thy servant Julia Chester Emery, might have vision and courage in proclaiming the Gospel to the ends of the earth; through Jesus Christ, our light and our salvation, who liveth and reigneth with thee and the Holy Ghost, ever one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Julia Chester Emery")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::KateriTekakwitha),
        CollectData::from(
            Document::from(
                Text::from("O God of justice and mercy, we remember before thee thy servants Peter Williams Cassey and Anna Besant Cassey, who, in the face of slavery and discrimination, gave the blessings of education and spiritual haven to the marginalized; Grant us to be fearless in the face of injustice and to work for blessings that will touch those whom the world does not count of value; through Jesus Christ our Lord, who with thee and the Holy Ghost liveth and reigneth for ever and ever.")
                    .response("Amen.")
                )
                .label("Kateri Tekakwitha")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Luke),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst inspire thy servant Luke the physician to set forth in the Gospel the love and healing power of thy Son: Graciously continue in thy church the like love and power to heal, to the praise and glory of thy Name; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::AllSoulsDay),
        CollectData::from(
            Document::from(
                Text::from("O God, the Maker and Redeemer of all believers: Grant to the faithful departed the unsearchable benefits of the passion of thy Son; that on the day of his appearing they may be manifested as thy children; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and ever.")
                    .response("Amen.")
                )
                .label("All Souls’/All the Faithful Departed")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::IgnatiusOfLoyola),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst call Ignatius of Loyola to the service of thy Divine Majesty and to seek thee in all things; Give us also the grace to labor without counting the cost and to seek no reward other than knowing that we do thy will; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Ignatius of Loyola")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TheTransfiguration),
        CollectData::from(
            Document::from(
                Text::from("O God, who on the holy mount didst reveal to chosen witnesses thy well-beloved Son, wonderfully transfigured, in raiment white and glistening: Mercifully grant that we, being delivered from the disquietude of this world, may by faith behold the King in his beauty; who with thee, O Father, and thee, O Holy Ghost, liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::VincentOfSaragossa),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, whose deacon Vincent, upheld by thee, was neither terrified by threats nor overcome by torments: Strengthen us, we beseech thee, to endure all adversity with invincible and steadfast faith; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Vincent of Saragossa")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::SergiusOfRadonezh),
        CollectData::from(
            Document::from(
                Text::from("O God, whose blessed Son became poor that we, through his poverty, might be rich: Deliver us from an inordinate love of this world, that we, inspired by the devotion of thy servant Sergius, may serve thee with singleness of heart and attain to the riches of the age to come; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Sergius of Radonezh")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::MechthildeAndGertude),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst give to thy servants Mechthilde and Gertrude special gifts of grace to understand and teach the truth in Christ Jesus: Grant, we beseech thee, that by their teachings we may know thee, the one true God, and Jesus Christ thy Son, who liveth and reigneth with thee and the Holy Ghost, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Mechthilde of Hackeborn and Gertrude the Great")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::WardClitherowLine),
        CollectData::from(
            Document::from(
                Text::from("Most Merciful God, who despisest not a broken and  contrite heart and hath promised to fill those who hunger and thirst after righteousness; We humbly beseech thee, remember not the sins and offenses of our forefathers, but grant that, like thy servants Margaret Ward, Margaret Clitherow, and Anne Line, we may sanctify thee in our hearts and be always ready to answer for our faith with meekness and fear; through our only Mediator and Advocate, Jesus Christ our Lord.")
                    .response("Amen.")
                )
                .label("Margaret Ward")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Thomas),
        CollectData::from(
            Document::from(
                Text::from("Everliving God, who didst strengthen thine apostle Thomas with sure and certain faith in thy Son’s resurrection: Grant us so perfectly and without doubt to believe in Jesus Christ, our Lord and our God, that our faith may never be found wanting in thy sight; through him who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::Matthias),
        CollectData::from(
            Document::from(
                Text::from("O Almighty God, who into the place of Judas didst choose thy faithful servant Matthias to be of the number of the Twelve: Grant that thy Church, being delivered from false apostles, may always be ordered and guided by faithful and true pastors; through Jesus Christ our Lord, who liveth and reigneth with thee, in the unity of the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::EdithCavell),
        CollectData::from(
            Document::from(
                Text::from("Living God, who art the source of all healing and wholeness: we bless thee for the compassionate witness of thy servant Edith Cavell. Inspire us, we beseech thee, to be agents of peace of and reconciliation in a world beset by injustice, poverty, and war. We ask this through Jesus Christ, the Prince of Peace, who livest and reignest with thee and the Holy Ghost, one God, unto the ages of ages.")
                    .response("Amen.")
                )
                .label("Edith Cavell")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::DorothyLSayers),
        CollectData::from(
            Document::from(
                Text::from("Almighty God, who didst strengthen thy servant Dorothy Sayers with eloquence to defend Christian teaching: Keep us, we pray, steadfast in thy true religion, that in constancy and peace we may always teach right doctrine, and teach doctrine rightly; through Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Ghost, one God, now and for ever.")
                    .response("Amen.")
                )
                .label("Dorothy L Sayers")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
        (
        CollectId::Feast(Feast::TeresaOfAvila),
        CollectData::from(
            Document::from(
                Text::from("O God, who by the Holy Ghost didst move Teresa of Avila to manifest to thy church the way of perfection: Grant us, we beseech thee, to be nourished by her teaching, and enkindle within us a lively and unquenchable longing for true holiness; through Jesus Christ, the joy of loving hearts, who with thee and the Holy Ghost liveth and reigneth, one God, for ever and ever.")
                    .response("Amen.")
                )
                .label("Teresa of Avila")
                .source(Reference {
                    source: Source::LFF2018,
                    page: 0
                })
                .version(Version::RiteI)
        )
    ),
    ];
}
