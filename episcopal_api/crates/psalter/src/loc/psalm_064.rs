use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_64: Psalm = Psalm {
		number: 64,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 567
				},
				local_name: String::from(""),
				latin_name: String::from("Exaudi, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Escucha, oh Dios, la voz de mi lamento; *"),
						b: String::from("guarda mi vida del temor del enemigo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Escóndeme de la conspiración de los malignos, *"),
						b: String::from("del populacho de los que obran iniquidad.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Afilan como espada su lengua, *"),
						b: String::from("y lanzan cual saetas palabras amargas,")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Para emboscar al íntegro y matarlo; *"),
						b: String::from("atacan por sorpresa y no temen.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Obstinados en su inicuo designio, *"),
						b: String::from("tratan de esconder sus lazos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Dicen: \"¿Quién nos ha de ver? ¿Quién descubrirá nuestro delito? *"),
						b: String::from("Urdimos el crimen perfecto\".")
					},
					PsalmVerse {
						number: 7,
						a: String::from("La mente y el corazón del hombre son un misterio, *"),
						b: String::from("mas Dios les tirará una saeta, y de repente serán heridos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Su propia lengua los hará caer; *"),
						b: String::from("y quienes los vean menearán la cabeza.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Todos se asombrarán, y anunciarán las obras de Dios; *"),
						b: String::from("reconocerán lo que él ha hecho.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El justo se alegrará en el Señor, y confiará en él; *"),
						b: String::from("se gloriarán todos los de recto corazón.")
					}
				]
			}
		]
	};
}