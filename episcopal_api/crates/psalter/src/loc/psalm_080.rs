use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_80: Psalm = Psalm {
		number: 80,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 597
				},
				local_name: String::from(""),
				latin_name: String::from("Qui regis Israel"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Pastor de Israel, escucha,\ntú que pastoreas a José como a un rebaño; *"),
						b: String::from("tú que te sientas sobre querubines, resplandece.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Ante Efraín, Benjamín y Manasés, *"),
						b: String::from("despierta tu poder, y ven a salvarnos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Oh Dios de los Ejércitos, restáuranos; *"),
						b: String::from("haz resplandecer tu rostro, y seremos salvos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Señor Dios de los Ejércitos, *"),
						b: String::from("¿hasta cuándo estarás airado,\na pesar de las súplicas de tu pueblo?")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Les diste de comer pan de lágrimas, *"),
						b: String::from("y a beber lágrimas en gran abundancia.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Nos pusiste por escarnio de nuestros vecinos, *"),
						b: String::from("y nuestros enemigos se burlan de nosotros.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Oh Dios de los Ejércitos, restáuranos; *"),
						b: String::from("haz resplandecer tu rostro, y seremos salvos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Sacaste una vid de Egipto; *"),
						b: String::from("expulsaste a las naciones, y la plantaste.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Preparaste sitio para ella; *"),
						b: String::from("se arraigó y llenó la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los montes fueron cubiertos por su sombra, *"),
						b: String::from("y los cedros altísimos por sus ramas.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Hiciste extender sus vástagos hasta el mar, *"),
						b: String::from("y hasta el río, sus renuevos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¿Por qué destruiste sus vallados, *"),
						b: String::from("y la saquean los viandantes?")
					},
					PsalmVerse {
						number: 13,
						a: String::from("La pisoteaban los jabalíes del bosque, *"),
						b: String::from("y la comían las bestias silvestres.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Vuélvete ahora, oh Dios de los Ejércitos,\nmira desde el cielo; considera, y visita esta viña; *"),
						b: String::from("preserva lo que plantó tu diestra.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("La han talado, y le han prendido fuego; *"),
						b: String::from("perezcan por la reprensión de tu rostro.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Sea tu mano sobre el varón de tu diestra, *"),
						b: String::from("el hijo del hombre que para ti fortaleciste.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Por ello, nunca nos apartaremos de ti; *"),
						b: String::from("danos vida, para que invoquemos tu Nombre.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Señor Dios de los Ejércitos, restáuranos; *"),
						b: String::from("haz resplandecer tu rostro, y seremos salvos.")
					}
				]
			}
		]
	};
}