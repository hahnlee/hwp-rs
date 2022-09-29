use std::io::Cursor;

use cfb::CompoundFile;

use super::section::Section;

#[derive(Debug)]
pub struct BodyText {
    pub sections: Vec<Section>,
}

impl BodyText {
    pub fn from_cfb(cfb: &mut CompoundFile<Cursor<Vec<u8>>>) -> BodyText {
        let body_text = cfb.read_storage("/BodyText").unwrap();

        let size = body_text.count();
        let mut sections: Vec<Section> = Vec::with_capacity(size);
        for i in 0..size {
            let mut stream = cfb.open_stream(format!("/BodyText/Section{}", i)).unwrap();
            let section = Section::from_stream(&mut stream);
            sections.push(section);
        }

        BodyText { sections }
    }
}
