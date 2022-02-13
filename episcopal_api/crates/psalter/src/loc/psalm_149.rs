use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_149: Psalm = Psalm {
		number: 149,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 697
				},
				local_name: String::from(""),
				latin_name: String::from("Cantate Domino"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nCanten al Señor cántico nuevo, *"),
						b: String::from("su alabanza en la congregación de los fieles")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Alégrese Israel en su Hacedor; *"),
						b: String::from("gócense los hijos de Sión en su Rey.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Alaben su Nombre con danzas, *"),
						b: String::from("con tambor y arpa cántenle alabanza;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Porque el Señor se complace en su pueblo, *"),
						b: String::from("y adorna con victoria a los humildes.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Regocíjense los fieles en su triunfo, *"),
						b: String::from("y alégrense sobre sus camas.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Estén las alabanzas de Dios en sus labios, *"),
						b: String::from("y la espada de dos filos en su mano;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Para tomar venganza de las naciones *"),
						b: String::from("y castigar a los pueblos;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("O Para atar a sus reyes con grillos, *"),
						b: String::from("y sus nobles con eslabones de hierro;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Para ejecutar en ellos la sentencia decretada; *"),
						b: String::from("esto es gloria para todos tus fieles.\n¡Aleluya!")
					}
				]
			}
		]
	};
}