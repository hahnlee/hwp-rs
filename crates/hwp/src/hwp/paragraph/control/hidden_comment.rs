use crate::hwp::{record::RecordCursor, version::Version};

use super::paragraph_list::ParagraphList;

/// 숨은 설명
#[derive(Debug, Clone)]
pub struct HiddenComment {
    pub paragraph_list: ParagraphList,
}

impl HiddenComment {
    pub fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let meta = cursor.current();
        let mut reader = meta.get_data_reader();
        let paragraph_list = ParagraphList::from_reader(&mut reader, cursor, version);

        Self { paragraph_list }
    }
}
