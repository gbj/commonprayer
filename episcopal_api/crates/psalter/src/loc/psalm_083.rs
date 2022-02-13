use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_83: Psalm = Psalm {
		number: 83,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 600
				},
				local_name: String::from(""),
				latin_name: String::from("Deus, quis similis?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Dios, no estés callado; *"),
						b: String::from("no guardes silencio, ni te quedes inmóvil.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("He aquí, tus enemigos se alborotan, *"),
						b: String::from("y los que te odian levantan la cabeza.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Contra tu pueblo urden intrigas, *"),
						b: String::from("y se conjuran contra tus protegidos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Han dicho: \"Vengan, extirpémosles de entre las naciones; *"),
						b: String::from("que no haya más memoria del nombre de Israel\".")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Se confabulan a una; *"),
						b: String::from("contra ti han hecho alianza:")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Las tiendas de los edomitas y de los ismaelitas; *"),
						b: String::from("los moabitas y los agarenos;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Gebal, Amón y Amalec, *"),
						b: String::from("los filisteos y los habitantes de Tiro.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("También los asirios se han juntado con ellos; *"),
						b: String::from("y prestaron refuerzos al pueblo de Lot.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Trátalos como a Madián, como a Sísara, *"),
						b: String::from("como a Jabín en el arroyo de Cisón:")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Fueron aniquilados en Endor; *"),
						b: String::from("sirvieron de estiércol para el campo.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Trata a sus príncipes como a Oreb y a Zeeb, *"),
						b: String::from("a sus reyes como a Zeba y a Zalmuna,")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Que dijeron: \"Entremos en posesión *"),
						b: String::from("de los campos de Dios\".")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Oh Dios mío, hazlos como torbellino, *"),
						b: String::from("y como tamo ante el viento;")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Como fuego que quema el bosque, *"),
						b: String::from("como llama que arrasa el monte.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Persíguelos con tu tempestad, *"),
						b: String::from("y atérralos con tu tormenta.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Cúbreles el rostro de ignominia, *"),
						b: String::from("para que busquen tu Nombre, oh Señor.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Sean afrentados y turbados para siempre; *"),
						b: String::from("que se avergüencen y mueran.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Reconozcan que tu Nombre es YAHVÉ, *"),
						b: String::from("que sólo tú eres Altísimo sobre toda la tierra.")
					}
				]
			}
		]
	};
}