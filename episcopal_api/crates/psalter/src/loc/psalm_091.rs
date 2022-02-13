use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_91: Psalm = Psalm {
		number: 91,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 613
				},
				local_name: String::from(""),
				latin_name: String::from("Qui habitat"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El que habita al abrigo del Altísimo, *"),
						b: String::from("mora bajo la sombra del Omnipotente.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Dirá al Señor: \"Refugio mío y castillo mío, *"),
						b: String::from("mi Dios, en quien confío\".")
					},
					PsalmVerse {
						number: 3,
						a: String::from("El te librará del lazo del cazador, *"),
						b: String::from("de la peste destructora.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Con sus plumas te cubrirá, y debajo de sus alas estarás seguro; *"),
						b: String::from("escudo y adarga será su fidelidad.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("No temerás espanto nocturno, *"),
						b: String::from("ni saeta que vuele de día;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Ni pestilencia que acecha en la oscuridad, *"),
						b: String::from("ni enfermedad que a mediodía desola.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Caerán a tu lado mil, y diez mil a tu diestra, *"),
						b: String::from("mas a ti no te alcanzará.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Ciertamente con tus ojos mirarás, *"),
						b: String::from("y verás la recompensa de los malvados;")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Porque hiciste del Señor tu refugio, *"),
						b: String::from("del Altísimo, tu habitación,")
					},
					PsalmVerse {
						number: 10,
						a: String::from("No te sobrevendrá mal alguno, *"),
						b: String::from("ni plaga tocará tu morada.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Pues a sus ángeles mandará cerca de ti, *"),
						b: String::from("que te guarden en todos tus caminos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("En las manos te llevarán, *"),
						b: String::from("para que tu pie no tropiece en piedra.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Sobre el león y el áspid pisarás; *"),
						b: String::from("hollarás al cachorro del león y a la serpiente.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("\"Por cuanto ha hecho pacto de amor conmigo, yo lo libraré; *"),
						b: String::from("lo protegeré, por cuanto ha conocido mi Nombre.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Me invocará, y yo le responderé; *"),
						b: String::from("con él estaré en la angustia;\nlo libraré, y le glorificaré.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Lo saciaré de largos días, *"),
						b: String::from("y le mostraré mi salvación\".")
					}
				]
			}
		]
	};
}