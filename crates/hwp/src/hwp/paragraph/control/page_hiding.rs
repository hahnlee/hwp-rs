use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{record::Record, utils::bits::get_flag};

/// 감추기
#[derive(Debug, Clone)]
pub struct PageHiding {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 머리말 숨김 여부
    pub hide_header: bool,
    /// 꼬리말 숨김 여부
    pub hide_footer: bool,
    /// 바탕쪽 숨김 여부
    pub hide_master_page: bool,
    /// 테두리 숨김 여부
    pub hide_border: bool,
    /// 배경 숨김 여부
    pub hide_fill: bool,
    /// 페이지 번호 숨김 여부
    pub hide_page_number: bool,
}

impl PageHiding {
    pub fn from_record(record: &mut Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u8().unwrap();
        let hide_header = get_flag(attribute, 1);
        let hide_footer = get_flag(attribute, 2);
        let hide_master_page = get_flag(attribute, 3);
        let hide_border = get_flag(attribute, 4);
        let hide_fill = get_flag(attribute, 5);
        let hide_page_number = get_flag(attribute, 6);

        Self {
            ctrl_id,
            hide_header,
            hide_footer,
            hide_master_page,
            hide_border,
            hide_fill,
            hide_page_number,
        }
    }
}
