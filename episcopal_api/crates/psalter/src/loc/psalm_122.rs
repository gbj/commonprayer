use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_122: Psalm = Psalm {
		number: 122,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 670
				},
				local_name: String::from(""),
				latin_name: String::from("Laetatus sum"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Me alegré cuando me dijeron: *"),
						b: String::from("\"Vamos a la casa del Señor\".")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Ya están pisando nuestros pies *"),
						b: String::from("tus umbrales, oh Jerusalén.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Jerusalén está edificada *"),
						b: String::from("como ciudad bien unida entre sí.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Allá suben las tribus, las tribus del Señor, la asamblea de Israel, *"),
						b: String::from("para alabar el Nombre del Señor;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque allá están los tronos del juicio, *"),
						b: String::from("los tronos de la casa de David.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Oren por la paz de Jerusalén: *"),
						b: String::from("\"Que prosperen los que te aman.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Haya paz dentro de tus muros, *"),
						b: String::from("sosiego dentro de tus ciudadelas.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Por amor de mis hermanos y mis compañeros, *"),
						b: String::from("digo de corazón: 'La paz contigo'.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Por amor de la casa del Señor nuestro Dios, *"),
						b: String::from("buscaré hacerte el bien\".")
					}
				]
			}
		]
	};
}