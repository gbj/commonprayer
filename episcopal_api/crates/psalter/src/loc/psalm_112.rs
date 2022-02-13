use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_112: Psalm = Psalm {
		number: 112,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 647
				},
				local_name: String::from(""),
				latin_name: String::from("Beatus vir"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\n¡Dichosos los que temen a mi Soberano, *"),
						b: String::from("y de corazón se deleitan en sus mandamientos !")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Su descendencia será poderosa en la tierra; la generación de los rectos será bendita."),
						b: String::from("")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Bienes y riquezas habrá en su casa, *"),
						b: String::from("y su benevolencia permanecerá para siempre.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("La luz resplandece en las tinieblas para los rectos; *"),
						b: String::from("los justos son clementes y compasivos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Buenos los que son generosos y prestan, *"),
						b: String::from("y administran sus asuntos con juicio.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Por eso jamás tropezarán; *"),
						b: String::from("en memoria eterna se tendrá a los justos.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("No temerán las malas noticias; *"),
						b: String::from("su corazón está firme, confiado en mi Soberano.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Firme está su corazón, y no temerá, *"),
						b: String::from("hasta ver cumplido en sus enemigos su deseo.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Han repartido liberalmente al pobre,\ny su generosidad permanece para siempre; *"),
						b: String::from("alzarán la frente con dignidad.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Los malvados, al verlo, se enfurecerán; crujirán los dientes, y se consumirán; *"),
						b: String::from("el deseo de los malvados fracasará.")
					}
				]
			}
		]
	};
}