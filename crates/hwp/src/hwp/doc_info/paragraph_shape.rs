use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecord, Record},
    version::Version, utils::bits::get_value_range,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ParagraphShape {
    /// 정렬 방식
    pub align: Align,
    /// 왼쪽 여백
    pub padding_left: i32,
    /// 오른쪽 여백
    pub padding_right: i32,
    /// 문단 간격 위
    pub margin_top: i32,
    /// 문단 간격 아래
    pub margin_bottom: i32,
    /// 탭 정의 아이디(TabDef ID) 참조 값
    pub tab_definition_id: u16,
    /// 번호 문단 ID(Numbering ID) 또는 글머리표 문단 모양 ID(Bullet ID) 참조 값
    pub numbering_bullet_id: u16,
    /// 테두리/배경 모양 ID(BorderFill ID) 참조 값
    pub border_fill_id: u16,
}

impl FromRecord for ParagraphShape {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_PARA_SHAPE as u32);
        let mut reader = record.get_data_reader();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 속성 모델링 고민
        // 줄 간격 종류. 한/글 2007 이하 버전에서 사용.
        get_value_range(attribute, 0, 1);
        let align = Align::from_u32(get_value_range(attribute, 2, 4)).unwrap();
        // TODO: (@hahnlee) 남은 속성 파싱

        let padding_left = reader.read_i32::<LittleEndian>().unwrap();
        let padding_right = reader.read_i32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 들여 쓰기/내어 쓰기
        reader.read_i32::<LittleEndian>().unwrap();

        let margin_top = reader.read_i32::<LittleEndian>().unwrap();
        let margin_bottom = reader.read_i32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 줄 간격. 한글 2007 이하 버전(5.0.2.5 버전 미만)에서 사용.
        reader.read_u32::<LittleEndian>().unwrap();

        let tab_definition_id = reader.read_u16::<LittleEndian>().unwrap();
        let numbering_bullet_id = reader.read_u16::<LittleEndian>().unwrap();
        let border_fill_id = reader.read_u16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 문단 테두리 왼쪽 간격
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 문단 테두리 오른쪽 간격
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 문단 테두리 위쪽 간격
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 문단 테두리 아래쪽 간격
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) UINT32 속성 2(표 40 참조) (5.0.1.7 버전 이상)
        // TODO: (@hahnlee) UINT32 속성 3(표 41 참조) (5.0.2.5 버전 이상)
        // TODO: (@hahnlee) UINT32 줄 간격(5.0.2.5 버전 이상)
        Self {
            align,
            padding_left,
            padding_right,
            margin_top,
            margin_bottom,
            tab_definition_id,
            numbering_bullet_id,
            border_fill_id,
        }
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum Align {
    /// 양쪽 정렬
    Justify,
    /// 왼쪽 정렬
    Left,
    /// 오른쪽 정렬
    Right,
    /// 가운데 정렬
    Center,
    /// 배분 정렬
    Distributive,
    /// 나눔 정렬
    Divide,
}
