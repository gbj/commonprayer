use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_92: Psalm = Psalm {
		number: 92,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 614
				},
				local_name: String::from(""),
				latin_name: String::from("Bonum est confiteri"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bueno es darte gracias, oh Señor, *"),
						b: String::from("y cantar alabanzas a tu Nombre, oh Altísimo;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Anunciar por la mañana tu misericordia, *"),
						b: String::from("y tu fidelidad por la noche;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Enlacítarayenlalira,*"),
						b: String::from("y con la melodía del arpa;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Por cuanto me has alegrado, oh Señor, con tus hazañas; *"),
						b: String::from("las obras de tus manos aclamo con júbilo.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¡Cuán grandes son tus obras, oh Señor! *"),
						b: String::from("¡Qué profundos tus designios!")
					},
					PsalmVerse {
						number: 6,
						a: String::from("El tonto no sabe, y el necio no entiende, *"),
						b: String::from("que si bien los malvados crecen como la hierba,\ny florecen todos los que hacen iniquidad;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Florecen sólo para ser destruidos eternamente; *"),
						b: String::from("mas tú, oh Señor, eres excelso por siempre jamás;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Porque he aquí, tus enemigos, oh Señor, he aquí, perecerán tus enemigos, *"),
						b: String::from("y serán esparcidos todos los que hacen iniquidad.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Pero tú aumentaste mis fuerzas como las del búfalo; *"),
						b: String::from("me ungiste con aceite fresco.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Mis ojos se han regocijado ante la huida de mis enemigos, *"),
						b: String::from("y mis oídos ante la derrota de los malignos,\nde los que se levantaron contra mí.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Los justos florecerán como palmera; *"),
						b: String::from("se alzarán como cedros del Líbano;")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Los plantados en la casa del Señor *"),
						b: String::from("florecerán en los atrios de nuestro Dios.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("En la vejez seguirán dando fruto, *"),
						b: String::from("y estarán lozanos y frondosos,")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Para proclamar la rectitud del Señor, *"),
						b: String::from("mi Roca, en quien no existe falta.")
					}
				]
			}
		]
	};
}