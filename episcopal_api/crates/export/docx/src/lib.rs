use liturgy::Document;

pub struct DocxDocument {}

impl DocxDocument {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl From<Document> for DocxDocument {
    fn from(doc: Document) -> Self {
        todo!()
    }
}
