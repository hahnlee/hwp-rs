use crate::hwp::{
    paragraph::{control::paragraph_list_header::ParagraphListHeader, Paragraph},
    record::{tags::BodyTextRecord, Record},
    version::Version,
};

/// 머리말 / 꼬리말
#[derive(Debug, Clone)]
pub struct HeaderFooter {
    pub record: HeaderFooterRecord,
}

impl HeaderFooter {
    pub fn from_record(mut record: Record, version: &Version) -> HeaderFooter {
        // TODO: (@hahnlee) 내용 파싱
        let record =
            HeaderFooterRecord::from_record(&mut record.next_child(), &mut record, version);

        HeaderFooter { record }
    }
}

#[derive(Debug, Clone)]
pub struct HeaderFooterRecord {
    pub header: ParagraphListHeader,
    pub paragraphs: Vec<Paragraph>,
}

impl HeaderFooterRecord {
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
            let mut next = content.next_child();
            assert_eq!(
                next.tag_id,
                BodyTextRecord::HWPTAG_PARA_HEADER as u32,
                "문단이 아닙니다"
            );
            let paragraph = Paragraph::from_record(&mut next, version);
            paragraphs.push(paragraph);
        }

        Self { header, paragraphs }
    }
}
