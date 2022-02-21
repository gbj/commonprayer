use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_119: Psalm = Psalm {
		number: 119,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 654
				},
				local_name: String::from("Alef"),
				latin_name: String::from("Beati immaculati"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¡Dichosos los de camino intachable, *"),
						b: String::from("los que andan en la ley del Señor!")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¡Dichosos los que guardan sus decretos, *"),
						b: String::from("y de todo corazón le buscan!")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Los que nunca cometen iniquidad, *"),
						b: String::from("mas siempre andan en sus caminos.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Tú promulgaste tus decretos, *"),
						b: String::from("para que los observemos plenamente.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¡Ojalá fuesen ordenados mis caminos *"),
						b: String::from("para que guardase tus estatutos!")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Entonces no sería yo avergonzado, *"),
						b: String::from("cuando atendiese a todos tus mandamientos.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Te daré gracias con sincero corazón, *"),
						b: String::from("cuando haya aprendido tus justos juicios.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 655
				},
				local_name: String::from("Bet"),
				latin_name: String::from("In quo corrigit?"),
				verses: vec![
					PsalmVerse {
						number: 8,
						a: String::from("Tus estatutos guardaré; *"),
						b: String::from("no me abandones enteramente.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("¿Cómo limpiará el joven su camino? *"),
						b: String::from("Guardando tu palabra.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Con todo el corazón te busco; *"),
						b: String::from("no dejes que me desvíe de tus mandamientos.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("En mi corazón atesoro tu promesa, *"),
						b: String::from("a fin de no pecar contra ti.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Bendito eres tú, oh Señor; *"),
						b: String::from("enséñame tus estatutos.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Con mis labios contaré *"),
						b: String::from("todos los juicios de tu boca.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Me he gozado más en el camino de tus decretos, *"),
						b: String::from("que en toda riqueza.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("En tus mandamientos meditaré; *"),
						b: String::from("me fijaré en tus caminos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 656
				},
				local_name: String::from("Guímel"),
				latin_name: String::from("Retribue servo tuo"),
				verses: vec![
					PsalmVerse {
						number: 16,
						a: String::from("Me regocijo en tus estatutos; *"),
						b: String::from("no me olvidaré de tus palabras.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Haz bien a este tu siervo, *"),
						b: String::from("para que viva y guarde tu palabra.")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Abreme los ojos, *"),
						b: String::from("para que mire las maravillas de tu ley.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Forastero soy aquí en la tierra; *"),
						b: String::from("no encubras de mí tus mandamientos.")
					},
					PsalmVerse {
						number: 20,
						a: String::from("Mi alma se consume continuamente, *"),
						b: String::from("de tanto anhelar tus juicios.")
					},
					PsalmVerse {
						number: 21,
						a: String::from("Reprendiste a los soberbios. *"),
						b: String::from("¡Malditos los que se desvían de tus mandamientos!")
					},
					PsalmVerse {
						number: 22,
						a: String::from("Aparta de mí la vergüenza y la afrenta, *"),
						b: String::from("porque tus decretos he observado.")
					},
					PsalmVerse {
						number: 23,
						a: String::from("Aunque los príncipes se sienten y conspiren contra mí, *"),
						b: String::from("meditaré en tus estatutos;")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 656
				},
				local_name: String::from("Dálet"),
				latin_name: String::from("Adhaesit pavimento"),
				verses: vec![
					PsalmVerse {
						number: 24,
						a: String::from("Pues tus juicios son mi delicia, *"),
						b: String::from("y tus decretos mis consejeros.")
					},
					PsalmVerse {
						number: 25,
						a: String::from("Mi alma está pegada al polvo; *"),
						b: String::from("vivifícame conforme a tu palabra.")
					},
					PsalmVerse {
						number: 26,
						a: String::from("Te he confesado mis caminos, y me has respondido; *"),
						b: String::from("enséñame tus estatutos.")
					},
					PsalmVerse {
						number: 27,
						a: String::from("Hazme entender el camino de tus mandamientos, *"),
						b: String::from("para que medite en tus maravillas.")
					},
					PsalmVerse {
						number: 28,
						a: String::from("Se derrite mi alma de tristeza; *"),
						b: String::from("fortaléceme conforme a tu palabra.")
					},
					PsalmVerse {
						number: 29,
						a: String::from("Aparta de mí el camino de la mentira; *"),
						b: String::from("que reciba yo gracia por tu ley.")
					},
					PsalmVerse {
						number: 30,
						a: String::from("He escogido el camino de la fidelidad; *"),
						b: String::from("he puesto tus juicios delante de mí.")
					},
					PsalmVerse {
						number: 31,
						a: String::from("Me he apegado a tus decretos; *"),
						b: String::from("oh Señor, no me avergüences.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 657
				},
				local_name: String::from("He"),
				latin_name: String::from("Legem pone"),
				verses: vec![
					PsalmVerse {
						number: 32,
						a: String::from("Por el camino de tus mandamientos correré, *"),
						b: String::from("porque me has ensanchado el corazón.")
					},
					PsalmVerse {
						number: 33,
						a: String::from("Enséñame, oh Señor, el camino de tus estatutos, *"),
						b: String::from("y lo guardaré hasta el fin.")
					},
					PsalmVerse {
						number: 34,
						a: String::from("Dame entendimiento, y guardaré tu ley; *"),
						b: String::from("la cumpliré de todo corazón.")
					},
					PsalmVerse {
						number: 35,
						a: String::from("Guíame por la senda de tus mandamientos, *"),
						b: String::from("porque ése es mi deseo.")
					},
					PsalmVerse {
						number: 36,
						a: String::from("Inclina mi corazón a tus decretos, *"),
						b: String::from("y no a las ganancias injustas.")
					},
					PsalmVerse {
						number: 37,
						a: String::from("Aparta mis ojos, que no miren lo que es inútil; *"),
						b: String::from("vivifícame en tus caminos.")
					},
					PsalmVerse {
						number: 38,
						a: String::from("Cumple tu promesa a tu siervo, *"),
						b: String::from("la que haces a los que te temen.")
					},
					PsalmVerse {
						number: 39,
						a: String::from("Quita de mí el oprobio que temo, *"),
						b: String::from("porque buenos son tus juicios.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 658
				},
				local_name: String::from("Vau"),
				latin_name: String::from("Et veniat super me"),
				verses: vec![
					PsalmVerse {
						number: 40,
						a: String::from("He aquí, anhelo tus mandamientos; *"),
						b: String::from("en tu justicia, preserva mi vida.")
					},
					PsalmVerse {
						number: 41,
						a: String::from("Venga a mí tu bondad, oh Señor, *"),
						b: String::from("tu salvación, conforme a tu promesa.")
					},
					PsalmVerse {
						number: 42,
						a: String::from("Entonces daré respuesta a los que me mofan, *"),
						b: String::from("porque confío en tus palabras.")
					},
					PsalmVerse {
						number: 43,
						a: String::from("No quites de mi boca la palabra de verdad, *"),
						b: String::from("porque en tus mandamientos está mi esperanza.")
					},
					PsalmVerse {
						number: 44,
						a: String::from("Guardaré tu ley continuamente, *"),
						b: String::from("para siempre y hasta la eternidad.")
					},
					PsalmVerse {
						number: 45,
						a: String::from("Andaré en libertad, *"),
						b: String::from("porque estudio tus mandamientos.")
					},
					PsalmVerse {
						number: 46,
						a: String::from("Hablaré de tus decretos ante los reyes, *"),
						b: String::from("y no me avergonzaré.")
					},
					PsalmVerse {
						number: 47,
						a: String::from("Me gozo en tus mandamientos, *"),
						b: String::from("los cuales he amado siempre.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 658
				},
				local_name: String::from("Zain"),
				latin_name: String::from("Memor esto verbi tui"),
				verses: vec![
					PsalmVerse {
						number: 48,
						a: String::from("Alzaré mis manos a tus mandamientos, *"),
						b: String::from("y meditaré en tus estatutos.")
					},
					PsalmVerse {
						number: 49,
						a: String::from("Acuérdate de tu palabra a tu siervo, *"),
						b: String::from("porque tú me has dado esperanza.")
					},
					PsalmVerse {
						number: 50,
						a: String::from("Esto es mi consuelo en la aflicción, *"),
						b: String::from("que tu promesa me da vida.")
					},
					PsalmVerse {
						number: 51,
						a: String::from("Los soberbios se han burlado cruelmente de mí, *"),
						b: String::from("mas no me he desviado de tu ley.")
					},
					PsalmVerse {
						number: 52,
						a: String::from("Cuando me acuerdo de tus juicios antiguos, *"),
						b: String::from("oh Señor, me consuelo en gran manera.")
					},
					PsalmVerse {
						number: 53,
						a: String::from("Siento gran indignación *"),
						b: String::from("ante los malvados que abandonan tu ley.")
					},
					PsalmVerse {
						number: 54,
						a: String::from("Como cánticos han sido para mí tus estatutos, *"),
						b: String::from("dondequiera que he morado como forastero.")
					},
					PsalmVerse {
						number: 55,
						a: String::from("Me acuerdo de tu Nombre en la noche, oh Señor, *"),
						b: String::from("y medito en tu ley.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 659
				},
				local_name: String::from("Chet"),
				latin_name: String::from("Portio mea, Domine"),
				verses: vec![
					PsalmVerse {
						number: 56,
						a: String::from("Estoesloqueamímetoca,*"),
						b: String::from("porque he guardado tus mandamientos.")
					},
					PsalmVerse {
						number: 57,
						a: String::from("Sólo tú, oh Señor, eres mi porción; *"),
						b: String::from("he prometido guardar tus palabras.")
					},
					PsalmVerse {
						number: 58,
						a: String::from("De todo corazón suplico tu favor; *"),
						b: String::from("ten misericordia de mí conforme a tu promesa.")
					},
					PsalmVerse {
						number: 59,
						a: String::from("He considerado mis caminos, *"),
						b: String::from("y he vuelto mis pies a tus decretos.")
					},
					PsalmVerse {
						number: 60,
						a: String::from("Me apresuro, y no me retardo *"),
						b: String::from("en guardar tus mandamientos.")
					},
					PsalmVerse {
						number: 61,
						a: String::from("Aunque los lazos de los malvados me envuelvan, *"),
						b: String::from("no me olvido de tu ley.")
					},
					PsalmVerse {
						number: 62,
						a: String::from("A medianoche me levantaré para darte gracias *"),
						b: String::from("por tus justos juicios.")
					},
					PsalmVerse {
						number: 63,
						a: String::from("Compañero soy de todos los que te temen, *"),
						b: String::from("de cuantos guardan tus mandamientos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 660
				},
				local_name: String::from("Tet"),
				latin_name: String::from("Bonitatem fecisti"),
				verses: vec![
					PsalmVerse {
						number: 64,
						a: String::from("De tu amor, oh Señor, está llena la tierra; *"),
						b: String::from("enséñame tus estatutos.")
					},
					PsalmVerse {
						number: 65,
						a: String::from("Has hecho bien a tu siervo, oh Señor, *"),
						b: String::from("conforme a tu palabra.")
					},
					PsalmVerse {
						number: 66,
						a: String::from("Enséñame criterio y conocimiento, *"),
						b: String::from("porque tus mandamientos he creído.")
					},
					PsalmVerse {
						number: 67,
						a: String::from("Antes que fuera afligido, descarriado andaba, *"),
						b: String::from("mas ahora guardo tu palabra.")
					},
					PsalmVerse {
						number: 68,
						a: String::from("Bueno eres tú, y bienhechor; *"),
						b: String::from("enséñame tus estatutos.")
					},
					PsalmVerse {
						number: 69,
						a: String::from("Los insolentes urden engaño contra mi, *"),
						b: String::from("Mas yo guardaré de todo corazón tus mandamientos.")
					},
					PsalmVerse {
						number: 70,
						a: String::from("Su corazón se espesa como el sebo, *"),
						b: String::from("mas yo en tu ley me regocijo.")
					},
					PsalmVerse {
						number: 71,
						a: String::from("Bueno me es haber sido afligido, *"),
						b: String::from("para que aprenda tus estatutos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 660
				},
				local_name: String::from("Yod"),
				latin_name: String::from("Manus tuae fecerunt me"),
				verses: vec![
					PsalmVerse {
						number: 72,
						a: String::from("Más estimo yo la ley de tu boca *"),
						b: String::from("que millares en oro y plata.")
					},
					PsalmVerse {
						number: 73,
						a: String::from("Tus manos me hicieron y me formaron; *"),
						b: String::from("dame entendimiento para que aprenda tus mandamientos.")
					},
					PsalmVerse {
						number: 74,
						a: String::from("Los que te temen se alegrarán al verme, *"),
						b: String::from("porque en tu palabra confío.")
					},
					PsalmVerse {
						number: 75,
						a: String::from("Yo sé, oh Señor, que tus juicios son justos, *"),
						b: String::from("y que conforme a tu fidelidad me afligiste.")
					},
					PsalmVerse {
						number: 76,
						a: String::from("Sea tu bondad mi consuelo, *"),
						b: String::from("según la promesa hecha a tu siervo.")
					},
					PsalmVerse {
						number: 77,
						a: String::from("Venga a mí tu compasión, para que viva, *"),
						b: String::from("porque tu ley es mi delicia.")
					},
					PsalmVerse {
						number: 78,
						a: String::from("Que se avergüencen los arrogantes,\nporque me han calumniado; *"),
						b: String::from("pero yo meditaré en tus mandamientos.")
					},
					PsalmVerse {
						number: 79,
						a: String::from("Vuélvanse a mí los que te temen, *"),
						b: String::from("y cuantos conocen tus decretos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 661
				},
				local_name: String::from("Caf"),
				latin_name: String::from("Defecit in salutare"),
				verses: vec![
					PsalmVerse {
						number: 80,
						a: String::from("Sea mi corazón íntegro en tus estatutos, *"),
						b: String::from("para que no sea yo avergonzado.")
					},
					PsalmVerse {
						number: 81,
						a: String::from("Ansía mi alma tu salvación; *"),
						b: String::from("he puesto mi esperanza en tu palabra.")
					},
					PsalmVerse {
						number: 82,
						a: String::from("Desfallecieron mis ojos, aguardando tu promesa, *"),
						b: String::from("y digo: \"¿Cuándo me consolarás?\"")
					},
					PsalmVerse {
						number: 83,
						a: String::from("Soy como el odre al humo, *"),
						b: String::from("pero no he olvidado tus estatutos.")
					},
					PsalmVerse {
						number: 84,
						a: String::from("¿Hasta cúando deberé aguardar? *"),
						b: String::from("¿Cuándo harás juicio contra los que me persiguen?")
					},
					PsalmVerse {
						number: 85,
						a: String::from("Los soberbios han cavado hoyos para mí; *"),
						b: String::from("ellos no guardan tu ley.")
					},
					PsalmVerse {
						number: 86,
						a: String::from("Todos tus mandamientos son ciertos; *"),
						b: String::from("ayúdame, pues sin causa me persiguen.")
					},
					PsalmVerse {
						number: 87,
						a: String::from("Casi me han acabado de la tierra, *"),
						b: String::from("pero no he abandonado tus mandamientos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 662
				},
				local_name: String::from("Lámed"),
				latin_name: String::from("In aeternum Domine"),
				verses: vec![
					PsalmVerse {
						number: 88,
						a: String::from("Vivifícame conforme a tu bondad, *"),
						b: String::from("para que guarde los decretos de tu boca.")
					},
					PsalmVerse {
						number: 89,
						a: String::from("Tu palabra, oh Señor, es eterna, *"),
						b: String::from("establecida es en los cielos.")
					},
					PsalmVerse {
						number: 90,
						a: String::from("Tu fidelidad perdura de generación en generación; *"),
						b: String::from("tú afirmaste la tierra, y permanece.")
					},
					PsalmVerse {
						number: 91,
						a: String::from("Por tu decreto permanecen hasta hoy, *"),
						b: String::from("porque todo está a tu servicio.")
					},
					PsalmVerse {
						number: 92,
						a: String::from("Si tu ley no hubiese sido mi delicia, *"),
						b: String::from("en mi aflicción hubiera perecido.")
					},
					PsalmVerse {
						number: 93,
						a: String::from("Jamás me olvidaré de tus mandamientos, *"),
						b: String::from("pues por ellos me das vida.")
					},
					PsalmVerse {
						number: 94,
						a: String::from("Tuyo soy; ¡ojalá me salvaras! *"),
						b: String::from("porque estudio tus mandamientos.")
					},
					PsalmVerse {
						number: 95,
						a: String::from("Aunque los malvados me asechen para destruirme, *"),
						b: String::from("yo consideraré tus decretos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 662
				},
				local_name: String::from("Mem"),
				latin_name: String::from("Quomodo dilexi!"),
				verses: vec![
					PsalmVerse {
						number: 96,
						a: String::from("He visto que todas las cosas tienen fin, *"),
						b: String::from("pero tus mandamientos son infinitos.")
					},
					PsalmVerse {
						number: 97,
						a: String::from("¡Oh, cuánto amo tu ley! *"),
						b: String::from("Todo el día la estoy meditando.")
					},
					PsalmVerse {
						number: 98,
						a: String::from("Tus mandamientos me han hecho más sabio que mis enemigos, *"),
						b: String::from("y siempre están conmigo.")
					},
					PsalmVerse {
						number: 99,
						a: String::from("Soy más docto que todos mis maestros, *"),
						b: String::from("porque estudio tus decretos.")
					},
					PsalmVerse {
						number: 100,
						a: String::from("Soy más sabio que los ancianos, *"),
						b: String::from("porque observo tus mandamientos.")
					},
					PsalmVerse {
						number: 101,
						a: String::from("De todo mal camino contengo mis pies, *"),
						b: String::from("para guardar tu palabra.")
					},
					PsalmVerse {
						number: 102,
						a: String::from("No me aparto de tus juicios, *"),
						b: String::from("porque tú mismo me has enseñado.")
					},
					PsalmVerse {
						number: 103,
						a: String::from("¡Cuán dulces son a mi paladar tus palabras, *"),
						b: String::from("más que la miel a mi boca!")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 663
				},
				local_name: String::from("Nun"),
				latin_name: String::from("Lucerna pedibus meis"),
				verses: vec![
					PsalmVerse {
						number: 104,
						a: String::from("De tus mandamientos adquiero comprensión; *"),
						b: String::from("por tanto, aborrezco el camino de la mentira.")
					},
					PsalmVerse {
						number: 105,
						a: String::from("Lámpara es a mis pies tu palabra, *"),
						b: String::from("y lumbrera en mi camino.")
					},
					PsalmVerse {
						number: 106,
						a: String::from("He jurado y estoy resuelto *"),
						b: String::from("a guardar tus justos juicios.")
					},
					PsalmVerse {
						number: 107,
						a: String::from("Afligido estoy en gran manera; *"),
						b: String::from("vivifícame, oh Señor, conforme a tu palabra.")
					},
					PsalmVerse {
						number: 108,
						a: String::from("Acepta, oh Señor, la ofrenda voluntaria de mis labios, *"),
						b: String::from("y enséñame tus juicios.")
					},
					PsalmVerse {
						number: 109,
						a: String::from("Mi vida está siempre en peligro; *"),
						b: String::from("por tanto, no olvido tu ley.")
					},
					PsalmVerse {
						number: 110,
						a: String::from("Me tendieron lazo los malvados, *"),
						b: String::from("pero yo no me desvié de tus mandamientos.")
					},
					PsalmVerse {
						number: 111,
						a: String::from("Son tus decretos mi herencia eterna; *"),
						b: String::from("en verdad, el gozo de mi corazón.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 664
				},
				local_name: String::from("Sámec"),
				latin_name: String::from("Iniquos odio habui"),
				verses: vec![
					PsalmVerse {
						number: 112,
						a: String::from("Mi corazón incliné a cumplir tus estatutos, *"),
						b: String::from("eternamente y hasta el fin.")
					},
					PsalmVerse {
						number: 113,
						a: String::from("Odio a los inconstantes; *"),
						b: String::from("por mi parte, yo amo tu ley.")
					},
					PsalmVerse {
						number: 114,
						a: String::from("Mi refugio y mi escudo eres tú; *"),
						b: String::from("en tu palabra yo espero.")
					},
					PsalmVerse {
						number: 115,
						a: String::from("¡Apártense de mí, malvados! *"),
						b: String::from("Guardaré los mandamientos de mi Dios.")
					},
					PsalmVerse {
						number: 116,
						a: String::from("Susténtame conforme a tu promesa, y viviré; *"),
						b: String::from("no quede frustrada mi esperanza.")
					},
					PsalmVerse {
						number: 117,
						a: String::from("Sosténme, y seré salvo, *"),
						b: String::from("y me deleitaré siempre en tus estatutos.")
					},
					PsalmVerse {
						number: 118,
						a: String::from("Rechazas a todos los que se desvían de tus estatutos; *"),
						b: String::from("su doblez no les sirve para nada.")
					},
					PsalmVerse {
						number: 119,
						a: String::from("Tienes por escoria a todos los malvados; *"),
						b: String::from("por tanto, yo amo tus decretos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 664
				},
				local_name: String::from("Ayin"),
				latin_name: String::from("Feci judicium"),
				verses: vec![
					PsalmVerse {
						number: 120,
						a: String::from("Mi carne se estremece por temor a ti; *"),
						b: String::from("de tus juicios tengo miedo.")
					},
					PsalmVerse {
						number: 121,
						a: String::from("Lo que es justo y recto he hecho; *"),
						b: String::from("no me entregues a mis opresores.")
					},
					PsalmVerse {
						number: 122,
						a: String::from("Afianza a tu siervo para bien; *"),
						b: String::from("no permitas que los soberbios me opriman.")
					},
					PsalmVerse {
						number: 123,
						a: String::from("Mis ojos han desfallecido, aguardando tu salvación *"),
						b: String::from("y tu promesa de justicia.")
					},
					PsalmVerse {
						number: 124,
						a: String::from("Haz con tu siervo según tu misericordia, *"),
						b: String::from("y enséñame tus estatutos.")
					},
					PsalmVerse {
						number: 125,
						a: String::from("Tu siervo soy; dame entendimiento *"),
						b: String::from("para conocer tus decretos.")
					},
					PsalmVerse {
						number: 126,
						a: String::from("Es hora de que actúes, oh Señor, *"),
						b: String::from("porque han quebrantado tu ley.")
					},
					PsalmVerse {
						number: 127,
						a: String::from("En verdad, yo amo tus mandamientos, *"),
						b: String::from("más que el oro, más que las piedras preciosas.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 665
				},
				local_name: String::from("Pe"),
				latin_name: String::from("Mirabilia"),
				verses: vec![
					PsalmVerse {
						number: 128,
						a: String::from("Sobre todo estimo rectos tus mandamientos; *"),
						b: String::from("aborrezco el camino de la mentira.")
					},
					PsalmVerse {
						number: 129,
						a: String::from("Maravillosos son tus decretos; *"),
						b: String::from("por tanto, los guardo de todo corazón.")
					},
					PsalmVerse {
						number: 130,
						a: String::from("La revelación de tu palabra ilumina; *"),
						b: String::from("hace entender a los inocentes.")
					},
					PsalmVerse {
						number: 131,
						a: String::from("Abro la boca y jadeo; *"),
						b: String::from("ansío tus mandamientos.")
					},
					PsalmVerse {
						number: 132,
						a: String::from("Vuélvete a mí, y ten misericordia, *"),
						b: String::from("como acostumbras con los que aman tu Nombre.")
					},
					PsalmVerse {
						number: 133,
						a: String::from("Afirma mis pasos con tu palabra; *"),
						b: String::from("que ninguna iniquidad me domine.")
					},
					PsalmVerse {
						number: 134,
						a: String::from("Rescátame de los que me oprimen, *"),
						b: String::from("y guardaré tus mandamientos.")
					},
					PsalmVerse {
						number: 135,
						a: String::from("Haz resplandecer tu rostro sobre tu siervo, *"),
						b: String::from("y enséñame tus estatutos.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 666
				},
				local_name: String::from("Tsade"),
				latin_name: String::from("Justus es, Domine"),
				verses: vec![
					PsalmVerse {
						number: 136,
						a: String::from("Ríos de aguas brotan de mis ojos *"),
						b: String::from("a causa de los que no guardan tu ley.")
					},
					PsalmVerse {
						number: 137,
						a: String::from("Justo eres tú, oh Señor, *"),
						b: String::from("y rectos son tus juicios.")
					},
					PsalmVerse {
						number: 138,
						a: String::from("Has promulgado tus decretos *"),
						b: String::from("con justicia y suma fidelidad.")
					},
					PsalmVerse {
						number: 139,
						a: String::from("La indignación me ha consumido, *"),
						b: String::from("porque mis enemigos se olvidan de tus palabras.")
					},
					PsalmVerse {
						number: 140,
						a: String::from("Tu palabra ha pasado las más duras pruebas, *"),
						b: String::from("y tu siervo la atesora.")
					},
					PsalmVerse {
						number: 141,
						a: String::from("Pequeño soy e insignificante, *"),
						b: String::from("pero no olvido tus mandamientos.")
					},
					PsalmVerse {
						number: 142,
						a: String::from("Tu justicia es justicia eterna, *"),
						b: String::from("y tu ley es la verdad.")
					},
					PsalmVerse {
						number: 143,
						a: String::from("Aflicción y angustia se han apoderado de mí, *"),
						b: String::from("mas tus mandamientos son mi delicia.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 666
				},
				local_name: String::from("Cof"),
				latin_name: String::from("Clamavi in toto corde meo"),
				verses: vec![
					PsalmVerse {
						number: 144,
						a: String::from("La rectitud de tus decretos es eterna; *"),
						b: String::from("dame entendimiento, y viviré.")
					},
					PsalmVerse {
						number: 145,
						a: String::from("Clamo con todo mi corazón; *"),
						b: String::from("respóndeme, oh Señor, y guardaré tus estatutos.")
					},
					PsalmVerse {
						number: 146,
						a: String::from("A ti clamo; ¡oh, que tú me salvaras! *"),
						b: String::from("Guardaré tus decretos.")
					},
					PsalmVerse {
						number: 147,
						a: String::from("Me anticipo al alba, pidiendo socorro, *"),
						b: String::from("porque en tu palabra espero.")
					},
					PsalmVerse {
						number: 148,
						a: String::from("Velan mis ojos en las vigilias de la noche, *"),
						b: String::from("para meditar en tu promesa.")
					},
					PsalmVerse {
						number: 149,
						a: String::from("Escucha mi voz, oh Señor, conforme a tu misericordia; *"),
						b: String::from("según tus juicios, vivifícame.")
					},
					PsalmVerse {
						number: 150,
						a: String::from("Se acercan los que me persiguen con malicia; *"),
						b: String::from("están muy lejos de tu ley.")
					},
					PsalmVerse {
						number: 151,
						a: String::from("Cercano estás tú, oh Señor, *"),
						b: String::from("y todos tus mandamientos son verdad.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 667
				},
				local_name: String::from("Resh"),
				latin_name: String::from("Vide humilitatem"),
				verses: vec![
					PsalmVerse {
						number: 152,
						a: String::from("Por tus decretos hace mucho he sabido *"),
						b: String::from("que los has establecido para siempre.")
					},
					PsalmVerse {
						number: 153,
						a: String::from("Mira mi humillación y líbrame, *"),
						b: String::from("porque no olvido tu ley.")
					},
					PsalmVerse {
						number: 154,
						a: String::from("Defiende mi causa y redímeme; *"),
						b: String::from("dame vida conforme a tu promesa.")
					},
					PsalmVerse {
						number: 155,
						a: String::from("Lejos está de los malvados la salvación, *"),
						b: String::from("porque no estudian tus estatutos.")
					},
					PsalmVerse {
						number: 156,
						a: String::from("Grande es tu compasión, oh Señor; *"),
						b: String::from("preserva mi vida conforme a tus juicios.")
					},
					PsalmVerse {
						number: 157,
						a: String::from("Muchos son mis perseguidores y mis enemigos, *"),
						b: String::from("mas de tus decretos no me he apartado.")
					},
					PsalmVerse {
						number: 158,
						a: String::from("Veo a los infieles, y me disgusto, *"),
						b: String::from("porque no han guardado tu palabra.")
					},
					PsalmVerse {
						number: 159,
						a: String::from("¡Mira, oh Señor, cómo amo tus mandamientos! *"),
						b: String::from("Por tu misericordia, preserva mi vida.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 668
				},
				local_name: String::from("Sin"),
				latin_name: String::from("Principes persecuti sunt"),
				verses: vec![
					PsalmVerse {
						number: 160,
						a: String::from("La esencia de tu palabra es la verdad; *"),
						b: String::from("eternos son todos tus justos juicios.")
					},
					PsalmVerse {
						number: 161,
						a: String::from("Príncipes me han perseguido sin causa, *"),
						b: String::from("pero mi corazón teme tu palabra.")
					},
					PsalmVerse {
						number: 162,
						a: String::from("Me regocijo tanto en tu promesa *"),
						b: String::from("como el que halla muchos despojos.")
					},
					PsalmVerse {
						number: 163,
						a: String::from("La mentira aborrezco y abomino, *"),
						b: String::from("pero tu ley yo amo.")
					},
					PsalmVerse {
						number: 164,
						a: String::from("Siete veces al día te alabo, *"),
						b: String::from("a causa de tus justos juicios.")
					},
					PsalmVerse {
						number: 165,
						a: String::from("Mucha paz tienen los que aman tu ley, *"),
						b: String::from("y no hay para ellos tropiezo.")
					},
					PsalmVerse {
						number: 166,
						a: String::from("Tu salvación he esperado, oh Señor, *"),
						b: String::from("y he cumplido tus mandamientos.")
					},
					PsalmVerse {
						number: 167,
						a: String::from("He guardado tus decretos, *"),
						b: String::from("y los he amado en gran manera.")
					}
				]
			},
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 668
				},
				local_name: String::from("Tau"),
				latin_name: String::from("Appropinquet deprecatio"),
				verses: vec![
					PsalmVerse {
						number: 168,
						a: String::from("He guardado tus mandamientos y tus decretos, *"),
						b: String::from("porque todos mis caminos están delante de ti.")
					},
					PsalmVerse {
						number: 169,
						a: String::from("Llegue mi clamor delante de ti, oh Señor; *"),
						b: String::from("dame entendimiento conforme a tu palabra.")
					},
					PsalmVerse {
						number: 170,
						a: String::from("Entre mi súplica en tu presencia; *"),
						b: String::from("líbrame conforme a tu promesa.")
					},
					PsalmVerse {
						number: 171,
						a: String::from("De mis labios brotará tu alabanza, *"),
						b: String::from("cuando me enseñes tus estatutos.")
					},
					PsalmVerse {
						number: 172,
						a: String::from("Cantará mi lengua de tu promesa, *"),
						b: String::from("porque todos tus mandamientos son justos.")
					},
					PsalmVerse {
						number: 173,
						a: String::from("Esté tu mano pronta para socorrerme, *"),
						b: String::from("porque tus mandamientos he escogido.")
					},
					PsalmVerse {
						number: 174,
						a: String::from("Ansío tu salvación, oh Señor, *"),
						b: String::from("y tu ley es mi delicia.")
					},
					PsalmVerse {
						number: 175,
						a: String::from("Viva mi alma para alabarte, *"),
						b: String::from("y tus juicios me ayuden.")
					},
					PsalmVerse {
						number: 176,
						a: String::from("Me extravié como oveja perdida; *"),
						b: String::from("busca a tu siervo porque no olvido tus mandamientos.")
					}
				]
			}
		]
	};
}