use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[repr(u16)]
#[derive(Debug, Clone, PartialEq)]
pub enum CharControls {
    Unusable = 0,
    LineBreak = 10,
    ParaBreak = 13,
    Hyphen = 24,
    Reserved1 = 25,
    Reserved2 = 26,
    Reserved3 = 27,
    Reserved4 = 28,
    Reserved5 = 29,
    /// 묶음 빈칸
    KeepWordSpace = 30,
    /// 고정폭 빈칸
    FixedWidthSpace = 31,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Char {
    CharCode(u16),
    CharControl(CharControls),
    InlineControl(u16, [u8; 12]),
    ExtendedControl(u16, [u8; 12]),
}

pub fn match_char_control(input: u16) -> Option<CharControls> {
    match input {
        0 => Some(CharControls::Unusable),
        10 => Some(CharControls::LineBreak),
        13 => Some(CharControls::ParaBreak),
        24 => Some(CharControls::Hyphen),
        25 => Some(CharControls::Reserved1),
        26 => Some(CharControls::Reserved2),
        27 => Some(CharControls::Reserved3),
        28 => Some(CharControls::Reserved4),
        29 => Some(CharControls::Reserved5),
        30 => Some(CharControls::KeepWordSpace),
        31 => Some(CharControls::FixedWidthSpace),
        _ => None,
    }
}

pub fn read_char<T: Read>(reader: &mut T) -> Char {
    let code = reader.read_u16::<LittleEndian>().unwrap();

    if code > 31 {
        return Char::CharCode(code);
    }

    let char_control = match_char_control(code);
    if char_control.is_some() {
        return Char::CharControl(char_control.unwrap());
    }

    let mut buf = [0u8; 12];
    reader.read_exact(&mut buf).unwrap();

    let other = reader.read_u16::<LittleEndian>().unwrap();
    if code != other {
        // TODO: (@hahnlee) 파싱 수정하기
        panic!("잘못된 자료형입니다");
    }

    let ext = match code {
        1 | 2 | 3 | 11 | 12 | 14 | 15 | 16 | 17 | 18 | 21 | 22 | 23 => {
            Some(Char::ExtendedControl(code, buf))
        }
        _ => None,
    };

    if ext.is_some() {
        return ext.unwrap();
    }

    return Char::InlineControl(code, buf);
}
