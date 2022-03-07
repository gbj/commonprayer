use lazy_static::lazy_static;
use liturgy::{Document, Rubric, Series, Version};

lazy_static! {
    pub static ref CONCERNING_THE_SERVICE : Document = Document::from(Series::from(vec![
        Document::from(Rubric::from("Christian marriage is a solemn and public covenant between a man and a woman in the presence of God. In the Episcopal Church it is required that one, at least, of the parties must be a baptized Christian; that the ceremony be attested by at least two witnesses; and that the marriage conform to the laws of the State and the canons of this Church.\n\nA priest or a bishop normally presides at the Celebration and Blessing of a Marriage, because such ministers alone have the function of pronouncing the nuptial blessing, and of celebrating the Holy Eucharist.\n\nWhen both a bishop and a priest are present and officiating, the bishop should pronounce the blessing and preside at the Eucharist.\n\nA deacon, or an assisting priest, may deliver the charge, ask for the Declaration of Consent, read the Gospel, and perform other assisting functions at the Eucharist.\n\nWhere it is permitted by civil law that deacons may perform marriages, and no priest or bishop is available, a deacon may use the service which follows, omitting the nuptial blessing which follows The Prayers.\n\nIt is desirable that the Lessons from the Old Testament and the Epistles be read by lay persons.\n\nIn the opening exhortation (at the symbol of *N.N.*), the full names of the persons to be married are declared. Subsequently, only their Christian names are used.").long())
    ]))
    .label("Concerning the Service")
    .page(422)
    .version(Version::BCP1979);

    pub static ref ADDITIONAL_DIRECTIONS : Document = Document::from(Series::from(vec![
        Document::from(Rubric::from("If Banns are to be published, the following form is used")),
        Document::from("I publish the Banns of Marriage between *N.N.* of ____________ and *N.N.* of ___________. If any of you know just cause why they may not be joined together in Holy Matrimony, you are bidden to declare it. This is the first (*or* second, *or* third) time of asking."),
        Document::from(Rubric::from("The Celebration and Blessing of a Marriage may be used with any authorized liturgy for the Holy Eucharist. This service then replaces the Ministry of the Word, and the Eucharist begins with the Offertory.\n\nAfter the Declaration of Consent, if there is to be a giving in marriage, or presentation, the Celebrant asks,").long()),
        Document::from("Who gives (presents) this woman to be married to this man?"),
        Document::from(Rubric::from("or the following")),
        Document::from("Who presents this woman and this man to be married to each other?"),
        Document::from(Rubric::from("To either question, the appropriate answer is,“I do.” If more than one person responds, they do so together.\n\nFor the Ministry of the Word it is fitting that the man and woman to be married remain where they may conveniently hear the reading of Scripture. They may approach the Altar, either for the exchange of vows, or for the Blessing of the Marriage.\n\nIt is appropriate that all remain standing until the conclusion of the Collect. Seating may be provided for the wedding party, so that all may be seated for the Lessons and the homily.\n\nThe Apostles’ Creed may be recited after the Lessons, or after the homily, if there is one.\n\nWhen desired, some other suitable symbol of the vows may be used in place of the ring.").long())
    ]))
    .label("Additional Directions")
    .page(437)
    .version(Version::BCP1979);
}

mod an_order_for_marriage;
mod blessing_civil_marriage;
mod celebration_and_blessing;

pub use an_order_for_marriage::*;
pub use blessing_civil_marriage::*;
pub use celebration_and_blessing::*;
