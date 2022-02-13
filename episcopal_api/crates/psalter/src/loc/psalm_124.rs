use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_124: Psalm = Psalm {
		number: 124,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 672
				},
				local_name: String::from(""),
				latin_name: String::from("Nisi quia Dominus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Si el Señor no hubiera estado de nuestra parte, *"),
						b: String::from("diga ahora Israel;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Si el Señor no hubiera estado de nuestra parte, *"),
						b: String::from("cuando los enemigos se levantaron contra nosotros;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Entonces nos habrían tragado vivos, *"),
						b: String::from("cuando se encendió su furor contra nosotros;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Entonces nos habrían sumergido las aguas, *"),
						b: String::from("hasta el cuello habría subido el torrente;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Entonces hasta el cuello habrían subido *"),
						b: String::from("las aguas furiosas.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¡Bendito sea el Señor! *"),
						b: String::from("No nos ha dado por presa a sus dientes.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Hemos escapado cual ave de la trampa del cazador; *"),
						b: String::from("se rompió la trampa, y hemos escapado.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Nuestro auxilio está en el Nombre del Señor, *"),
						b: String::from("que hizo los cielos y la tierra.")
					}
				]
			}
		]
	};
}