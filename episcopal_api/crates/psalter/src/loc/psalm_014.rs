use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_14: Psalm = Psalm {
		number: 14,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 498
				},
				local_name: String::from(""),
				latin_name: String::from("Dixit insipiens"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dijo el necio: \"No hay Dios\". *"),
						b: String::from("Se han corrompido todos, hicieron obras abominables;\nno hay quien haga bien.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El Señor mira desde los cielos sobre el género humano, *"),
						b: String::from("para ver si hay algún entendido,\nque busque a Dios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Todos se desviaron, a una se han corrompido; *"),
						b: String::from("no hay quien haga lo bueno,\nno hay ni siquiera uno.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¿No tienen discernimiento, todos los que hacen iniquidad, *"),
						b: String::from("que devoran a mi pueblo como si comiesen pan,\ny al Señor no invocan?")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¡Mira! Ellos temblaron de espanto, *"),
						b: String::from("porque Dios está con la generación de los justos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Del consejo de los afligidos se han burlado, *"),
						b: String::from("pero el Señor es su refugio.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("¡Ojalá que de Sión saliera la salvación de Israel! *"),
						b: String::from("Cuando el Señor hiciere volver la suerte de su pueblo,\nse gozará Jacob, y se alegrará Israel.")
					}
				]
			}
		]
	};
}