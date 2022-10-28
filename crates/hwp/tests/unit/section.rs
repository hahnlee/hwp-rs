use hwp::hwp::paragraph::control::section::{format_number_shape, NumberShape};

#[test]
fn test_circled_digit() {
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 1), "①");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 2), "②");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 3), "③");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 4), "④");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 5), "⑤");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 6), "⑥");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 7), "⑦");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 8), "⑧");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 9), "⑨");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 10), "⑩");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 11), "⑪");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 12), "⑫");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 13), "⑬");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 14), "⑭");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 15), "⑮");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 16), "⑯");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 17), "⑰");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 18), "⑱");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 19), "⑲");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 20), "⑳");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 21), "①");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 22), "②");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 30), "⑩");
    assert_eq!(format_number_shape(&NumberShape::CircledDigit, 40), "⑳");
}

#[test]
fn test_latin_capital() {
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 1), "A");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 2), "B");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 3), "C");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 4), "D");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 5), "E");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 6), "F");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 7), "G");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 8), "H");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 9), "I");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 10), "J");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 11), "K");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 12), "L");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 13), "M");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 14), "N");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 15), "O");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 16), "P");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 17), "Q");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 18), "R");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 19), "S");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 20), "T");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 21), "U");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 22), "V");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 23), "W");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 24), "X");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 25), "Y");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 26), "Z");
    assert_eq!(format_number_shape(&NumberShape::LatinCapital, 27), "A");
}

#[test]
fn test_latin_small() {
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 1), "a");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 2), "b");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 3), "c");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 4), "d");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 5), "e");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 6), "f");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 7), "g");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 8), "h");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 9), "i");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 10), "j");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 11), "k");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 12), "l");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 13), "m");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 14), "n");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 15), "o");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 16), "p");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 17), "q");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 18), "r");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 19), "s");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 20), "t");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 21), "u");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 22), "v");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 23), "w");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 24), "x");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 25), "y");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 26), "z");
    assert_eq!(format_number_shape(&NumberShape::LatinSmall, 27), "a");
}

#[test]
fn test_circled_latin_capital() {
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 1),
        "Ⓐ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 2),
        "Ⓑ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 3),
        "Ⓒ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 4),
        "Ⓓ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 5),
        "Ⓔ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 6),
        "Ⓕ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 7),
        "Ⓖ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 8),
        "Ⓗ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 9),
        "Ⓘ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 10),
        "Ⓙ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 11),
        "Ⓚ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 12),
        "Ⓛ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 13),
        "Ⓜ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 14),
        "Ⓝ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 15),
        "Ⓞ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 16),
        "Ⓟ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 17),
        "Ⓠ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 18),
        "Ⓡ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 19),
        "Ⓢ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 20),
        "Ⓣ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 21),
        "Ⓤ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 22),
        "Ⓥ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 23),
        "Ⓦ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 24),
        "Ⓧ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 25),
        "Ⓨ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 26),
        "Ⓩ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinCapital, 27),
        "Ⓐ"
    );
}

#[test]
fn test_circled_latin_small() {
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 1), "ⓐ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 2), "ⓑ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 3), "ⓒ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 4), "ⓓ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 5), "ⓔ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 6), "ⓕ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 7), "ⓖ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 8), "ⓗ");
    assert_eq!(format_number_shape(&NumberShape::CircledLatinSmall, 9), "ⓘ");
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 10),
        "ⓙ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 11),
        "ⓚ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 12),
        "ⓛ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 13),
        "ⓜ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 14),
        "ⓝ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 15),
        "ⓞ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 16),
        "ⓟ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 17),
        "ⓠ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 18),
        "ⓡ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 19),
        "ⓢ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 20),
        "ⓣ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 21),
        "ⓤ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 22),
        "ⓥ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 23),
        "ⓦ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 24),
        "ⓧ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 25),
        "ⓨ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 26),
        "ⓩ"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledLatinSmall, 27),
        "ⓐ"
    );
}

#[test]
fn test_hangul_syllable() {
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 1), "가");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 2), "나");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 3), "다");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 4), "라");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 5), "마");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 6), "바");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 7), "사");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 8), "아");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 9), "자");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 10), "차");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 11), "카");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 12), "타");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 13), "파");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 14), "하");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 15), "거");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 16), "너");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 17), "더");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 18), "러");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 19), "머");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 20), "버");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 21), "서");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 22), "어");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 23), "저");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 24), "처");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 25), "커");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 26), "터");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 27), "퍼");
    assert_eq!(format_number_shape(&NumberShape::HangulSyllable, 28), "허");
}

