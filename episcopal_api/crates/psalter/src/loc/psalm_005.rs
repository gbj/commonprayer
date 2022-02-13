use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_5: Psalm = Psalm {
		number: 5,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 488
				},
				local_name: String::from(""),
				latin_name: String::from("Verba mea auribus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Escucha, oh Señor, mis palabras; *"),
						b: String::from("considera mi gemir.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Está atento a la voz de mi clamor, Rey mío y Dios mío, *"),
						b: String::from("porque a ti suplico.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Oh Señor, de mañana oirás mi voz; *"),
						b: String::from("de mañana me presentaré delante de ti, y esperaré;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque tú no eres un Dios que se complace en la maldad; *"),
						b: String::from("el malo no habitará junto a ti.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("No estarán los jactanciosos delante de tus ojos; *"),
						b: String::from("aborreces a todos los que obran iniquidad.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Destruirás a los que hablan mentira; *"),
						b: String::from("al hombre sanguinario y engañador, tú abominas,\noh Señor.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Mas yo, por la abundancia de tu misericordia, entraré en tu casa; *"),
						b: String::from("adoraré hacia el santo templo en tu temor.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Guíame, oh Señor, en tu justicia, a causa de mis enemigos; *"),
						b: String::from("endereza delante de mí tu camino;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Porque en la boca de ellos no hay sinceridad; *"),
						b: String::from("sus entrañas son maldad;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Sepulcro abierto es su garganta; *"),
						b: String::from("con su lengua hablan lisonjas.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Castígalos, oh Dios; *"),
						b: String::from("caigan por sus mismos consejos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Por la multitud de sus transgresiones échalos fuera, *"),
						b: String::from("porque se rebelaron contra ti.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Pero alégrense todos los que en ti confían; *"),
						b: String::from("den voces de júbilo para siempre;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Porque tú los defiendes; *"),
						b: String::from("en ti se regocijen los que aman tu Nombre;")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Porque tú, oh Señor, bendecirás al justo; *"),
						b: String::from("como con un escudo lo rodearás de tu favor.")
					}
				]
			}
		]
	};
}