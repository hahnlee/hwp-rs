use crate::hwp::parameter_set::ParameterSetReader;
use crate::hwp::record::Record;
use byteorder::{LittleEndian, ReadBytesExt};

/// 찾아보기 표식
#[derive(Debug, Clone)]
pub struct Bookmark {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 파라미터 셋 ID
    pub parameter_set_id: u16,
    /// 파라미터 아이템 ID
    pub parameter_item_id: u16,
    /// 이름
    pub name: String,
}

impl Bookmark {
    pub fn from_record(mut record: Record) -> Self {
        let mut reader = record.get_data_reader();
        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let child = record.next_child();
        let mut data = child.get_data_reader();

        // NOTE: (@hahnlee) 실제를 보니
        let parameter_set_id = data.read_u16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) 확인필요. 표준문서에는 WORD로 제시되어 있으나 실제론 2바이트가 남음.
        // 컨트롤 API에서도 count는 long으로 반환하기에 u32로 추정함
        let count = data.read_u32::<LittleEndian>().unwrap();
        assert_eq!(count, 1);

        let parameter_item_id = data.read_u16::<LittleEndian>().unwrap();

        let name = data.read_pit_bstr::<LittleEndian>().unwrap();

        Self {
            ctrl_id,
            parameter_set_id,
            parameter_item_id,
            name,
        }
    }
}
