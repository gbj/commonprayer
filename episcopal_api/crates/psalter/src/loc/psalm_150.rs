use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_150: Psalm = Psalm {
		number: 150,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 698
				},
				local_name: String::from(""),
				latin_name: String::from("Laudate Dominum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nAlaben a Dios en su santo templo; *"),
						b: String::from("alábenle en la bóveda de su poder.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Alábenle por sus proezas; *"),
						b: String::from("alábenle por su inmensa grandeza.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Alábenle con el bramido del corno; *"),
						b: String::from("alábenle con lira y arpa.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Alábenle con tambores y danzas; *"),
						b: String::from("alábenle con cuerdas y caramillo.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Alábenle con címbalos resonantes; *"),
						b: String::from("alábenle con címbalos clamorosos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Todo lo que respira, *"),
						b: String::from("alabe al Señor.\n¡Aleluya!")
					}
				]
			}
		]
	};
}