use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_1: Psalm = Psalm {
		number: 1,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 485
				},
				local_name: String::from(""),
				latin_name: String::from("Beatus vir qui non abiit"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bienaventurado el que no anduvo en consejo de malos, *"),
						b: String::from("ni estuvo en camino de Pecadores,\nni en silla de escarnecedores se ha sentado;")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sino que en la ley del Señor está su delicia, *"),
						b: String::from("y en su ley medita de día y de noche.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Será como el árbol plantado junto a corrientes de aguas, que da su fruto en su tiempo, y su hoja no cae, *"),
						b: String::from("y todo lo que hace prosperará.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("No así los malos, no así, *"),
						b: String::from("que son como el tamo que arrebata el viento.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Por tanto, no se levantarán los malos en el juicio, *"),
						b: String::from("ni los pecadores en la congregación de los justos;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Porque el Señor conoce el camino de los justos, *"),
						b: String::from("mas la senda de los malos perecerá.")
					}
				]
			}
		]
	};
}