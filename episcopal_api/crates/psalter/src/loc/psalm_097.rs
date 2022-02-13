use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_97: Psalm = Psalm {
		number: 97,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 620
				},
				local_name: String::from(""),
				latin_name: String::from("Dominus regnavit"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Señor es Rey; regocíjese la tierra; *"),
						b: String::from("alégrense la multitud de las islas.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Nubes y oscuridad alrededor de él; *"),
						b: String::from("rectitud y justicia el cimiento de tu trono.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Fuego va delante de él, *"),
						b: String::from("y abrasa a sus enemigos alrededor.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Sus relámpagos alumbran el mundo; *"),
						b: String::from("viéndolo, la tierra se estremece.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Los montes se derriten como cera a la vista del Señor, *"),
						b: String::from("a la vista del Soberano de toda la tierra.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Los cielos anuncian su justicia, *"),
						b: String::from("y todos los pueblos contemplan su gloria.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Avergüéncense todos los que adoran imágenes de talla, *"),
						b: String::from("los que se glorían en dioses falsos;\npóstrense ante él, dioses todos.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Sión oye, y se alegra,\ny las ciudades de Judá se gozan, *"),
						b: String::from("a causa de tus juicios, oh Señor;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Porque tú eres el Señor, altísimo sobre toda la tierra; *"),
						b: String::from("eres muy excelso sobre todos los dioses.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("El Señor ama a los que aborrecen el mal; *"),
						b: String::from("él preserva la vida de sus santos,\ny de mano de los malvados los libra.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Brota la luz para el justo, *"),
						b: String::from("y alegría para los rectos de corazón.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Alégrense, justos, en el Señor, *"),
						b: String::from("dando gracias a su santo Nombre.")
					}
				]
			}
		]
	};
}