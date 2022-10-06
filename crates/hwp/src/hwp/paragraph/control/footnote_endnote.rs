use crate::hwp::{record::Record, version::Version};

use super::paragraph_list::ParagraphList;

/// 머리말 / 꼬리말
#[derive(Debug, Clone)]
pub struct FootnoteEndnote {
    pub paragraph_list: ParagraphList,
}

impl FootnoteEndnote {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let meta = record.next_child();
        let mut reader = meta.get_data_reader();
        let paragraph_list = ParagraphList::from_record(&mut reader, &mut record, version);

        Self { paragraph_list }
    }
}
