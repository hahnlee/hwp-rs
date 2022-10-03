use std::io::Seek;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::Paragraph,
    record::{reader::RecordReader, tags::BodyTextRecord, Record},
    version::Version,
};

use super::paragraph_list_header::ParagraphListHeader;

/// 개체 공통 속성
#[derive(Debug)]
pub struct CommonProperties {
    /// 개체 설명문
    pub description: String,
    pub caption: Option<Caption>,
}

impl CommonProperties {
    pub fn from_record(record: &mut Record, version: &Version) -> Self {
        let size = record.data.len() as u64;
        let mut reader = record.get_data_reader();

        // ctrl_id
        reader.read_u32::<LittleEndian>().unwrap();

        // 속성
        reader.read_u32::<LittleEndian>().unwrap();

        // 세로 오프셋 값
        reader.read_u32::<LittleEndian>().unwrap();
        // 가로 오프셋 값
        reader.read_u32::<LittleEndian>().unwrap();

        // width 오브젝트의 폭
        reader.read_u32::<LittleEndian>().unwrap();
        // height 오브젝트의 높이
        reader.read_u32::<LittleEndian>().unwrap();

        // z-order
        reader.read_i32::<LittleEndian>().unwrap();

        // 2x4 오브젝트의 바깥 4방향 여백
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();

        // 문서 내 각 개체에 대한 고유 아이디(instance ID)
        reader.read_u32::<LittleEndian>().unwrap();

        // 쪽나눔 방지 on(1) / off(0)
        reader.read_i32::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) len이 0이 아니라 아예 값이 없을 수도 있다
        let description = if reader.stream_position().unwrap() < size {
            reader.read_string::<LittleEndian>().unwrap()
        } else {
            format!("")
        };

        assert_eq!(
            reader.stream_position().unwrap(),
            size as u64,
            "안읽은 바이트가 있습니다"
        );

        let caption = if record.is_next_child_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(Caption::from_record(
                &mut record.next_child(),
                &mut record.next_child(),
                version,
            ))
        } else {
            None
        };

        Self {
            description,
            caption,
        }
    }
}

#[derive(Debug)]
pub struct Caption {
    pub header: ParagraphListHeader,
    pub paragraphs: Vec<Paragraph>,
}

impl Caption {
    pub fn from_record(meta: &mut Record, content: &mut Record, version: &Version) -> Self {
        assert_eq!(
            meta.tag_id,
            BodyTextRecord::HWPTAG_LIST_HEADER as u32,
            "다른 레코드 입니다"
        );

        let mut reader = meta.get_data_reader();

        let header = ParagraphListHeader::from_reader(&mut reader);

        // TODO: (@hahnlee) 속성을 파싱해야한다. 문서와 크기가 매우 다르고, 없는 경우도 있어 파악이 필요하다

        let mut paragraphs = Vec::new();
        for _ in 0..header.count {
            let paragraph = Paragraph::from_record(content, version);
            paragraphs.push(paragraph);
        }

        Self { header, paragraphs }
    }
}
