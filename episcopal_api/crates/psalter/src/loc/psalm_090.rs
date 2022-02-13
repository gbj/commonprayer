use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_90: Psalm = Psalm {
		number: 90,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 611
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, refugium"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Soberano mío, tú has sido nuestro refugio *"),
						b: String::from("de generación en generación.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Antes que naciesen los montes,\no fueran engendrados la tierra y el mundo, *"),
						b: String::from("desde el siglo y hasta el siglo, tú eres Dios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Devuelves el hombre al polvo, diciendo: *"),
						b: String::from("\"Retorna, hijo de Adán\".")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque mil años delante de tus ojos\nson como el ayer, que pasó, *"),
						b: String::from("y como una vigilia en la noche.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Nos arrebatas como en un sueño, *"),
						b: String::from("como la hierba que pronto se marchita:")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Por la mañana florece y crece; *"),
						b: String::from("por la tarde es cortada y se seca;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque en tu furor somos consumidos, *"),
						b: String::from("y por tu indignación somos conturbados.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Pusiste nuestras iniquidades ante ti, *"),
						b: String::from("nuestros pecados secretos a la luz de tu rostro.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Todos nuestros días fallecen a causa de tu ira; *"),
						b: String::from("acabamos nuestros años como un suspiro.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los días de nuestra vida son setenta años,\ny quizás en los más robustos hasta ochenta; *"),
						b: String::from("con todo, la suma de ellos es sólo pesar y trabajo, porque pronto pasan, y desaparecemos.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("¿Quién conoce la vehemencia de tu ira? *"),
						b: String::from("¿Quién teme debidamente tu indignación?")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Enséñanos de tal modo a contar nuestros días, *"),
						b: String::from("que traigamos al corazón sabiduría.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Vuélvete, oh Señor, ¿hasta cuándo tardarás? *"),
						b: String::from("Ten compasión de tus siervos.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Por la mañana sácianos de tu misericordia, *"),
						b: String::from("y así cantaremos y nos alegraremos todos nuestros días.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Alégranos conforme a los días que nos afligiste, *"),
						b: String::from("y a los años en que sufrimos desdichas.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Que tus siervos vean tus obras, *"),
						b: String::from("y su descendencia tu gloria.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Sea la bondad del Señor nuestro Dios sobre nosotros, *"),
						b: String::from("y haga prosperar las obras de nuestras manos;\nsí, haga prosperar nuestras obras.")
					}
				]
			}
		]
	};
}