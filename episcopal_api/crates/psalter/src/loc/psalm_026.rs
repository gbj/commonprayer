use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_26: Psalm = Psalm {
		number: 26,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 515
				},
				local_name: String::from(""),
				latin_name: String::from("Judica me, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Júzgame, oh Señor,\nporque en integridad he andado; *"),
						b: String::from("he confiado asimismo en el Señor sin titubear.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Escudríñame, oh Señor, y pruébame; *"),
						b: String::from("examina mis pensamientos y mi corazón;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque tu amor está delante de mis ojos; *"),
						b: String::from("he andado fielmente contigo.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("No he frecuentado personas inútiles, *"),
						b: String::from("ni me he asociado con los engañadores.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Aborrecí la reunión de los malhechores, *"),
						b: String::from("y con los impíos nunca me sentaré.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Lavaré en inocencia mis manos, *"),
						b: String::from("y así andaré alrededor de tu altar, oh Señor,")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Cantando himnos de alabanza, *"),
						b: String::from("y contando todas tus obras maravillosas.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Señor, la habitación de tu casa yo amo, *"),
						b: String::from("y el lugar de la morada de tu gloria.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("No arrebates mi alma con los pecadores, *"),
						b: String::from("ni mi vida con los sanguinarios,")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Cuyas manos están llenas de tramas, *"),
						b: String::from("y cuya diestra está llena de sobornos.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Mas yo andaré en integridad; *"),
						b: String::from("redímeme, oh Señor, y ten misericordia de mí.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Mi pie se mantiene firme en medio de los justos; *"),
						b: String::from("en las asambleas bendeciré al Señor.")
					}
				]
			}
		]
	};
}