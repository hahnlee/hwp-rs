use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::Record,
    utils::bits::{get_flag, get_value_range},
};

/// 단 정의
#[derive(Debug, Clone)]
pub struct ColumnControl {
    pub ctrl_id: u32,
    /// 단 종류
    pub kind: ColumnKind,
    /// 단 수
    pub count: u16,
    /// 단 방향 지정
    pub direction: ColumnDirection,
    /// 단 너비 동일하게 여부
    pub same_width: bool,
    /// 단 사이 간격
    pub gap: i16,
    /// 단 너비가 동일하지 않으면, 단의 개수만큼 단의 폭
    pub widths: Vec<u16>,
}

impl ColumnControl {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let properties = reader.read_u16::<LittleEndian>().unwrap();
        let kind = ColumnKind::from_u16(get_value_range(properties, 0, 1)).unwrap();
        let count = get_value_range(properties, 2, 9);
        let direction = ColumnDirection::from_u16(get_value_range(properties, 10, 11)).unwrap();
        let same_width = get_flag(properties, 12);

        let gap = reader.read_i16::<LittleEndian>().unwrap();

        let mut widths = vec![];
        for _ in 0..count {
            widths.push(reader.read_u16::<LittleEndian>().unwrap());
        }

        // NOTE: (@hahnlee) 속성의 bit 16-32 (표 139 참조)
        // TODO: (@hahnlee) 문서에 정의 되어 있지 않음. 확인 필요.
        reader.read_u16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee)
        // UINT8 1 단 구분선 종류(테두리/배경의 테두리 선 종류 참조)
        // UINT8 1 단 구분선 굵기(테두리/배경의 테두리 선 굵기 참조)
        // CORORREF 4 단 구분선 색상(테두리/배경의 테두리 선 색상 참조)

        Self {
            ctrl_id,
            kind,
            count,
            direction,
            same_width,
            gap,
            widths,
        }
    }
}

/// 단 방향
#[repr(u16)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum ColumnKind {
    /// 일반 다단
    Normal,
    /// 배분 다단
    Distributed,
    /// 평행 다단
    Parallel,
}

#[repr(u16)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum ColumnDirection {
    /// 왼쪽부터
    Left,
    /// 오른쪽부터
    Right,
    /// 맞쪽
    Both,
}
