use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_75: Psalm = Psalm {
		number: 75,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 586
				},
				local_name: String::from(""),
				latin_name: String::from("Confitebimur tibi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Gracias te damos, oh Dios, gracias te damos, *"),
						b: String::from("invocando tu Nombre, y contando todas tus maravillas.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Dice Dios: \"Señalaré un tiempo; *"),
						b: String::from("juzgaré rectamente.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aunque tiemble la tierra y todos sus habitantes, *"),
						b: String::from("yo afianzaré sus columnas.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Diré a los jactanciosos: 'No se jacten más', *"),
						b: String::from("y a los malvados: 'No alcen el testuz;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("No alcen el testuz contra los cielos, *"),
						b: String::from("ni levanten la cerviz' \";")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Porque ni de oriente ni de occidente viene el juicio, *"),
						b: String::from("ni del desierto ni de los montes.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Es Dios el que juzga; *"),
						b: String::from("a éste humilla, y a aquél enaltece;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Porque en la mano del Señor hay un cáliz, lleno de vino espumante que él derrama; *"),
						b: String::from("y todos los malvados de la tierra\nlo beberán hasta las heces.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Pero me regocijaré para siempre; *"),
						b: String::from("cantaré alabanzas al Dios de Jacob.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Quebrantará todo el poder de los malvados, *"),
						b: String::from("pero el poder del justo será exaltado.")
					}
				]
			}
		]
	};
}