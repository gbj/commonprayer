use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_76: Psalm = Psalm {
		number: 76,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 587
				},
				local_name: String::from(""),
				latin_name: String::from("Notus in Judaea"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dios es conocido en Judá; *"),
						b: String::from("en Israel es grande su Nombre.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("En Salem está su tabernáculo, *"),
						b: String::from("y su morada en Sión.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Allí quebró las saetas centellantes, *"),
						b: String::from("el escudo, la espada y las armas de guerra.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¡Cuán glorioso eres tú, *"),
						b: String::from("más espléndido que los montes eternos!")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Los fuertes de corazón son despojados, duermen su sueño; *"),
						b: String::from("a los guerreros no les responden sus brazos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("A tu reprensión, oh Dios de Jacob, *"),
						b: String::from("el carro y los caballos fueron aturdidos.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¡Cuán temible eres tú! *"),
						b: String::from("¿Quién puede estar de pie ante ti,\ncuando se encienda tu ira?")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Desde los cielos proclamaste la sentencia; *"),
						b: String::from("la tierra temió, y quedó inmóvil;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Cuando Dios se levantó para juzgar, *"),
						b: String::from("y para salvar a todos los mansos de la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Ciertamente, Edom el colérico te alabará, *"),
						b: String::from("y el remanente de Hamat celebrará tus fiestas.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Hagan votos al Señor su Dios, y páguenlos; *"),
						b: String::from("que cuantos estén alrededor de él,\ntraigan dones al Temible.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("El quebranta el espíritu de los príncipes, *"),
						b: String::from("e inspira temor a los reyes de la tierra.")
					}
				]
			}
		]
	};
}