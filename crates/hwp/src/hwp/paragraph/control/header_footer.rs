use crate::hwp::{record::Record, version::Version};

use super::paragraph_list::ParagraphList;

/// 머리말 / 꼬리말
#[derive(Debug, Clone)]
pub struct HeaderFooter {
    pub paragraph_list: ParagraphList,
}

impl HeaderFooter {
    pub fn from_record(mut record: Record, version: &Version) -> HeaderFooter {
        let meta = record.next_child();
        let mut reader = meta.get_data_reader();

        let paragraph_list = ParagraphList::from_record(&mut reader, &mut record, version);
        HeaderFooter { paragraph_list }
    }
}
