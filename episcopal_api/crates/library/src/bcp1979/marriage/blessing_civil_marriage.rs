use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref BLESSING_OF_A_CIVIL_MARRIAGE: Document = Document::new()
        .label("Blessing of a Civil Marriage")
        .version(Version::BCP1979)
        .page(433)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Blessing\nof a Civil Marriage"))),
            Document::from(Rubric::from("The Rite begins as prescribed for celebrations of the Holy Eucharist, using the Collect and Lessons appointed in the Marriage service.")),
            Document::from(Content::DocumentLink {
                label: "Holy Eucharist".into(),
                path: SlugPath::from([Slug::Eucharist]),
                rotate: false,
                link_only: false
            }),
            Document::from(Content::DocumentLink {
                label: "Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing]),
                rotate: false,
                link_only: false
            }),

            // Consent
            Document::from(Rubric::from("After the Gospel (and homily), the husband and wife stand before the Celebrant, who addresses them in these or similar words")),
            Document::from("*N.* and *N.*, you have come here today to seek the blessing of God and of his Church upon your marriage. I require, therefore, that you promise, with the help of God, to fulfill the obligations which Christian Marriage demands."),

            Document::from(Rubric::from("The Celebrant then addresses the husband, saying")),
            Document::from("*N.*, you have taken *N.* to be your wife. Do you promise to love her, comfort her, honor and keep her, in sickness and in health; and, forsaking all others, to be faithful to her as long as you both shall live?"),
            Document::from(Rubric::from("The Husband answers")),
            Document::from("I do."),

            Document::from(Rubric::from("The Celebrant then addresses the wife, saying")),
            Document::from("*N.*, you have taken *N.* to be your husband. Do you promise to love him, comfort him, honor and keep him, in sickness and in health; and, forsaking all others, to be faithful to him as long as you both shall live?"),
            Document::from(Rubric::from("The Wife answers")),
            Document::from("I do."),

            Document::from(Rubric::from("The Celebrant then addresses the congregation, saying")),
            Document::from("Will you who have witnessed these promises do all in your power to uphold these two persons in their marriage?"),
            Document::from(Preces::from([
                ("", ""),
                ("People", "We will.")
            ])),

            Document::from(Rubric::from("If a ring or rings are to be blessed, the wife extends her hand (and the husband extends his hand) toward the Priest, who says")),
            Document::from(Text::from("Bless, O Lord, *this ring* to be *a sign* of the vows by which this man and this woman have bound themselves to each other; through Jesus Christ our Lord.").response("Amen.")),
            Document::from(Rubric::from("The Celebrant joins the right hands of the husband and wife and says")),
            Document::from("Those whom God has joined together let no one put asunder."),
            Document::from(Rubric::from("The Congregation responds")),
            Document::from(Text::from("").response("Amen.")),

            Document::from(Rubric::from("The service continues with The Prayers on page 428 [in the marriage service].")),
            Document::from(Content::DocumentLink {
                label: "Marriage Service".into(),
                path: SlugPath::from([Slug::Marriage, Slug::CelebrationAndBlessing]),
                rotate: false,
                link_only: false
            }),
        ])
    )));
}
