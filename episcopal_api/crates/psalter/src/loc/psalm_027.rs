use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_27: Psalm = Psalm {
		number: 27,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 516
				},
				local_name: String::from(""),
				latin_name: String::from("Dominus illuminatio"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("El Señor es mi luz y mi salvación; ¿a quién temeré? *"),
						b: String::from("El Señor es la fortaleza de mi vida; ¿de quién he de atemorizarme?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Cuando se juntaron contra mí los malignos para comer mis carnes, *"),
						b: String::from("ellos mismos, mis adversarios y mis enemigos, tropezaron y cayeron.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Aunque un ejército acampe contra mí, *"),
						b: String::from("no temerá mi corazón;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Y aunque contra mí se levante guerra, *"),
						b: String::from("yo estaré confiado.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Una cosa he demandado del Señor; ésta buscaré: *"),
						b: String::from("que esté yo en la casa del Señor,\ntodos los días de mi vida;")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Para contemplar la hermosura del Señor, *"),
						b: String::from("y despertarme cada día en su templo;")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Porque él me esconderá en su tabernáculo en el día del mal; *"),
						b: String::from("me ocultará en lo reservado de su morada, y sobre una roca me pondrá en alto.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Aún ahora él levanta mi cabeza *"),
						b: String::from("sobre mis enemigos en derredor de mí.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Por tanto ofreceré en su morada sacrificios de júbilo; *"),
						b: String::from("cantaré y tañeré al Señor.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Escucha, oh Señor, mi voz cuando a ti clamo; *"),
						b: String::from("ten misericordia de mí y respóndeme.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Tú hablas en mi corazón y dices: \"Busca mi rostro\". *"),
						b: String::from("Tu rostro buscaré, oh Señor.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("No escondas tu rostro de mí; *"),
						b: String::from("no apartes con ira a tu siervo.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Mi ayuda has sido; no me deseches; *"),
						b: String::from("no me desampares, oh Dios de mi salvación.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Aunque mi padre y mi madre me desamparen, *"),
						b: String::from("aun con todo el Señor me recogerá.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Enséñame, oh Señor, tu camino; *"),
						b: String::from("guíame por senda llana a causa de mis enemigos.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("No me entregues al rencor de mis adversarios, porque se han levantado contra mí testigos falsos; *"),
						b: String::from("y también los que respiran maldad.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Hubiera yo desmayado si no creyese que tengo de ver la bondad del Señor *"),
						b: String::from("en la tierra de los vivientes.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Aguarda al Señor;\nesfuérzate, y aliéntese tu corazón; *"),
						b: String::from("sí, aguarda al Señor.")
					}
				]
			}
		]
	};
}