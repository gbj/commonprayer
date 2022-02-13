use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_101: Psalm = Psalm {
		number: 101,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 623
				},
				local_name: String::from(""),
				latin_name: String::from("Misericordiam et judicium"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Misericordia y justicia cantaré; *"),
						b: String::from("a ti cantaré alabanzas, oh Señor.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Me esforzaré por seguir un camino intachable; ¿cuándo vendrás a mi? *"),
						b: String::from("Andaré con sencillez de corazón dentro de mi casa.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("No pondré delante de mis ojos cosa indigna; *"),
						b: String::from("aborrezco a los malhechores;\nno se quedarán conmigo.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Corazón perverso alejaré de mí; *"),
						b: String::from("no conoceré el mal.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("A los que en secreto difaman a su prójimo, haré callar; *"),
						b: String::from("ojos engreídos, corazones arrogantes,\nno los puedo soportar.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Mis ojos pondré en los fieles de la tierra, para que vivan conmigo; *"),
						b: String::from("sólo los que siguen un camino intachable me servirán.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("No habitarán en mi casa los que hacen fraudes, *"),
						b: String::from("y cuantos hablan mentiras no durarán en mi presencia.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Pronto destruiré a todos los malvados de la tierra, *"),
						b: String::from("para extirpar de la ciudad del Señor\na todos los malhechores.")
					}
				]
			}
		]
	};
}