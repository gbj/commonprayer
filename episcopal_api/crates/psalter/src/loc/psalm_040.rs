use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_40: Psalm = Psalm {
		number: 40,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 537
				},
				local_name: String::from(""),
				latin_name: String::from("Expectans, expectavi"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("Con paciencia esperé al Señor; *"),
						b: String::from("se inclinó a mí, y oyó mi clamor.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Me sacó del pozo de la desolación, del lodo cenagoso; *"),
						b: String::from("puso mis pies sobre peña, y enderezó mis pasos.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Puso luego en mi boca canción nueva, un himno de alabanza a nuestro Dios. *"),
						b: String::from("Muchos verán esto, y temerán, y así confiarán en el Señor.")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Bienaventurados los que ponen en el Señor su confianza, *"),
						b: String::from("que no acuden a malos espíritus, ni recurren a dioses falsos.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("¡Cuántas maravillas has hecho, oh Señor Dios mío, cuántos planes en favor nuestro! *"),
						b: String::from("Nadie se te puede comparar.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Si yo pudiera anunciarlos y hablar de ellos, *"),
						b: String::from("pero no pueden ser contados.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Sacrificio y ofrenda no te agradan; *"),
						b: String::from("(tú me has dado oídos para escucharte);")
					},
					PsalmVerse {
						number: 8,
						a: String::from("Holocausto y sacrificio para expiación no has demandado, *"),
						b: String::from("y entonces dije: \"He aquí, yo vengo.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("En el rollo está escrito de mí: *"),
						b: String::from("'El hacer tu voluntad, Dios mío, me ha agradado; tu ley está en lo profundo de mi corazón' \".")
					},
					PsalmVerse {
						number: 10,
						a: String::from("He anunciado justicia en la gran asamblea; *"),
						b: String::from("he aquí, no refrené mis labios,\ny esto, oh Señor, tú lo sabes.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("No escondí tu benevolencia dentro de mi corazón; he pregonado tu fidelidad y salvación; *"),
						b: String::from("no oculté tu bondad y fidelidad en la gran asamblea.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("Tú eres el Señor; no retengas de mí tu compasión; *"),
						b: String::from("tu bondad y tu fidelidad me guarden siempre;")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Porque me han rodeado males innumerables; me han alcanzado mis maldades, y no puedo\nlevantar la vista; *"),
						b: String::from("se han aumentado más que los cabellos de mi cabeza,\ny mi corazón me falla.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("Dígnate, oh Señor, librarme; *"),
						b: String::from("Señor, apresúrate a socorrerme.")
					},
					PsalmVerse {
						number: 15,
						a: String::from("Sean avergonzados y confundidos a una, los que buscan mi vida para destruirla; *"),
						b: String::from("vuelvan atrás y averguéncense, los que mi ruina desean.")
					},
					PsalmVerse {
						number: 16,
						a: String::from("Sean esquivados a causa de su afrenta, *"),
						b: String::from("los que me dicen: \"¡Ajá!\" con malicia.")
					},
					PsalmVerse {
						number: 17,
						a: String::from("Gócense y alégrense en ti todos los que te buscan; *"),
						b: String::from("digan siempre los que aman tu salvación:\n\"Grande es el Señor\".")
					},
					PsalmVerse {
						number: 18,
						a: String::from("Aunque yo esté afligido y necesitado, *"),
						b: String::from("el Señor pensará en mí.")
					},
					PsalmVerse {
						number: 19,
						a: String::from("Mi ayuda y mi libertador eres tú; *"),
						b: String::from("Dios mío, no te tardes.")
					}
				]
			}
		]
	};
}