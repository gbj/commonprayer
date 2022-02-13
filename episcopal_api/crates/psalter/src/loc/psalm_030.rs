use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_30: Psalm = Psalm {
		number: 30,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 520
				},
				local_name: String::from(""),
				latin_name: String::from("Exaltabo te, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Te ensalzaré, oh Señor, porque me has alzado, *"),
						b: String::from("y no permitiste que mis enemigos triunfaran sobre mí.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Oh Señor Dios mío, a ti clamé, *"),
						b: String::from("y tú me sanaste.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Oh Señor, me sacaste del abismo; *"),
						b: String::from("me hiciste revivir, para que no descendiese a la\nsepultura.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Canten al Señor, ustedes sus fieles, *"),
						b: String::from("y celebren su santo Nombre;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque sólo un momento dura su ira, *"),
						b: String::from("pero su favor toda la vida.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Aunque al anochecer nos visite el llanto, *"),
						b: String::from("en la mañana vendrá la alegría.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Dije yo en mi comodidad, \"No seré jamás conmovido; *"),
						b: String::from("tú, oh Señor, con tu favor\nme afirmaste como monte fuerte\".")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Luego escondiste tu rostro, *"),
						b: String::from("y fui muy turbado.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("A ti, oh Señor, clamé, *"),
						b: String::from("y a mi Soberano supliqué, diciendo:")
					},
					PsalmVerse {
						number: 10,
						a: String::from("\"¿Qué provecho hay en mi muerte, cuando yo descienda a la fosa? *"),
						b: String::from("¿Te alabará el polvo? ¿Anunciará tu fidelidad?")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Escucha, oh Señor, y ten misericordia de mí; oh Señor sé tú mi ayudador.\""),
						b: String::from("")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Has cambiado mi lamento en danzas; *"),
						b: String::from("me has quitado el luto, y me has vestido de fiesta.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Por tanto a ti canta mi corazón, y no llora más; *"),
						b: String::from("oh Señor Dios mío, te daré gracias para siempre.")
					}
				]
			}
		]
	};
}