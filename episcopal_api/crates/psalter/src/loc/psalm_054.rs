use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_54: Psalm = Psalm {
		number: 54,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 556
				},
				local_name: String::from(""),
				latin_name: String::from("Deus in nomine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios, sálvame por tu Nombre, *"),
						b: String::from("y con tu poder defiéndeme.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Escucha mi oración, oh Dios, *"),
						b: String::from("atiende a las palabras de mi boca.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Insolentes se han levantado contra mí, *"),
						b: String::from("y matones buscan mi vida; \nno tienen presente a Dios.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("He aquí, Dios es el que me ayuda; *"),
						b: String::from("es el Señor quien sostiene mi vida.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Devuelve el mal a mis adversarios; *"),
						b: String::from("destrúyelos, por tu fidelidad.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Te ofreceré sacrificios voluntarios; *"),
						b: String::from("alabaré tu Nombre, oh Señor, porque es bueno;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque me has librado de toda angustia, *"),
						b: String::from("y mis ojos han visto la ruina de mis enemigos.")
					}
				]
			}
		]
	};
}