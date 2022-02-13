use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_3: Psalm = Psalm {
		number: 3,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 487
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, quid multiplicati?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, ¡cuánto se han multiplicado mis adversarios! *"),
						b: String::from("Muchos son los que se levantan contra mí.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Muchos son los que de mí dicen: *"),
						b: String::from("\"No hay salvación para él en Dios\".")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Mas tu, oh Señor, eres escudo alrededor de mí; *"),
						b: String::from("mi gloria, y el que levanta mi cabeza.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Con mi voz clamé al Señor, *"),
						b: String::from("y él me respondió desde su santo monte.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Yo me acosté y dormí, *"),
						b: String::from("y desperté, porque el Señor me sustentaba.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("No temeré a diez millares de gente, *"),
						b: String::from("que pusieron sitio contra mí.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¡Levántate, oh Señor; sálvame, oh Dios mío! *"),
						b: String::from("Por cierto, herirás a todos mis enemigos en la quijada;\nlos dientes de los perversos quebrantarás.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("La salvación es del Señor; *"),
						b: String::from("sobre tu pueblo sea tu bendición.")
					}
				]
			}
		]
	};
}