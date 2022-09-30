use std::io::Read;

use super::char::{read_char, Char};

#[derive(Debug)]
pub struct Chars {
    data: Vec<Char>,
}

impl Chars {
    pub fn from_reader<T: Read>(reader: &mut T, count: usize) -> Chars {
        let mut data = Vec::with_capacity(count as usize);
        let mut i = 0;
        loop {
            if i >= count {
                break;
            }
            let char = read_char(reader);
            match char {
                Char::CharCode(_) => {
                    i += 1;
                }
                _ => {
                    i += 8;
                }
            };
            data.push(char);
        }

        Chars { data }
    }

    /// 컨트롤 개수를 반환
    pub fn extend_control_count(&self) -> usize {
        self.data.iter().fold(0, |result, char| match char {
            Char::ExtendedControl(_, _) => result + 1,
            _ => result,
        })
    }

    pub fn to_string(&self) -> String {
        // TODO: (@hahnlee) 테이블 어떻게 하는지 알아보기
        let mut buf: Vec<u16> = Vec::new();

        for char in &self.data {
            // TODO: (@hahnlee) CharControl 확인하기
            if let Char::CharCode(char_code) = char {
                buf.push(*char_code);
            }
        }

        String::from_utf16(&buf).unwrap()
    }
}
