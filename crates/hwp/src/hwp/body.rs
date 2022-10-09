use std::io::{Read, Seek};

use cfb::CompoundFile;

use super::{header::Header, section::Section};

#[derive(Debug)]
pub struct Body {
    pub sections: Vec<Section>,
}

impl Body {
    pub fn from_cfb<T: Read + Seek>(cfb: &mut CompoundFile<T>, header: &Header) -> Body {
        let body_text = cfb.read_storage("/BodyText").unwrap();

        let size = body_text.count();
        let mut sections: Vec<Section> = Vec::with_capacity(size);
        for i in 0..size {
            let mut stream = cfb.open_stream(format!("/BodyText/Section{}", i)).unwrap();
            let section = Section::from_stream(&mut stream, header);
            sections.push(section);
        }

        Body { sections }
    }

    pub fn from_distributed<T: Read + Seek>(cfb: &mut CompoundFile<T>, header: &Header) -> Body {
        let view_text = cfb.read_storage("/ViewText").unwrap();
        let size = view_text.count();

        let mut sections: Vec<Section> = Vec::with_capacity(size);

        // TODO: (@hahnlee) 통합 방법 생각하기
        for i in 0..size {
            let mut stream = cfb.open_stream(format!("/ViewText/Section{}", i)).unwrap();
            let section = Section::from_distributed(&mut stream, header);
            sections.push(section);
        }

        Body { sections }
    }
}
