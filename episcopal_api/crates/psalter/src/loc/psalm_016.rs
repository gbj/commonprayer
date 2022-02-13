use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_16: Psalm = Psalm {
		number: 16,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 499
				},
				local_name: String::from(""),
				latin_name: String::from("Conserva me Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Guárdame, oh Dios, porque a ti me acojo; *"),
						b: String::from("dije al Señor: \"Tú eres mi Soberano;\nno hay para mí bien fuera de ti\".")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Para los santos que están en la tierra, *"),
						b: String::from("y para los íntegros, es toda mi complacencia.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Se multiplicarán los dolores, *"),
						b: String::from("de aquéllos que sirven diligentes a otros dioses.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("No ofreceré yo sus libaciones de sangre, *"),
						b: String::from("ni en mis labios tomaré los nombres de sus dioses")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Tú, oh Señor, eres la porción de mi herencia y de mi copa; *"),
						b: String::from("tú sustentarás mi suerte.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Me toca una parcela hermosa; *"),
						b: String::from("en verdad, una heredad magnífica.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Bendeciré al Señor que me aconseja; *"),
						b: String::from("aun en las noches me enseña mi corazón.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Al Señor he puesto siempre delante de mí; *"),
						b: String::from("porque está a mi diestra no seré conmovido.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Por tanto se alegra mi corazón, y se goza mi espíritu; *"),
						b: String::from("también mi carne reposará segura;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Porque no me dejarás al sepulcro; *"),
						b: String::from("ni permitirás que tu santo vea la fosa.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Me mostrarás la senda de la vida; *"),
						b: String::from("en tu presencia hay plenitud de gozo,\ndeleites a tu diestra para siempre.")
					}
				]
			}
		]
	};
}