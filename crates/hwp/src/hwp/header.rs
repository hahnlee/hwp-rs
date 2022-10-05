use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};
use cfb::CompoundFile;

use super::{
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

#[derive(Debug)]
pub struct Header {
    pub version: Version,
    pub flags: Flags,
    pub license: License,
    // TODO: (@hahnlee) enum
    pub encrypt_version: u32,
    // TODO: (@hahnlee) enum
    pub kogl: u8,
    signature: [u8; 32],
    reserved: [u8; 207],
}

const SIGNATURE_STR: &str = "HWP Document File";

impl Header {
    pub fn from_cfb<T: Read + Seek>(cfb: &mut CompoundFile<T>) -> Header {
        let mut stream = cfb.open_stream("/FileHeader").unwrap();

        assert_eq!(stream.len(), 256, "헤더 사이즈가 맞지 않습니다");

        let mut signature = [0; 32];
        stream.read(&mut signature).unwrap();

        assert_eq!(
            String::from_utf8(signature[0..17].to_vec()).unwrap_or_default(),
            SIGNATURE_STR,
            "파일 시그니처가 맞지 않습니다"
        );

        let mut version = [0; 4];
        stream.read(&mut version).unwrap();
        let version = Version::from_bytes(version);

        // Flags
        let flags = stream.read_u32::<LittleEndian>().unwrap();
        let flags = Flags::from_bits(flags);

        let license = stream.read_u32::<LittleEndian>().unwrap();
        let license = License::from_bits(license);

        let encrypt_version = stream.read_u32::<LittleEndian>().unwrap();
        let kogl = stream.read_u8().unwrap();

        let mut reserved: [u8; 207] = [0; 207];
        stream.read(&mut reserved).unwrap();

        Header {
            version,
            flags,
            license,
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

#[derive(Debug)]
pub struct Flags {
    pub compressed: bool,
    pub encrypted: bool,
    pub distributed: bool,
    pub has_script: bool,
    pub drm: bool,
    pub has_xml_template_storage: bool,
    pub vcs: bool,
    pub has_electron_signature: bool,
    pub certificate_encryption: bool,
    pub prepare_signature: bool,
    pub certificate_drm: bool,
    pub ccl: bool,
    pub mobile_optimized: bool,
    pub is_privacy_security_document: bool,
    pub tracking_changes: bool,
    pub kogl: bool,
    pub has_video_control: bool,
    pub has_order_field_control: bool,
    // TODO: (@hahnlee) to_bytes / to_u32 구현시 처리하기
    #[allow(dead_code)]
    reserved: u32,
}

impl Flags {
    fn from_bits(bits: u32) -> Flags {
        let reserved = get_value_range(bits, 14, 32);

        Flags {
            compressed: get_flag(bits, 0),
            encrypted: get_flag(bits, 1),
            distributed: get_flag(bits, 2),
            has_script: get_flag(bits, 3),
            drm: get_flag(bits, 4),
            has_xml_template_storage: get_flag(bits, 5),
            vcs: get_flag(bits, 6),
            has_electron_signature: get_flag(bits, 7),
            certificate_encryption: get_flag(bits, 8),
            prepare_signature: get_flag(bits, 9),
            certificate_drm: get_flag(bits, 10),
            ccl: get_flag(bits, 11),
            mobile_optimized: get_flag(bits, 12),
            is_privacy_security_document: get_flag(bits, 13),
            tracking_changes: get_flag(bits, 14),
            kogl: get_flag(bits, 15),
            has_video_control: get_flag(bits, 16),
            has_order_field_control: get_flag(bits, 17),
            reserved,
        }
    }
}

#[derive(Debug)]
pub struct License {
    pub ccl: bool,
    pub replication_restrictions: bool,
    pub replication_alike: bool,
    // TODO: (@hahnlee) to_bytes / to_u32 구현시 처리하기
    #[allow(dead_code)]
    reserved: u32,
}

impl License {
    fn from_bits(bits: u32) -> License {
        let reserved = get_value_range(bits, 3, 32);

        License {
            ccl: get_flag(bits, 0),
            replication_restrictions: get_flag(bits, 1),
            replication_alike: get_flag(bits, 2),
            reserved,
        }
    }
}
