use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_143: Psalm = Psalm {
		number: 143,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 689
				},
				local_name: String::from(""),
				latin_name: String::from("Domine, exaudi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Oh Señor, escucha mi oración;\ntú que eres fiel, atiende a mis súplicas; *"),
						b: String::from("respóndeme, pues tú eres justo.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("No llames a juicio a tu siervo, *"),
						b: String::from("porque ante ti ninguno será justificado;")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Porque el enemigo ha buscado mi vida; me ha aplastado hasta el suelo; *"),
						b: String::from("me ha hecho habitar en tinieblas como los ya muertos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Mi espíritu desfallece dentro de mí; *"),
						b: String::from("está desolado mi corazón.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Me acuerdo de los tiempos antiguos; medito en todos tus hechos; *"),
						b: String::from("considero las obras de tus manos.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Extiendo mis manos hacia ti; *"),
						b: String::from("mi alma tiene sed de ti como la tierra seca.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Oh Señor, apresúrate a responderme; mi espíritu desfallece; *"),
						b: String::from("no escondas tu rostro de mí,\no seré como los que descienden a la fosa.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Hazme oír tu gracia por la mañana, porque en ti confío; *"),
						b: String::from("hazme ver el camino por donde debo andar, porque a ti levanto mi alma.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Líbrame de mis enemigos, oh Señor, *"),
						b: String::from("porque me acojo a ti por refugio.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Enséñame a cumplir tu voluntad, porque tú eres mi Dios; *"),
						b: String::from("que tu buen Espíritu me guíe por tierra llana.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Por amor de tu Nombre, vivifícame; *"),
						b: String::from("por tu justicia sácame de la angustia.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Por tu bondad, destruye a mis enemigos y aniquila a todos los que me acosan; *"),
						b: String::from("porque en verdad soy tu siervo.")
					}
				]
			}
		]
	};
}