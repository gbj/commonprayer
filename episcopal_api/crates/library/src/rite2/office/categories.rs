use calendar::{Feast, Season};
use liturgy::{Antiphon, Condition, Document, ResponsivePrayer, Sentence};

lazy_static! {
  pub static ref OPENING_SENTENCES: Vec<Document> = vec![
    Document::from(Sentence::from("Watch, for you know not when the master of the house will come, in the evening, or at midnight, or at cockcrow, or in the morning; lest he come suddenly and find you asleep.").citation("Mark 13:35, 36")).condition(Condition::Season(Season::Advent)),
    Document::from(Sentence::from("In the wilderness prepare the way of the Lord, make straight in the desert a highway for our God.").citation("Isaiah 40:3")).condition(Condition::Season(Season::Advent)),
    Document::from(Sentence::from("The glory of the Lord shall be revealed, and all flesh shall see it together.").citation("Isaiah 40:5")).condition(Condition::Season(Season::Advent)),
    Document::from(Sentence::from("Behold, I bring you good news of a great joy, which will come to all the people; for unto you is born this day in the city of David, a Savior, who is Christ the Lord.").citation("Luke 2:10, 11")).condition(Condition::Season(Season::Christmas)),
    Document::from(Sentence::from("Behold, the dwelling of God is with mankind. He will dwell with them, and they shall be his people, and God himself will be with them, and be their God.").citation("Revelation 21:3")).condition(Condition::Season(Season::Christmas)),
    Document::from(Sentence::from("Nations shall come to your light, and kings to the brightness of your rising.").citation("Isaiah 60:3")).condition(Condition::Season(Season::Epiphany)),
    Document::from(Sentence::from("I will give you as a light to the nations, that my salvation may reach to the end of the earth. ").citation("Isaiah 49:6b")).condition(Condition::Season(Season::Epiphany)),
    Document::from(Sentence::from("From the rising of the sun to its setting my Name shall be great among the nations, and in every place incense shall be offered to my Name, and a pure offering; for my Name shall be great among the nations, says the Lord of hosts.").citation("Malachi 1:11")).condition(Condition::Season(Season::Epiphany)),
    Document::from(Sentence::from("If we say we have no sin, we deceive ourselves, and the truth is not in us, but if we confess our sins, God, who is faithful and just, will forgive our sins and cleanse us from all unrighteousness.").citation("1 John 1:8, 9")).condition(Condition::Season(Season::Lent)),
    Document::from(Sentence::from("Rend your hearts and not your garments. Return to the Lord your God, for he is gracious and merciful, slow to anger and abounding in steadfast love, and repents of evil.").citation("Joel 2:13")).condition(Condition::Season(Season::Lent)),
    Document::from(Sentence::from("I will arise and go to my father, and I will say to him, “Father, I have sinned against heaven and before you; I am no longer worthy to be called your son.”").citation("Luke 15:18, 19")).condition(Condition::Season(Season::Lent)),
    Document::from(Sentence::from("Jesus said, “If anyone would come after me, let him deny himself and take up his cross and follow me.”").citation("Mark 8:34")).condition(Condition::Season(Season::Lent)),
    Document::from(Sentence::from("To the Lord our God belong mercy and forgiveness, because we have rebelled against him and have not obeyed the voice of the Lord our God by following his laws which he set before us.").citation("Daniel 9:9, 10")).condition(Condition::Season(Season::Lent)),
    Document::from(Sentence::from("All we like sheep have gone astray; we have turned every one to his own way; and the Lord has laid on him the iniquity of us all.").citation("Isaiah 53:6")).condition(Condition::Season(Season::HolyWeek)),
    Document::from(Sentence::from("Is it nothing to you, all you who pass by?  Look and see if there is any sorrow like my sorrow which was brought upon me, whom the Lord has afflicted.").citation("Lamentations 1:12")).condition(Condition::Season(Season::HolyWeek)),
    Document::from(ResponsivePrayer::from([
      "Alleluia! Christ is risen.", "The Lord is risen indeed. Alleluia!"
    ])).condition(Condition::Any(vec![Condition::Season(Season::Easter),Condition::Season(Season::Ascension),Condition::Season(Season::Pentecost)])).label("Easter Season, including Ascension Day and the Day of Pentecost"),
    Document::from(Sentence::from("On this day the Lord has acted; we will rejoice and be glad in it.").citation("Psalm 118:24")).condition(Condition::Any(vec![Condition::Season(Season::Easter),Condition::Season(Season::Ascension),Condition::Season(Season::Pentecost)])).label("Easter Season, including Ascension Day and the Day of Pentecost"),
    Document::from(Sentence::from("Thanks be to God, who gives us the victory through our Lord Jesus Christ.").citation("1 Corinthians 15:57")).condition(Condition::Any(vec![Condition::Season(Season::Easter),Condition::Season(Season::Ascension),Condition::Season(Season::Pentecost)])).label("Easter Season, including Ascension Day and the Day of Pentecost"),
    Document::from(Sentence::from("If then you have been raised with Christ, seek the things that are above, where Christ is, seated at the right hand of God.").citation("Colossians 3:1")).condition(Condition::Any(vec![Condition::Season(Season::Easter),Condition::Season(Season::Ascension),Condition::Season(Season::Pentecost)])).label("Easter Season, including Ascension Day and the Day of Pentecost"),
    Document::from(Sentence::from("Christ has entered, not into a sanctuary made with hands, a copy of the true one, but into heaven itself, now to appear in the presence of God on our behalf.").citation("Hebrews 9:24")).condition(Condition::Any(vec![Condition::Season(Season::Easter),Condition::Season(Season::Ascension),Condition::Season(Season::Pentecost)])).label("Easter Season, including Ascension Day and the Day of Pentecost"),
    Document::from(Sentence::from("You shall receive power when the Holy Spirit has come upon you; and you shall be my witness in Jerusalem, and in all Judea, and Samaria, and to the ends of the earth.").citation("Acts 1:8")).condition(Condition::Any(vec![Condition::Season(Season::Easter),Condition::Season(Season::Ascension),Condition::Season(Season::Pentecost)])).label("Easter Season, including Ascension Day and the Day of Pentecost"),
    Document::from(Sentence::from("Holy, holy, holy is the Lord God Almighty, who was, and is, and is to come.").citation("Revelation 4:8")).condition(Condition::Season(Season::Trinity)),
    Document::from(Sentence::from("We give thanks to the Father, who has made us worthy to share in the inheritance of the saints in light.").citation("Colossians 1:12")).condition(Condition::Season(Season::Saints)).label("All Saints and other Major Saints’ Days"),
    Document::from(Sentence::from("You are no longer strangers and sojourners, but fellow citizens with the saints and members of the household of God.").citation("Ephesians 2:19")).condition(Condition::Season(Season::Saints)).label("All Saints and other Major Saints’ Days"),
    Document::from(Sentence::from("Their sound has gone out into all lands, and their message to the ends of the world.").citation("Psalm 19:4")).condition(Condition::Season(Season::Saints)).label("All Saints and other Major Saints’ Days"),
    Document::from(Sentence::from("O give thanks to the Lord, and call upon his Name; make known his deeds among the peoples.").citation("Psalm 105:1")).condition(Condition::Season(Season::Thanksgiving)).label("Occasions of Thanksgiving"),
    Document::from(Sentence::from("Grace to you and peace from God our Father and the Lord Jesus Christ.").citation("Philippians 1:2")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("I was glad when they said to me, “Let us go to the house of the Lord.”").citation("Psalm 122:1")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("Let the words of my mouth and the meditation of my heart be acceptable in your sight, O Lord, my strength and my redeemer.").citation("Psalm 19:14")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("Send out your light and your truth, that they may lead me, and bring me to your holy hill and to your dwelling.").citation("Psalm 43:3")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("The Lord is in his holy temple; let all the earth keep silence before him.").citation("Habakkuk 2:20")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("The hour is coming, and now is, when the true worshipers will worship the Father in spirit and truth, for such the Father seeks to worship him.").citation("John 4:23")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("Thus says the high and lofty One who inhabits eternity, whose name is Holy, “I dwell in the high and holy place and also with the one who has a contrite and humble spirit, to revive the spirit of the humble and to revive the heart of the contrite.”").citation("Isaiah 57:15")).condition(Condition::Season(Season::OrdinaryTime)),
    Document::from(Sentence::from("Let my prayer be set forth in your sight as incense, the lifting up of my hands as the evening sacrifice.").citation("Psalm 141:2")).condition(Condition::Evening),
    Document::from(Sentence::from("Grace to you and peace from God our Father and the Lord Jesus Christ.").citation("Philippians 1:2")).condition(Condition::Evening),
    Document::from(Sentence::from("Worship the Lord in the beauty of his holiness; let the whole earth tremble before him.").citation("Psalm 96:9")).condition(Condition::Evening),
    Document::from(Sentence::from("Yours is the day, O God, yours also the night; you established the moon and the sun. You fixed all the boundaries of the earth; you made both summer and winter.").citation("Psalm 74:15,16")).condition(Condition::Evening),
    Document::from(Sentence::from("I will bless the Lord who gives me counsel; my heart teaches me, night after night. I have set the Lord always before me; because he is at my right hand, I shall not fall.").citation("Psalm 16:7,8")).condition(Condition::Evening),
    Document::from(Sentence::from("Seek him who made the Pleiades and Orion, and turns deep darkness into the morning, and darkens the day into night; who calls for the waters of the sea and pours them out upon the surface of the earth: The Lord is his name.").citation("Amos 5:8")).condition(Condition::Evening),
    Document::from(Sentence::from("If I say, “Surely the darkness will cover me, and the light around me turn to night,” darkness is not dark to you, O Lord; the night is as bright as the day; darkness and light to you are both alike.").citation("Psalm 139:10,11")).condition(Condition::Evening),
    Document::from(Sentence::from("Jesus said, “I am the light of the world; whoever follows me will not walk in darkness, but will have the light of life.”").citation("John 8:12")).condition(Condition::Evening)
  ];

  pub static ref CLOSING_SENTENCES: Vec<Document> = vec![
    Document::from(Sentence::from("The grace of our Lord Jesus Christ, and the love of God, and the fellowship of the Holy Spirit, be with us all evermore. ").citation("2 Corinthians 13:14")),
    Document::from(Sentence::from("May the God of hope fill us with all joy and peace in believing through the power of the Holy Spirit.").citation("Romans 15:13").response("Amen.")),
    Document::from(Sentence::from("Glory to God whose power, working in us, can do infinitely more than we can ask or imagine: Glory to him from generation to generation in the Church, and in Christ Jesus for ever and ever.").citation("Ephesians 3:20,21").response("Amen."))
 ];

  pub static ref INVITATORY_ANTIPHONS: Vec<Document> = vec![
    Document::from(Antiphon::from("Our King and Savior now draws near: Come, let us adore him."))
      .condition(Condition::Season(Season::Advent))
      .label("In Advent"),
    Document::from(Antiphon::from("Alleluia. To us a child is born: O come, let us adore him. Alleluia."))
      .condition(Condition::Season(Season::Christmas))
      .label("On the Twelve Days of Christmas"),
    Document::from(Antiphon::from("The Lord has shown forth his glory: Come let us adore him."))
      .condition(Condition::Any(vec![
        Condition::Season(Season::Epiphany),
        Condition::Feast(Feast::TheTransfiguration),
        Condition::Feast(Feast::HolyCross)])
      )
      .label("From the Epiphany through the Baptism of Christ, and on the Feasts of the Transfiguration and Holy Cross"),
    Document::from(Antiphon::from("The Lord is full of compassion and mercy: Come let us adore him."))
      .condition(Condition::Any(vec![
          Condition::Season(Season::Lent),
          Condition::Season(Season::HolyWeek)])
      )
      .label("In Lent"),
    Document::from(Antiphon::from("Alleluia. The Lord is risen indeed: Come let us adore him. Alleluia."))
      .condition(Condition::Season(Season::Easter))
      .label("From Easter Day until the Ascension"),
    Document::from(Antiphon::from("Alleluia. Christ the Lord has ascended into heaven: Come let us adore him. Alleluia."))
      .condition(Condition::Season(Season::Ascension))
      .label("From Ascension Day until the Day of Pentecost"),
    Document::from(Antiphon::from("Alleluia. The Spirit of the Lord renews the face of the earth: Come let us adore him. Alleluia. "))
      .condition(Condition::Season(Season::Pentecost))
      .label("On the Day of Pentecost"),
    Document::from(Antiphon::from("Father, Son, and Holy Spirit, one God: Come let us adore him. "))
      .condition(Condition::Season(Season::Trinity))
      .label("On Trinity Sunday"),
    Document::from(Antiphon::from("The earth is the Lord’s for he made it: Come let us adore him. "))
      .condition(Condition::Season(Season::OrdinaryTime))
      .label("On other Sundays and weekdays"),
    Document::from(Antiphon::from("Worship the Lord in the beauty of holiness: Come let us adore him. "))
      .condition(Condition::Season(Season::OrdinaryTime))
      .label("On other Sundays and weekdays"),
    Document::from(Antiphon::from("The mercy of the Lord is everlasting: Come let us adore him."))
      .condition(Condition::Season(Season::OrdinaryTime))
      .label("On other Sundays and weekdays"),
    Document::from(Antiphon::from("[Alleluia.] The Word was made flesh and dwelt among us: Come, let us adore him. [Alleluia.] "))
      .condition(Condition::Any(vec![
        Condition::Feast(Feast::Annunciation),
        Condition::Feast(Feast::TheVisitation),
        Condition::Feast(Feast::ChristmasDay),
        Condition::Feast(Feast::HolyName),
        Condition::Feast(Feast::ThePresentation),
      ]))
      .label("On Feasts of the Incarnation"),
    Document::from(Antiphon::from("[Alleluia.] The Lord is glorious in his saints: Come, let us adore him. [Alleluia.] "))
      .condition(Condition::Season(Season::Saints))
      .label("On All Saints and other Major Saints’ Days")
  ];
}
