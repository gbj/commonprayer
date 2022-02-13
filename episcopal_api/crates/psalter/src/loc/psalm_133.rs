use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_133: Psalm = Psalm {
		number: 133,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 678
				},
				local_name: String::from(""),
				latin_name: String::from("Ecce, quam bonum!"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Oh cuán bueno y agradable es *"),
						b: String::from("convivir los hermanos en unidad!")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Es como el buen óleo sobre la cabeza, *"),
						b: String::from("el cual desciende sobre la barba,")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Sobre la barba de Aarón, *"),
						b: String::from("y baja hasta el collar de sus vestiduras.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Es como el rocío del Hermón, *"),
						b: String::from("que desciende sobre los montes de Sión;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Porque allí manda el Señor la bendición: *"),
						b: String::from("la vida por siempre jamás.")
					}
				]
			}
		]
	};
}