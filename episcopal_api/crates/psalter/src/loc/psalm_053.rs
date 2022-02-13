use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_53: Psalm = Psalm {
		number: 53,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 555
				},
				local_name: String::from(""),
				latin_name: String::from("Dixit insipiens"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dice el necio en su corazón: \"No hay Dios\". *"),
						b: String::from("Se han corrompido todos, hicieron abominable maldad;\nno hay quien haga bien.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Dios desde los cielos observa al género humano, *"),
						b: String::from("para ver si hay algún entendido que busque a Dios.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Todos se han extraviado; todos se han pervertido; *"),
						b: String::from("no hay quien haga bien, no hay ni siquiera uno.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¿No tienen conocimiento, todos los que hacen iniquidad, *"),
						b: String::from("que devoran a mi pueblo como si comiesen pan,\ny a Dios no invocan?")
					},
					PsalmVerse {
						number: 5,
						a: String::from("He aquí, ahora tiemblan grandemente como nunca antes temblaron, *"),
						b: String::from("porque Dios ha esparcido los huesos de los malvados; se avergüenzan porque Dios los ha rechazado.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¡Oh, si la liberación de Israel saliese de Sión! *"),
						b: String::from("Cuando Dios cambie la suerte de su pueblo,\nse gozará Jacob, y se alegrará Israel.")
					}
				]
			}
		]
	};
}