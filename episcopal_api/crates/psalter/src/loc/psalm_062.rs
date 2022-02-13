use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
	pub static ref PSALM_62: Psalm = Psalm {
		number: 62,
		citation: None,
		sections: vec![
		 PsalmSection {
				reference: Reference {
					source: Source::LibroDeOracionComun,
					page: 565
				},
				local_name: String::from(""),
				latin_name: String::from("Nonne Deo?"),
				verses: vec![
					PsalmVerse {
						number: 1,
						a: String::from("En silencio aguarda mi alma a Dios; *"),
						b: String::from("sólo de él viene mi salvación.")
					},
					PsalmVerse {
						number: 2,
						a: String::from("Sólo él es mi roca y mi salvación, *"),
						b: String::from("mi fortaleza; jamás seré conmovido.")
					},
					PsalmVerse {
						number: 3,
						a: String::from("¿Hasta cuándo me asediarán todos juntos para aplastarme, *"),
						b: String::from("como si fueran pared que cede o tapia ruinosa?")
					},
					PsalmVerse {
						number: 4,
						a: String::from("Sólo piensan en derribarme de mi altura; *"),
						b: String::from("su mayor placer es la mentira.")
					},
					PsalmVerse {
						number: 5,
						a: String::from("Con la boca bendicen, *"),
						b: String::from("pero en su corazón maldicen.")
					},
					PsalmVerse {
						number: 6,
						a: String::from("En silencio aguarda mi alma a Dios; *"),
						b: String::from("ciertamente, en él esta mi esperanza.")
					},
					PsalmVerse {
						number: 7,
						a: String::from("Sólo él es mi roca y mi salvación, *"),
						b: String::from("mi fortaleza; no seré conmovido.")
					},
					PsalmVerse {
						number: 8,
						a: String::from("En Dios está mi salvación y mi gloria; *"),
						b: String::from("Dios es mi roca fuerte y mi refugio.")
					},
					PsalmVerse {
						number: 9,
						a: String::from("Confíen siempre en él, oh pueblos; *"),
						b: String::from("desahoguen delante de él su corazón,\nporque Dios es nuestro refugio.")
					},
					PsalmVerse {
						number: 10,
						a: String::from("Por cierto, la plebe no es más que un soplo; *"),
						b: String::from("aun los nobles son apariencia.")
					},
					PsalmVerse {
						number: 11,
						a: String::from("Poniéndolos a todos en la balanza, *"),
						b: String::from("serán más leves que un soplo.")
					},
					PsalmVerse {
						number: 12,
						a: String::from("No confíen en la opresión;\nen la rapiña no se envanezcan; *"),
						b: String::from("aunque aumenten las riquezas, no pongan en ellas el corazón.")
					},
					PsalmVerse {
						number: 13,
						a: String::from("Habló Dios una vez; dos veces lo he oído: *"),
						b: String::from("de Dios es el poder.")
					},
					PsalmVerse {
						number: 14,
						a: String::from("De ti, oh Soberano mío, es la misericordia, *"),
						b: String::from("porque tú pagas a cada uno conforme a su obra.")
					}
				]
			}
		]
	};
}