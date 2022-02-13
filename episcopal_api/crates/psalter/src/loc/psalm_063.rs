use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_63: Psalm = Psalm {
		number: 63,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 566
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, Deus meus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios, tú eres mi Dios; ardientemente te busco; *"),
						b: String::from("mi alma tiene sed de ti, mi carne te anhela,\ncomo tierra seca y árida donde no hay agua.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¡Oh, que pudiera yo contemplarte en tu santuario! *"),
						b: String::from("¡Que pudiera ver tu poder y tu gloria!")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque mejor es tu gracia que la vida; *"),
						b: String::from("te alabarán mis labios.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Te bendeciré mientras viva; *"),
						b: String::from("en tu Nombre alzaré mis manos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mi alma será saciada como de meollo y grosura, *"),
						b: String::from("y con labios de júbilo te alabará mi boca,")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Cuando me acuerde de ti en mi lecho, *"),
						b: String::from("cuando medite en ti en las vigilias de la noche;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque tú has sido mi socorro; *"),
						b: String::from("y a la sombra de tus alas me regocijaré.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Mi alma está apegada a ti; *"),
						b: String::from("tu diestra me sostiene.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Que cuantos buscan mi vida para destruirla *"),
						b: String::from("bajen a lo profundo de la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Caigan a filo de espada; *"),
						b: String::from("sean pasto para los chacales.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Pero el rey se alegrará en Dios;\ntodos los que juran por él se regocijarán, *"),
						b: String::from("porque la boca de los que hablan mentira será cerrada.")
					}
				]
			}
		]
	};
}