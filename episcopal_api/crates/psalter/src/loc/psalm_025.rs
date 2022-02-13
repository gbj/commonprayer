use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_25: Psalm = Psalm {
		number: 25,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 513
				},
				local_name: String::from(""),
				latin_name: String::from("Ad te, Domine, levavi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("A ti, oh Señor, levanto mi alma; Dios mío, en ti confío; *"),
						b: String::from("no sea yo humillado,\nno triunfen mis enemigos sobre mí.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Ciertamente ninguno de cuantos en ti esperan será avergonzado; *"),
						b: String::from("serán avergonzados los que se rebelan sin causa.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Muéstrame, oh Señor, tus caminos; *"),
						b: String::from("enséñame tus sendas.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Encamíname en tu verdad, y enséñame; *"),
						b: String::from("porque tú eres el Dios de mi salvación;\nen ti he esperado todo el día.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Acuérdate, oh Señor, de tus piedades y de tus misericordias, *"),
						b: String::from("porque son perpetuas.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("De los pecados de mi juventud, y de mis rebeliones, no te acuerdes; *"),
						b: String::from("conforme a tu misericordia acuérdate de mí, por tu bondad, oh Señor.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Bueno y recto es el Señor; *"),
						b: String::from("por tanto, enseña a los pecadores el camino.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Encamina a los humildes por el juicio, *"),
						b: String::from("y enseña a los mansos su carrera.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Todas las sendas del Señor son amor y fidelidad, *"),
						b: String::from("para los que guardan su pacto y sus testimonios.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Por amor de tu Nombre, oh Señor, *"),
						b: String::from("perdona mi pecado, porque es grande.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("¿Quién es el que teme al Señor? *"),
						b: String::from("El Señor le enseñará el camino que ha de escoger.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Su alma reposará en el bien, *"),
						b: String::from("y su descendencia heredará la tierra.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("La amistad del Señor es con los que le temen, *"),
						b: String::from("y a ellos hará conocer su pacto.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Mis ojos están siempre hacia el Señor; *"),
						b: String::from("porque él sacará mis pies de la red.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Vuélvete y ten misericordia de mí, *"),
						b: String::from("porque estoy solo y afligido.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Las angustias de mi corazón se han aumentado; *"),
						b: String::from("sácame de mis congojas.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Mira mi aflicción y miseria, *"),
						b: String::from("y perdona todos mis pecados.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Mira mis enemigos, que se han multiplicado, *"),
						b: String::from("y con odio violento me aborrecen.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Preserva mi vida y líbrame; *"),
						b: String::from("no sea yo avergonzado, porque en ti confié.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Integridad y rectitud me guarden, *"),
						b: String::from("porque en ti he esperado.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Redime, oh Dios, a Israel *"),
						b: String::from("de todas sus angustias.")
					}
				]
			}
		]
	};
}