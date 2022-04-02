use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref PRAYERS_OF_THE_PEOPLE: Vec<Document> = vec![pop::FORM_I.clone()];
}

mod pop {
    use liturgy::*;

    lazy_static! {
        pub static ref FORM_I: Document = Document::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading3, "Form I"))),
            Document::from(Rubric::from("Deacon or other leader")),
            Document::from("With all our heart and with all our mind, let us pray to the Lord, saying, “Lord, have mercy.”"),
            Document::from(Litany::from((
                "Lord, have mercy.",
                [
                    "| For the peace from above, for the loving kindness of God, and for the salvation of our souls, let us pray to the Lord.",
                    "For the peace of the world, for the welfare of the holy Church of God, and for the unity of all peoples, let us pray to the Lord.",
                    "For our Bishop, and for all the clergy and people, let us pray to the Lord.",
                    "For our President, for the leaders of the nations, and for all in authority, let us pray to the Lord.",
                    "For this city (town, village, ___________), for every city and community, and for those who live in them, let us pray to the Lord.",
                    "| For seasonable weather, and for an abundance of the fruits of the earth, let us pray to the Lord.",
                    "For the good earth which God has given us, and for the wisdom and will to conserve it, let us pray to the Lord.",
                    "For those who travel on land, on water, or in the air [or through outer space], let us pray to the Lord.",
                    "For the aged and infirm, for the widowed and orphans, and for the sick and the suffering, let us pray to the Lord.",
                    "For ___________, let us pray to the Lord.",
                    "For the poor and the oppressed, for the unemployed and the destitute, for prisoners and captives, and for all who remember and care for them, let us pray to the Lord.",
                    "For all who have died in the hope of the resurrection, and for all the departed, let us pray to the Lord.",
                    "For deliverance from all danger, violence, oppression, and degradation, let us pray to the Lord.",
                    "| For the absolution and remission of our sins and offenses, let us pray to the Lord.",
                    "That we may end our lives in faith and hope, without suffering and without reproach, let us pray to the Lord.",
                    "| Defend us, deliver us, and in thy compassion protect us, O Lord, by thy grace.",
                ]
            ))),
            Document::from(Litany::from((
                "To thee, O Lord our God.",
                ["In the communion of [___________ and of all the] saints, let us commend ourselves, and one another, and all our life, to Christ our God."]
            ))),
            Document::from(Rubric::from("Silence")),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect."))
        ,]))
        .page(383);

        pub static ref FORM_II: Document = Document::from(Series::from(vec![
            Document::from(Rubric::from("In the course of the silence after each bidding, the People offer their own prayers, either silently or aloud.")),
            Document::from("I ask your prayers for God’s people throughout the world; for our Bishop(s) ___________ ; for this gathering; and for all ministers and people.\nPray for the Church."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for peace; for goodwill among nations; and for the well-being of all people.\nPray for justice and peace."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for the poor, the sick, the hungry, the oppressed, and those in prison.\nPray for those in any need or trouble."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for all who seek God, or a deeper knowledge of him.\nPray that they may find and be found by him."),
            Document::from(Rubric::from("Silence")),
            Document::from("I ask your prayers for the departed [especially __________ ].\nPray for those who have died."),
            Document::from(Series::from(vec![
                Document::from(Rubric::from("Members of the congregation may ask the prayers or the thanksgivings of those present")),
                Document::from("I ask your prayers for ____________.\n\nI ask your thanksgiving for _____________."),
                Document::from(Rubric::from("Silence."))
            ])).optional(),
            Document::from("Praise God for those in every generation in whom Christ has been honored [especially ____________whom we remember today].\nPray that we may have grace to glorify Christ in our own day."),
            Document::from(Rubric::from("Silence")),
            Document::from(Rubric::from("The Celebrant adds a concluding Collect."))
        ])).page(385);
    }
}
