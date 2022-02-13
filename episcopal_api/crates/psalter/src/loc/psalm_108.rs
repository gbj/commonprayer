use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_108: Psalm = Psalm {
		number: 108,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 641
				},
				local_name: String::from(""),
				latin_name: String::from("Paratum cor meum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Mi corazón está firme, oh Dios, mi corazón está firme; *"),
						b: String::from("tocaré y cantaré salmos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Despierta, oh alma mía; despierten, lira y arpa; *"),
						b: String::from("yo mismo despertaré al alba.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Te confesaré entre los pueblos, oh Señor; *"),
						b: String::from("cantaré tus alabanzas entre las naciones;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque tu gracia es más grande que los cielos, *"),
						b: String::from("y tu fidelidad alcanza hasta las nubes.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Exáltate sobre los cielos, oh Dios, *"),
						b: String::from("y tu gloria sobre toda la tierra.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Para que sean librados tus amados, *"),
						b: String::from("salva con tu diestra y respóndeme.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Dios habló desde su santuario, y dijo: *"),
						b: String::from("\"Yo me alegraré, y repartiré a Siquén,\ndividiré el valle de Sucot.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Mío es Galaad, mío Manasés; *"),
						b: String::from("Efraín es mi yelmo, y Judá mi cetro.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Moab es mi jofaina;\nsobre Edom lanzaré mi sandalia; *"),
						b: String::from("sobre Filistea cantaré victoria\".")
					},
					PsalmVerse {
						number: 10,
						a: String::from("¿Quién me llevará a la ciudad fortificada? *"),
						b: String::from("¿Quién me llevará hasta Edom,")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Si tú, oh Dios, nos has desechado, *"),
						b: String::from("Si no sales, oh Dios, con nuestros ejercitos?")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Danos tu ayuda contra el enemigo, *"),
						b: String::from("porque vana es la ayuda humana.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Con Dios haremos proezas, *"),
						b: String::from("y él hollará a nuestros enemigos.")
					}
				]
			}
		]
	};
}