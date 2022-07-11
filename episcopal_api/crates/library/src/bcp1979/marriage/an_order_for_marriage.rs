use crate::marriage_alternatives::parallels::*;
use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref AN_ORDER_FOR_MARRIAGE: Document = Document::new()
        .label("An Order for Marriage")
        .version(Version::BCP1979)
        .page(435)
        .explainer("A bare-bones outline given in the 1979 Book of Common Prayer for the requirements of a marriage service that could be celebrated otherwise than as provided in the traditional service.")
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "An Order for Marriage"))).tags([TITLE]),
            Document::from(Rubric::from("If it is desired to celebrate a marriage otherwise than as provided on page 423 of this Book [The Celebration and Blessing of a Marriage], this Order is used.")),
            Document::from(Content::DocumentLink {
                label: "Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing]),
                rotate: false,
                link_only: false
            }),
            Document::from(Rubric::from("Normally, the celebrant is a priest or bishop. Where permitted by civil law, and when no priest or bishop is available, a deacon may function as celebrant, but does not pronounce a nuptial blessing.")),
            Document::from(Rubric::from("The laws of the State and the canons of this Church having been complied with, the man and the woman, together with their witnesses, families, and friends assemble in the church or in some other convenient place.")).tags([PROCESSION_RUBRIC]),
            Document::from(Rubric::from("1. The teaching of the Church concerning Holy Matrimony, as it is declared in the formularies and canons of this Church, is briefly stated.").long()).tags([OPENING_ADDRESS]),
            Document::from(Rubric::from("2. The intention of the man and the woman to enter the state of matrimony, and their free consent, is publicly ascertained.").long()).tags([CONSENT]),
            Document::from(Rubric::from("3. One or more Readings, one of which is always from Holy Scripture, may precede the exchange of vows. If there is to be a Communion,\n\na Reading from the Gospel is always included.").long()),
            Document::from(Rubric::from("4. The vows of the man and woman are exchanged, using the following form").long()).tags([THE_MARRIAGE]),
            Document::from(Choice::from(vec![
                Document::from("In the Name of God, I, *N.*, take you, *N.*, to be my (wife) (husband), to have and to hold from this day forward, for better for worse, for richer for poorer, in sickness and in health, to love and to cherish, until we are parted by death. This is my solemn vow."),
                Document::from("I, *N.*, take thee *N.*, to my wedded (wife) (husband), to have and to hold from this day forward, for better for worse, for richer for poorer, in sickness and in health, to love and to cherish, till death us do part, according to God’s holy ordinance; and thereto I (plight) (give) thee my troth.")
            ])).tags([THE_MARRIAGE]),
            Document::from(Rubric::from("5. The Celebrant declares the union of the man and woman as husband and wife, in the Name of the Father, and of the Son, and of the Holy Spirit.").long()).tags([PRONOUNCEMENT]),
            Document::from(Rubric::from("6. Prayers are offered for the husband and wife, for their life together, for the Christian community, and for the world.").long()).tags([THE_PRAYERS_HEADER]),
            Document::from(Rubric::from("7. A priest or bishop pronounces a solemn blessing upon the couple.").long()).tags([BLESSING_OF_THE_MARRIAGE]),
            Document::from(Rubric::from("8. If there is no Communion, the service concludes with the Peace, the husband and wife first greeting each other. The Peace may be exchanged throughout the assembly.").long()).tags([THE_PEACE]),
            Document::from(Rubric::from("9. If there is to be a Communion, the service continues with the Peace and the Offertory. The Holy Eucharist may be celebrated either according to Rite One or Rite Two in this Book, or according to the Order on page 401 [An Order for Celebrating the Holy Eucharist].").long()).tags([AT_THE_EUCHARIST]),
            Document::from(Content::DocumentLink {
                label: "Holy Eucharist".into(),
                path: SlugPath::from([Slug::Eucharist]),
                rotate: false,
                link_only: false
            }).tags([AT_THE_EUCHARIST]),
        ]))));
}
