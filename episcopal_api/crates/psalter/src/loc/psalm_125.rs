use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_125: Psalm = Psalm {
		number: 125,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 672
				},
				local_name: String::from(""),
				latin_name: String::from("Qui confidunt"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Los que confían en el Señor son como el monte Sión, *"),
						b: String::from("que no será movido, sino que permanece para siempre.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Los montes rodean a Jerusalén; *"),
						b: String::from("así el Señor rodea a su pueblo,\ndesde ahora y para siempre.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("No pesará el cetro de los malvados sobre la heredad de los justos; *"),
						b: String::from("no sea que extiendan los justos sus manos a la maldad.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Haz bien, oh Señor, a los buenos, *"),
						b: String::from("y a los que son rectos de corazón;")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mas a los que se desvían por sendas tortuosas, el Señor los llevará con los malhechores, *"),
						b: String::from("pero la paz sea sobre Israel.")
					}
				]
			}
		]
	};
}