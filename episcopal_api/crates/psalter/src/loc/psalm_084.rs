use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_84: Psalm = Psalm {
		number: 84,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 602
				},
				local_name: String::from(""),
				latin_name: String::from("Quam dilecta!"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Cuán amable tu morada, Señor de los Ejércitos! *"),
						b: String::from("Anhela mi alma y con ardor desea los atrios del Señor;\nmi corazón y mi carne se regocijan en el Dios vivo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("El gorrión ha encontrado casa,\ny la golondrina nido donde poner sus polluelos: *"),
						b: String::from("en tus altares, oh Señor de los Ejércitos, Rey mío y Dios mío.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("¡Dichosos los que habitan en tu casa! *"),
						b: String::from("Perpetuamente te alabarán.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("¡Dichosos los que en ti encuentran su fuerza, *"),
						b: String::from("cuyos corazones están resueltos a peregrinar!")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Los que atraviesan el valle desolado lo hallan un lugar de fuentes, *"),
						b: String::from("porque la lluvia temprana lo ha cubierto de charcos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Treparán de baluarte en baluarte, *"),
						b: String::from("y se revelará el Dios de los dioses en Sión.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Señor Dios de los Ejércitos, escucha mi oración; *"),
						b: String::from("atiéndeme, oh Dios de Jacob.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Mira, oh Dios, a nuestro Escudo; *"),
						b: String::from("pon los ojos en el rostro de tu Ungido.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Mejor es pasar un día en tus atrios que mil en mi propia casa; *"),
						b: String::from("vale más estar en el umbral de la casa de mi Dios, que vivir en las tiendas de los malvados;")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Porque sol y escudo es el Señor Dios; *"),
						b: String::from("él dará la gracia y la gloria.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("No quitará el Señor ningún bien *"),
						b: String::from("a los que andan en integridad.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("¡Oh Señor de los Ejércitos, *"),
						b: String::from("dichosos los que en ti confían!")
					}
				]
			}
		]
	};
}