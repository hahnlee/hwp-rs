use std::io::Cursor;

use super::char::{read_char, Char, CharControls};

#[derive(Debug)]
pub struct Chars {
    chars: Vec<Char>,
}

impl Chars {
    pub fn new() -> Chars {
        let chars = vec![Char::CharControl(CharControls::ParaBreak)];
        Self { chars }
    }

    pub fn from_data(data: Vec<u8>, count: usize) -> Chars {
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

        Chars { chars }
    }

    /// 컨트롤 개수를 반환
    pub fn extend_control_count(&self) -> usize {
        self.chars.iter().fold(0, |result, char| match char {
            Char::ExtendedControl(_, _) => result + 1,
            _ => result,
        })
    }

    pub fn to_string(&self) -> String {
        // TODO: (@hahnlee) 테이블 어떻게 하는지 알아보기
        let mut buf: Vec<u16> = Vec::new();

        for char in &self.chars {
            match char {
                Char::CharCode(code) => {
                    buf.push(*code);
                },
                Char::CharControl(CharControls::LineBreak) => {
                    // new line
                    buf.push(10);
                },
                _ => {}
            };
        }

        String::from_utf16(&buf).unwrap()
    }
}
