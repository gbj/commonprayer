use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_146: Psalm = Psalm {
		number: 146,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 693
				},
				local_name: String::from(""),
				latin_name: String::from("Lauda anima mea"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Aleluya!\nAlaba, alma mía, al Señor; *"),
						b: String::from("alabaré al Señor mientras viva;\ncantaré alabanzas a mi Dios mientras exista.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("No confíes en los príncipes, ni en ningún hijo de Adán, *"),
						b: String::from("porque no hay en ellos seguridad.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Al exhalar el espíritu, vuelven al polvo, *"),
						b: String::from("y en ese día perecen todos sus planes.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¡Dichosos aquéllos cuya ayuda es el Dios de Jacob, *"),
						b: String::from("cuya esperanza está en el Señor su Dios!")
					},
					PsalmVerse {
						number: 5,
						a: String::from("El cual hizo los cielos y la tierra, el mar, y cuanto en ellos hay, *"),
						b: String::from("que guarda su promesa para siempre;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Que hace justicia a los oprimidos, *"),
						b: String::from("y da pan a los hambrientos.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("El Señor liberta a los cautivos;\nel Señor abre los ojos a los ciegos; *"),
						b: String::from("el Señor levanta a los caídos;")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El Señor ama a los justos;\nel Señor protege a los forasteros; *"),
						b: String::from("sostiene al huérfano y a la viuda,\npero trastorna el camino de los malvados.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Reinará el Señor para siempre, *"),
						b: String::from("tu Dios, oh Sión, de generación en generación.\n¡Aleluya!")
					}
				]
			}
		]
	};
}