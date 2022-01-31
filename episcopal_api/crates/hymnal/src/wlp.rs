use crate::{Hymn, HymnNumber, Hymnal, Hymnals};

#[cfg(test)]
mod tests {
    use crate::WLP;

    #[test]
    fn contains_all_consecutive_hymn_numbers() {
        let mut last_number = 720;
        for hymn in WLP.hymns {
            let num = match hymn.number {
                crate::HymnNumber::S(_) => panic!(), // no S section in WLP
                crate::HymnNumber::H(n) => n,
            };
            assert_eq!(num, last_number + 1);
            last_number = num;
        }
    }
}

pub const WLP : Hymnal = Hymnal {
    title: "Wonder, Love, and Praise",
	subtitle: "A Supplement to The Hymnal 1982",
    copyright: "Copyright © 1997 by The Church Pension Fund",
    year: 1997,
    hymns: &[
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(721),
			title: "Signs of endings all around us",
			tune: "TON-Y-BOTEL"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(722),
			title: "The desert shall rejoice",
			tune: "STERLING"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(723),
			title: "Isaiah the prophet has written of old",
			tune: "SAMANTHRA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(724),
			title: "People, look East. The time is near",
			tune: "BESANÇON CAROL"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(725),
			title: "Mingxing canlan tiandi ning (Stars shine brightly, earth is still)",
			tune: "SHENG YE JING"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(726),
			title: "Where is this stupendous stranger?",
			tune: "MARIPOSA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(727),
			title: "As panting deer desire the waterbrooks",
			tune: "WOODSLAKE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(728),
			title: "Mantos y palmas esparciendo va (Filled with excitement, all the happy throng)",
			tune: "HOSANNA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(729),
			title: "As in that upper room you left your seat",
			tune: "SURSUM CORDA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(730),
			title: "As in that upper room you left your seat",
			tune: "CHAPPELL"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(731),
			title: "Three holy days enfold us now",
			tune: "GRACE CHURCH"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(732),
			title: "Three holy days enfold us now",
			tune: "LUX VERA LUCIS RADIUM (Mode I)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(733),
			title: "Three holy days enfold us now",
			tune: "LUX VERA LUCIS RADIUM"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(734),
			title: "You laid aside your rightful reputation",
			tune: "INTERCESSOR"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(735),
			title: "O sacred head, sore wounded",
			tune: "REDDING"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(736),
			title: "When Jesus came to Golgotha they hanged him on a tree",
			tune: "INDIFFERENCE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(737),
			title: "Faithful cross, above all other (Crux fidelis inter omnes)",
			tune: "MONROVIA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(738),
			title: "Sing of the sun from darkness appearing",
			tune: "IN DIR IST FREUDE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(739),
			title: "Mira allá en el Calvario (Look on Calvary's summit)",
			tune: "NUEVA CREACIÓN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(740),
			title: "See that host all dressed in white",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(741),
			title: "Filled with the Spirit's power, with one accord",
			tune: "ASHLEY"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(742),
			title: "Loving Spirit, loving Spirit",
			tune: "OMNI DIE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(743),
			title: "O threefold God of tender unity",
			tune: "FLENTGE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(744),
			title: "O Trinity of blessed light",
			tune: "ST. MARTIN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(745),
			title: "God, beyond all human praises",
			tune: "DOMINUS REGNAVIT"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(746),
			title: "God the sculptor of the mountains",
			tune: "SANDRIA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(747),
			title: "God the sculptor of the mountains",
			tune: "URBS BEATA (Sarum Plainsong, Mode II)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(748),
			title: "From the dawning of creation",
			tune: "TIMELESS LOVE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(749),
			title: "The tree of life my soul hath seen",
			tune: "APPLE TREE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(750),
			title: "So the day dawn for me",
			tune: "WILDRIDGE &amp; ST. CHARLES, QUEENSBOROUGH TERRACE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(751),
			title: "Up on the mountain my Lord spoke",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(752),
			title: "There's a sweet, sweet Spirit in this place",
			tune: "[There&#039;s a sweet, sweet Spirit in this place]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(753),
			title: "When from bondage we are summoned",
			tune: "GRID"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(754),
			title: "When from bondage we are summoned",
			tune: "HAYWOOD&#039;S HOME"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(755),
			title: "The steadfast love of the Lord never ceases",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(756),
			title: "I am weak and I need thy strength and power",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(757),
			title: "Will you come and follow me if I but call your name?",
			tune: "MARY ALEXANDRA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(758),
			title: "Tú has venido a la orilla (You have come down to the lakeshore)",
			tune: "PESCADOR"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(759),
			title: "With awe approach the mysteries",
			tune: "HELENSONG"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(760),
			title: "O wheat, whose crushing was for bread",
			tune: "NEW LIFE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(761),
			title: "All who hunger gather gladly",
			tune: "HOLY MANNA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(762),
			title: "Whoever comes to me shall never hunger",
			tune: "KUSIK"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(763),
			title: "As we gather at your table",
			tune: "RAQUEL"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(764),
			title: "I will bless the Lord at all times",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(765),
			title: "O blessed spring, where Word and sign",
			tune: "BERGLUND"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(766),
			title: "You're called by name, forever loved",
			tune: "SARA H."
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(767),
			title: "Baptized in water",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(768),
			title: "I believe in God almighty",
			tune: "DOMHNACH TRIONOIDE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(769),
			title: "I believe in God almighty",
			tune: "ARFON (MAJOR)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(770),
			title: "O God of gentle strength",
			tune: "SHOSHANA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(771),
			title: "O God of gentle strength",
			tune: "CARLISLE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(772),
			title: "O Christ, the healer we have come",
			tune: "KEDRON"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(773),
			title: "Heal me, hands of Jesus",
			tune: "SHARPE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(774),
			title: "From miles around the sick ones came",
			tune: "TUCKER"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(775),
			title: "Give thanks for life, the measure of our days",
			tune: "SINE NOMINE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(776),
			title: "No saint on earth lives life to self alone",
			tune: "SONG I"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(777),
			title: "Sing alleluia forth in duteous praise",
			tune: "PIEPKORN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(778),
			title: "We all are one in mission",
			tune: "NYLAND"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(779),
			title: "The church of Christ in every age",
			tune: "DUNEDIN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(780),
			title: "Lord, you give the great commission",
			tune: "ABBOT&#039;S LEIGH"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(781),
			title: "Now let us rise and hymn the grace",
			tune: "OWEN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(782),
			title: "Gracious Spirit, give your servants",
			tune: "ABBOT&#039;S LEIGH"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(783),
			title: "Heleluyan, heleluyan (Alleluia, Alleluia)",
			tune: "[Heleluyan, heleluyan]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(784),
			title: "Christ the Lord to us said",
			tune: "HALELUYA! PELO TSO RONA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(785),
			title: "Santo, santo, santo (Holy, holy, holy)",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(786),
			title: "Cantad al Señor un cántico nuevo",
			tune: "CANTAI AO SENHOR"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(787),
			title: "We are marching in the light of God (Siyahamb' ekukhanyen' kwenkhos') (Marcharemos en la luz de Dios)",
			tune: "SIYAHAMBA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(788),
			title: "As newborn stars were stirred to song",
			tune: "ALEXANDRA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(789),
			title: "Peace among earth's peoples is like a star",
			tune: "PEACE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(790),
			title: "Put peace into each other's hands",
			tune: "PETA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(791),
			title: "Peace before us, peace behind us",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(792),
			title: "Holy God, you raise up prophets",
			tune: "MARTIN&#039;S SONG"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(793),
			title: "Here, O Lord, your servants gather (Sekai no tomo to te o tsunagi)",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(794),
			title: "Muchos resplandores Muchos resplandores, sólo una luz (Many are the light-beams from the one light)",
			tune: "TJÄNSTERNA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(795),
			title: "Come now, O Prince of Peace",
			tune: "O-SO-SO"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(796),
			title: "Unidos, unidos, en tu nombre (Together, together, in your name)",
			tune: "UNIDOS"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(797),
			title: "Not my brother, not my sister, but it's me, O Lord",
			tune: "[Not my brother, not my sister,but it&#039;s me, O Lord]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(798),
			title: "Lord Jesus, think on me",
			tune: "BARNFIELD"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(799),
			title: "Abide with me: fast falls the eventide",
			tune: "DORLAND MOUNTAIN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(800),
			title: "Precious Lord, take my hand",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(801),
			title: "God be with you till we meet again",
			tune: "RANDOLPH"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(802),
			title: "Cuando el pobre nada tiene (When the poor one who has nothing)",
			tune: "EL CAMINO"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(803),
			title: "These three are the treasures",
			tune: "SONG OF LAU TSU"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(804),
			title: "My Lord he calls me by the thunder",
			tune: "[My Lord, he calls me]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(805),
			title: "I want Jesus to walk with me",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(806),
			title: "If you believe and I believe",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(807),
			title: "Put down your nets and follow me",
			tune: "DILLOW"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(808),
			title: "Thuma mina (Send me, Lord)",
			tune: "THUMA MINA"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(809),
			title: "We adore you. Lord, we love you",
			tune: "[We adore you]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(810),
			title: "You who dwell in the shelter of the Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(811),
			title: "You shall cross the barren desert",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(812),
			title: "I, the Lord of sea and sky",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(813),
			title: "Way, way, way",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(814),
			title: "Jesus Christ, Son of God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(815),
			title: "Jesus said: The first commandment is this",
			tune: "AUDI, ISRAEL"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(816),
			title: "Christ is risen from the dead",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(817),
			title: "Christ is risen from the dead",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(818),
			title: "Sh'ma Yisrael (Hear, O Israel)",
			tune: "[Sh&#039;ma Yisrael (Hear, O Israel)]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(819),
			title: "Guide my feet Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(820),
			title: "The eyes of all wait upon you",
			tune: "BAYOU"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(821),
			title: "Glory to God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(822),
			title: "Through north and south and east and west",
			tune: "LASST UNS ERFREUEN"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(823),
			title: "Benedictus benedicat, per Jesum Christum",
			tune: "[Benedictus benedicat]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(824),
			title: "God grant them many years!",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(825),
			title: "Bless the Lord my soul",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(826),
			title: "Stay with me (Noho pū)",
			tune: "[Stay with me]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(827),
			title: "O Lord hear my pray'r",
			tune: "[O Lord hear my pray&#039;r]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(828),
			title: "Beati in domo Domini",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(829),
			title: "Laudate Dominum",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(830),
			title: "Laudate omnes gentes (E nā lāhuikanaka)",
			tune: "[Laudate omnes gentes]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(831),
			title: "Ubi caritas (Aia nō e loa'a)",
			tune: "[Ubi caritas]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(832),
			title: "Come, Holy Spirit, from heaven",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(833),
			title: "Our Father in heaven",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(834),
			title: "Pater Noster (Our Father)",
			tune: "[Pater noster]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(835),
			title: "Lord, have mercy",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(836),
			title: "Lord, have mercy",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(837),
			title: "Kyrie eleison",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(838),
			title: "Lord, have mercy",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(839),
			title: "Lord, have mercy",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(840),
			title: "Kyrie eleison",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(841),
			title: "Kyrie eleison",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(842),
			title: "Lord, have mercy",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(843),
			title: "Holy God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(844),
			title: "Holy God",
			tune: "Mode 1 melody"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(845),
			title: "Holy God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(846),
			title: "Holy God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(847),
			title: "Alleluia, Alleluia",
			tune: "[Alleluia, Alleluia] (Tone 2)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(848),
			title: "Alleluia, Alleluia",
			tune: "[Alleluia, Alleluia] (Tone 5)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(849),
			title: "We believe in one God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(850),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(851),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(852),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(853),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(854),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(855),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(856),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(857),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(858),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(859),
			title: "Holy, holy, holy Lord",
			tune: ""
		},
				Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(860),
			title: "Christ has died",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(861),
			title: "Christ has died",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(862),
			title: "Amen",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(863),
			title: "Amen",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(864),
			title: "Our Father in Heaven",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(865),
			title: "Alleluia. Christ our Passover is sacrificed for us",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(866),
			title: "Alleluia. Alleluia. Alleluia. Christ our Passover",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(867),
			title: "The bread which we break is a sharing",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(868),
			title: "Lamb of God, you take away the sins of the world",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(869),
			title: "Cordero de Dios (Agnus Dei)",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(870),
			title: "Cordero de Dios",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(871),
			title: "Lamb of God, you take away the sins of the world",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(872),
			title: "Agnus Dei qui tollis peccata mundi",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(873),
			title: "Alleluia. Alleluia. Those who eat my flesh",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(874),
			title: "Whoever eats this bread",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(875),
			title: "Be known to us, Lord Jesus",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(876),
			title: "The disciples knew the Lord Jesus",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(877),
			title: "The disciples knew the Lord Jesus",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(878),
			title: "Whoever comes to me shall not hunger",
			tune: "[Whoever comes to me]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(879),
			title: "Christ our Passover",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(880),
			title: "God's Paschal Lamb is sacrificed for us",
			tune: "SINE NOMINE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(881),
			title: "Surely, it is God who saves me",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(882),
			title: "Surely, it is God who saves me",
			tune: "[Surely, it is God who saves me] (Tone 7)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(883),
			title: "Arise, shine, for your light has come",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(884),
			title: "O all ye works of God",
			tune: "ROCKVILLE"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(885),
			title: "Let all creation bless the Lord",
			tune: "WHITEHEAD"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(886),
			title: "Glory to you, Lord God of our fathers",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(887),
			title: "Glory to you, Lord God of our fathers",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(888),
			title: "Almighty Lord Most High draw near",
			tune: "KEISER NEW"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(889),
			title: "Blessed be the God of Israel",
			tune: "SHEPHERD&#039;S PIPES"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(890),
			title: "Blessed are you O Lord our God",
			tune: "[Blessed are you O Lord our God] (Tone 5)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(891),
			title: "With my own eyes I have seen the salvation",
			tune: "PORT ARTHUR"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(892),
			title: "Splendor and honor, majesty and power",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(893),
			title: "Splendor and honor and kingly power",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(894),
			title: "Splendor and honor and royal pow'r",
			tune: "[Splendor and honor and royal pow&#039;r]"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(895),
			title: "O ruler of the universe, Lord God",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(896),
			title: "Glory to God in the highest",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(897),
			title: "Glory to God in the highest",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(898),
			title: "Glory to God in the highest",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(899),
			title: "Glory, Glory, hallelujah, Lord we praise your Holy name",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(900),
			title: "Glory to God in the highest",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(901),
			title: "Glory to God in the highest",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(902),
			title: "You are God: we praise you",
			tune: "[You are God: we praise you] (Plainsong, Tone 8)"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(903),
			title: "You are God: we praise you",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(904),
			title: "Wisdom freed from a nation of oppressors",
			tune: ""
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(905),
			title: "Wisdom freed a holy people",
			tune: "BREWER"
		},
		Hymn {
			source: Hymnals::WLP,
			number: HymnNumber::H(906),
			title: "Even when young, I prayed for wisdom's grace",
			tune: "DILIGENCE"
		},
	],
};
