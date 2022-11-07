use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::record::{tags::DocInfoRecord, RecordCursor};

#[derive(Debug)]
pub struct CompatibleDocument {
    /// 대상 프로그램
    pub target_program: TargetProgram,
    /// 레이아웃 호환성
    pub layout_compatibility: LayoutCompatibility,
}

impl CompatibleDocument {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        let mut reader = record.get_data_reader();

        let target_program =
            TargetProgram::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        assert_eq!(reader.position(), record.size.into());

        let layout_compatibility = LayoutCompatibility::from_record_cursor(cursor);

        Self {
            target_program,
            layout_compatibility,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum TargetProgram {
    /// 한/글 문서(현재 버전)
    HWP201X,
    /// 한/글 2007 호환 문서
    HWP200X,
    /// MS 워드 호환 문서
    MSWord,
}

#[derive(Debug)]
pub struct LayoutCompatibility {
    /// 글자 단위 서식
    pub text_attribute: u32,
    /// 문단 단위 서식
    pub paragraph_attribute: u32,
    /// 구역 단위 서식
    pub section_attribute: u32,
    /// 개체 단위 서식
    pub object_attribute: u32,
    /// 필드 단위 서식
    pub field_attribute: u32,
}

impl LayoutCompatibility {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_LAYOUT_COMPATIBILITY as u32
        );

        let mut reader = record.get_data_reader();

        // NOTE: (@hahnlee) 문서와 되어있지 않음, 정확한 정보는 HWPX와 대조해서 유추해야함
        let text_attribute = reader.read_u32::<LittleEndian>().unwrap();
        let paragraph_attribute = reader.read_u32::<LittleEndian>().unwrap();
        let section_attribute = reader.read_u32::<LittleEndian>().unwrap();
        let object_attribute = reader.read_u32::<LittleEndian>().unwrap();
        let field_attribute = reader.read_u32::<LittleEndian>().unwrap();

        Self {
            text_attribute,
            paragraph_attribute,
            section_attribute,
            object_attribute,
            field_attribute,
        }
    }
}
