use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_43: Psalm = Psalm {
		number: 43,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 542
				},
				local_name: String::from(""),
				latin_name: String::from("Judica me, Deus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Hazme justicia, oh Dios, y aboga mi causa contra la gente impía; *"),
						b: String::from("líbrame de los mentirosos y los inicuos.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Tú eres el Dios de mi fortaleza; ¿por qué me has desechado? *"),
						b: String::from("¿Por qué he de andar enlutado por la opresión de mis enemigos?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Envía tu luz y tu verdad; que éstas me guíen, *"),
						b: String::from("y me conduzcan a tu santo monte, a tus moradas;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Para que me acerque al altar de Dios, al Dios de mi alegría y de mi gozo; *"),
						b: String::from("y te alabe con arpa, oh Dios, Dios mío.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¿Por qué te abates, oh alma mía, *"),
						b: String::from("y te turbas dentro de mí?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Pon tu confianza en Dios, *"),
						b: String::from("porque aún he de alabarle,\nSalvador, Presencia y Dios mío.")
					}
				]
			}
		]
	};
}