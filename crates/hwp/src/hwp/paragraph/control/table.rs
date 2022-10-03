use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::{control::paragraph_list_header::ParagraphListHeader, Paragraph},
    record::{tags::BodyTextRecord, Record},
    version::Version,
};

use super::common_properties::CommonProperties;

/// 표 컨트롤
#[derive(Debug)]
pub struct TableControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    pub record: TableRecord,
    pub cells: Vec<Cell>,
}

impl TableControl {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        assert!(
            record.is_next_child_id(BodyTextRecord::HWPTAG_TABLE as u32),
            "테이블이 아닙니다"
        );

        let table_record = TableRecord::from_record(&mut record.next_child(), version);
        let mut cells = Vec::new();
        while record.is_next_child_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            cells.push(Cell::from_record(
                &mut record.next_child(),
                &mut record,
                version,
            ));
        }

        assert!(
            !record.has_next_children(),
            "해석할 수 없는 추가 데이터가 있습니다"
        );

        Self {
            common_properties,
            record: table_record,
            cells,
        }
    }
}

#[derive(Debug)]
pub struct TableRecord {
    pub rows: u16,
    pub cols: u16,
    pub row_count: Vec<u16>,
}

impl TableRecord {
    pub fn from_record(record: &mut Record, version: &Version) -> Self {
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_TABLE as u32,
            "테이블 레코드가 아닙니다"
        );

        let mut reader = record.get_data_reader();

        // TODO: (@hahnlee) 속성
        reader.read_u32::<LittleEndian>().unwrap();

        let rows = reader.read_u16::<LittleEndian>().unwrap();
        let cols = reader.read_u16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) CellSpacing
        reader.read_i16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 안쪽 여백 정보
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();

        let mut row_count: Vec<u16> = Vec::with_capacity(rows as usize);
        for _ in 0..(rows as usize) {
            row_count.push(reader.read_u16::<LittleEndian>().unwrap());
        }

        // TODO: (@hahnlee) Border Fill ID
        reader.read_u16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 영역 속성
        if version.ge(&Version::from_str("5.0.1.0")) {
            let size = reader.read_u16::<LittleEndian>().unwrap();
            for _ in 0..size {
                let mut buf = [0u8; 10];
                reader.read_exact(&mut buf).unwrap();
            }
        }

        Self {
            rows,
            cols,
            row_count,
        }
    }

    pub fn cell_count(&self) -> u16 {
        self.row_count.iter().fold(0, |result, cols| result + cols)
    }
}

#[derive(Debug)]
pub struct Cell {
    pub header: ParagraphListHeader,
    pub paragraphs: Vec<Paragraph>,
}

impl Cell {
    pub fn from_record(meta: &mut Record, content: &mut Record, version: &Version) -> Self {
        let mut reader = meta.get_data_reader();
        let header = ParagraphListHeader::from_reader(&mut reader);

        let mut paragraphs = Vec::with_capacity(header.count as usize);
        for _ in 0..header.count {
            paragraphs.push(Paragraph::from_record(&mut content.next_child(), version));
        }

        Self {
            header,
            paragraphs,
        }
    }
}
