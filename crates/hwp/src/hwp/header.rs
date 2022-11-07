use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};
use cfb::CompoundFile;
use num::FromPrimitive;
use num_derive::FromPrimitive;

use super::{
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

#[derive(Debug)]
pub struct Header {
    pub version: Version,
    pub flags: Flags,
    pub license: License,
    pub encrypt_version: EncryptVersion,
    pub kogl: KOGL,
    pub signature: [u8; 32],
    pub reserved: [u8; 207],
}

const SIGNATURE_STR: &str = "HWP Document File";

impl Header {
    pub fn from_cfb<T: Read + Seek>(cfb: &mut CompoundFile<T>) -> Self {
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

        let encrypt_version =
            EncryptVersion::from_u32(stream.read_u32::<LittleEndian>().unwrap()).unwrap();
        let kogl = KOGL::from_u8(stream.read_u8().unwrap()).unwrap();

        let mut reserved: [u8; 207] = [0; 207];
        stream.read(&mut reserved).unwrap();

        Self {
            version,
            flags,
            license,
            encrypt_version,
            kogl,
            signature,
            reserved,
        }
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum EncryptVersion {
    None,
    /// 한/글 2.5 버전 이하
    HWP2_5,
    /// 한/글 3.0 버전 Enhanced
    HWP3Enhanced,
    /// 한/글 3.0 버전 Old
    HWP3Old,
    /// 한/글 7.0 버전 이후
    HWP7,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum KOGL {
    None,
    KOR = 6,
    US = 15,
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
    pub reserved: u32,
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
    pub reserved: u32,
}

impl License {
    fn from_bits(bits: u32) -> Self {
        let reserved = get_value_range(bits, 3, 32);

        Self {
            ccl: get_flag(bits, 0),
            replication_restrictions: get_flag(bits, 1),
            replication_alike: get_flag(bits, 2),
            reserved,
        }
    }
}
