use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_28: Psalm = Psalm {
		number: 28,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 518
				},
				local_name: String::from(""),
				latin_name: String::from("Ad te, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("A ti, oh Señor, clamo;\nRoca mía, no me desatiendas; *"),
						b: String::from("para que no sea yo, dejándome tú, semejante a los que descienden a la fosa.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Oye la voz de mis ruegos cuando clamo a ti, *"),
						b: String::from("cuando alzo mis manos hacia tu lugar santísimo.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("No me arrebates con los malos, y con los que hacen iniquidad, *"),
						b: String::from("los cuales hablan paz con su prójimo, pero la maldad está en su corazón.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Dales conforme a su obra, *"),
						b: String::from("y conforme a la perversidad de sus hechos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Dales su merecido, *"),
						b: String::from("conforme a la obra de sus manos;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Porque no atendieron a las obras del Señor, ni a los hechos de sus manos, *"),
						b: String::from("él los derribará, y no los edificará.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¡Bendito sea el Señor! *"),
						b: String::from("porque ha oído la voz de mis ruegos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El Señor es mi fortaleza y mi escudo; *"),
						b: String::from("en él confía mi corazón, y fui ayudado.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Por ello salta mi corazón con júbilo, *"),
						b: String::from("y con mi canción le alabaré.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El Señor es la fortaleza de su pueblo, *"),
						b: String::from("el refugio de su ungido.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Salva a tu pueblo, y bendice a tu heredad; *"),
						b: String::from("pastoréales y susténtales para siempre.")
					}
				]
			}
		]
	};
}