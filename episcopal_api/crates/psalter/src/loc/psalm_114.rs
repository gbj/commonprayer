use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_114: Psalm = Psalm {
		number: 114,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 648
				},
				local_name: String::from(""),
				latin_name: String::from("In exitu Israel"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nCuando salió Israel de Egipto, *"),
						b: String::from("la casa de Jacob de entre un pueblo de idio ma ajeno,")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Judá vino a ser el santuario de Dios, *"),
						b: String::from("e Israel su dominio.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Elmarlovio,yhuyó:*"),
						b: String::from("el Jordán se volvió atrás.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Los montes saltaron como carneros, *"),
						b: String::from("y como corderos las colinas.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¿Qué te afligió, oh mar, que huiste, *"),
						b: String::from("y a ti, oh Jordán, que te volviste atrás?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Oh montes, ¿por qué saltaron como carneros, *"),
						b: String::from("y como corderos, oh colinas?")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Tiembla, oh tierra, a la presencia de mi Soberano, *"),
						b: String::from("a la presencia del Dios de Jacob,")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Quien cambió la peña en estanque de aguas, *"),
						b: String::from("y el pedernal en manantiales.")
					}
				]
			}
		]
	};
}