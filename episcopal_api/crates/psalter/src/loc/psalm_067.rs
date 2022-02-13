use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_67: Psalm = Psalm {
		number: 67,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 571
				},
				local_name: String::from(""),
				latin_name: String::from("Deus misereatur"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dios tenga misericordia de nosotros, y nos bendiga, *"),
						b: String::from("haga resplandecer su rostro y venga a nosotros.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sean conocidos en la tierra tus caminos, *"),
						b: String::from("en todas las naciones tu salvación.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Te alaben los pueblos, oh Dios; *"),
						b: String::from("todos los pueblos te alaben.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Alégrense las naciones y aclamen con júbilo, *"),
						b: String::from("porque juzgas los pueblos con equidad,\ny diriges todas las naciones de la tierra.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Te alaben los pueblos, oh Dios; *"),
						b: String::from("todos los pueblos te alaben.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("La tierra ha dado su fruto; *"),
						b: String::from("nos bendiga Dios, el Dios nuestro.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Dios nos bendiga; *"),
						b: String::from("témanlo todos los confines de la tierra.")
					}
				]
			}
		]
	};
}