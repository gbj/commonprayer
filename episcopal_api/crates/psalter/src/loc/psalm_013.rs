use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_13: Psalm = Psalm {
		number: 13,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 497
				},
				local_name: String::from(""),
				latin_name: String::from("Usquequo, Domine?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("¿Hasta cuándo, oh Señor? ¿Me olvidarás para siempre? *"),
						b: String::from("¿Hasta cuándo esconderás tu rostro de mí?")
					},
					PsalmVerse {
						number: 2,
						a: String::from("¿Hasta cuándo tendré dudas en mi mente, y tristezas en mi corazón cada día? *"),
						b: String::from("¿Hasta cuándo triunfará mi enemigo sobre mí?")
					},
					PsalmVerse {
						number: 3,
						a: String::from("Mira, respóndeme, oh Señor Dios mío; *"),
						b: String::from("alumbra mis ojos, para que no duerma de muerte;")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Para que no diga mi enemigo: \"Lo vencí\", *"),
						b: String::from("ni se alegre mi adversario, si yo resbalare.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Mas yo en tu misericordia he confiado; *"),
						b: String::from("mi corazón se alegrará en tu salvación.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("Cantaré al Señor, porque me ha hecho bien; *"),
						b: String::from("alabaré el Nombre del Señor Altísimo.")
					}
				]
			}
		]
	};
}