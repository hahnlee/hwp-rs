pub mod body_text;
pub mod section;

mod record;

use self::body_text::BodyText;

use std::io::Cursor;

use cfb::CompoundFile;

#[derive(Debug)]
pub struct HWP {
    pub body_text: BodyText,
}

impl HWP {
    pub fn from_bytes(bytes: Vec<u8>) -> HWP {
        let cursor = Cursor::new(bytes);
        let mut cfb = CompoundFile::open(cursor).unwrap();

        // TODO: (@hahnlee) 배포용문서
        let body_text = BodyText::from_cfb(&mut cfb);

        HWP { body_text }
    }
}
