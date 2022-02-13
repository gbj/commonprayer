use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_82: Psalm = Psalm {
		number: 82,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 600
				},
				local_name: String::from(""),
				latin_name: String::from("Deus stetit"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dios preside en la asamblea divina; *"),
						b: String::from("en medio de los dioses juzga.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("\"¿Hasta cuándo juzgarán injustamente, *"),
						b: String::from("y mostrarán parcialidad a los malvados?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Defiendan al desvalido y al huérfano; *"),
						b: String::from("vindiquen al afligido y al menesteroso.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Rescaten a los débiles y a los pobres; *"),
						b: String::from("de mano de los malvados líbrenlos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Ellos no saben, no entienden, caminan a oscuras; *"),
						b: String::from("tiemblan todos los cimientos de la tierra.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("He dicho que ustedes son dioses, *"),
						b: String::from("y todos hijos del Altísimo.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Sin embargo, morirán como mortales, *"),
						b: String::from("y caerán como cualquier príncipe\".")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Levántate, oh Dios, y reina en la tierra; *"),
						b: String::from("entra en posesión de todas las naciones.")
					}
				]
			}
		]
	};
}