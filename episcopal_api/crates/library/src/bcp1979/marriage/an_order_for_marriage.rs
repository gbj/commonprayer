use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref AN_ORDER_FOR_MARRIAGE: Document = Document::new()
        .label("An Order for Marriage")
        .version(Version::BCP1979)
        .page(435)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "An Order for Marriage"))),
            Document::from(Rubric::from("If it is desired to celebrate a marriage otherwise than as provided on page 423 of this Book [The Celebration and Blessing of a Marriage], this Order is used.")),
            Document::from(Content::DocumentLink(Version::BCP1979, "Marriage Service".into(), "marriage".into(), "celebration-and-blessing-of-a-marriage".into())),
            Document::from(Rubric::from("Normally, the celebrant is a priest or bishop. Where permitted by civil law, and when no priest or bishop is available, a deacon may function as celebrant, but does not pronounce a nuptial blessing.\n\nThe laws of the State and the canons of this Church having been complied with, the man and the woman, together with their witnesses, families, and friends assemble in the church or in some other convenient place.")),
            Document::from(Rubric::from("1. The teaching of the Church concerning Holy Matrimony, as it is declared in the formularies and canons of this Church, is briefly stated.\n\n2. The intention of the man and the woman to enter the state of matrimony, and their free consent, is publicly ascertained.\n\n3. One or more Readings, one of which is always from Holy Scripture, may precede the exchange of vows. If there is to be a Communion,\n\na Reading from the Gospel is always included.\n\n4. The vows of the man and woman are exchanged, using the following form").long()),
            Document::from(Choice::from(vec![
                Document::from("In the Name of God, I, *N.*, take you, *N.*, to be my (wife) (husband), to have and to hold from this day forward, for better for worse, for richer for poorer, in sickness and in health, to love and to cherish, until we are parted by death. This is my solemn vow."),
                Document::from("I, *N.*, take thee *N.*, to my wedded (wife) (husband), to have and to hold from this day forward, for better for worse, for richer for poorer, in sickness and in health, to love and to cherish, till death us do part, according to God’s holy ordinance; and thereto I (plight) (give) thee my troth.")
            ])),
            Document::from(Rubric::from("5. The Celebrant declares the union of the man and woman as husband and wife, in the Name of the Father, and of the Son, and of the Holy Spirit.\n\n6. Prayers are offered for the husband and wife, for their life together, for the Christian community, and for the world.\n\n7. A priest or bishop pronounces a solemn blessing upon the couple.\n\n8. If there is no Communion, the service concludes with the Peace, the\n\nhusband and wife first greeting each other. The Peace may be exchanged throughout the assembly.\n\n9. If there is to be a Communion, the service continues with the Peace and the Offertory. The Holy Eucharist may be celebrated either according to Rite One or Rite Two in this Book, or according to the Order on page 401 [An Order for Celebrating the Holy Eucharist].").long()),
            Document::from(Content::DocumentLink(Version::BCP1979, "Holy Eucharist".into(), "eucharist".into(), "holy-eucharist".into())),
        ]))));
}
