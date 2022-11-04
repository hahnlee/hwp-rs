use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::record::{tags::DocInfoRecord, RecordCursor};

#[derive(Debug)]
pub struct CompatibleDocument {
    /// 대상 프로그램
    pub target_program: TargetProgram,
}

impl CompatibleDocument {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        let mut reader = record.get_data_reader();

        let target_program =
            TargetProgram::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        assert!(cursor.record_id(DocInfoRecord::HWPTAG_LAYOUT_COMPATIBILITY as u32));
        // TODO: (@hahnlee) 파싱하기
        cursor.current();

        assert_eq!(reader.position(), record.size.into());

        Self { target_program }
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TargetProgram {
    /// 한/글 문서(현재 버전)
    HWP201X,
    /// 한/글 2007 호환 문서
    HWP200X,
    /// MS 워드 호환 문서
    MSWord,
}
