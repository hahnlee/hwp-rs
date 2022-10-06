use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::record::Record;

/// hwpx 표준의 pageNumCtl 요소 참고
///
/// 쪽 번호를 홀수쪽, 짝수쪽, 양쪽 모두에 표시할지를 설정하는 요소
/// https://www.hancom.com/board/devmanualList.do?artcl_seq=6139
#[derive(Debug, Clone)]
pub struct PageNumberControl {
    pub ctrl_id: u32,
    pub kind: PageNumberControlKind,
}

impl PageNumberControl {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();
        let kind =
            PageNumberControlKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        Self { ctrl_id, kind }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum PageNumberControlKind {
    /// 양 쪽
    Both,
    /// 짝수 쪽
    Even,
    /// 홀수 쪽
    Odd,
}
