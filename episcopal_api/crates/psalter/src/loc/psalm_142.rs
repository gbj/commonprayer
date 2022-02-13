use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_142: Psalm = Psalm {
		number: 142,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 688
				},
				local_name: String::from(""),
				latin_name: String::from("Voce mea ad Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("En voz alta clamo al Señor; *"),
						b: String::from("en voz alta suplico al Señor.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Delante de él expongo mi queja, *"),
						b: String::from("y desahogo ante él mis afanes.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cuando decae mi espíritu dentro de mí, tú conoces mi senda; *"),
						b: String::from("en el camino en que ando, me escondieron lazo.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Miro a mi derecha,\ny no hallo a nadie que quiera conocerme; *"),
						b: String::from("no tengo a donde huir, y no hay quien me cuide.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("A ti clamo, oh Señor; *"),
						b: String::from("digo: \"Tú eres mi refugio,\nmi porción en la tierra de los vivientes\".")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Escucha mi clamor, porque estoy muy afligido; *"),
						b: String::from("líbrame de los que me persiguen,\nporque son mas fuertes que yo.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Sácame de la prisión, para que alabe tu Nombre; *"),
						b: String::from("cuando me hayas tratado bien, me rodearán los justos.")
					}
				]
			}
		]
	};
}