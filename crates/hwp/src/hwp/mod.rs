pub mod body;
pub mod doc_info;
pub mod header;
pub mod paragraph;
pub mod section;
pub mod version;

mod record;
mod utils;

use self::{body::Body, doc_info::DocInfo, header::Header};

use std::io::Cursor;

use cfb::CompoundFile;

#[derive(Debug)]
pub struct HWP {
    pub header: Header,
    pub body_texts: Body,
    pub view_texts: Option<Body>,
    pub doc_info: DocInfo,
}

impl HWP {
    pub fn from_bytes(bytes: &[u8]) -> HWP {
        let cursor = Cursor::new(bytes);
        let mut cfb = CompoundFile::open(cursor).unwrap();

        let header = Header::from_cfb(&mut cfb);

        let doc_info = DocInfo::from_cfb(&mut cfb, &header.version);

        let body_texts = Body::from_cfb(&mut cfb, &header.version);
        let view_texts = if header.flags.distributed {
            Some(Body::from_distributed(&mut cfb, &header.version))
        } else {
            None
        };

        HWP {
            header,
            doc_info,
            body_texts,
            view_texts,
        }
    }
}
