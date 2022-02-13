use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_60: Psalm = Psalm {
		number: 60,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 563
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, repulisti nos"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios, tú nos has desechado, nos quebrantaste; *"),
						b: String::from("te has airado; acéptanos de nuevo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Hiciste temblar la tierra, abrístela; *"),
						b: String::from("cierra sus grietas, que se desmorona.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Has hecho pasar a tu pueblo duras pruebas; *"),
						b: String::from("nos hiciste beber del vino que nos aturde.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Has alzado estandarte a los que te temen, *"),
						b: String::from("como refugio contra los arqueros.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Sálvanos con tu diestra, y respóndenos, *"),
						b: String::from("para que sean librados los que amas.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Dios habló desde su santuario, y dijo: *"),
						b: String::from("“Yo me alegraré, y repartiré a Siquén,\ndividiré el valle de Sucot.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Mío es Galaad, mío Manasés; *"),
						b: String::from("Efraín es mi yelmo, y Judá mi cetro.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Moab es mi jofaina;\nsobre Edom lanzaré mi sandalia; *"),
						b: String::from("sobre Filistea cantaré victoria\".")
					},
					PsalmVerse {
						number: 9,
						a: String::from("¿Quién me llevará a la ciudad fortificada? *"),
						b: String::from("¿Quién me llevará hasta Edom,")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Si tú, oh Dios, nos has desechado, *"),
						b: String::from("si no sales, oh Dios, con nuestros ejércitos?")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Danos tu ayuda contra el enemigo, *"),
						b: String::from("porque vana es la ayuda del hombre.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Con Dios haremos proezas, *"),
						b: String::from("y él hollará a nuestros enemigos.")
					}
				]
			}
		]
	};
}