use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_4: Psalm = Psalm {
		number: 4,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 487
				},
				local_name: String::from(""),
				latin_name: String::from("Cum invocarem"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Respóndeme cuando clamo, oh Dios de mi justicia; *"),
						b: String::from("cuando estaba en angustia, tú me libraste;\nten misericordia de mí, y escucha mi oración.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("\"Mortales, ¿hasta cuándo volverán mi honra en infamia, *"),
						b: String::from("amarán la vanidad, y buscarán la mentira?\"")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Sepan, pues, que el Señor ha escogido a los fieles para sí; *"),
						b: String::from("el Señor oirá cuando yo a él clamare.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Tiemblen y no pequen; *"),
						b: String::from("mediten en su corazón estando en su cama, y callen.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Ofrezcan sacrificios rectos, *"),
						b: String::from("y confíen en el Señor.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Muchos son los que dicen: \"¿Quién nos mostrará el bien?\" *"),
						b: String::from("Alza sobre nosotros, oh Señor, la luz de tu rostro.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Tú diste alegría a mi corazón, *"),
						b: String::from("mayor que la de ellos cuando abundaba su grano\ny su mosto.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("En paz me acostaré, y en seguida dormiré; *"),
						b: String::from("porque sólo tú, oh Señor, me haces vivir seguro.")
					}
				]
			}
		]
	};
}