use crate::hwp::{record::Record, version::Version};

use super::paragraph_list::ParagraphList;

/// 숨은 설명
#[derive(Debug, Clone)]
pub struct HiddenComment {
    pub paragraph_list: ParagraphList,
}

impl HiddenComment {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let meta = record.next_child();
        let mut reader = meta.get_data_reader();
        let paragraph_list = ParagraphList::from_record(&mut reader, &mut record, version);

        Self { paragraph_list }
    }
}
