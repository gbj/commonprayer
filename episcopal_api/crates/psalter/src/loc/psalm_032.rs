use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_32: Psalm = Psalm {
		number: 32,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 523
				},
				local_name: String::from(""),
				latin_name: String::from("Beati quorum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bienaventurados aquéllos cuyas transgresiones son perdonadas, *"),
						b: String::from("y quitados sus pecados.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Bienaventurados a quienes no atribuye culpa el Señor, *"),
						b: String::from("y en cuyo espíritu no hay engaño.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Mientras callé, se envejecieron mis huesos *"),
						b: String::from("porque gemí todo el día;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque de día y de noche pesó sobre mí tu mano; *"),
						b: String::from("se volvió mi verdor en sequedad de verano.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mi pecado entonces te declaré, *"),
						b: String::from("y no encubrí mi culpa.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Dije: \"Confesaré a ti mis transgresiones\"; *"),
						b: String::from("y luego tú perdonaste la culpa de mi pecado")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Por ello orarán los fieles en tiempo de necesidad *"),
						b: String::from("ciertamente en la inundación de muchas aguas\nno llegará ésta a ellos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Tú eres mi escondite; me guardarás de angustias; *"),
						b: String::from("con gritos de liberación me rodearás.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("\"Te instruiré, y te enseñaré el camino en que debes andar;*"),
						b: String::from("sobre ti fijaré mis ojos.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("No seas como el caballo, o como el mulo, sin entendimiento; *"),
						b: String::from("que ha de ser sujetado con cabestro y con freno, porque si no, no se acerca a ti\".")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Muchos dolores habrá para los malvados, *"),
						b: String::from("mas a los que esperan en el Señor,\nlos abraza la misericordia.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Alégrense en el Señor, y gócense, justos; *"),
						b: String::from("vitoreen con júbilo, todos los rectos de corazón.")
					}
				]
			}
		]
	};
}