use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_61: Psalm = Psalm {
		number: 61,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 564
				},
				local_name: String::from(""),
				latin_name: String::from("Exaudi Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Escucha, oh Dios, mi clamor; *"),
						b: String::from("atiende a mi oración.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Desde el confín de la tierra te invoco, con el corazón abatido; *"),
						b: String::from("ponme en una roca más alta que yo;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque tú has sido mi refugio, *"),
						b: String::from("torre fuerte delante del enemigo.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Yo habitaré siempre en tu morada; *"),
						b: String::from("me refugiaré bajo la sombra de tus alas;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque tú, oh Dios, has oído mis promesas; *"),
						b: String::from("me has dado la heredad de los que veneran tu Nombre.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Añade días a los días del rey; *"),
						b: String::from("que sus años alcancen muchas generaciones.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Que permanezca en su trono delante de Dios para siempre; *"),
						b: String::from("haz que tu misericordia y fidelidad le guarden;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Así cantaré el loor de tu Nombre para siempre, *"),
						b: String::from("pagando mis votos día tras día.")
					}
				]
			}
		]
	};
}