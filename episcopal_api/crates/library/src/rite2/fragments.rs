use liturgy::{Choice, DisplayFormat, Document, GloriaPatri, Preces, Text};

lazy_static! {
    pub static ref WORD_OF_THE_LORD: Preces = Preces::from([
        ("", "The Word of the Lord."),
        ("People", "Thanks be to God.")
    ]);

    pub static ref GLORIA_PATRI: GloriaPatri = GloriaPatri::from((
      "Glory to the Father, and to the Son, ",
      "and to the Holy Spirit: ",
      "as it was in the beginning, is now,",
      "and will be for ever. Amen. "
    ));

    pub static ref LORDS_PRAYER_CONTEMPORARY_AND_TRADITIONAL: Choice = Choice::from([
      Document::from(Text::from("Our Father, who art in heaven,\n	hallowed be thy Name,\n	thy kingdom come,\n	thy will be done,\n	on earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n	as we forgive those\n	who trespass against us.\nAnd lead us not into temptation,\n	but deliver us from evil.\nFor thine is the kingdom,\n	and the power, and the glory,\n	for ever and ever.").response("Amen.").display_format(DisplayFormat::Unison)).label("The Lord’s Prayer").version_label("Traditional"),
      Document::from(Text::from("Our Father in heaven,\n	hallowed be your Name,\n	your kingdom come,\n	your will be done,\n	on earth as in heaven.\nGive us today our daily bread.\nForgive us our sins,\n	as we forgive those\n	who sin against us.\nSave us from the time of trial,\n	and deliver us from evil.\nFor the kingdom, the power,\n	and the glory are yours,\n	now and for ever.").response("Amen.").display_format(DisplayFormat::Unison)).label("The Lord’s Prayer").version_label("Contemporary")
    ]);

    pub static ref LORDS_PRAYER_ABBREV: Choice = Choice::from([
      Document::from(
        Text::from("Our Father, who art in heaven,\n\thallowed be thy Name,\n\tthy kingdom come,\n\tthy will be done,\n\ton earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n\tas we forgive those\n\twho trespass against us.\nAnd lead us not into temptation,\n\tbut deliver us from evil.")
          .display_format(DisplayFormat::Unison)
      ).label("Traditional"),
      Document::from(
        Text::from("Our Father in heaven,\n\thallowed be your Name,\n\tyour kingdom come,\n\tyour will be done,\n\ton earth as in heaven.\nGive us today our daily bread.\nForgive us our sins,\n\tas we forgive those\n\twho sin against us.\nSave us from the time of trial,\n\tand deliver us from evil.")
          .display_format(DisplayFormat::Unison)
      ).label("Contemporary")
    ]);

    pub static ref APOSTLES_CREED: Text = Text::from("I believe in God, the Father almighty,\n	creator of heaven and earth.\nI believe in Jesus Christ, his only Son, our Lord.\n	He was conceived by the power of the Holy Spirit\n	and born of the Virgin Mary.\n	He suffered under Pontius Pilate,\n	was crucified, died, and was buried. \n	He descended to the dead.\n	On the third day he rose again.\n	He ascended into heaven,\n	and is seated at the right hand of the Father.\n	He will come again to judge the living and the dead.\nI believe in the Holy Spirit,\n	the holy catholic Church,\n	the communion of saints,\n	the forgiveness of sins,\n	the resurrection of the body,\n	and the life everlasting.")
      .display_format(DisplayFormat::Unison);
}
