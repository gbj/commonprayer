use lazy_static::lazy_static;
use liturgy::*;
use status::Status;

lazy_static! {
    pub static ref AN_ORDER_FOR_MARRIAGE_2: Document = Document::new()
        .label("An Order for Marriage 2")
        .version(Version::Expansive)
        .status(Status::TrialUse)
        .source(Reference {
            source: Source::LiturgicalResources2,
            page: 26
        })
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "An Order for Marriage"))),
            Document::from(Rubric::from("If it is desired to celebrate a marriage otherwise than as provided on page 423 of The Book of Common Prayer, or in the trial-use liturgies “The Witnessing and Blessing of a Marriage” or “The Celebration and Blessing of a Marriage 2,” this Order is used.")),
            Document::from(Content::DocumentLink {
                label: "BCP Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing]),
                rotate: false,
                link_only: false
            }),
            Document::from(Content::DocumentLink {
                label: "The Witnessing and Blessing of a Marriage".into(),
                path: SlugPath::from([Slug::Marriage, Slug::WitnessingAndBlessing]),
                rotate: false,
                link_only: false
            }),
            Document::from(Content::DocumentLink {
                label: "The Celebration and Blessing of a Marriage".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing, Slug::Version(Version::Expansive)]),
                rotate: false,
                link_only: false
            }),
            Document::from(Rubric::from("Normally, the celebrant is a priest or bishop. Where permitted by civil law, and when no priest or bishop is available, a deacon may function as celebrant, but does not pronounce a nuptial blessing.\n\nThe laws of the State and the Canons of this Church having been complied with, the couple, together with their witnesses, families, and friends assemble in the church or in some other convenient place.")),
            Document::from(Rubric::from("1. The teaching of the Church concerning Holy Matrimony, as it is declared in the formularies and Canons of this Church, is briefly stated. \n\n2. The intention of the couple to enter the state of matrimony, and their free consent, is publicly ascertained. \n\n3. One or more Readings, one of which is always from Holy Scripture, may precede the exchange of vows. If there is to be a Communion, a Reading from the Gospel is always included. \n\n4. The vows are exchanged, using the following form").long()),
            Document::from(Choice::from(vec![
                Document::from("In the Name of God,\nI, *N.*, take you, *N.*, to be my *wife/husband/spouse*, \nto have and to hold from this day forward,\nfor better for worse, for richer for poorer,\nin sickness and in health, to love and to cherish, \nuntil we are parted by death.\nThis is my solemn vow."),
                Document::from("I, *N.*, take thee *N.*, to my wedded *wife/husband/spouse*, \nto have and to hold from this day forward,\nfor better for worse, for richer for poorer,\nin sickness and in health, to love and to cherish,\ntill death us do part, according to God’s holy ordinance; \nand thereto I plight [*or* give] thee my troth.")
            ])),
            Document::from(Rubric::from("5. The Celebrant declares the union of the couple, in the Name of the Father, and of the Son, and of the Holy Spirit. \n\n6. Prayers are offered for the couple, for their life together, for the Christian community, and for the world. \n\n7. A priest or bishop pronounces a solemn blessing upon the couple. \n\n8. If there is no Communion, the service concludes with the Peace, the couple first greeting each other. The Peace may be exchanged throughout the assembly. \n\n9. If there is to be a Communion, the service continues with the Peace and the Offertory. The Holy Eucharist may be celebrated either according to Rite One or Rite Two, or according to the Order on page 401 of the Book of Common Prayer 1979.").long()),
            Document::from(Content::DocumentLink {
                    label: "Holy Eucharist".into(),
                    path: SlugPath::from([Slug::Eucharist]),
                    rotate: false,
                    link_only: false
                }),
        ]))));
}
