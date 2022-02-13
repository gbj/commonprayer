use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_141: Psalm = Psalm {
		number: 141,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 687
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, clamavi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, a ti clamo; apresúrate; *"),
						b: String::from("escucha mi voz cuando te invoco.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Ascienda mi oración como incienso ante tu presencia, *"),
						b: String::from("el alzar de mis manos como el sacrificio vespertino.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Pon centinela delante de mi boca, oh Señor, y guardia a la puerta de mis labios; *"),
						b: String::from("no dejes que mi corazón se incline al mal.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("No me dedique a la maldad con los malvados, *"),
						b: String::from("y no coma yo de sus deleites.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Que el justo me castigue con censura benévola, mas el ungüento del impío no perfume mi cabeza; *"),
						b: String::from("mi oración es continuamente contra sus maldades.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Que sus jefes sean derribados en lugares peñascosos, *"),
						b: String::from("para que sepan que mis palabras son verdaderas.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Así como la tierra es surcada por el arador, *"),
						b: String::from("así sean esparcidos sus huesos a la boca de la tumba.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Empero mis ojos están vueltos a ti, Señor Dios; *"),
						b: String::from("en ti me refugio; no me despojes de la vida.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Guárdame de los lazos que me han tendido, *"),
						b: String::from("y de las trampas de los malhechores.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Caigan los malvados en sus propias redes, *"),
						b: String::from("mientras yo escapo.")
					}
				]
			}
		]
	};
}