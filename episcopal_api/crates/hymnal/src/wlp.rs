use crate::{Hymn, HymnNumber, Hymnal, Hymnals};

use lazy_static::lazy_static;

#[cfg(test)]
mod tests {
    use crate::WLP;

    #[test]
    fn contains_all_consecutive_hymn_numbers() {
        let mut last_number = 720;
        for hymn in &WLP.hymns {
            let num = match hymn.number {
                crate::HymnNumber::S(_) => panic!(), // no S section in WLP
                crate::HymnNumber::H(n) => n,
            };
            assert_eq!(num, last_number + 1);
            last_number = num;
        }
    }
}

lazy_static! {
    pub static ref WLP : Hymnal = Hymnal {
        id: Hymnals::WLP,
        title: "Wonder, Love, and Praise".to_string(),
        subtitle: "A Supplement to The Hymnal 1982".to_string(),
        copyright: "Copyright © 1997 by The Church Pension Fund".to_string(),
        year: 1997,
        hymns: vec![
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(721),
                title: "Signs of endings all around us".to_string(),
                tune: "TON-Y-BOTEL".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(722),
                title: "The desert shall rejoice".to_string(),
                tune: "STERLING".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(723),
                title: "Isaiah the prophet has written of old".to_string(),
                tune: "SAMANTHRA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(724),
                title: "People, look East. The time is near".to_string(),
                tune: "BESANÇON CAROL".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(725),
                title: "Mingxing canlan tiandi ning (Stars shine brightly, earth is still)".to_string(),
                tune: "SHENG YE JING".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(726),
                title: "Where is this stupendous stranger?".to_string(),
                tune: "MARIPOSA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(727),
                title: "As panting deer desire the waterbrooks".to_string(),
                tune: "WOODSLAKE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(728),
                title: "Mantos y palmas esparciendo va (Filled with excitement, all the happy throng)".to_string(),
                tune: "HOSANNA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(729),
                title: "As in that upper room you left your seat".to_string(),
                tune: "SURSUM CORDA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(730),
                title: "As in that upper room you left your seat".to_string(),
                tune: "CHAPPELL".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(731),
                title: "Three holy days enfold us now".to_string(),
                tune: "GRACE CHURCH".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(732),
                title: "Three holy days enfold us now".to_string(),
                tune: "LUX VERA LUCIS RADIUM (Mode I)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(733),
                title: "Three holy days enfold us now".to_string(),
                tune: "LUX VERA LUCIS RADIUM".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(734),
                title: "You laid aside your rightful reputation".to_string(),
                tune: "INTERCESSOR".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(735),
                title: "O sacred head, sore wounded".to_string(),
                tune: "REDDING".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(736),
                title: "When Jesus came to Golgotha they hanged him on a tree".to_string(),
                tune: "INDIFFERENCE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(737),
                title: "Faithful cross, above all other (Crux fidelis inter omnes)".to_string(),
                tune: "MONROVIA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(738),
                title: "Sing of the sun from darkness appearing".to_string(),
                tune: "IN DIR IST FREUDE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(739),
                title: "Mira allá en el Calvario (Look on Calvary's summit)".to_string(),
                tune: "NUEVA CREACIÓN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(740),
                title: "See that host all dressed in white".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(741),
                title: "Filled with the Spirit's power, with one accord".to_string(),
                tune: "ASHLEY".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(742),
                title: "Loving Spirit, loving Spirit".to_string(),
                tune: "OMNI DIE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(743),
                title: "O threefold God of tender unity".to_string(),
                tune: "FLENTGE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(744),
                title: "O Trinity of blessed light".to_string(),
                tune: "ST. MARTIN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(745),
                title: "God, beyond all human praises".to_string(),
                tune: "DOMINUS REGNAVIT".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(746),
                title: "God the sculptor of the mountains".to_string(),
                tune: "SANDRIA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(747),
                title: "God the sculptor of the mountains".to_string(),
                tune: "URBS BEATA (Sarum Plainsong, Mode II)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(748),
                title: "From the dawning of creation".to_string(),
                tune: "TIMELESS LOVE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(749),
                title: "The tree of life my soul hath seen".to_string(),
                tune: "APPLE TREE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(750),
                title: "So the day dawn for me".to_string(),
                tune: "WILDRIDGE &amp; ST. CHARLES, QUEENSBOROUGH TERRACE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(751),
                title: "Up on the mountain my Lord spoke".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(752),
                title: "There's a sweet, sweet Spirit in this place".to_string(),
                tune: "[There&#039;s a sweet, sweet Spirit in this place]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(753),
                title: "When from bondage we are summoned".to_string(),
                tune: "GRID".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(754),
                title: "When from bondage we are summoned".to_string(),
                tune: "HAYWOOD&#039;S HOME".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(755),
                title: "The steadfast love of the Lord never ceases".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(756),
                title: "I am weak and I need thy strength and power".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(757),
                title: "Will you come and follow me if I but call your name?".to_string(),
                tune: "MARY ALEXANDRA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(758),
                title: "Tú has venido a la orilla (You have come down to the lakeshore)".to_string(),
                tune: "PESCADOR".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(759),
                title: "With awe approach the mysteries".to_string(),
                tune: "HELENSONG".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(760),
                title: "O wheat, whose crushing was for bread".to_string(),
                tune: "NEW LIFE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(761),
                title: "All who hunger gather gladly".to_string(),
                tune: "HOLY MANNA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(762),
                title: "Whoever comes to me shall never hunger".to_string(),
                tune: "KUSIK".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(763),
                title: "As we gather at your table".to_string(),
                tune: "RAQUEL".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(764),
                title: "I will bless the Lord at all times".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(765),
                title: "O blessed spring, where Word and sign".to_string(),
                tune: "BERGLUND".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(766),
                title: "You're called by name, forever loved".to_string(),
                tune: "SARA H.".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(767),
                title: "Baptized in water".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(768),
                title: "I believe in God almighty".to_string(),
                tune: "DOMHNACH TRIONOIDE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(769),
                title: "I believe in God almighty".to_string(),
                tune: "ARFON (MAJOR)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(770),
                title: "O God of gentle strength".to_string(),
                tune: "SHOSHANA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(771),
                title: "O God of gentle strength".to_string(),
                tune: "CARLISLE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(772),
                title: "O Christ, the healer we have come".to_string(),
                tune: "KEDRON".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(773),
                title: "Heal me, hands of Jesus".to_string(),
                tune: "SHARPE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(774),
                title: "From miles around the sick ones came".to_string(),
                tune: "TUCKER".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(775),
                title: "Give thanks for life, the measure of our days".to_string(),
                tune: "SINE NOMINE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(776),
                title: "No saint on earth lives life to self alone".to_string(),
                tune: "SONG I".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(777),
                title: "Sing alleluia forth in duteous praise".to_string(),
                tune: "PIEPKORN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(778),
                title: "We all are one in mission".to_string(),
                tune: "NYLAND".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(779),
                title: "The church of Christ in every age".to_string(),
                tune: "DUNEDIN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(780),
                title: "Lord, you give the great commission".to_string(),
                tune: "ABBOT&#039;S LEIGH".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(781),
                title: "Now let us rise and hymn the grace".to_string(),
                tune: "OWEN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(782),
                title: "Gracious Spirit, give your servants".to_string(),
                tune: "ABBOT&#039;S LEIGH".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(783),
                title: "Heleluyan, heleluyan (Alleluia, Alleluia)".to_string(),
                tune: "[Heleluyan, heleluyan]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(784),
                title: "Christ the Lord to us said".to_string(),
                tune: "HALELUYA! PELO TSO RONA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(785),
                title: "Santo, santo, santo (Holy, holy, holy)".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(786),
                title: "Cantad al Señor un cántico nuevo".to_string(),
                tune: "CANTAI AO SENHOR".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(787),
                title: "We are marching in the light of God (Siyahamb' ekukhanyen' kwenkhos') (Marcharemos en la luz de Dios)".to_string(),
                tune: "SIYAHAMBA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(788),
                title: "As newborn stars were stirred to song".to_string(),
                tune: "ALEXANDRA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(789),
                title: "Peace among earth's peoples is like a star".to_string(),
                tune: "PEACE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(790),
                title: "Put peace into each other's hands".to_string(),
                tune: "PETA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(791),
                title: "Peace before us, peace behind us".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(792),
                title: "Holy God, you raise up prophets".to_string(),
                tune: "MARTIN&#039;S SONG".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(793),
                title: "Here, O Lord, your servants gather (Sekai no tomo to te o tsunagi)".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(794),
                title: "Muchos resplandores Muchos resplandores, sólo una luz (Many are the light-beams from the one light)".to_string(),
                tune: "TJÄNSTERNA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(795),
                title: "Come now, O Prince of Peace".to_string(),
                tune: "O-SO-SO".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(796),
                title: "Unidos, unidos, en tu nombre (Together, together, in your name)".to_string(),
                tune: "UNIDOS".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(797),
                title: "Not my brother, not my sister, but it's me, O Lord".to_string(),
                tune: "[Not my brother, not my sister,but it&#039;s me, O Lord]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(798),
                title: "Lord Jesus, think on me".to_string(),
                tune: "BARNFIELD".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(799),
                title: "Abide with me: fast falls the eventide".to_string(),
                tune: "DORLAND MOUNTAIN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(800),
                title: "Precious Lord, take my hand".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(801),
                title: "God be with you till we meet again".to_string(),
                tune: "RANDOLPH".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(802),
                title: "Cuando el pobre nada tiene (When the poor one who has nothing)".to_string(),
                tune: "EL CAMINO".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(803),
                title: "These three are the treasures".to_string(),
                tune: "SONG OF LAU TSU".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(804),
                title: "My Lord he calls me by the thunder".to_string(),
                tune: "[My Lord, he calls me]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(805),
                title: "I want Jesus to walk with me".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(806),
                title: "If you believe and I believe".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(807),
                title: "Put down your nets and follow me".to_string(),
                tune: "DILLOW".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(808),
                title: "Thuma mina (Send me, Lord)".to_string(),
                tune: "THUMA MINA".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(809),
                title: "We adore you. Lord, we love you".to_string(),
                tune: "[We adore you]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(810),
                title: "You who dwell in the shelter of the Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(811),
                title: "You shall cross the barren desert".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(812),
                title: "I, the Lord of sea and sky".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(813),
                title: "Way, way, way".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(814),
                title: "Jesus Christ, Son of God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(815),
                title: "Jesus said: The first commandment is this".to_string(),
                tune: "AUDI, ISRAEL".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(816),
                title: "Christ is risen from the dead".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(817),
                title: "Christ is risen from the dead".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(818),
                title: "Sh'ma Yisrael (Hear, O Israel)".to_string(),
                tune: "[Sh&#039;ma Yisrael (Hear, O Israel)]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(819),
                title: "Guide my feet Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(820),
                title: "The eyes of all wait upon you".to_string(),
                tune: "BAYOU".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(821),
                title: "Glory to God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(822),
                title: "Through north and south and east and west".to_string(),
                tune: "LASST UNS ERFREUEN".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(823),
                title: "Benedictus benedicat, per Jesum Christum".to_string(),
                tune: "[Benedictus benedicat]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(824),
                title: "God grant them many years!".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(825),
                title: "Bless the Lord my soul".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(826),
                title: "Stay with me (Noho pū)".to_string(),
                tune: "[Stay with me]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(827),
                title: "O Lord hear my pray'r".to_string(),
                tune: "[O Lord hear my pray&#039;r]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(828),
                title: "Beati in domo Domini".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(829),
                title: "Laudate Dominum".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(830),
                title: "Laudate omnes gentes (E nā lāhuikanaka)".to_string(),
                tune: "[Laudate omnes gentes]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(831),
                title: "Ubi caritas (Aia nō e loa'a)".to_string(),
                tune: "[Ubi caritas]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(832),
                title: "Come, Holy Spirit, from heaven".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(833),
                title: "Our Father in heaven".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(834),
                title: "Pater Noster (Our Father)".to_string(),
                tune: "[Pater noster]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(835),
                title: "Lord, have mercy".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(836),
                title: "Lord, have mercy".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(837),
                title: "Kyrie eleison".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(838),
                title: "Lord, have mercy".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(839),
                title: "Lord, have mercy".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(840),
                title: "Kyrie eleison".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(841),
                title: "Kyrie eleison".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(842),
                title: "Lord, have mercy".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(843),
                title: "Holy God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(844),
                title: "Holy God".to_string(),
                tune: "Mode 1 melody".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(845),
                title: "Holy God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(846),
                title: "Holy God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(847),
                title: "Alleluia, Alleluia".to_string(),
                tune: "[Alleluia, Alleluia] (Tone 2)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(848),
                title: "Alleluia, Alleluia".to_string(),
                tune: "[Alleluia, Alleluia] (Tone 5)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(849),
                title: "We believe in one God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(850),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(851),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(852),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(853),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(854),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(855),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(856),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(857),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(858),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(859),
                title: "Holy, holy, holy Lord".to_string(),
                tune: "".to_string()
            },
                    Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(860),
                title: "Christ has died".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(861),
                title: "Christ has died".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(862),
                title: "Amen".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(863),
                title: "Amen".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(864),
                title: "Our Father in Heaven".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(865),
                title: "Alleluia. Christ our Passover is sacrificed for us".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(866),
                title: "Alleluia. Alleluia. Alleluia. Christ our Passover".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(867),
                title: "The bread which we break is a sharing".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(868),
                title: "Lamb of God, you take away the sins of the world".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(869),
                title: "Cordero de Dios (Agnus Dei)".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(870),
                title: "Cordero de Dios".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(871),
                title: "Lamb of God, you take away the sins of the world".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(872),
                title: "Agnus Dei qui tollis peccata mundi".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(873),
                title: "Alleluia. Alleluia. Those who eat my flesh".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(874),
                title: "Whoever eats this bread".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(875),
                title: "Be known to us, Lord Jesus".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(876),
                title: "The disciples knew the Lord Jesus".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(877),
                title: "The disciples knew the Lord Jesus".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(878),
                title: "Whoever comes to me shall not hunger".to_string(),
                tune: "[Whoever comes to me]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(879),
                title: "Christ our Passover".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(880),
                title: "God's Paschal Lamb is sacrificed for us".to_string(),
                tune: "SINE NOMINE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(881),
                title: "Surely, it is God who saves me".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(882),
                title: "Surely, it is God who saves me".to_string(),
                tune: "[Surely, it is God who saves me] (Tone 7)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(883),
                title: "Arise, shine, for your light has come".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(884),
                title: "O all ye works of God".to_string(),
                tune: "ROCKVILLE".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(885),
                title: "Let all creation bless the Lord".to_string(),
                tune: "WHITEHEAD".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(886),
                title: "Glory to you, Lord God of our fathers".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(887),
                title: "Glory to you, Lord God of our fathers".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(888),
                title: "Almighty Lord Most High draw near".to_string(),
                tune: "KEISER NEW".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(889),
                title: "Blessed be the God of Israel".to_string(),
                tune: "SHEPHERD&#039;S PIPES".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(890),
                title: "Blessed are you O Lord our God".to_string(),
                tune: "[Blessed are you O Lord our God] (Tone 5)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(891),
                title: "With my own eyes I have seen the salvation".to_string(),
                tune: "PORT ARTHUR".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(892),
                title: "Splendor and honor, majesty and power".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(893),
                title: "Splendor and honor and kingly power".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(894),
                title: "Splendor and honor and royal pow'r".to_string(),
                tune: "[Splendor and honor and royal pow&#039;r]".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(895),
                title: "O ruler of the universe, Lord God".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(896),
                title: "Glory to God in the highest".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(897),
                title: "Glory to God in the highest".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(898),
                title: "Glory to God in the highest".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(899),
                title: "Glory, Glory, hallelujah, Lord we praise your Holy name".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(900),
                title: "Glory to God in the highest".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(901),
                title: "Glory to God in the highest".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(902),
                title: "You are God: we praise you".to_string(),
                tune: "[You are God: we praise you] (Plainsong, Tone 8)".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(903),
                title: "You are God: we praise you".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(904),
                title: "Wisdom freed from a nation of oppressors".to_string(),
                tune: "".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(905),
                title: "Wisdom freed a holy people".to_string(),
                tune: "BREWER".to_string()
            },
            Hymn {
                source: Hymnals::WLP,
                number: HymnNumber::H(906),
                title: "Even when young, I prayed for wisdom's grace".to_string(),
                tune: "DILIGENCE".to_string()
            },
        ],
    };
}
