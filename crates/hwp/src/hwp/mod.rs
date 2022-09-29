pub mod body;
pub mod section;

mod record;

use self::body::Body;

use std::io::Cursor;

use cfb::CompoundFile;

#[derive(Debug)]
pub struct HWP {
    pub body: Body,
}

impl HWP {
    pub fn from_bytes(bytes: Vec<u8>) -> HWP {
        let cursor = Cursor::new(bytes);
        let mut cfb = CompoundFile::open(cursor).unwrap();

        // TODO: (@hahnlee) 배포용문서
        let body = Body::from_cfb(&mut cfb);

        HWP { body }
    }
}
