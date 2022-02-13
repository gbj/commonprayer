use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_12: Psalm = Psalm {
		number: 12,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 496
				},
				local_name: String::from(""),
				latin_name: String::from("Salvum me fac"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Sálvame, oh Señor, porque se acabaron los piadosos; *"),
						b: String::from("porque han desaparecido los fieles de entre el pueblo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Habla mentira cada uno con su prójimo; *"),
						b: String::from("con labios lisonjeros hablan con doblez de corazón.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("¡Ojalá que destruyese el Señor los labios lisonjeros, *"),
						b: String::from("y la lengua que habla con soberbia!")
					},
					PsalmVerse {
						number: 4,
						a: String::from("A los que dicen: \"Por nuestra lengua prevaleceremos; *"),
						b: String::from("nuestro labios son nuestros;\n¿quién se enseñorea de nosotros?\"")
					},
					PsalmVerse {
						number: 5,
						a: String::from("\"Por la opresión de los pobres,\npor el gemido de los menesterosos, *"),
						b: String::from("ahora me levantaré\", dice el Señor, \"y pondré a salvo al que lo anhela\".")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Las palabras del Señor son limpias, *"),
						b: String::from("como plata refinada en horno de tierra,\ny purificada siete veces.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Tú, oh Señor, nos guardarás; *"),
						b: String::from("de esta generación nos preservarás para siempre.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Andan los malos de un lado al otro, *"),
						b: String::from("y estimada es la vileza por todos.")
					}
				]
			}
		]
	};
}