use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_65: Psalm = Psalm {
		number: 65,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 568
				},
				local_name: String::from(""),
				latin_name: String::from("Te decet hymnus"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Tú eres digno de alabanza en Sión, oh Dios; *"),
						b: String::from("a ti se pagarán los votos en Jerusalén.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("A ti, que escuchas la oración, vendrá toda carne, *"),
						b: String::from("a causa de sus transgresiones.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Nuestros pecados nos abruman, *"),
						b: String::from("pero tú los borrarás.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Dichosos los que tú escogieres y atrajeres a ti, para que habiten en tus atrios; *"),
						b: String::from("se saciarán de la belleza de tu casa, de la santidad de tu templo.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Cosas asombrosas nos mostrarás en tu justicia, oh Dios de nuestra salvación, *"),
						b: String::from("tú, la esperanza de todos los términos de la tierra, y de los más remotos mares.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Tú afirmas los montes con tu poder; *"),
						b: String::from("están ceñidos de valentía.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Tú calmas el estruendo de los mares, *"),
						b: String::from("el estruendo de sus olas,\ny el alboroto de las gentes.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Los que habitan los confines de la tierra se estremecerán ante tus maravillas; *"),
						b: String::from("tú haces gritar de júbilo al lucero y al héspero.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Visitas la tierra, y la riegas en abundancia; en gran manera la enriqueces; *"),
						b: String::from("la acequia de Dios va llena de agua.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Tú preparas el grano, *"),
						b: String::from("pues así abasteces la tierra.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Haces que se empapen los surcos, y rasas los terrones; *"),
						b: String::from("la ablandas con lluvias copiosas, y bendices sus renuevos.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Tú coronas el año con tus bienes, *"),
						b: String::from("y tus carriles rebosan con abundancia.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Rebosen los pastos del páramo, *"),
						b: String::from("y los collados se vistan de alegría.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Se cubran las praderas de manadas, y los valles se revistan de grano; *"),
						b: String::from("den voces de júbilo y canten.")
					}
				]
			}
		]
	};
}