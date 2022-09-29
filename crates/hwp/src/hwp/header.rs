use std::io::{Cursor, Read};

use cfb::CompoundFile;

use super::version::Version;

#[derive(Debug)]
pub struct Header {
    pub version: Version,
}

const SIGNATURE_STR: &str = "HWP Document File";

impl Header {
    pub fn from_cfb(cfb: &mut CompoundFile<Cursor<Vec<u8>>>) -> Header {
        let mut stream = cfb.open_stream("/FileHeader").unwrap();

        if stream.len() != 256 {
            // TODO: (@hahnlee) 에러 주기
        }

        // TODO: (@hahnlee) 유틸함수 개발 필요
        let mut signature = [0; 32];
        stream.read(&mut signature).unwrap();

        if String::from_utf8(signature.to_vec()).unwrap() != SIGNATURE_STR {
            // TODO: (@hahnlee) 에러주기
        }

        let mut version = [0; 4];
        stream.read(&mut version).unwrap();

        Header {
            version: Version::from_byte(version),
        }
    }
}
