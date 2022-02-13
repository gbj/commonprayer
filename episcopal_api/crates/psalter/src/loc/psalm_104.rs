use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_104: Psalm = Psalm {
		number: 104,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 628
				},
				local_name: String::from(""),
				latin_name: String::from("Benedic, anima mea"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Bendice, alma mía, al Señor; *"),
						b: String::from("Señor Dios mío, ¡cuán excelsa tu grandeza!\nTe has vestido de majestad y esplendor.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Te envuelves de luz como con un manto, *"),
						b: String::from("y extiendes los cielos como una cortina.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Cimientas tu habitación sobre las aguas, *"),
						b: String::from("pones las nubes por tu carroza,\ncabalgas sobre las alas del viento.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Haces a los vientos tus mensajeros, *"),
						b: String::from("a las llamas de fuego tus siervos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Asentaste la tierra sobre sus cimientos, *"),
						b: String::from("para que lamas se mueva.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Con el abismo, como con un manto, la cubriste; *"),
						b: String::from("las aguas cubrieron los montes.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("A tu reto huyeron, *"),
						b: String::from("al fragor de tu trueno corrieron.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Subieron a los montes y bajaron a los valles, *"),
						b: String::from("a los lugares que tú les asignaste.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Fijaste los límites que no debían pasar; *"),
						b: String::from("no volverán a cubrir la tierra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Enviaste los manantiales a los valles; *"),
						b: String::from("fluyen entre los montes.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Todas las bestias del campo beben de ellos, *"),
						b: String::from("y los asnos salvajes mitigan su sed.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Junto a ellos las aves del aire hacen sus nidos, *"),
						b: String::from("y cantan entre las ramas.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Desde tu morada en las alturas riegas los montes; *"),
						b: String::from("del fruto de tus obras se sacia la tierra.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Haces brotar hierba para los rebaños, *"),
						b: String::from("y plantas para el uso de la humanidad;")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Para que produzcan alimento de la tierra: *"),
						b: String::from("vino que alegra el corazón,")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Aceite que hace brillar el rostro *"),
						b: String::from("y pan que fortalece el corazón.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Se llenan de savia los árboles del Señor, *"),
						b: String::from("los cedros del Líbano que él plantó.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Allí anidan los pájaros; *"),
						b: String::from("en sus copas la cigüeña hace morada.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Los riscos son madriguera para las cabras monteses, *"),
						b: String::from("y los peñascos para los hiráceos.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Hiciste la luna como señal de las estaciones, *"),
						b: String::from("y el sol conoce su ocaso.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Haces las tinieblas, y viene la noche, *"),
						b: String::from("en la cual rondan las fieras de la selva.")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Los leoncillos rugen por la presa, *"),
						b: String::from("buscando de Dios su comida.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Sale el sol, y se retiran, *"),
						b: String::from("y se echan en sus guaridas.")
					},
					PsalmVerse {
						number: 24,
						a: String::from("El hombre sale a su trabajo, *"),
						b: String::from("y a su labor hasta la tarde.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("¡Cuán múltiples tus obras, oh Señor *"),
						b: String::from("Hiciste todas ellas con sabiduría;\nla tierra está llena de tus criaturas.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("He allí el grande y anchuroso mar,\nen donde bullen criaturas sin número, *"),
						b: String::from("tanto pequeñas como grandes.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Allí se mueven las naves, allí está ese Leviatán, *"),
						b: String::from("que modelaste para jugar con él.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Todos ellos te aguardan, *"),
						b: String::from("para que les des comida a su tiempo.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Se la das, la recogen; *"),
						b: String::from("abres tu mano, se sacian de bienes.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("Escondes tu rostro y se espantan; *"),
						b: String::from("les quitas el aliento;\nexpiran y vuelven a su polvo.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Envías tu Espíritu y son creados; *"),
						b: String::from("así renuevas la faz de la tierra.")
					},
					PsalmVerse {
						number: 32,
						a: String::from("Perdure la gloria del Señor para siempre; *"),
						b: String::from("alégrese el Señor en todas sus obras.")
					},
					PsalmVerse {
						number: 33,
						a: String::from("El mira a la tierra, y ella tiembla; *"),
						b: String::from("toca los montes, y humean.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("Cantaré al Señor mientras viva; *"),
						b: String::from("alabaré a mi Dios mientras exista.")
					},
					PsalmVerse {
						number: 35,
						a: String::from("Que le sea agradable mi poema; *"),
						b: String::from("me regocijaré en el Señor.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Sean consumidos de la tierra los pecadores, *"),
						b: String::from("y los malvados dejen de ser.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Bendice, alma mía, al Señor. *"),
						b: String::from("¡Aleluya!")
					}
				]
			}
		]
	};
}