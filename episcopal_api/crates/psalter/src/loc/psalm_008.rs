use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_8: Psalm = Psalm {
		number: 8,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 491
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, Dominus noster"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, soberano nuestro, *"),
						b: String::from("¡cuán glorioso es tu Nombre en toda la tierra!")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Alabadá es tu gloria sobre los cielos, *"),
						b: String::from("por la boca de los ninos y de los que maman.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Has fundado la fortaleza, a causa de tus enemigos, *"),
						b: String::from("para hacer callar al enemigo y al vengador.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Cuando contemplo tus cielos, obra de tus dedos, *"),
						b: String::from("la luna y las estrellas que tú formaste,")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Digo: \"¿Qué es el hombre, para que tengas de él memoria, el hijo del hombre, que lo ampares?\""),
						b: String::from("")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Le has hecho poco menor que los ángeles, *"),
						b: String::from("y lo coronaste de gloria y honra.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Lo hiciste señorear sobre las obras de tus manos; *"),
						b: String::from("todo lo pusiste debajo de sus pies:")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Ovejas y bueyes, todo ello, *"),
						b: String::from("y asimismo las bestias del campo;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Las aves de los cielos y los peces del mar, *"),
						b: String::from("todo cuanto pasa por los senderos del mar.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Oh Señor, soberano nuestro, *"),
						b: String::from("¡cuán glorioso es tu Nombre en toda la tierra!")
					}
				]
			}
		]
	};
}