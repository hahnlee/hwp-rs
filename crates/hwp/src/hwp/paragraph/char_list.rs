use std::io::Cursor;

use super::char::{read_char, Char, CharControls};

#[derive(Debug, Clone)]
pub struct CharList {
    pub chars: Vec<Char>,
}

impl CharList {
    pub fn new() -> Self {
        let chars = vec![Char::CharControl(CharControls::ParaBreak)];
        Self { chars }
    }

    pub fn from_data(data: Vec<u8>, count: usize) -> Self {
        let mut chars = Vec::with_capacity(count as usize);
        let mut reader = Cursor::new(data);

        let mut i = 0;
        loop {
            if i >= count {
                break;
            }
            let char = read_char(&mut reader);
            match char {
                Char::CharCode(_) => {
                    i += 1;
                }
                _ => {
                    i += 8;
                }
            };
            chars.push(char);
        }

        Self { chars }
    }

    /// 컨트롤 개수를 반환
    pub fn extend_control_count(&self) -> usize {
        self.chars.iter().fold(0, |result, char| match char {
            Char::ExtendedControl(_, _) => result + 1,
            _ => result,
        })
    }
}
