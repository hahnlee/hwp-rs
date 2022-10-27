use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    color_ref::ColorRef,
    record::{tags::DocInfoRecord, FromRecord, Record},
    utils::bits::get_flag,
    version::Version,
};

#[derive(Debug)]
pub struct CharShape {
    /// 언어별 글꼴 ID(FaceID) 참조 값
    pub font_ids: [u16; 7],
    /// 언어별 장평, 50%～200%
    pub font_scales: [u8; 7],
    /// 언어별 자간
    pub font_spacings: [i8; 7],
    /// 언어별 상대 크기, 10%～250%
    pub font_sizes: [u8; 7],
    /// 언어별 글자 위치, -100%～100%
    pub font_positions: [i8; 7],
    /// 기준 크기, 0pt～4096pt
    pub base_size: i32,
    /// 기울임 여부
    pub italic: bool,
    /// 진하게 여부
    pub bold: bool,
    /// 글자 색
    pub color: ColorRef,
    /// 밑줄 색
    pub underline_color: ColorRef,
    /// 음영 색
    pub shade_color: ColorRef,
    /// 그림자 색
    pub shadow_color: ColorRef,
    /// 글자 테두리/배경 ID 참조 값 (5.0.2.1 이상)
    pub border_fill_id: Option<u16>,
    /// 취소선 색 (5.0.3.0 이상)
    pub strikethrough_color: Option<ColorRef>,
}

impl FromRecord for CharShape {
    fn from_record(record: &mut Record, version: &Version) -> Self {
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_CHAR_SHAPE as u32,
            "올바르지 않은 정보"
        );

        let mut reader = record.get_data_reader();

        let font_ids = [
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
        ];

        let font_scales = [
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        ];

        let font_spacings = [
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
        ];

        let font_sizes = [
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        ];

        let font_positions = [
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
        ];

        let base_size = reader.read_i32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let italic = get_flag(attribute, 0);
        let bold = get_flag(attribute, 1);
        // TODO: (@hahnlee) 나머지 파싱

        // TODO: (@hahnlee) 그림자 간격, -100%～100%
        reader.read_u8().unwrap();
        // TODO: (@hahnlee) 그림자 간격, -100%～100%
        reader.read_u8().unwrap();

        let color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let underline_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let shade_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let shadow_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        let border_fill_id = if *version >= Version::from_str("5.0.2.1") {
            Some(reader.read_u16::<LittleEndian>().unwrap())
        } else {
            None
        };

        let strikethrough_color = if *version >= Version::from_str("5.0.3.0") {
            Some(ColorRef::from_u32(
                reader.read_u32::<LittleEndian>().unwrap(),
            ))
        } else {
            None
        };

        Self {
            font_ids,
            font_scales,
            font_spacings,
            font_sizes,
            font_positions,
            base_size,
            italic,
            bold,
            color,
            underline_color,
            shade_color,
            shadow_color,
            border_fill_id,
            strikethrough_color,
        }
    }
}
