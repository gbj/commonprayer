use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_46: Psalm = Psalm {
		number: 46,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 546
				},
				local_name: String::from(""),
				latin_name: String::from("Deus noster refugium"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Dios es nuestro refugio y fortaleza, *"),
						b: String::from("nuestro pronto auxilio en las tribulaciones.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Por tanto, no temeremos, aunque la tierra sea removida, *"),
						b: String::from("y se desplomen los montes en el corazón de la mar;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aunque bramen y espumen sus aguas, *"),
						b: String::from("y tiemblen los montes a causa de su braveza.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("El Señor de las huestes está con nosotros; *"),
						b: String::from("nuestro refugio es el Dios de Jacob.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Hay un río cuyas corrientes alegran la ciudad de Dios, *"),
						b: String::from("el santuario de las moradas del Altísimo.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Dios está en medio de ella; no será conmovida; *"),
						b: String::from("Dios la ayudará al clarear la mañana.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Braman las naciones, titubean los reinos; *"),
						b: String::from("Dios habló; se derretirá la tierra.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("El Señor de las huestes está con nosotros; *"),
						b: String::from("nuestro refugio es el Dios de Jacob.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Vengan a ver las obras del Señor, *"),
						b: String::from("las maravillas que ha hecho en la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Hace que las guerras cesen en todo el orbe; *"),
						b: String::from("rompe el arco, destroza la lanza\ny quema los escudos en el fuego.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("\"Estén, pues, quietos, y sepan que yo soy Dios; *"),
						b: String::from("he de ser ensalzado entre las naciones,\nensalzado seré en la tierra\".")
					},
					PsalmVerse {
						number: 12,
						a: String::from("El Señor de las huestes está con nosotros; *"),
						b: String::from("nuestro refugio es el Dios de Jacob.")
					}
				]
			}
		]
	};
}