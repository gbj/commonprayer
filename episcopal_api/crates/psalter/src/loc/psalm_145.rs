use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_145: Psalm = Psalm {
		number: 145,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 691
				},
				local_name: String::from(""),
				latin_name: String::from("Exaltabo te, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Te exaltaré, oh Dios, mi Rey, *"),
						b: String::from("y bendeciré tu Nombre por siempre jamás.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Día tras día te bendeciré, *"),
						b: String::from("y alabaré tu Nombre por siempre jamás.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Grande es el Señor, y digno de toda alabanza; *"),
						b: String::from("ilimitable es su grandeza.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Generación a generación loará tus obras, *"),
						b: String::from("y proclamará tus hazañas.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Meditaré en la gloria y el esplendor de tu majestad, *"),
						b: String::from("y en todas tus acciones maravillosas.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Se anunciará el poder de tus hechos temibles, *"),
						b: String::from("y yo cantaré tus grandes proezas.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Se publicará la memoria de tu inmensa bondad; *"),
						b: String::from("se cantará tu justicia.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Clemente y compasivo es el Señor, *"),
						b: String::from("lento para la ira y grande en misericordia.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Amante es el Señor para con todos; *"),
						b: String::from("su compasión está sobre todas sus obras.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Te alaban, oh Señor, todas tus obras, *"),
						b: String::from("y tus fieles siervos te bendicen.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("La gloria de tu reino declaran, *"),
						b: String::from("y hablan de tu poder;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Para que sepan los pueblos de tus proezas, *"),
						b: String::from("y de la gloria y magnificencia de tu reino.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Tu reino es reino eterno, *"),
						b: String::from("y tu dominio perdura para siempre.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Fiel es el Señor en todas sus palabras, *"),
						b: String::from("misericordioso en todas sus hazañas.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Sostiene el Señor a los que caen, *"),
						b: String::from("y levanta a todos los oprimidos.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Los ojos de todos esperan en ti, oh Señor, *"),
						b: String::from("y tú les das su comida a su tiempo.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Abres bien tu mano, *"),
						b: String::from("y sacias de favores a todo viviente.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Justo es el Señor en todos sus caminos, *"),
						b: String::from("y bondadoso en todas sus acciones.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Cercano está el Señor a todos los que le invocan, *"),
						b: String::from("a los que le invocan confiadamente.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Satisface los deseos de los que le temen; *"),
						b: String::from("escucha su clamor, y los salva.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("El Señor guarda a todos los que le aman, *"),
						b: String::from("mas destruye a los malvados.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Mi boca pronunciará la alabanza del Señor; *"),
						b: String::from("que bendiga toda carne su santo Nombre,\neternamente y para siempre.")
					}
				]
			}
		]
	};
}