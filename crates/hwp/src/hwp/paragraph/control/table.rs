use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{tags::BodyTextRecord, Record, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

use super::{common_properties::CommonProperties, paragraph_list::ParagraphList};

/// 표 컨트롤
#[derive(Debug, Clone)]
pub struct TableControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    pub record: TableRecord,
    pub cells: Vec<Cell>,
}

impl TableControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);

        assert!(
            cursor.record_id(BodyTextRecord::HWPTAG_TABLE as u32),
            "테이블이 아닙니다"
        );

        let table_record = TableRecord::from_record(&mut cursor.current(), version);
        let mut cells = Vec::new();
        let cell_count = table_record
            .row_count
            .clone()
            .into_iter()
            .reduce(|result, current| result + current)
            .unwrap();

        for _ in 0..cell_count {
            cells.push(Cell::from_record_cursor(cursor, version));
        }

        Self {
            common_properties,
            record: table_record,
            cells,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableRecord {
    pub page_break: PageBreak,
    pub repeat_header: bool,
    pub rows: u16,
    pub cols: u16,
    pub cell_spacing: i16,
    pub padding: [i16; 4],
    /// row에 몇개의 column이 있는지 기록 (표준문서의 Row Size)
    pub row_count: Vec<u16>,
    pub border_fill_id: u16,
    /// 영역 속성 (5.0.1.0 이상)
    pub valid_zones: Vec<ValidZone>,
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum PageBreak {
    /// 나누지 않음
    None,
    /// 셀 단위로 나눔
    Cell,
    /// 나눔 - NOTE: (@hahnlee) 문서에는 나누지 않음으로 되어있으나 나눔이 맞다
    Table,
}

impl TableRecord {
    pub fn from_record(record: &mut Record, version: &Version) -> Self {
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_TABLE as u32,
            "테이블 레코드가 아닙니다"
        );

        let mut reader = record.get_data_reader();

        let properties = reader.read_u32::<LittleEndian>().unwrap();
        let page_break = PageBreak::from_u32(get_value_range(properties, 0, 1)).unwrap();
        let repeat_header = get_flag(properties, 2);

        let rows = reader.read_u16::<LittleEndian>().unwrap();
        let cols = reader.read_u16::<LittleEndian>().unwrap();

        let cell_spacing = reader.read_i16::<LittleEndian>().unwrap();

        let padding = [
            reader.read_i16::<LittleEndian>().unwrap(),
            reader.read_i16::<LittleEndian>().unwrap(),
            reader.read_i16::<LittleEndian>().unwrap(),
            reader.read_i16::<LittleEndian>().unwrap(),
        ];

        let mut row_count: Vec<u16> = Vec::with_capacity(rows as usize);
        for _ in 0..(rows as usize) {
            row_count.push(reader.read_u16::<LittleEndian>().unwrap());
        }

        let border_fill_id = reader.read_u16::<LittleEndian>().unwrap();

        let mut valid_zones = vec![];
        if *version >= Version::from_str("5.0.1.0") {
            let size = reader.read_u16::<LittleEndian>().unwrap();
            for _ in 0..size {
                valid_zones.push(ValidZone::from_reader(&mut reader));
            }
        }

        assert_eq!(reader.position(), record.data.len() as u64);

        Self {
            page_break,
            repeat_header,
            rows,
            cols,
            cell_spacing,
            padding,
            row_count,
            border_fill_id,
            valid_zones,
        }
    }

    pub fn cell_count(&self) -> u16 {
        self.row_count.iter().fold(0, |result, cols| result + cols)
    }
}

#[derive(Debug, Clone)]
pub struct ValidZone {
    /// 시작 열 주소
    pub start_column: u16,
    /// 시작 행 주소
    pub start_row: u16,
    /// 끝 열 주소
    pub end_column: u16,
    /// 끝 행 주소
    pub end_row: u16,
    /// 테두리 채우기 ID
    pub border_fill_id: u16,
}

impl ValidZone {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            start_column: reader.read_u16::<LittleEndian>().unwrap(),
            start_row: reader.read_u16::<LittleEndian>().unwrap(),
            end_column: reader.read_u16::<LittleEndian>().unwrap(),
            end_row: reader.read_u16::<LittleEndian>().unwrap(),
            border_fill_id: reader.read_u16::<LittleEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    /// 문단 리스트
    pub paragraph_list: ParagraphList,
    /// 열 주소
    ///
    /// 0 부터 시작, 왼쪽으로 갈수록 커진다
    pub column: u16,
    /// 행 주소
    ///
    /// 0 부터 시작, 왼쪽으로 갈수록 커진다
    pub row: u16,
    /// 열의 병합 개수
    pub col_span: u16,
    /// 행의 병합 개수
    pub row_span: u16,
    /// 너비
    pub width: u32,
    /// 높이
    pub height: u32,
    pub padding: [u16; 4],
    pub border_fill_id: u16,
}

impl Cell {
    pub fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, BodyTextRecord::HWPTAG_LIST_HEADER as u32);

        let mut reader = record.get_data_reader();
        let paragraph_list = ParagraphList::from_reader(&mut reader, cursor, version);

        let column = reader.read_u16::<LittleEndian>().unwrap();
        let row = reader.read_u16::<LittleEndian>().unwrap();

        let col_span = reader.read_u16::<LittleEndian>().unwrap();
        let row_span = reader.read_u16::<LittleEndian>().unwrap();

        let width = reader.read_u32::<LittleEndian>().unwrap();
        let height = reader.read_u32::<LittleEndian>().unwrap();

        let padding = [
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
        ];

        let border_fill_id = reader.read_u16::<LittleEndian>().unwrap() - 1;

        Self {
            paragraph_list,
            column,
            row,
            col_span,
            row_span,
            width,
            height,
            padding,
            border_fill_id,
        }
    }
}
