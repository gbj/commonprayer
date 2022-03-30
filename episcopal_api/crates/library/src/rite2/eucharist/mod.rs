use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref GLORIA_IN_EXCELSIS: Text = Text::from("Glory to God in the highest, 	and peace to his people on earth.\n\nLord God, heavenly King, almighty God and Father, 	we worship you, we give you thanks, 	we praise you for your glory.\n\nLord Jesus Christ, only Son of the Father, Lord God, Lamb of God, you take away the sin of the world: 	have mercy on us; you are seated at the right hand of the Father: 	receive our prayer.\n\nFor you alone are the Holy One, you alone are the Lord, you alone are the Most High, 	Jesus Christ, 	with the Holy Spirit, 	in the glory of God the Father. Amen.");

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
}
