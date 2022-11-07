use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::get_value_range,
    version::Version,
};

#[derive(Debug)]
pub struct Style {
    /// 로컬 스타일 이름. 한글 윈도우에서는 한글 스타일 이름
    pub name: String,
    /// 영문 스타일 이름
    pub english_name: String,
    /// 스타일 종류
    pub kind: StyleKind,
    /// 다음 스타일 아이디 참조값
    /// 문단 스타일에서 사용자가 리턴키를 입력하여 다음 문단으로 이동했을때 적용할 스타일
    pub next_style_id: u8,
    /// 언어코드
    /// https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/6c085406-a698-4e12-9d4d-c3b0ee3dbc4a
    pub lang_id: u16,
    /// 문단 모양 아이디 참조값(문단 모양의 아이디 속성)
    pub paragraph_shape_id: u16,
    /// 글자 모양 아이디(글자 모양의 아이디 속성)
    pub char_shape_id: u16,
    /// HWP 포맷문서에는 없지만 HWPX 포맷문서에 정의되어 있음
    /// 양식모드에서 style 보호하기 여부
    pub lock_form: u16,
}

impl FromRecordCursor for Style {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_STYLE as u32);

        let mut reader = record.get_data_reader();

        let name = reader.read_string::<LittleEndian>().unwrap();
        let english_name = reader.read_string::<LittleEndian>().unwrap();
        let kind = StyleKind::from_u8(get_value_range(reader.read_u8().unwrap(), 0, 2)).unwrap();
        let next_style_id = reader.read_u8().unwrap();
        let lang_id = reader.read_u16::<LittleEndian>().unwrap();
        let paragraph_shape_id = reader.read_u16::<LittleEndian>().unwrap();
        let char_shape_id = reader.read_u16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) HWP 포맷문서에는 없지만 HWPX 포맷문서에 정의되어 있음
        let lock_form = reader.read_u16::<LittleEndian>().unwrap();

        Self {
            name,
            english_name,
            kind,
            next_style_id,
            lang_id,
            paragraph_shape_id,
            char_shape_id,
            lock_form,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum StyleKind {
    /// 문단 스타일
    Para,
    /// 글자 스타일
    Char,
}
