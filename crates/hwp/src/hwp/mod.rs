pub mod body;
pub mod header;
pub mod section;
pub mod version;

mod record;

use self::{body::Body, header::Header};

use std::io::Cursor;

use cfb::CompoundFile;

#[derive(Debug)]
pub struct HWP {
    pub header: Header,
    pub body: Body,
}

impl HWP {
    pub fn from_bytes(bytes: Vec<u8>) -> HWP {
        let cursor = Cursor::new(bytes);
        let mut cfb = CompoundFile::open(cursor).unwrap();

        let header = Header::from_cfb(&mut cfb);

        // TODO: (@hahnlee) 배포용문서
        let body = Body::from_cfb(&mut cfb);

        HWP { header, body }
    }
}
