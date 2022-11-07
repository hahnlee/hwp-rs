use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::get_flag,
    version::Version,
};

#[derive(Debug)]
pub struct TabDefinition {
    /// 문단 왼쪽 끝 자동 탭(내어 쓰기용 자동 탭) 유무
    pub left_tab: bool,
    /// 문단 오른쪽 끝 자동 탭 유무
    pub right_tab: bool,
    pub tab_infos: Vec<TabInfo>,
}

impl FromRecordCursor for TabDefinition {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_TAB_DEF as u32);

        let mut reader = record.get_data_reader();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let left_tab = get_flag(attribute, 0);
        let right_tab = get_flag(attribute, 1);

        let count = reader.read_u32::<LittleEndian>().unwrap();
        let mut tab_infos = Vec::with_capacity(count as usize);
        for _ in 0..count {
            tab_infos.push(TabInfo::from_reader(&mut reader));
        }

        assert_eq!(reader.position(), record.size as u64);

        Self {
            left_tab,
            right_tab,
            tab_infos,
        }
    }
}

#[derive(Debug)]
pub struct TabInfo {
    pub position: u32,
    pub kind: TabKind,
    pub border_kind: u8,
}

impl TabInfo {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let position = reader.read_u32::<LittleEndian>().unwrap();
        let kind = TabKind::from_u8(reader.read_u8().unwrap()).unwrap();
        let border_kind = reader.read_u8().unwrap();
        // 8 바이트를 맞추기 위한 예약
        reader.read_u16::<LittleEndian>().unwrap();

        Self {
            position,
            kind,
            border_kind,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum TabKind {
    Left,
    Right,
    Center,
    Decimal,
}
