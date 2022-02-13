use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_129: Psalm = Psalm {
		number: 129,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 675
				},
				local_name: String::from(""),
				latin_name: String::from("Saepe expugnaverunt"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("\"Mucho me han oprimido desde mi juventud\", *"),
						b: String::from("diga ahora Israel;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("\"Mucho me han oprimido desde mi juventud, *"),
						b: String::from("mas no prevalecieron contra mí\".")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Sobre mis espaldas araron los aradores, *"),
						b: String::from("y alargaron sus surcos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("El Señor, el Justo, *"),
						b: String::from("ha cortado las coyundas de los malvados.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Sean avergonzados y vueltos atrás *"),
						b: String::from("cuantos aborrecen a Sión.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Sean como la hierba de los tejados, *"),
						b: String::from("que se marchita antes que se le segue;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Que no llena la mano del segador, *"),
						b: String::from("ni el pecho del que ata las gavillas;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("De modo que los que pasan ni siquiera dicen: \"Bendígate el Señor. *"),
						b: String::from("Te deseamos buena suerte en el Nombre del Señor\".")
					}
				]
			}
		]
	};
}