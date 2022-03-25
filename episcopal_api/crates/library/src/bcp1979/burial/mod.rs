pub mod parallels;

use lazy_static::lazy_static;
use liturgy::*;

use self::parallels::*;

lazy_static! {
    pub static ref AN_ORDER_FOR_BURIAL : Document = Document::new()
        .page(506)
        .label("An Order for Burial")
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "An Order for Burial"))).tags([TITLE]),
            Document::from(Rubric::from("When, for pastoral considerations, neither of the burial rites in this Book is deemed appropriate, the following form is used")).tags([OPENING_RUBRIC]),
            Document::from(Rubric::from("1. The body is received. The celebrant may meet the body and conduct it into the church or chapel, or it may be in place before the congregation assembles").long()).tags([RECEPTION]),
            Document::from(Rubric::from("2. Anthems from Holy Scripture or psalms may be sung or said, or a hymn may be sung.").long()).tags([ANTHEMS]),
            Document::from(Rubric::from("3. Prayer may be offered for the bereaved.").long()).tags([OPENING_PRAYERS]),
            Document::from(Rubric::from("4. One or more passages of Holy Scripture are read. Psalms, hymns, or anthems may follow the readings. If there is to be a Communion, the last Reading is from the Gospel.").long()).tags([LESSONS_RUBRIC]),
            Document::from(Rubric::from("5. A homily may follow the Readings, and the Apostles’ Creed may be recited.").long()).tags([HOMILY]),
            Document::from(Rubric::from("6. Prayer, including the Lord’s Prayer, is offered for the deceased, for those who mourn, and for the Christian community, remembering the promises of God in Christ about eternal life.").long()).tags([PRAYERS]),
            Document::from(Rubric::from("7. The deceased is commended to God, and the body is committed to its resting place. The committal may take place either where the preceding service has been held, or at the graveside.").long()).tags([COMMENDATION]),
            Document::from(Rubric::from("8. If there is a Communion, it precedes the commendation, and begins with the Peace and Offertory of the Eucharist. Any of the authorized eucharistic prayers may be used.").long()).tags([COMMUNION]),
            Document::from(Rubric::from("Note:\n\nThe liturgy for the dead is an Easter liturgy. It finds all its meaning in the resurrection. Because Jesus was raised from the dead, we, too, shall be raised.\n\nThe liturgy, therefore, is characterized by joy, in the certainty that “neither death, nor life, nor angels, nor principalities, nor things present, nor things to come, nor powers, nor height, nor depth, nor anything else in all creation, will be able to separate us from the love of God in Christ Jesus our Lord.”\n\nThis joy, however, does not make human grief unchristian. The very love we have for each other in Christ brings deep sorrow when we are parted by death. Jesus himself wept at the grave of his friend. So, while we rejoice that one we love has entered into the nearer presence of our Lord, we sorrow in sympathy with those who mourn.")).tags([CLOSING_RUBRIC])
        ]))));
}