#[test]
fn test_circled_hangul_syllable() {
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 1),
        "㉮"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 2),
        "㉯"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 3),
        "㉰"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 4),
        "㉱"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 5),
        "㉲"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 6),
        "㉳"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 7),
        "㉴"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 8),
        "㉵"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 9),
        "㉶"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 10),
        "㉷"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 11),
        "㉸"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 12),
        "㉹"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 13),
        "㉺"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 14),
        "㉻"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 15),
        "㉮"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 16),
        "㉯"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 17),
        "㉰"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 18),
        "㉱"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 19),
        "㉲"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 20),
        "㉳"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 21),
        "㉴"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 22),
        "㉵"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 23),
        "㉶"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 24),
        "㉷"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 25),
        "㉸"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 26),
        "㉹"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 27),
        "㉺"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulSyllable, 28),
        "㉻"
    );
}

#[test]
fn test_hangul_jamo() {
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 1), "ᄀ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 2), "ᄂ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 3), "ᄃ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 4), "ᄅ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 5), "ᄆ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 6), "ᄇ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 7), "ᄉ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 8), "ᄋ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 9), "ᄌ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 10), "ᄎ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 11), "ᄏ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 12), "ᄐ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 13), "ᄑ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 14), "ᄒ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 15), "ᄀ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 16), "ᄂ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 17), "ᄃ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 18), "ᄅ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 19), "ᄆ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 20), "ᄇ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 21), "ᄉ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 22), "ᄋ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 23), "ᄌ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 24), "ᄎ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 25), "ᄏ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 26), "ᄐ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 27), "ᄑ");
    assert_eq!(format_number_shape(&NumberShape::HangulJamo, 28), "ᄒ");
}

#[test]
fn test_circled_hangul_jamo() {
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 1),
        "㉠"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 2),
        "㉡"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 3),
        "㉢"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 4),
        "㉣"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 5),
        "㉤"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 6),
        "㉥"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 7),
        "㉦"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 8),
        "㉧"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 9),
        "㉨"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 10),
        "㉩"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 11),
        "㉪"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 12),
        "㉫"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 13),
        "㉬"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 14),
        "㉭"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 15),
        "㉠"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 16),
        "㉡"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 17),
        "㉢"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 18),
        "㉣"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 19),
        "㉤"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 20),
        "㉥"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 21),
        "㉦"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 22),
        "㉧"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 23),
        "㉨"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 24),
        "㉩"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 25),
        "㉪"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 26),
        "㉫"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 27),
        "㉬"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledHangulJamo, 28),
        "㉭"
    );
}

#[test]
fn test_hangul_phonetic() {
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 1), "일");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 2), "이");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 3), "삼");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 4), "사");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 5), "오");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 6), "육");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 7), "칠");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 8), "팔");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 9), "구");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 10), "십");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 11), "일");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 12), "이");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 13), "삼");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 14), "사");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 15), "오");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 16), "육");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 17), "칠");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 18), "팔");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 19), "구");
    assert_eq!(format_number_shape(&NumberShape::HangulPhonetic, 20), "십");
}

#[test]
fn test_ideograph() {
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 1), "一");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 2), "二");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 3), "三");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 4), "四");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 5), "五");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 6), "六");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 7), "七");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 8), "八");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 9), "九");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 10), "十");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 11), "一");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 12), "二");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 13), "三");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 14), "四");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 15), "五");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 16), "六");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 17), "七");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 18), "八");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 19), "九");
    assert_eq!(format_number_shape(&NumberShape::Ideograph, 20), "十");
}

#[test]
fn test_circled_ideograph() {
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 1), "㊀");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 2), "㊁");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 3), "㊂");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 4), "㊃");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 5), "㊄");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 6), "㊅");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 7), "㊆");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 8), "㊇");
    assert_eq!(format_number_shape(&NumberShape::CircledIdeograph, 9), "㊈");
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 10),
        "㊉"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 11),
        "㊀"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 12),
        "㊁"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 13),
        "㊂"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 14),
        "㊃"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 15),
        "㊄"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 16),
        "㊅"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 17),
        "㊆"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 18),
        "㊇"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 19),
        "㊈"
    );
    assert_eq!(
        format_number_shape(&NumberShape::CircledIdeograph, 20),
        "㊉"
    );
}

#[test]
fn test_decagon_circle() {
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 1), "갑");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 2), "을");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 3), "병");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 4), "정");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 5), "무");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 6), "기");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 7), "경");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 8), "신");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 9), "임");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 10), "계");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 11), "갑");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 12), "을");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 13), "병");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 14), "정");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 15), "무");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 16), "기");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 17), "경");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 18), "신");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 19), "임");
    assert_eq!(format_number_shape(&NumberShape::DecagonCircle, 20), "계");
}

#[test]
fn test_decagon_circle_hanja() {
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 1),
        "甲"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 2),
        "乙"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 3),
        "丙"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 4),
        "丁"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 5),
        "戊"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 6),
        "己"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 7),
        "庚"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 8),
        "辛"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 9),
        "壬"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 10),
        "癸"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 11),
        "甲"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 12),
        "乙"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 13),
        "丙"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 14),
        "丁"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 15),
        "戊"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 16),
        "己"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 17),
        "庚"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 18),
        "辛"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 19),
        "壬"
    );
    assert_eq!(
        format_number_shape(&NumberShape::DecagonCircleHanja, 20),
        "癸"
    );
}
