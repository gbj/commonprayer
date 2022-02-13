use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_36: Psalm = Psalm {
		number: 36,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 530
				},
				local_name: String::from(""),
				latin_name: String::from("Dixit injustus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oráculo de rebelión hay para el malvado, en lo íntimo de su corazón; *"),
						b: String::from("no hay temor de Dios delante de sus ojos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Se lisonjea en sus propios ojos *"),
						b: String::from("de que su pecado odioso no será hallado.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Las palabras de su boca son iniquidad y fraude; *"),
						b: String::from("ha dejado de ser cuerdo y de hacer el bien.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Concibe maldad en su cama; se obstina en el mal camino; *"),
						b: String::from("el mal no aborrece.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Oh Señor, hasta los cielos llega tu amor; *"),
						b: String::from("tu fidelidad alcanza hasta las nubes.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Tu benevolencia es como las montañas más altas, tu providencia, como el abismo grande; *"),
						b: String::from("tú salvas, oh Señor, tanto a los humanos como a las bestias.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¡Cuán precioso es tu amor! *"),
						b: String::from("Mortales e inmortales se acogen\nbajo la sombra de tus alas.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Festejan la abundancia de tu casa; *"),
						b: String::from("los abrevarás del torrente de tus delicias;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Porque contigo está el manantial de la vida, *"),
						b: String::from("y en tu luz vemos la luz.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Extiende tu bondad a los que te conocen, *"),
						b: String::from("y tu favor a los rectos de corazón.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Que no me pisotee el pie del soberbio, *"),
						b: String::from("ni me eche al lado la mano del malvado.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¡Mira cómo han caído los obradores de maldad! *"),
						b: String::from("Fueron derribados, y no podrán levantarse.\nDía Séptimo: oración Vespertina")
					}
				]
			}
		]
	};
}