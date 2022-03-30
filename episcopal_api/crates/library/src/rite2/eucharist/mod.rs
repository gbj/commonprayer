use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref GLORIA_IN_EXCELSIS: Text = Text::from("Glory to God in the highest,\n\tand peace to his people on earth. \n\nLord God, heavenly King,\nalmighty God and Father,\n\twe worship you, we give you thanks,\n\twe praise you for your glory. \n\nLord Jesus Christ, only Son of the Father,\nLord God, Lamb of God,\nyou take away the sin of the world:\n\thave mercy on us;\nyou are seated at the right hand of the Father:\n\treceive our prayer. \n\nFor you alone are the Holy One,\nyou alone are the Lord,\nyou alone are the Most High,\n\tJesus Christ,\n\twith the Holy Spirit,\n\tin the glory of God the Father. Amen.");

    pub static ref KYRIE_ELEISON: Choice = Choice::from(vec![
        Document::from(ResponsivePrayer::from([
            "Lord, have mercy.",
            "Christ, have mercy.",
            "Lord, have mercy."
        ])),
        Document::from(ResponsivePrayer::from([
            "Kyrie eleison.",
            "Christe eleison.",
            "Kyrie eleison."
        ]))
    ]);

    pub static ref TRISAGION: ResponsivePrayer = ResponsivePrayer::from([
        "Holy God,\nHoly and Mighty,\nHoly Immortal One,",
        "Have mercy upon us."
    ]);

    pub static ref NICENE_CREED_II: Document = Document::from(Text::from("We believe in one God,\n\tthe Father, the Almighty,\n\tmaker of heaven and earth,\n\tof all that is, seen and unseen.\n\nWe believe in one Lord, Jesus Christ,\n\tthe only Son of God,\n\teternally begotten of the Father,\n\tGod from God, Light from Light,\n\ttrue God from true God,\n\tbegotten, not made,\n\tof one Being with the Father.\n\tThrough him all things were made.\n\tFor us and for our salvation\n\t\the came down from heaven:\n\tby the power of the Holy Spirit\n\t\the became incarnate from the Virgin Mary,\n\t\tand was made man.\n\tFor our sake he was crucified under Pontius Pilate;\n\t\the suffered death and was buried.\n\t\tOn the third day he rose again\n\t\t\tin accordance with the Scriptures;\n\t\the ascended into heaven\n\t\t\tand is seated at the right hand of the Father.\n\tHe will come again in glory to judge the living and the dead,\n\t\tand his kingdom will have no end.\n\nWe believe in the Holy Spirit, the Lord, the giver of life,\n\twho proceeds from the Father and the Son.\n\tWith the Father and the Son he is worshiped and glorified.\n\tHe has spoken through the Prophets.\n\tWe believe in one holy catholic and apostolic Church.\n\tWe acknowledge one baptism for the forgiveness of sins.\n\tWe look for the resurrection of the dead,\n\t\tand the life of the world to come. Amen.").display_format(DisplayFormat::Unison)).label("The Nicene Creed").page(358);
}
