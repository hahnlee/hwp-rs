use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use cfb::CompoundFile;

use super::version::Version;

#[derive(Debug)]
pub struct Header {
    pub version: Version,
    // TODO: (@hahnlee) enum
    pub encrypt_version: u32,
    pub kogl: u8,
    signature: [u8; 32],
    reserved: [u8; 207],
}

const SIGNATURE_STR: &str = "HWP Document File";

impl Header {
    pub fn from_cfb(cfb: &mut CompoundFile<Cursor<Vec<u8>>>) -> Header {
        let mut stream = cfb.open_stream("/FileHeader").unwrap();

        if stream.len() != 256 {
            // TODO: (@hahnlee) 에러 주기
        }

        let mut signature = [0; 32];
        stream.read(&mut signature).unwrap();

        if String::from_utf8(signature[0..17].to_vec()).unwrap() != SIGNATURE_STR {
            // TODO: (@hahnlee) 에러주기
        }

        let mut version = [0; 4];
        stream.read(&mut version).unwrap();
        let version = Version::from_bytes(version);

        // Flags
        stream.read_u32::<LittleEndian>().unwrap();

        // License
        stream.read_u32::<LittleEndian>().unwrap();

        let encrypt_version = stream.read_u32::<LittleEndian>().unwrap();
        let kogl = stream.read_u8().unwrap();

        let mut reserved: [u8; 207] = [0; 207];
        stream.read(&mut reserved).unwrap();

        Header {
            version,
            encrypt_version,
            kogl,
            signature,
            reserved,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO (@hahnlee) 나머지 영역도 추가하기
        [self.signature.to_vec(), self.reserved.to_vec()].concat()
    }
}
