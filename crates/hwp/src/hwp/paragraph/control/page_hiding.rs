use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{record::Record, utils::bits::get_flag};

/// 감추기
#[derive(Debug, Clone)]
pub struct PageHiding {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 머리말 숨김 여부
    pub header: bool,
    /// 꼬리말 숨김 여부
    pub footer: bool,
    /// 바탕쪽 숨김 여부
    pub master_page: bool,
    /// 테두리 숨김 여부
    pub border: bool,
    /// 배경 숨김 여부
    pub fill: bool,
    /// 페이지 번호 숨김 여부
    pub page_number: bool,
}

impl PageHiding {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 확인 필요
        let properties = reader.read_u8().unwrap();
        let header = get_flag(properties, 1);
        let footer = get_flag(properties, 2);
        let master_page = get_flag(properties, 3);
        let border = get_flag(properties, 4);
        let fill = get_flag(properties, 5);
        let page_number = get_flag(properties, 6);

        Self {
            ctrl_id,
            header,
            footer,
            master_page,
            border,
            fill,
            page_number,
        }
    }
}
