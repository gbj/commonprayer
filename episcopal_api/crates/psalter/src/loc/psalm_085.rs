use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_85: Psalm = Psalm {
		number: 85,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 603
				},
				local_name: String::from(""),
				latin_name: String::from("Benedixisti, Domine"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Fuiste propicio a tu tierra, oh Señor; *"),
						b: String::from("restauraste la suerte de Jacob.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Perdonaste la iniquidad de tu pueblo; *"),
						b: String::from("todos sus pecados cubriste.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Reprimiste todo tu enojo; *"),
						b: String::from("te apartaste del ardor de tu ira.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Restáuranos, oh Dios nuestro Salvador, *"),
						b: String::from("y haz cesar tu cólera contra nosotros.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¿Estarás siempre enojado contra nosotros? *"),
						b: String::from("¿Prolongarás tu ira de edad en edad?")
					},
					PsalmVerse {
						number: 6,
						a: String::from("¿No volverás a darnos vida, *"),
						b: String::from("para que tu pueblo se regocije en ti?")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Señor, muéstranos tu misericordia, *"),
						b: String::from("y concédenos tu salvación.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Escucharé lo que dice el Señor Dios; *"),
						b: String::from("porque anuncia paz a su pueblo fiel,\na los que se convierten de corazón.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Ciertamente cercana está su salvación a cuantos le temen,*"),
						b: String::from("para que habite su gloria en nuestra tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("La misericordia y la verdad se encontraron; *"),
						b: String::from("la justicia y la paz se besaron.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("La verdad brotará de la tierra, *"),
						b: String::from("y la justicia mirará desde los cielos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("En verdad el Señor dará la lluvia, *"),
						b: String::from("y nuestra tierra dará su fruto.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("La justicia irá delante de él, *"),
						b: String::from("y la paz será senda para sus pasos.")
					}
				]
			}
		]
	};
}