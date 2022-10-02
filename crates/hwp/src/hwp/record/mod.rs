use std::io::Cursor;

pub mod reader;
pub mod tags;

#[derive(Debug, Clone)]
pub struct Record {
    pub tag_id: u32,
    pub level: u32,
    pub size: u32,
    pub data: Vec<u8>,
    pub children: Vec<Record>,
}

impl Record {
    pub fn new(tag_id: u32, level: u32, size: u32, data: Vec<u8>) -> Self {
        let children = Vec::new();
        Self {
            tag_id,
            level,
            size,
            children,
            data,
        }
    }

    pub fn add(&mut self, other: Record) {
        // NOTE: (@hahnlee) VecDeq와 유사한 기능을 위해 사용
        // VecDeq를 사용하는 것이 편하나, 최종적으로 children을 Vec로 변환할 필요가 있어 남겨둠
        self.children.insert(0, other)
    }

    pub fn is_next_child_id(&self, tag_id: u32) -> bool {
        if self.has_next_children() {
            return self.children.last().unwrap().tag_id == tag_id;
        }

        return false;
    }

    pub fn has_next_children(&self) -> bool {
        return self.children.len() > 0;
    }

    pub fn get_data_reader(&self) -> Cursor<&Vec<u8>> {
        Cursor::new(&self.data)
    }

    pub fn next_child(&mut self) -> Record {
        self.children.pop().unwrap()
    }

    pub fn remain_children(&self) -> Vec<Record> {
        // @NOTE: (@hahnlee): 순서를 반대로 저장했으므로 반환전 순서를 돌린다
        // 파서의 커버리지가 높을수록 모르는 바이트가 적어지기 때문에 성능에 큰 문제가 없을것으로 생각
        let mut out = self.children.clone();
        out.reverse();

        out
    }
}
