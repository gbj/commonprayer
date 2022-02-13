use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_123: Psalm = Psalm {
		number: 123,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 671
				},
				local_name: String::from(""),
				latin_name: String::from("Ad te levavi oculos meus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("A ti levanto mis ojos, *"),
						b: String::from("a ti entronizado en los cielos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Así como los ojos de los siervos miran a las manos de sus señores, *"),
						b: String::from("y los ojos de la sierva\na la mano de su señora,")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Así nuestros ojos miran al Señor nuestro Dios, *"),
						b: String::from("hasta que tenga misericordia de nosotros.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Ten misericordia de nosotros, oh Señor, ten misericordia, *"),
						b: String::from("porque estamos hartos de desprecio,")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Hartos del escarnio de los ricos indolentes, *"),
						b: String::from("del menosprecio de los orgullosos.")
					}
				]
			}
		]
	};
}